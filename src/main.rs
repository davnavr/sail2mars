use clap::Parser;
use sailar_get::loader;

mod assembler;

/// Assembles SAILAR modules into MIPS32 assembly.
#[derive(Parser, Debug)]
#[clap(about)]
struct Arguments {
    /// The module containing the entry point to assemble.
    #[clap(short, long)]
    program: std::path::PathBuf,
    /// Path to the output assembly file.
    #[clap(short, long)]
    output: Option<std::path::PathBuf>,
}

const MIPS_32_POINTER_SIZE: u8 = 4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Arguments::parse();

    let mut loader = None;
    let (loader, program) = loader::Loader::initialize(
        &mut loader,
        MIPS_32_POINTER_SIZE,
        sailar::parser::parse_module(&mut std::fs::File::open(&arguments.program)?)?,
    );

    let mut output = std::fs::File::create(
        arguments
            .output
            .unwrap_or_else(|| arguments.program.with_extension("asm")),
    )?;

    assembler::write_program(&mut output, program)?;
    Ok(())
}
