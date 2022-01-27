use sailar_get::loader;
use std::cell::{Ref, RefCell};
use std::collections::hash_map;

mod error;
mod output;

pub use error::{Error, Result};

use output::{Instruction as Instr, Label, MarsServiceNumber, Register as Reg};

struct Function<'b> {
    label: &'b str,
    instructions: Vec<Instr<'b>>,
}

#[derive(Default)]
struct Lookup<'a, 'b> {
    labels: typed_arena::Arena<String>,
    functions: std::cell::RefCell<hash_map::HashMap<loader::FunctionSymbol<'a>, Function<'b>>>,
}

fn create_label(buffer: &mut String, label: &str) {
    use std::fmt::Write;
    for (index, c) in label.chars().enumerate() {
        if c.is_ascii_alphabetic() || c == '_' || (c.is_ascii_digit() && index > 0) {
            buffer.push(c);
        } else {
            write!(buffer, "u{:#04}", u32::from(c));
        }
    }
}

fn create_function_label<'a>(symbol: &loader::FunctionSymbol<'a>) -> String {
    use std::fmt::Write;
    let mut buffer = String::with_capacity(1 + symbol.module().name.len() + symbol.symbol().len());
    create_label(&mut buffer, &symbol.module().name);
    buffer.push('_');
    for digit in symbol.module().version.0.iter().copied() {
        write!(&mut buffer, "{}_", digit);
    }
    create_label(&mut buffer, &symbol.symbol());
    buffer
}

fn generate_instructions<'a>(
    function: &'a loader::Function<'a>,
    unassembled_functions: &mut Vec<&'a loader::Function<'a>>,
) -> Result<()> {
    Ok(())
}

fn build_functions<'a, 'b>(
    lookup: &'b Lookup<'a, 'b>,
    program: &'a loader::Module<'a>,
) -> Result<&'a loader::Function<'a>> {
    let mut remaining_functions = Vec::new();
    let entry_point = program
        .entry_point()?
        .ok_or(Error::MissingEntryPointFunction)?;

    remaining_functions.push(entry_point);

    while let Some(function) = remaining_functions.pop() {
        let symbol = function.full_symbol()?;
        let mut entry = Function {
            label: lookup.labels.alloc(create_function_label(&symbol)),
            instructions: Vec::new(),
        };

        generate_instructions(function, &mut remaining_functions)?;

        match lookup.functions.borrow_mut().entry(symbol) {
            hash_map::Entry::Vacant(vacant) => {
                vacant.insert(entry);
            }
            hash_map::Entry::Occupied(_) => unreachable!(),
        }
    }

    Ok(entry_point)
}

/// The register containing the exit code once the entry point function returns.
const EXIT_CODE_REGISTER: Reg = Reg::V0;

const EXIT_CODE_MESSAGE: &str = r"exit_code_message";

const ENTRY_POINT_EXIT: [Instr<'_>; 9] = [
    // Exit code is assumed to be stored in the $v0 register
    Instr::Move(Reg::S0, EXIT_CODE_REGISTER),
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

pub fn write_program<'a, W: std::io::Write>(
    output: &mut W,
    program: &'a loader::Module<'a>,
) -> Result<()> {
    use std::io::Write;
    let lookup = Lookup::default();
    build_functions(&lookup, program)?;
    let mut out = std::io::BufWriter::new(output);
    writeln!(out, ".data")?;
    writeln!(out, "{}: .asciiz \"Exited with code \"", EXIT_CODE_MESSAGE)?;
    writeln!(out, ".text")?;
    // TODO: Jump to entry point function
    writeln!(out, "main:")?;
    for instruction in ENTRY_POINT_EXIT {
        writeln!(out, "{}", instruction)?;
    }
    for function in lookup.functions.take().values() {
        writeln!(out, "{}:", function.label)?;
        for instruction in &function.instructions {
            writeln!(out, "{}", instruction)?;
        }
    }
    out.flush()?;
    Ok(())
}
