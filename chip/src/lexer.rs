#[derive(Debug, Clone)]
pub struct Token {
    token_type: String,
    value: String,
}


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

    INITIALIZE,
    ASSIGN,

    PRINTLN,
}

pub fn lex(file: String) {

    for current_char in file.chars()  {
        print!("{}",current_char);
    }
}
