#[derive(Debug, Clone)]
pub struct Token {
    token_type: String,
    value: String,
}

pub enum Tokens {
    LParens,
    RParens,
    RBrace,
    LBrace,
}

fn main() {
    println!("Hello World!");
}

pub fn lex(file: String) {
    println!("{}", file);
}
