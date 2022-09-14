use std::{
    convert::TryInto,
    ffi::{c_void, OsStr},
};

pub mod amd_isa_devices;
pub mod dxbc;
mod interop;
pub use interop::AmdDxGsaCompileOption;
pub use interop::AmdDxGsaCompileOptionEnum;

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
    OperationAborted,
    AccessDenied,
    UnspecifiedFailure,
    InvalidArg,
    OutOfMemory,
    InvalidPointer,
    Other { result: interop::HRESULT },
}

/// Struct providing a safe interface to shader compilation via the atidxx64 DLL
///
/// Intended usage:
/// ```no_run
/// use amd_dx_gsa::Atidxx64;
/// let lib = Atidxx64::try_load_lib().expect("no library found");
/// lib.inspect_compiled_shader(...);
/// ```
pub struct Atidxx64(interop::atidxx64);
impl Atidxx64 {
    pub unsafe fn try_load_lib_from<P: AsRef<OsStr>>(
        filename: P,
    ) -> Result<Self, libloading::Error> {
        Ok(Self(interop::atidxx64::new(filename)?))
    }
    pub unsafe fn try_load_lib() -> Result<Self, libloading::Error> {
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
        options: Vec<AmdDxGsaCompileOption>,

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
            let result = self.0.AmdDxGsaCompileShader(&compile_in, &mut compile_out);
            if result != 0
                || compile_out.pShaderBinary == std::ptr::null_mut()
                || compile_out.shaderBinarySize < 16
            {
                // Based on https://docs.microsoft.com/en-us/windows/win32/seccrypto/common-hresult-values
                let err = match result as u32 {
                    0x80004004 => ShaderCompileError::OperationAborted,
                    0x80070005 => ShaderCompileError::AccessDenied,
                    0x80004005 => ShaderCompileError::UnspecifiedFailure,
                    0x80070057 => ShaderCompileError::InvalidArg,
                    0x80004003 => ShaderCompileError::InvalidPointer,
                    0x8007000E => ShaderCompileError::OutOfMemory,

                    _ => ShaderCompileError::Other { result },
                };
                return Err(err);
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
            self.0.AmdDxGsaFreeCompiledShader(compile_out.pShaderBinary);

            // Return the operation output
            Ok(out)
        }
    }
}
