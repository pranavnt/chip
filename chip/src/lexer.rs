use crate::token::{Token, TokenType};

pub struct Lexer {
    pub code: String,
    pub pos: i32,
    pub chars: Vec<char>,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        let chars = code.chars().collect::<Vec<char>>();

        Lexer {
            code,
            pos: 0,
            chars,
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) {
        // iterate through the code and create tokens
        while self.pos < self.chars.len() as i32 {
            self.get_next_token();
            self.pos += 1;
        }
    }

    pub fn get_next_token(&mut self) {
        // skip whitespace
        while self.pos < self.chars.len() as i32 && self.chars[self.pos as usize] == ' '
            || self.chars[self.pos as usize] == '\t'
            || self.chars[self.pos as usize] == '\n'
            || self.chars[self.pos as usize] == '\r'
            || self.chars[self.pos as usize] == '\0'
        {
            self.pos += 1;
        }

        // if we're at the end of the file, return EOF
        if self.pos >= self.chars.len() as i32 {
            self.tokens.push(Token {
                token_type: TokenType::EOF,
                value: String::new(),
            });
            return;
        }

        // get the current character
        let current_char = self.chars[self.pos as usize];

        match current_char {
            // if the character is a digit, create a new token with the value of the number
            '0'..='9' => {
                let mut value = String::new();
                while self.pos < self.chars.len() as i32
                    && self.chars[self.pos as usize].is_digit(10)
                {
                    value.push(self.chars[self.pos as usize]);
                    self.pos += 1;
                }
                self.tokens.push(Token {
                    token_type: TokenType::NUMBER,
                    value,
                });
            }
            '=' => {
                if self.chars[self.pos as usize + 1] == '=' {
                    self.pos += 1;
                    self.tokens.push(Token {
                        token_type: TokenType::EQUAL_OPERATOR,
                        value: String::from("=="),
                    });
                } else {
                    self.tokens.push(Token {
                        token_type: TokenType::ASSIGNMENT_OPERATOR,
                        value: String::from("="),
                    });
                }
            }
            ':' => {
                if self.chars[self.pos as usize + 1] == '=' {
                    self.pos += 1;
                    self.tokens.push(Token {
                        token_type: TokenType::INITIALIZE_OPERATOR,
                        value: String::from(":="),
                    });
                }
            }
            '"' => {
                let mut value = String::new();

                self.pos += 1;

                while self.pos < self.chars.len() as i32 && self.chars[self.pos as usize] != '"' {
                    value.push(self.chars[self.pos as usize]);
                    self.pos += 1;
                }

                self.pos += 1;
                self.tokens.push(Token {
                    token_type: TokenType::STRING,
                    value,
                });
            }
            '&' => {
                if self.chars[self.pos as usize + 1] == '&' {
                    self.pos += 1;
                    self.tokens.push(Token {
                        token_type: TokenType::AND_OPERATOR,
                        value: String::from("&&"),
                    });
                }
            }
            '\'' => {
                let mut value = String::new();

                self.pos += 1;

                while self.pos < self.chars.len() as i32 && self.chars[self.pos as usize] != '\'' {
                    value.push(self.chars[self.pos as usize]);
                    self.pos += 1;
                }

                self.tokens.push(Token {
                    token_type: TokenType::CHARACTER,
                    value,
                });
            }
            '|' => {
                if self.chars[self.pos as usize + 1] == '|' {
                    self.pos += 1;
                    self.tokens.push(Token {
                        token_type: TokenType::OR_OPERATOR,
                        value: String::from("||"),
                    });
                }
            }
            '+' => {
                self.tokens.push(Token {
                    token_type: TokenType::ADDITION_OPERATOR,
                    value: String::from("+"),
                });
            }
            '-' => {
                self.tokens.push(Token {
                    token_type: TokenType::SUBTRACTION_OPERATOR,
                    value: String::from("-"),
                });
            }
            '*' => {
                self.tokens.push(Token {
                    token_type: TokenType::MULTIPLICATION_OPERATOR,
                    value: String::from("*"),
                });
            }
            '/' => {
                self.tokens.push(Token {
                    token_type: TokenType::DIVISION_OPERATOR,
                    value: String::from("/"),
                });
            }
            '%' => {
                self.tokens.push(Token {
                    token_type: TokenType::MODULUS_OPERATOR,
                    value: String::from("%"),
                });
            }
            '<' => {
                self.tokens.push(Token {
                    token_type: TokenType::LESS_THAN_OPERATOR,
                    value: String::from("<"),
                });
            }
            '>' => {
                self.tokens.push(Token {
                    token_type: TokenType::GREATER_THAN_OPERATOR,
                    value: String::from(">"),
                });
            }
            '!' => {
                if self.chars[self.pos as usize + 1] == '=' {
                    self.pos += 1;
                    self.tokens.push(Token {
                        token_type: TokenType::NOT_EQUAL_OPERATOR,
                        value: String::from("!="),
                    });
                } else {
                    self.tokens.push(Token {
                        token_type: TokenType::NOT_OPERATOR,
                        value: String::from("!"),
                    });
                }
            }
            '(' => {
                self.tokens.push(Token {
                    token_type: TokenType::L_PARENS,
                    value: String::from("("),
                });
            }
            ')' => {
                self.tokens.push(Token {
                    token_type: TokenType::R_PARENS,
                    value: String::from(")"),
                });
            }
            '{' => {
                self.tokens.push(Token {
                    token_type: TokenType::L_BRACE,
                    value: String::from("{"),
                });
            }
            '}' => {
                self.tokens.push(Token {
                    token_type: TokenType::R_BRACE,
                    value: String::from("}"),
                });
            }
            '[' => {
                self.tokens.push(Token {
                    token_type: TokenType::L_BRACKET,
                    value: String::from("["),
                });
            }
            ']' => {
                self.tokens.push(Token {
                    token_type: TokenType::R_BRACKET,
                    value: String::from("]"),
                });
            }
            // all letters
            'a'..='z' | 'A'..='Z' => {
                let mut value = String::new();
                while self.pos < self.chars.len() as i32
                    && self.chars[self.pos as usize].is_alphabetic()
                {
                    value.push(self.chars[self.pos as usize]);
                    self.pos += 1;
                }
                      // check for, while, fn, if, else, return, true, false
                      if value == "if" {
                        self.tokens.push(Token {
                            token_type: TokenType::IF,
                            value,
                        });
                    } else if value == "else" {
                        self.tokens.push(Token {
                            token_type: TokenType::ELSE,
                            value,
                        });
                    } else if value == "while" {
                        self.tokens.push(Token {
                            token_type: TokenType::WHILE_LOOP,
                            value,
                        });
                    } else if value == "for" {
                        self.tokens.push(Token {
                            token_type: TokenType::FOR_LOOP,
                            value,
                        });
                    } else if value == "fn" {
                        self.tokens.push(Token {
                            token_type: TokenType::FUNCTION,
                            value,
                        });
                    } else if value == "return" {
                        self.tokens.push(Token {
                            token_type: TokenType::RETURN,
                            value,
                        });
                    } else if value == "true" || value == "false"{
                        self.tokens.push(Token {
                            token_type: TokenType::BOOLEAN,
                            value,
                        });
                    } else {
                        self.tokens.push(Token {
                            token_type: TokenType::IDENTIFIER,
                            value,
                        });
                    }
            }
            '\0' => {
                self.tokens.push(Token {
                    token_type: TokenType::EOF,
                    value: String::from(""),
                });
            }

            _ => {
                self.pos += 1;
            }
        }
    }
}
