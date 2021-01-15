#[derive(Debug, Clone)]
pub struct Token {
    token_type: String,
    value: String,
}

pub

/*
helloWorld := fn() {
    println("Hello world");
}

helloWorld();
*/

#[derive(Debug, Clone)]
pub enum Tokens {
    L_PARENS,
    R_PARENS,
    R_BRACE,
    L_BRACE,

    STRING,
    INT,

    FUNCTION,

    ASSIGN,

    PRINTLN,
}

pub fn lex(file: String) {
    println("{}", file);
}
