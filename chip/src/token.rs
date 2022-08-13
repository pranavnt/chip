pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

#[allow(non_camel_case_types)]
pub enum TokenType {
    IDENTIFIER,
    BOOLEAN,

    L_PARENS,
    R_PARENS,
    L_BRACE,
    R_BRACE,
    L_BRACKET,
    R_BRACKET,

    // comparison operators
    EQUAL_OPERATOR,
    NOT_EQUAL_OPERATOR,
    GREATER_THAN_OPERATOR,
    LESS_THAN_OPERATOR,

    // logical operators
    AND_OPERATOR,
    OR_OPERATOR,
    NOT_OPERATOR,

    // assignment operators
    INITIALIZE_OPERATOR,
    ASSIGNMENT_OPERATOR,

    // math operators
    ADDITION_OPERATOR,
    SUBTRACTION_OPERATOR,
    MULTIPLICATION_OPERATOR,
    DIVISION_OPERATOR,
    MODULUS_OPERATOR,

    // conditionals
    IF,
    ELSE,

    // loops
    FOR_LOOP,
    WHILE_LOOP,

    FUNCTION,
    RETURN,

    NUMBER,
    STRING,
    CHARACTER,

    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
            TokenType::BOOLEAN => write!(f, "BOOLEAN"),
            TokenType::L_PARENS => write!(f, "L_PARENS"),
            TokenType::R_PARENS => write!(f, "R_PARENS"),
            TokenType::L_BRACE => write!(f, "L_BRACE"),
            TokenType::R_BRACE => write!(f, "R_BRACE"),
            TokenType::L_BRACKET => write!(f, "L_BRACKET"),
            TokenType::R_BRACKET => write!(f, "R_BRACKET"),
            TokenType::EQUAL_OPERATOR => write!(f, "EQUAL_OPERATOR"),
            TokenType::NOT_EQUAL_OPERATOR => write!(f, "NOT_EQUAL_OPERATOR"),
            TokenType::GREATER_THAN_OPERATOR => write!(f, "GREATER_THAN_OPERATOR"),
            TokenType::LESS_THAN_OPERATOR => write!(f, "LESS_THAN_OPERATOR"),
            TokenType::AND_OPERATOR => write!(f, "AND_OPERATOR"),
            TokenType::OR_OPERATOR => write!(f, "OR_OPERATOR"),
            TokenType::NOT_OPERATOR => write!(f, "NOT_OPERATOR"),
            TokenType::INITIALIZE_OPERATOR => write!(f, "INITIALIZE_OPERATOR"),
            TokenType::ASSIGNMENT_OPERATOR => write!(f, "ASSIGNMENT_OPERATOR"),
            TokenType::ADDITION_OPERATOR => write!(f, "ADDITION_OPERATOR"),
            TokenType::SUBTRACTION_OPERATOR => write!(f, "SUBTRACTION_OPERATOR"),
            TokenType::MULTIPLICATION_OPERATOR => write!(f, "MULTIPLICATION_OPERATOR"),
            TokenType::DIVISION_OPERATOR => write!(f, "DIVISION_OPERATOR"),
            TokenType::MODULUS_OPERATOR => write!(f, "MODULUS_OPERATOR"),
            TokenType::IF => write!(f, "IF"),
            TokenType::ELSE => write!(f, "ELSE"),
            TokenType::FOR_LOOP => write!(f, "FOR_LOOP"),
            TokenType::WHILE_LOOP => write!(f, "WHILE_LOOP"),
            TokenType::FUNCTION => write!(f, "FUNCTION"),
            TokenType::RETURN => write!(f, "RETURN"),
            TokenType::NUMBER => write!(f, "NUMBER"),
            TokenType::STRING => write!(f, "STRING"),
            TokenType::CHARACTER => write!(f, "CHARACTER"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}