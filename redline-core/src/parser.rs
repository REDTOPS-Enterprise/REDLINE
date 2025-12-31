use crate::lexer::Token;
use crate::ast::{Program, Statement, Expression, Type, Literal, BinaryOperator}; // Import AST nodes

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser Error: {}", self.message)
    }
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected: &Token, error_msg: &str) -> Result<(), ParserError> {
        if let Some(token) = self.current_token() {
            if *token == *expected {
                self.advance();
                Ok(())
            } else {
                Err(ParserError { message: format!("{}: Expected {:?}, got {:?}", error_msg, expected, token) })
            }
        } else {
            Err(ParserError { message: format!("{}: Expected {:?}, got EOF", error_msg, expected) })
        }
    }

    fn parse_type(&mut self) -> Result<Type, ParserError> {
        if let Some(Token::Type(ty_str)) = self.current_token() {
            let ty = match ty_str.as_str() {
                "int" => Type::Int,
                "float" => Type::Float,
                "string" => Type::String,
                _ => return Err(ParserError { message: format!("Unknown type: {}", ty_str) }),
            };
            self.advance();
            Ok(ty)
        } else {
            Err(ParserError { message: format!("Expected type, got {:?}", self.current_token()) })
        }
    }

    // Parses primary expressions: literals, identifiers, parenthesized expressions, function calls
    fn parse_expression_primary(&mut self) -> Result<Expression, ParserError> {
        if let Some(token) = self.current_token() {
            match token {
                Token::Int(n) => {
                    let val = Expression::Literal(Literal::Int(*n));
                    self.advance();
                    Ok(val)
                },
                Token::Float(n) => {
                    let val = Expression::Literal(Literal::Float(*n));
                    self.advance();
                    Ok(val)
                },
                Token::Str(s) => {
                    let val = Expression::Literal(Literal::String(s.clone()));
                    self.advance();
                    Ok(val)
                },
                Token::Ident(name) => {
                    let name = name.clone();
                    self.advance(); // Consume identifier

                    // Check if this is a function call
                    if let Some(Token::LParen) = self.current_token() {
                        self.advance(); // Consume '('

                        // Parse arguments
                        let mut args = Vec::new();
                        if !matches!(self.current_token(), Some(Token::RParen)) {
                            loop {
                                args.push(self.parse_expression()?);

                                if let Some(Token::Comma) = self.current_token() {
                                    self.advance(); // Consume comma and continue
                                } else {
                                    break;
                                }
                            }
                        }

                        self.expect(&Token::RParen, "Expected ')' after function arguments")?; // Consume ')'

                        Ok(Expression::Call(name, args))
                    } else {
                        Ok(Expression::Identifier(name))
                    }
                },
                Token::LParen => {
                    self.advance(); // Consume '('
                    let expr = self.parse_expression()?;
                    self.expect(&Token::RParen, "Expected ')' after parenthesized expression")?;
                    Ok(expr)
                },
                _ => Err(ParserError { message: format!("Expected a primary expression, got {:?}", token) }),
            }
        } else {
            Err(ParserError { message: "Expected a primary expression, got EOF".to_string() })
        }
    }

    // Operator precedence (higher value means higher precedence)
    fn get_precedence(op_token: &Token) -> u8 {
        match op_token {
            Token::Op(op) => match op.as_str() {
                "*" | "/" => 5,
                "+" | "-" => 4,
                "==" | "!=" | ">" | "<" | ">=" | "<=" => 3, // Comparison operators
                _ => 0,
            },
            _ => 0,
        }
    }

    // Converts a Token::Op to a BinaryOperator enum
    fn token_to_binary_op(op_token: &Token) -> Result<BinaryOperator, ParserError> {
        if let Token::Op(op_str) = op_token {
            match op_str.as_str() {
                "+" => Ok(BinaryOperator::Add),
                "-" => Ok(BinaryOperator::Subtract),
                "*" => Ok(BinaryOperator::Multiply),
                "/" => Ok(BinaryOperator::Divide),
                "==" => Ok(BinaryOperator::Equal),
                "!=" => Ok(BinaryOperator::NotEqual),
                ">" => Ok(BinaryOperator::GreaterThan),
                "<" => Ok(BinaryOperator::LessThan),
                ">=" => Ok(BinaryOperator::GreaterThanEqual),
                "<=" => Ok(BinaryOperator::LessThanEqual),
                _ => Err(ParserError { message: format!("Unknown binary operator: {}", op_str) }),
            }
        } else {
            Err(ParserError { message: format!("Expected operator token, got {:?}", op_token) })
        }
    }

    // Implements precedence climbing algorithm
    fn parse_expression_binop(&mut self, min_precedence: u8) -> Result<Expression, ParserError> {
        let mut left = self.parse_expression_primary()?;

        while let Some(current_token) = self.current_token() {
            // Only continue if the current token is an operator
            let is_operator = matches!(current_token, Token::Op(_));
            if !is_operator {
                break;
            }

            let precedence = Self::get_precedence(current_token);

            if precedence == 0 || precedence < min_precedence {
                break;
            }

            let op_token = self.current_token().unwrap().clone(); // We know it's an operator
            let op = Self::token_to_binary_op(&op_token)?;
            self.advance(); // Consume the operator

            let right = self.parse_expression_binop(precedence + 1)?; // Recursively parse right-hand side with higher precedence

            left = Expression::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // Main entry point for parsing expressions
    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_expression_binop(0) // Start with the lowest precedence
    }


    fn parse_declaration(&mut self) -> Result<Statement, ParserError> {
        let is_mutable = match self.current_token() {
            Some(Token::Val) => false,
            Some(Token::Var) => true,
            _ => return Err(ParserError { message: format!("Expected 'val' or 'var', got {:?}", self.current_token()) }),
        };
        self.advance(); // Consume 'val' or 'var'

        let name = if let Some(Token::Ident(n)) = self.current_token() {
            n.clone()
        } else {
            return Err(ParserError { message: format!("Expected identifier after var/val, got {:?}", self.current_token()) });
        };
        self.advance(); // Consume identifier

        self.expect(&Token::Colon, "Expected ':' after identifier in declaration")?; // Consume ':'

        let data_type = self.parse_type()?; // Consume type

        self.expect(&Token::Assign, "Expected '=' in declaration")?; // Consume '='

        let initializer = self.parse_expression()?; // Consume initializer expression

        Ok(Statement::Declaration { is_mutable, name, data_type, initializer })
    }

    fn parse_print_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect(&Token::Print, "Expected 'print'")?; // Consume 'print'
        self.expect(&Token::LParen, "Expected '(' after 'print'")?; // Consume '('

        let arg = self.parse_expression()?; // Parse the expression argument

        self.expect(&Token::RParen, "Expected ')' after print argument")?; // Consume ')'
        Ok(Statement::Print(arg))
    }

    fn parse_function_definition(&mut self) -> Result<Statement, ParserError> {
        self.expect(&Token::Def, "Expected 'def'")?; // Consume 'def'

        let name = if let Some(Token::Ident(n)) = self.current_token() {
            n.clone()
        } else {
            return Err(ParserError { message: format!("Expected function name after 'def', got {:?}", self.current_token()) });
        };
        self.advance(); // Consume function name

        self.expect(&Token::LParen, "Expected '(' after function name")?; // Consume '('

        // Parse parameters
        let mut params = Vec::new();
        if !matches!(self.current_token(), Some(Token::RParen)) {
            loop {
                let param_name = if let Some(Token::Ident(n)) = self.current_token() {
                    n.clone()
                } else {
                    return Err(ParserError { message: format!("Expected parameter name, got {:?}", self.current_token()) });
                };
                self.advance(); // Consume parameter name

                self.expect(&Token::Colon, "Expected ':' after parameter name")?; // Consume ':'

                let param_type = self.parse_type()?; // Parse parameter type

                params.push((param_name, param_type));

                // Check for comma or closing paren
                if let Some(Token::Comma) = self.current_token() {
                    self.advance(); // Consume comma and continue
                } else {
                    break;
                }
            }
        }

        self.expect(&Token::RParen, "Expected ')' after parameters")?; // Consume ')'

        self.expect(&Token::Arrow, "Expected '->' after parameters")?; // Consume '->'

        let return_type = self.parse_type()?; // Parse return type

        self.expect(&Token::Colon, "Expected ':' after return type")?; // Consume ':'

        self.expect(&Token::Newline, "Expected newline after function definition")?; // Consume newline

        let body = self.parse_block()?; // Parse function body

        Ok(Statement::FunctionDefinition { name, params, return_type, body })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect(&Token::Return, "Expected 'return'")?; // Consume 'return'

        // Check if there's an expression after return
        let expr = if matches!(self.current_token(), Some(Token::Newline) | None) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        Ok(Statement::Return(expr))
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect(&Token::If, "Expected 'if'")?; // Consume 'if'

        let condition = self.parse_expression()?; // Parse condition

        self.expect(&Token::Colon, "Expected ':' after if condition")?; // Consume ':'
        // Ensure that a newline directly follows the colon
        self.expect(&Token::Newline, "Expected newline after if colon")?;

        let consequence = self.parse_block()?; // Parse 'then' block

        let mut alternative = None;
        // Check for 'else'
        // 'else' token must appear at the same logical indentation level as 'if'
        // For now, we assume it's directly after the 'if' block concludes.
        // The parser needs to handle dedent to know a block has ended.
        // For simplicity, we'll assume a newline and then 'else' means it's part of the same logical block structure.
        if let Some(Token::Else) = self.current_token() {
            self.advance(); // Consume 'else'
            // Optional colon after else
            if let Some(Token::Colon) = self.current_token() {
                self.advance();
            }
            self.expect(&Token::Newline, "Expected newline after else keyword/colon")?;
            alternative = Some(self.parse_block()?); // Parse 'else' block
        }

        Ok(Statement::If { condition, consequence, alternative })
    }

    // Parses a block of statements. Assumes that current_token is the first token of the block.
    // Stops when it encounters a token that is not part of the block's logical flow (e.g., 'else' for 'if' blocks, or EOF)
    // For now, this is simplified and assumes indentation will be handled implicitly by token stream (e.g., Newline tokens)
    fn parse_block(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut block_statements = Vec::new();
        loop {
            // Skip newlines.
            // In an indentation-sensitive language, dedent would be detected here.
            // For now, we rely on `parse_statement` to skip leading newlines for itself
            // and `parse` to skip top-level newlines.
            // This `parse_block` will continue until it hits a non-statement-starting token
            // or a token that signifies the end of a block (like 'else' for 'if' blocks, or EOF).
            if let Some(Token::Newline) = self.current_token() {
                self.advance();
                continue;
            }

            // Break conditions for the block
            if self.current_token().is_none() || matches!(self.current_token(), Some(Token::Else)) {
                break;
            }

            block_statements.push(self.parse_statement()?);
        }
        Ok(block_statements)
    }


    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        // Skip leading newlines for cleaner parsing of statements
        while let Some(Token::Newline) = self.current_token() {
            self.advance();
        }

        match self.current_token() {
            Some(Token::Val) | Some(Token::Var) => self.parse_declaration(),
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::Print) => self.parse_print_statement(),
            Some(Token::Def) => self.parse_function_definition(),
            Some(Token::Return) => self.parse_return_statement(),
            // Other statements will be added here
            Some(token) => {
                Err(ParserError { message: format!("Unexpected token at start of statement: {:?}", token) })
            }
            None => Err(ParserError { message: "Unexpected EOF while parsing statement".to_string() }),
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut statements = Vec::new();

        while self.pos < self.tokens.len() {
            // Skip top-level newlines
            if let Some(Token::Newline) = self.current_token() {
                self.advance();
                continue;
            }

            // If EOF is reached, break
            if self.current_token().is_none() {
                break;
            }

            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }
}