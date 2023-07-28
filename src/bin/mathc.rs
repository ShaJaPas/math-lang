use std::{
    fs::File,
    io::{Read, Write},
};

use clap::Parser;
use math_lang::{vm::bytecode::Bytecode, Compile};

#[derive(Parser)]
/// Math-lang IL compiler
struct Cli {
    /// path to file with Math lang source code
    #[arg(long, group = "input", short = 'p')]
    file_path: Option<String>,

    /// Math lang source code
    #[arg(long, group = "input", short = 'd')]
    data: Option<String>,

    /// output file name
    #[arg(long, short = 'o')]
    output_name: String,
}

fn main() {
    let args = Cli::parse();
    if let Some(path) = args.file_path {
        let file = File::open(path);
        match file {
            Ok(mut f) => {
                let mut source = String::new();
                f.read_to_string(&mut source).unwrap();
                let bytecode = Bytecode::from_source(&source);
                match bytecode {
                    Ok(bytecode) => {
                        let bytes = bytecode.compile();
                        let mut file = File::create(args.output_name).unwrap();
                        file.write_all(&bytes).unwrap();
                    }
                    Err(e) => println!("{e}"),
                }
            }
            Err(e) => println!("Cannot read a file: {e}"),
        }
    } else if let Some(source) = args.data {
        let bytecode = Bytecode::from_source(&source);
        match bytecode {
            Ok(bytecode) => {
                let bytes = bytecode.compile();
                let mut file = File::create(args.output_name).unwrap();
                file.write_all(&bytes).unwrap();
            }
            Err(e) => println!("{e}"),
        }
    } else {
        println!("No data received, use of the input options!");
    }
}
