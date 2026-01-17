use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Var, Val, Def, Pub, Print, Return, If, Else, True, False, While, For, In, Import, Class, This, Try, Catch, New,

    // Literals and Identifiers
    Ident(String), Int(i64), Float(f64), Str(String), Type(String),

    // Operators and Punctuation
    Op(String), Arrow, Colon, Assign, LParen, RParen, LBracket, RBracket, Comma, Newline, Range, Dot,

    // Indentation
    Indent, Dedent,

    // Special
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self { token_type, line, column }
    }
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at line {}, column {}", self.message, self.line, self.column)
    }
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn advance(&mut self) {
        if self.pos < self.input.len() {
            if self.input[self.pos] == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.pos += 1;
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        let mut indent_stack = vec![0];

        while self.pos < self.input.len() {
            if self.column == 1 {
                let mut spaces = 0;
                let mut lookahead = self.pos;
                let mut is_empty_line = false;
                while lookahead < self.input.len() {
                    match self.input[lookahead] {
                        ' ' => spaces += 1,
                        '\t' => spaces += 4,
                        '\n' | '\r' => {
                            is_empty_line = true;
                            break;
                        }
                        _ => break,
                    }
                    lookahead += 1;
                }

                if !is_empty_line {
                    let last_indent = *indent_stack.last().unwrap();
                    if spaces > last_indent {
                        indent_stack.push(spaces);
                        tokens.push(Token::new(TokenType::Indent, self.line, self.column));
                    } else if spaces < last_indent {
                        while spaces < *indent_stack.last().unwrap() {
                            indent_stack.pop();
                            tokens.push(Token::new(TokenType::Dedent, self.line, self.column));
                        }
                        if spaces != *indent_stack.last().unwrap() {
                            return Err(LexerError { message: "Unindent does not match any outer indentation level".to_string(), line: self.line, column: self.column });
                        }
                    }
                    self.pos = lookahead;
                    self.column = spaces + 1;
                }
            }

            if self.pos >= self.input.len() { break; }

            let start_col = self.column;
            let c = self.input[self.pos];

            match c {
                ' ' | '\r' | '\t' => { self.advance(); },
                '\n' => { tokens.push(Token::new(TokenType::Newline, self.line, start_col)); self.advance(); },
                ':' => { tokens.push(Token::new(TokenType::Colon, self.line, start_col)); self.advance(); },
                '(' => { tokens.push(Token::new(TokenType::LParen, self.line, start_col)); self.advance(); },
                ')' => { tokens.push(Token::new(TokenType::RParen, self.line, start_col)); self.advance(); },
                '[' => { tokens.push(Token::new(TokenType::LBracket, self.line, start_col)); self.advance(); },
                ']' => { tokens.push(Token::new(TokenType::RBracket, self.line, start_col)); self.advance(); },
                ',' => { tokens.push(Token::new(TokenType::Comma, self.line, start_col)); self.advance(); },
                '=' => {
                    if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '=' {
                        tokens.push(Token::new(TokenType::Op("==".to_string()), self.line, start_col));
                        self.advance(); self.advance();
                    } else {
                        tokens.push(Token::new(TokenType::Assign, self.line, start_col));
                        self.advance();
                    }
                },
                '.' => {
                    if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '.' {
                        tokens.push(Token::new(TokenType::Range, self.line, start_col));
                        self.advance(); self.advance();
                    } else {
                        tokens.push(Token::new(TokenType::Dot, self.line, start_col));
                        self.advance();
                    }
                },
                '>' | '<' | '!' => {
                    if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '=' {
                        tokens.push(Token::new(TokenType::Op(format!("{}=", c)), self.line, start_col));
                        self.advance(); self.advance();
                    } else {
                        tokens.push(Token::new(TokenType::Op(c.to_string()), self.line, start_col));
                        self.advance();
                    }
                },
                '+' | '*' | '/' => { tokens.push(Token::new(TokenType::Op(c.to_string()), self.line, start_col)); self.advance(); },
                '-' => {
                    if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '>' {
                        tokens.push(Token::new(TokenType::Arrow, self.line, start_col));
                        self.advance(); self.advance();
                    } else {
                        tokens.push(Token::new(TokenType::Op("-".to_string()), self.line, start_col));
                        self.advance();
                    }
                },
                '#' => { while self.pos < self.input.len() && self.input[self.pos] != '\n' { self.advance(); } },
                '"' => {
                    self.advance();
                    let mut s = String::new();
                    while self.pos < self.input.len() {
                        if self.input[self.pos] == '"' { break; }
                        if self.input[self.pos] == '\\' {
                            self.advance();
                            if self.pos < self.input.len() {
                                match self.input[self.pos] {
                                    'n' => s.push('\n'), 't' => s.push('\t'), 'r' => s.push('\r'),
                                    '\\' => s.push('\\'), '"' => s.push('"'),
                                    _ => s.push(self.input[self.pos]),
                                }
                            }
                        } else { s.push(self.input[self.pos]); }
                        self.advance();
                    }
                    if self.pos < self.input.len() && self.input[self.pos] == '"' {
                        tokens.push(Token::new(TokenType::Str(s), self.line, start_col));
                        self.advance();
                    } else {
                        return Err(LexerError { message: "Unterminated string literal".to_string(), line: self.line, column: start_col });
                    }
                },
                _ if c.is_alphabetic() => {
                    let mut ident = String::new();
                    while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_') {
                        ident.push(self.input[self.pos]);
                        self.advance();
                    }
                    let token_type = match ident.as_str() {
                        "var" => TokenType::Var, "val" => TokenType::Val, "def" => TokenType::Def,
                        "if" => TokenType::If, "else" => TokenType::Else, "pub" => TokenType::Pub,
                        "return" => TokenType::Return, "print" => TokenType::Print,
                        "true" => TokenType::True, "false" => TokenType::False,
                        "while" => TokenType::While, "for" => TokenType::For, "in" => TokenType::In,
                        "import" => TokenType::Import, "class" => TokenType::Class, "this" => TokenType::This,
                        "try" => TokenType::Try, "catch" => TokenType::Catch, "new" => TokenType::New,
                        "int" | "float" | "string" | "bool" | "list" | "void" => TokenType::Type(ident),
                        _ => TokenType::Ident(ident),
                    };
                    tokens.push(Token::new(token_type, self.line, start_col));
                },
                _ if c.is_numeric() => {
                    let mut num = String::new();
                    let mut is_float = false;
                    while self.pos < self.input.len() && (self.input[self.pos].is_numeric() || self.input[self.pos] == '.') {
                        if self.input[self.pos] == '.' {
                            if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '.' { break; }
                            if is_float { return Err(LexerError { message: "Invalid number: multiple decimal points".to_string(), line: self.line, column: self.column }); }
                            is_float = true;
                        }
                        num.push(self.input[self.pos]);
                        self.advance();
                    }
                    let token_type = if is_float {
                        match num.parse() {
                            Ok(n) => TokenType::Float(n),
                            Err(_) => return Err(LexerError { message: format!("Invalid float: {}", num), line: self.line, column: start_col }),
                        }
                    } else {
                        match num.parse() {
                            Ok(n) => TokenType::Int(n),
                            Err(_) => return Err(LexerError { message: format!("Invalid integer: {}", num), line: self.line, column: start_col }),
                        }
                    };
                    tokens.push(Token::new(token_type, self.line, start_col));
                },
                _ => return Err(LexerError { message: format!("Unknown character: {}", c), line: self.line, column: start_col }),
            }
        }

        while indent_stack.len() > 1 {
            indent_stack.pop();
            tokens.push(Token::new(TokenType::Dedent, self.line, self.column));
        }
        tokens.push(Token::new(TokenType::EOF, self.line, self.column));
        Ok(tokens)
    }
}
