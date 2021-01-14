#[derive(Debug, Clone)]
pub struct Token {
    token_type: String,
    value: String,
}

#[derive(Debug, Clone)]
pub enum Tokens {
    LParens,
    RParens,
    RBrace,
    LBrace,

    
}

pub fn lex(file: String) {
    println!("{}", file);
}
