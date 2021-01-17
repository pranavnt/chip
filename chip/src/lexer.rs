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


// parens/braces/backets
let L_PARENS = "L_PARENS";
let R_PARENS = "R_PARENS"; 
let L_BRACE = "L_BRACE";
let R_BRACE = "R_BRACE";
let L_SBRACKET = "L_BRACKET";
let R_BRACKET = "R_BRACKET"; 

// comparison operators
let EQUAL_OPERATOR = "EQUAL_OPERATOR";
let NOT_EQUAL_OPERATOR = "NOT_EQUAL_OPERATOR";
let GREATER_THAN_OPERATOR = "GREATER_THAN_OPERATOR";
let LESS_THAN_OPERATOR = "LESS_THAN_OPERATOR";
let GREATER_THAN_EQUAL_OPERATOR = "GREATER_THAN_EQUAL_OPERATOR";
let LESS_THAN_EQUAL_OPERATOR = "LESS_THAN_EQUAL_OPERATOR";



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

struct lexer {
    text,
    pos,
    current_char,
}

impl Lexer for lexer {
    
}

pub fn lex(file: String) {

    for current_char in file.chars()  {
        if current_char.is_numeric() {
            println!("{}",current_char);
        }
        print!("{}",current_char);
        
    }
}
