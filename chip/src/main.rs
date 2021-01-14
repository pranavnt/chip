use std::env;
use std::fs;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let file_name = &args[1];

        let file = fs::read_to_string(file_name).expect("Couldn't read from file");

        lexer::lex(file)
    } else {
        println!("Too many arguments")
    }
}
