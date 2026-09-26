use lexer::Lexer;
use std::fs::read_to_string;

mod lexer;
mod tokens;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match read_to_string("docs/hello.msqt") {
        Ok(program) => {
            println!("{}", program);
            let mut lexer = Lexer::new(&program);

            match lexer.tokenize() {
                Ok(tokens) => {
                    println!("{:?}", tokens);
                    Ok(())
                }
                Err(error) => {
                    eprintln!("Could not tokenize program! Error: {}", error);
                    Err(Box::new(error))
                }
            }
        }
        Err(error) => {
            eprintln!("Could not read program! Error: {}", error);
            Err(Box::new(error))
        }
    }
}
