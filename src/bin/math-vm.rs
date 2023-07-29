use std::{fs::File, io::Read, time::Instant};

use clap::Parser;
use math_lang::vm::{StackVM, VM};

#[derive(Parser)]
/// Math-lang IL compiler
struct Cli {
    /// path to file with Math lang IL
    #[arg(long, short = 'p')]
    file_path: String,
}

fn main() {
    let args = Cli::parse();
    let file = File::open(args.file_path);
    match file {
        Ok(mut f) => {
            let mut buf = Vec::new();
            f.read_to_end(&mut buf).unwrap();
            let mut vm = VM::new(&buf);
            let mut ip = 0;
            let instant = Instant::now();
            loop {
                match vm.exec_next() {
                    Ok(flag) => {
                        if !flag {
                            let elapsed = instant.elapsed();
                            println!(
                                "Got result \"{}\" in {} executed commands\nExec time: {:?}",
                                vm.pop(),
                                ip,
                                elapsed
                            );
                            break;
                        } else {
                            ip += 1;
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => println!("Cannot read a file: {e}"),
    }
}
