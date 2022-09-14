use std::path::PathBuf;

use amd_dx_gsa::amd_isa_devices::{ASIC_COUNT, ASIC_INFO};
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

    let lib = Atidxx64::try_load_lib_from(args.dll_path).expect("no library found");
    let symbols = Atidxx64::try_load_symbols(&lib).expect("no matching symbols");

    let shader_asm = std::fs::read(args.shader_path).expect("couldn't read shader DXASM");

    let asic = ASIC_INFO[ASIC_COUNT - 1];

    let disasm = symbols
        .inspect_compiled_shader(
            asic,
            amd_dx_gsa::AmdDxGsaShaderSource::DxAsmBinary(shader_asm.as_slice()),
            vec![],
            |compiled_elf| {
                let obj_file = object::File::parse(compiled_elf).expect("no valid ELF");
                let disasm_section = obj_file
                    .section_by_name(".amdil_disassembly")
                    .expect("no .amdil_disassembly section");
                let disasm = String::from_utf8(disasm_section.data().unwrap().to_vec())
                    .expect("amdil disassembly not valid UTF_8?");

                return disasm;
            },
        )
        .expect("couldn't compile shader");

    print!("{}", disasm);
}
