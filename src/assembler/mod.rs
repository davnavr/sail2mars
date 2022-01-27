use sailar_get::loader;
use std::collections::hash_map;
use std::io::Write;

mod error;
mod output;

pub use error::{Error, Result};

use output::{Instruction as Instr, Label, MarsServiceNumber, Register as Reg};

#[derive(Debug)]
struct Function<'a> {
    instructions: Vec<Instr<'a>>,
}

#[derive(Debug)]
struct Lookup<'a> {
    functions: hash_map::HashMap<loader::FunctionSymbol<'a>, &'a loader::Function<'a>>,
}

fn build_functions<'a>(program: &'a loader::Module<'a>) -> Result<Lookup<'a>> {
    let mut functions = Vec::new();
    functions.push(
        program
            .entry_point()?
            .ok_or(Error::MissingEntryPointFunction)?,
    );

    let mut lookup = Lookup {
        functions: hash_map::HashMap::new(),
    };

    //while let Some(function) = functions.first() {}

    Ok(lookup)
}

const EXIT_CODE_MESSAGE: &str = r"exit_code_message";

const ENTRY_POINT_EXIT: [Instr<'_>; 9] = [
    // Exit code is assumed to be stored in the $v0 register
    Instr::Move(Reg::S0, Reg::V0),
    // Print message
    Instr::Li(Reg::V0, MarsServiceNumber::PrintString as i32),
    Instr::La(Reg::A0, Label::Borrowed(EXIT_CODE_MESSAGE)),
    Instr::Syscall,
    // Print exit code
    Instr::Li(Reg::V0, MarsServiceNumber::PrintInteger as i32),
    Instr::Move(Reg::A0, Reg::V0),
    Instr::Syscall,
    // Exit the program
    Instr::Li(Reg::V0, MarsServiceNumber::Exit as i32),
    Instr::Syscall,
];

pub fn write_program<'a, W: Write>(output: &mut W, program: &'a loader::Module<'a>) -> Result<()> {
    let lookup = build_functions(program)?;

    let mut out = std::io::BufWriter::new(output);

    writeln!(out, ".data")?;
    writeln!(out, "{}: .asciiz \"Exited with code \"", EXIT_CODE_MESSAGE)?;

    writeln!(out, ".text")?;
    writeln!(out, "main:")?;
    for instruction in ENTRY_POINT_EXIT {
        writeln!(out, "{}", instruction)?;
    }
    out.flush()?;

    Ok(())
}
