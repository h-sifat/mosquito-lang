use std::fs::read_to_string;

fn main() {
    match read_to_string("docs/hello.msqt") {
        Ok(program) => {
            println!("{}", program);
        }
        Err(error) => {
            eprintln!("Could not read program! Error: {}", error);
            std::process::exit(1);
        }
    }
}
