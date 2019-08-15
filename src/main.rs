extern crate clap;

use std::fs;
use std::io::Read;
use clap::{Arg, App};

fn main() {
    let matches = App::new("rbf")
        .version("0.1")
        .author("Olli Johnson")
        .about("Rusty brainfuck")
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .value_name("FILE")
             .help("File to use as input")
             .takes_value(true)
             .required(true))
        .get_matches();
    let input_file = matches.value_of("file").unwrap();
    let contents = fs::read_to_string(input_file)
        .expect("Couldn't read the input file");

    let mut cells = [0u8; 30000];
    let mut pointer = 0;

    let instructions: Vec<_> = contents.chars().collect();
    let mut inst_index: usize = 0;

    let mut loop_stack: Vec<usize> = Vec::new();

    while inst_index < instructions.len() {
        let ch = instructions[inst_index];

        if ch == '+' {
            // Rust doesn't allo overflow so explicitly implement it
            if cells[pointer] == 255 {
                cells[pointer] = 0;
            } else {
                cells[pointer] += 1;
            }
        } else if ch == '-' {
            // Rust doesn't allow overflow so explicitly implement it
            if cells[pointer] == 0 {
                cells[pointer] = 255;
            } else {
                cells[pointer] -= 1;
            }
        } else if ch == '>' {
            pointer += 1;
        } else if ch == '<' {
            pointer -= 1;
        } else if ch == '.' {
            print!("{}", cells[pointer]);
        } else if ch == ',' {
            match std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as u8) {
                Some(input) => cells[pointer] = input,
                None => panic!("Invalid input character recieved on stdin")
            }
        } else if ch == '[' {
            // If this is the first entry, push this point onto the loop stack
            if loop_stack.len() == 0 || loop_stack[loop_stack.len() - 1] != inst_index {
                loop_stack.push(inst_index);
            }
        } else if ch == ']' {
            // Continue loop if current cell is non-zero
            // otherwise pop the loop from the stack and continue
            if cells[pointer] > 0 {
                inst_index = loop_stack[loop_stack.len() - 1];
            } else {
                loop_stack.pop();
            }
        }

        inst_index += 1;
    }

    println!();
    println!("Done");
}
