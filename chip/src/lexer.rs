/*
helloWorld := fn() {
    println("Hello world");
}

helloWorld();
*/

const L_PARENS: &str = "L_PARENS";
const R_PARENS: &str = "R_PARENS";
const L_BRACE: &str = "L_BRACE";
const R_BRACE: &str = "R_BRACE";
const L_SBRACKET: &str = "L_BRACKET";
const R_BRACKET: &str = "R_BRACKET";

// comparison operators
const EQUAL_OPERATOR: &str = "EQUAL_OPERATOR";
const NOT_EQUAL_OPERATOR: &str = "NOT_EQUAL_OPERATOR";
const GREATER_THAN_OPERATOR: &str = "GREATER_THAN_OPERATOR";
const LESS_THAN_OPERATOR: &str = "LESS_THAN_OPERATOR";
const GREATER_THAN_EQUAL_OPERATOR: &str = "GREATER_THAN_EQUAL_OPERATOR";
const LESS_THAN_EQUAL_OPERATOR: &str = "LESS_THAN_EQUAL_OPERATOR";

// logical operators
const AND_OPERATOR: &str = "AND_OPERATOR";
const OR_OPERATOR: &str = "OR_OPERATOR";
const NOT_OPERATOR: &str = "NOT_OPERATOR";

// assignment operators
const INITIALIZE_OPERATOR: &str = "INITIALIZE_OPERATOR";
const ASSIGNMENT_OPERATOR: &str = "ASSIGNMENT_OPERATOR";
const PLUS_EQUALS: &str = "PLUS_EQUALS";
const MINUS_EQUALS: &str = "MINUS_EQUALS";

// math operators
const ADDITION_OPERATOR: &str = "ADDITION_OPERATOR";
const SUBTRACTION_OPERATOR: &str = "SUBTRACTION_OPERATOR";
const MULTIPLICATION_OPERATOR: &str = "MULTIPLICATION_OPERATOR";
const DIVISION_OPERATOR: &str = "DIVISION_OPERATOR";
const MODULUS_OPERATOR: &str = "MODULUS_OPERATOR";

// conditionals
const IF: &str = "IF";
const ELSE: &str = "ELSE";

// loops
const FOR_LOOP: &str = "FOR_LOOP";
const WHILE_LOOP: &str = "WHILE_LOOP";

//
const FUNCTION: &str = "FUNCTION";
const RETURN: &str = "RETURN";

pub struct Token {
    token_type: String,
    value: String,
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub code: String,
    pub pos: i32,
    pub chars: Vec<char>,
    pub current_char: String,
}

impl Lexer {
    pub fn lex(&mut self) {
        // break up into lines
        let lines = self.code.split(";");
        self.chars = self.code.chars().collect();

        println!("{}", self.chars.len());

        while self.pos < (self.chars.len() as i32 - 1) {
            println!("{}",self.chars.get(self.pos as usize).unwrap());
            self.pos += 1;
            self.current_char = self.chars.get(self.pos as usize).unwrap().to_string();
            println!("{}",self.current_char);
        }
    }
    

    pub fn read_line() {
        println!("hi")
    }

    pub fn consume_char() {
        
    }

    pub fn get_char(&mut self, n: i32) {}

}
