use std::env;
use std::fs;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};

use rust_brainfuck::compiler::Compiler;
use rust_brainfuck::machine::Machine;

fn main() {
    let file_name = match env::args().nth(1) {
        None => {
            println!("Usage: rust-brainfuck <filename>");
            return;
        }
        Some(file_name) => file_name,
    };

    let code = match fs::read_to_string(file_name) {
        Ok(code) => code,
        Err(e) => {
            println!("error reading file: {}", e);
            return;
        }
    };

    let mut compiler = Compiler::new(code);
    match compiler.compile() {
        Ok(..) => {},
        Err(e) => {
            println!("error compiling file: {}", e);
            return;
        }
    }

    let mut reader: BufReader<Box<dyn Read>> = BufReader::new(Box::new(stdin()));
    let mut writer: BufWriter<Box<dyn Write>> = BufWriter::new(Box::new(stdout()));

    let mut machine = Machine::new(compiler.instructions, &mut reader, &mut writer);
    machine.execute();
}
