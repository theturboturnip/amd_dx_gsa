use libloading::os::windows::{Library, Symbol};
use std::{
    convert::TryInto,
    ffi::{c_void, OsStr},
};

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

/// Struct holding symbols for compiling shaders and freeing the results
///
/// Intended usage:
/// ```no_run
/// use amd_dx_gsa::Atidxx64;
/// let lib = Atidxx64::try_load_lib().expect("no library found");
/// let symbols = Atidxx64::try_load_symbols(lib.as_ref()).expect("no matching symbols");
/// ```
///
/// TODO make compile_func and free_func use the types of the functions from libloading
pub struct Atidxx64<'lib> {
    lib: &'lib Library,
    compile_func: Symbol<
        unsafe extern "C" fn(
            *const interop::AmdDxGsaCompileShaderInput,
            *mut interop::AmdDxGsaCompileShaderOutput,
        ) -> interop::HRESULT,
    >,
    free_func: Symbol<unsafe extern "C" fn(*const c_void) -> c_void>,
}
impl<'lib> Atidxx64<'lib> {
    pub fn try_load_symbols(lib: &'lib Library) -> Result<Self, libloading::Error> {
        unsafe {
            Ok(Self {
                lib,
                compile_func: lib.get(b"AmdDxGsaCompileShader\0")?,
                free_func: lib.get(b"AmdDxGsaFreeCompiledShader\0")?,
            })
        }
    }
    pub fn try_load_lib_from<P: AsRef<OsStr>>(
        filename: P,
    ) -> Result<Box<Library>, libloading::Error> {
        unsafe { Ok(Box::new(Library::new(filename)?)) }
    }
    pub fn try_load_lib() -> Result<Box<Library>, libloading::Error> {
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
        shader: AmdDxGsaShaderSource,
        options: Vec<interop::AmdDxGsaCompileOption>,

        operation: fn(&[u8]) -> T,
    ) -> Result<T, &str> {
        let (inputType, shaderByteCode) = shader.to_interop();
        unsafe {
            let compile_in = interop::AmdDxGsaCompileShaderInput {
                chipFamily: todo!(),
                chipRevision: todo!(),

                pShaderByteCode: shaderByteCode.as_ptr() as *const c_void,
                byteCodeLength: shaderByteCode
                    .len()
                    .try_into()
                    .expect("shader length doesn't fit in u64"),
                inputType,

                pCompileOptions: options.as_ptr(),
                numCompileOptions: options
                    .len()
                    .try_into()
                    .expect("num compile options doesn't fit in u32"),

                reserved: [0; 6],
            };

            let compile_out = std::mem::MaybeUninit::zeroed();
            let result = (*self.compile_func)(&compile_in, compile_out.as_mut_ptr());
            let compile_out = compile_out.assume_init();
            if result != 0
                || compile_out.pShaderBinary == std::ptr::null_mut()
                || compile_out.shaderBinarySize < 16
            {
                panic!("failed to compile shader");
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
