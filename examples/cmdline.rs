use std::path::PathBuf;

use amd_dx_gsa::{amd_isa_devices::ASIC_INFO, dxbc::get_shader_bytecode};
use clap::Parser;
use object::{Object, ObjectSection};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// DLL path
    #[clap(short, long, value_parser, default_value = "assets/atidxx64.dll")]
    dll_path: PathBuf,

    /// Shader path
    #[clap(short, long, value_parser, default_value = "assets/example.fxc")]
    shader_path: PathBuf,
}

fn main() {
    use amd_dx_gsa::Atidxx64;
    let args = Args::parse();

    let lib = unsafe { Atidxx64::try_load_lib_from(args.dll_path).expect("no library found") };

    let shader_dxbc = std::fs::read(args.shader_path).expect("couldn't read shader DXBC");
    let (_, shader_bytecode) =
        get_shader_bytecode(shader_dxbc.as_slice()).expect("couldn't extract bytecode from DXBC");

    for asic in ASIC_INFO {
        println!("\nATTEMPT: {}", asic.name);
        let disasm = lib.inspect_compiled_shader(
            asic,
            amd_dx_gsa::AmdDxGsaShaderSource::DxAsmBinary(shader_bytecode),
            vec![],
            |compiled_elf| {
                let obj_file = object::File::parse(compiled_elf).expect("no valid ELF");
                let amdil_disasm_section = obj_file
                    .section_by_name(".amdil_disassembly")
                    .expect("no .amdil_disassembly section");
                let amdil_disasm = String::from_utf8(amdil_disasm_section.data().unwrap().to_vec())
                    .expect("amdil disassembly not valid UTF_8?");

                let disasm_section = obj_file
                    .section_by_name(".disassembly")
                    .expect("no .disassembly section");
                let disasm = String::from_utf8(disasm_section.data().unwrap().to_vec())
                    .expect("disassembly not valid UTF_8?");

                return (amdil_disasm, disasm);
            },
        );
        match disasm {
            Ok((amdil_disasm, disasm)) => {
                println!("Success\nAMDIL: {}\nRAW: {}\n", amdil_disasm, disasm)
            }
            Err(shader_err) => println!("Fail\n{:?}", shader_err),
        };
    }
}
