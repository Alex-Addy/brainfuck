#[macro_use]
extern crate clap;

mod program;
use program::Program;

use std::io::{self, Read, BufRead, BufReader};
use std::fs::File;

fn main() {
    let matches = clap_app!(bfi =>
            (version: "0.1")
            (about: "A simple brainfuck interpreter.")
            (after_help: "If both PROGRAM and INPUT are to be read from the same source, '!' will be treated as a separator")
            (@arg PROGRAM: +required +takes_value "Sets the program source, '-' will read the program from stdin")
            (@arg INPUT: !required +takes_value "Input file, defaults to stdin")
            (@arg debug: -d "Enables the use of '#' as a debug print command")
        ).get_matches();

    let program_arg = matches.value_of("PROGRAM").unwrap();
    let input_arg = matches.value_of("INPUT").unwrap_or("-");
    let debug = matches.is_present("debug");

    let (program_raw, mut input) = get_program_and_input(&program_arg, &input_arg).unwrap();
    let mut prog = Program::new(Program::compile(&program_raw, debug));

    let mut output = io::stdout();
    match prog.run(&mut input, &mut output) {
        Ok(_) => {},
        Err(e) => print!("Error occurred during execution: {:?}", e),
    }
}

fn get_program_and_input(prog_arg: &str, input_arg: &str) -> io::Result<(String, Box<Read>)> {
    if prog_arg == input_arg {
        // read input until '!' for program, rest is for input
        let input = if input_arg == "-" {
            Box::new(io::stdin()) as Box<Read>
        } else {
            Box::new(File::open(prog_arg)?) as Box<Read>
        };
        let mut buf = Vec::new();
        let mut buffered = BufReader::new(input);
        buffered.read_until('!' as u8, &mut buf)?;

        Ok((String::from_utf8(buf).unwrap(), Box::new(buffered) as Box<Read>))
    } else {
        let mut prog = String::new();
        if prog_arg == "-" {
            io::stdin().read_to_string(&mut prog)?;
        } else {
            File::open(prog_arg)?.read_to_string(&mut prog)?;
        };

        let input = if input_arg == "-" {
            Box::new(io::stdin()) as Box<Read>
        } else {
            Box::new(File::open(input_arg)?) as Box<Read>
        };

        Ok((prog, input))
    }
}

