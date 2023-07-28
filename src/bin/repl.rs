use math_lang::{interpreter::Interpreter, Compile};
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    println!("Calculator prompt. Expressions are line evaluated.");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match Interpreter::from_source(&line) {
                Ok(res) => match res {
                    Ok(value) => println!("{value}"),
                    Err(e) => println!("{e}"),
                },
                Err(e) => println!("{}", e),
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
