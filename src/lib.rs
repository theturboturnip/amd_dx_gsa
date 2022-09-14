use libloading::os::windows::{Library, Symbol};
use std::{
    convert::TryInto,
    ffi::{c_void, OsStr},
};

pub mod amd_isa_devices;
pub mod dxbc;
mod interop;

/// Source code for a shader.
///
/// Maps to the `AmdDxGsaInputType` C enum.
pub enum AmdDxGsaShaderSource<'a> {
    DxAsmBinary(&'a [u8]),
    IlText(&'a [u8]),
}
impl<'a> AmdDxGsaShaderSource<'a> {
    fn to_interop(&self) -> (interop::AmdDxGsaInputType, &'a [u8]) {
        match &self {
            Self::DxAsmBinary(data) => (interop::AmdDxGsaInputType_GsaInputDxAsmBin, data),
            Self::IlText(data) => (interop::AmdDxGsaInputType_GsaInputIlText, data),
        }
    }
}

#[derive(Debug)]
pub enum ShaderCompileError {
    CompileFail { result: interop::HRESULT },
}

/// Struct holding symbols for compiling shaders and freeing the results
///
/// Intended usage:
/// ```no_run
/// use amd_dx_gsa::Atidxx64;
/// let lib = Atidxx64::try_load_lib().expect("no library found");
/// let symbols = Atidxx64::try_load_symbols(lib.as_ref()).expect("no matching symbols");
/// ```
pub struct Atidxx64 {
    compile_func: Symbol<
        unsafe extern "C" fn(
            *const interop::AmdDxGsaCompileShaderInput,
            *mut interop::AmdDxGsaCompileShaderOutput,
        ) -> interop::HRESULT,
    >,
    free_func: Symbol<unsafe extern "C" fn(*const c_void) -> c_void>,
}
impl Atidxx64 {
    pub fn try_load_symbols(lib: &Library) -> Result<Self, libloading::Error> {
        unsafe {
            Ok(Self {
                compile_func: lib.get(b"AmdDxGsaCompileShader\0")?,
                free_func: lib.get(b"AmdDxGsaFreeCompiledShader\0")?,
            })
        }
    }
    pub fn try_load_lib_from<P: AsRef<OsStr>>(filename: P) -> Result<Library, libloading::Error> {
        unsafe { Ok(Library::new(filename)?) }
    }
    pub fn try_load_lib() -> Result<Library, libloading::Error> {
        Self::try_load_lib_from("atidxx64.dll")
    }

    /// Takes shader source, target GPU, and compile options.
    ///
    /// Compilation results in an ELF file with the following sections:
    /// - .amdil             - IL binary
    /// - .amdil_disassembly - IL text string
    /// - .text              - ISA binary
    /// - .disassembly       - ISA text string
    /// - .stats             - AmdDxGsaCompileStats structure
    ///
    /// The `operation` function is run on a byte-slice containing that ELF file,
    /// and may return a value - that value will be returned from this function.
    pub fn inspect_compiled_shader<T>(
        &self,
        gpu: crate::amd_isa_devices::Asic,
        shader: AmdDxGsaShaderSource,
        options: Vec<interop::AmdDxGsaCompileOption>,

        operation: fn(&[u8]) -> T,
    ) -> Result<T, ShaderCompileError> {
        let (input_type, shader_bytecode) = shader.to_interop();
        unsafe {
            let compile_in = interop::AmdDxGsaCompileShaderInput {
                chipFamily: gpu.chipFamily as u32,
                chipRevision: gpu.chipRevision as u32,

                pShaderByteCode: shader_bytecode.as_ptr() as *const c_void,
                byteCodeLength: shader_bytecode
                    .len()
                    .try_into()
                    .expect("shader length doesn't fit in u64"),
                inputType: input_type,

                pCompileOptions: options.as_ptr(),
                numCompileOptions: options
                    .len()
                    .try_into()
                    .expect("num compile options doesn't fit in u32"),

                reserved: [0; 6],
            };

            let mut compile_out = interop::AmdDxGsaCompileShaderOutput {
                size: std::mem::size_of::<interop::AmdDxGsaCompileShaderOutput>() as u64,
                pShaderBinary: std::ptr::null_mut(),
                shaderBinarySize: 0,
            };
            let result = (*self.compile_func)(&compile_in, &mut compile_out);
            if result != 0
                || compile_out.pShaderBinary == std::ptr::null_mut()
                || compile_out.shaderBinarySize < 16
            {
                return Err(ShaderCompileError::CompileFail { result });
            }

            // Create slice pointing to data
            let slice = std::slice::from_raw_parts(
                compile_out.pShaderBinary as *const u8,
                compile_out
                    .shaderBinarySize
                    .try_into()
                    .expect("compiled shader size doesn't fit in usize"),
            );

            // Run an operation on the data
            let out = operation(slice);

            // Free the original data
            (*self.free_func)(compile_out.pShaderBinary);

            // Return the operation output
            Ok(out)
        }
    }
}
