use std::env;
use std::fs;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let file_name = &args[1];

        let file = fs::read_to_string(file_name).expect("Couldn't read from file");

        let mut newLexer = lexer::Lexer {
            code: file,
            current_char: "".to_string(),
            pos: 0,
        };

        newLexer.lex()
    } else {
        println!("Too many arguments")
    }
}
