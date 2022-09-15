use std::path::PathBuf;

use amd_dx_gsa::{amd_isa_devices::ASIC_INFO, dxbc::get_shader_bytecode};
use clap::Parser;
use object::{Object, ObjectSection};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// DLL path
    #[clap(long, value_parser, default_value = "assets/atidxx64.dll")]
    dll_path: PathBuf,

    /// Shader path
    #[clap(long, value_parser, default_value = "assets/example.fxc")]
    shader_path: PathBuf,

    /// ASIC name
    #[clap(long, value_parser, default_value = "RDNA2 (gfx1034)")]
    asic: String,

    /// section to dump
    #[clap(long, value_parser, default_value = ".text", possible_values(&[".text", ".amdil"]))]
    section: String,

    #[clap(value_parser)]
    output: PathBuf,
}

fn main() {
    use amd_dx_gsa::Atidxx64;
    let args = Args::parse();

    let lib = unsafe { Atidxx64::try_load_lib_from(args.dll_path).expect("no library found") };

    let shader_dxbc = std::fs::read(args.shader_path).expect("couldn't read shader DXBC");
    let (_, shader_bytecode) =
        get_shader_bytecode(shader_dxbc.as_slice()).expect("couldn't extract bytecode from DXBC");

    for asic in ASIC_INFO {
        if asic.name != args.asic {
            continue;
        }

        let disasm = lib.inspect_compiled_shader(
            asic,
            amd_dx_gsa::AmdDxGsaShaderSource::DxAsmBinary(shader_bytecode),
            vec![],
            |compiled_elf| {
                let obj_file = object::File::parse(compiled_elf).expect("no valid ELF");
                let section = obj_file
                    .section_by_name(args.section.as_str())
                    .expect("no matching section");
                section.data().unwrap().to_vec()
            },
        );
        match disasm {
            Ok(data) => {
                println!("Success");
                std::fs::write(args.output, data).expect("failed write");
            }
            Err(shader_err) => println!("Fail\n{:?}", shader_err),
        };
        return;
    }
    panic!("No matching ASIC");
}
