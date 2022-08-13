use std::env;
use std::fs;

mod lexer;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let file_name = &args[1];

        let file = fs::read_to_string(file_name).expect("Couldn't read from file");

        let mut lexer = lexer::Lexer::new(file);

        lexer.lex();

        for token in lexer.tokens {
            println!("{{\n\ttoken_type: {},\n\ttoken: {}\n}}", token.token_type, token.value);
        }

    } else if args.len() == 1 {
        println!("Welcome to Chip!!");
    }
}
