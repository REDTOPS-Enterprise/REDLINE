use crate::lexer::{Token, TokenType};
use crate::ast::{Program, Statement, Expression, Type, Literal, BinaryOperator};

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub line: usize,
    pub column: usize,
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

    fn current_token(&self) -> Token {
        self.tokens.get(self.pos).cloned().unwrap_or_else(|| Token::new(TokenType::EOF, 0, 0))
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn error(&self, message: String) -> ParserError {
        let token = self.current_token();
        ParserError { message, line: token.line, column: token.column }
    }

    fn expect(&mut self, expected: TokenType, error_msg: &str) -> Result<(), ParserError> {
        if self.current_token().token_type == expected {
            self.advance();
            Ok(())
        } else {
            Err(self.error(format!("{}: Expected {:?}, got {:?}", error_msg, expected, self.current_token().token_type)))
        }
    }

    fn parse_type(&mut self) -> Result<Type, ParserError> {
        if let TokenType::Type(ty_str) = &self.current_token().token_type {
            match ty_str.as_str() {
                "int" => { self.advance(); Ok(Type::Int) },
                "float" => { self.advance(); Ok(Type::Float) },
                "string" => { self.advance(); Ok(Type::String) },
                "bool" => { self.advance(); Ok(Type::Bool) },
                "list" => {
                    self.advance();
                    self.expect(TokenType::LBracket, "Expected '[' after 'list'")?;
                    let inner_type = self.parse_type()?;
                    self.expect(TokenType::RBracket, "Expected ']' after list inner type")?;
                    Ok(Type::List(Box::new(inner_type)))
                },
                _ => Err(self.error(format!("Unknown type: {}", ty_str))),
            }
        } else {
            Err(self.error(format!("Expected type, got {:?}", self.current_token().token_type)))
        }
    }

    fn parse_expression_primary(&mut self) -> Result<Expression, ParserError> {
        let token = self.current_token();
        let mut expr = match &token.token_type {
            TokenType::Int(n) => { self.advance(); Ok(Expression::Literal(Literal::Int(*n))) },
            TokenType::Float(n) => { self.advance(); Ok(Expression::Literal(Literal::Float(*n))) },
            TokenType::Str(s) => { self.advance(); Ok(Expression::Literal(Literal::String(s.clone()))) },
            TokenType::True => { self.advance(); Ok(Expression::Literal(Literal::Bool(true))) },
            TokenType::False => { self.advance(); Ok(Expression::Literal(Literal::Bool(false))) },
            TokenType::Ident(name) => {
                let name = name.clone();
                self.advance();
                if self.current_token().token_type == TokenType::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    if self.current_token().token_type != TokenType::RParen {
                        loop {
                            args.push(self.parse_expression()?);
                            if self.current_token().token_type != TokenType::Comma { break; }
                            self.advance();
                        }
                    }
                    self.expect(TokenType::RParen, "Expected ')' after function arguments")?;
                    Ok(Expression::Call(name, args))
                } else {
                    Ok(Expression::Identifier(name))
                }
            },
            TokenType::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenType::RParen, "Expected ')' after parenthesized expression")?;
                Ok(expr)
            },
            TokenType::LBracket => self.parse_list_literal(),
            _ => Err(self.error(format!("Expected a primary expression, got {:?}", token.token_type))),
        }?;

        // Handle index access
        while self.current_token().token_type == TokenType::LBracket {
            self.advance();
            let index = self.parse_expression()?;
            self.expect(TokenType::RBracket, "Expected ']' after index expression")?;
            expr = Expression::Index { list: Box::new(expr), index: Box::new(index) };
        }

        Ok(expr)
    }

    fn parse_list_literal(&mut self) -> Result<Expression, ParserError> {
        self.expect(TokenType::LBracket, "Expected '[' to start a list literal")?;
        let mut elements = Vec::new();
        if self.current_token().token_type != TokenType::RBracket {
            loop {
                elements.push(self.parse_expression()?);
                if self.current_token().token_type != TokenType::Comma {
                    break;
                }
                self.advance();
            }
        }
        self.expect(TokenType::RBracket, "Expected ']' to end a list literal")?;
        Ok(Expression::ListLiteral(elements))
    }

    fn get_precedence(token_type: &TokenType) -> u8 {
        match token_type {
            TokenType::Op(op) => match op.as_str() {
                "*" | "/" => 5,
                "+" | "-" => 4,
                "==" | "!=" | ">" | "<" | ">=" | "<=" => 3,
                _ => 0,
            },
            _ => 0,
        }
    }

    fn token_to_binary_op(&self, token_type: &TokenType) -> Result<BinaryOperator, ParserError> {
        if let TokenType::Op(op_str) = token_type {
            match op_str.as_str() {
                "+" => Ok(BinaryOperator::Add), "-" => Ok(BinaryOperator::Subtract),
                "*" => Ok(BinaryOperator::Multiply), "/" => Ok(BinaryOperator::Divide),
                "==" => Ok(BinaryOperator::Equal), "!=" => Ok(BinaryOperator::NotEqual),
                ">" => Ok(BinaryOperator::GreaterThan), "<" => Ok(BinaryOperator::LessThan),
                ">=" => Ok(BinaryOperator::GreaterThanEqual), "<=" => Ok(BinaryOperator::LessThanEqual),
                _ => Err(self.error(format!("Unknown binary operator: {}", op_str))),
            }
        } else {
            Err(self.error(format!("Expected operator token, got {:?}", token_type)))
        }
    }

    fn parse_expression_binop(&mut self, min_precedence: u8) -> Result<Expression, ParserError> {
        let mut left = self.parse_expression_primary()?;
        while self.current_token().token_type != TokenType::EOF {
            let precedence = Self::get_precedence(&self.current_token().token_type);
            if precedence == 0 || precedence < min_precedence { break; }

            let op_token = self.current_token();
            let op = self.token_to_binary_op(&op_token.token_type)?;
            self.advance();

            let right = self.parse_expression_binop(precedence + 1)?;
            left = Expression::BinaryOp { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_expression_binop(0)
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, ParserError> {
        self.expect(TokenType::Indent, "Expected indentation at start of block")?;
        let mut block_statements = Vec::new();
        while self.current_token().token_type != TokenType::Dedent && self.current_token().token_type != TokenType::EOF {
            if self.current_token().token_type == TokenType::Newline {
                self.advance();
                continue;
            }
            block_statements.push(self.parse_statement()?);
        }
        self.expect(TokenType::Dedent, "Expected dedent at end of block")?;
        Ok(block_statements)
    }

    fn parse_declaration(&mut self) -> Result<Statement, ParserError> {
        let is_mutable = match self.current_token().token_type {
            TokenType::Val => false,
            TokenType::Var => true,
            _ => return Err(self.error("Expected 'val' or 'var'".to_string())),
        };
        self.advance();

        let name = if let TokenType::Ident(n) = &self.current_token().token_type { n.clone() }
            else { return Err(self.error("Expected identifier after var/val".to_string())); };
        self.advance();

        self.expect(TokenType::Colon, "Expected ':' after identifier in declaration")?;
        let data_type = self.parse_type()?;
        self.expect(TokenType::Assign, "Expected '=' in declaration")?;
        let initializer = self.parse_expression()?;
        Ok(Statement::Declaration { is_mutable, name, data_type, initializer })
    }

    fn parse_function_definition(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::Def, "Expected 'def'")?;
        let name = if let TokenType::Ident(n) = &self.current_token().token_type { n.clone() }
            else { return Err(self.error("Expected function name after 'def'".to_string())); };
        self.advance();

        self.expect(TokenType::LParen, "Expected '(' after function name")?;
        let mut params = Vec::new();
        if self.current_token().token_type != TokenType::RParen {
            loop {
                let param_name = if let TokenType::Ident(n) = &self.current_token().token_type { n.clone() }
                    else { return Err(self.error("Expected parameter name".to_string())); };
                self.advance();
                self.expect(TokenType::Colon, "Expected ':' after parameter name")?;
                let param_type = self.parse_type()?;
                params.push((param_name, param_type));
                if self.current_token().token_type != TokenType::Comma { break; }
                self.advance();
            }
        }
        self.expect(TokenType::RParen, "Expected ')' after parameters")?;
        self.expect(TokenType::Arrow, "Expected '->' after parameters")?;
        let return_type = self.parse_type()?;
        self.expect(TokenType::Colon, "Expected ':' after return type")?;
        self.expect(TokenType::Newline, "Expected newline after function definition")?;
        let body = self.parse_block()?;
        Ok(Statement::FunctionDefinition { name, params, return_type, body })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::If, "Expected 'if'")?;
        let condition = self.parse_expression()?;
        self.expect(TokenType::Colon, "Expected ':' after if condition")?;
        self.expect(TokenType::Newline, "Expected newline after if colon")?;
        let consequence = self.parse_block()?;
        let mut alternative = None;
        if self.current_token().token_type == TokenType::Else {
            self.advance();
            if self.current_token().token_type == TokenType::Colon { self.advance(); }
            self.expect(TokenType::Newline, "Expected newline after else keyword/colon")?;
            alternative = Some(self.parse_block()?);
        }
        Ok(Statement::If { condition, consequence, alternative })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::While, "Expected 'while'")?;
        let condition = self.parse_expression()?;
        self.expect(TokenType::Colon, "Expected ':' after while condition")?;
        self.expect(TokenType::Newline, "Expected newline after while colon")?;
        let body = self.parse_block()?;
        Ok(Statement::While { condition, body })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::For, "Expected 'for'")?;
        let iterator = if let TokenType::Ident(n) = &self.current_token().token_type { n.clone() }
            else { return Err(self.error("Expected iterator name after 'for'".to_string())); };
        self.advance();
        self.expect(TokenType::In, "Expected 'in' after iterator")?;
        let start = self.parse_expression()?;
        self.expect(TokenType::Range, "Expected '..' range operator")?;
        let end = self.parse_expression()?;
        self.expect(TokenType::Colon, "Expected ':' after range")?;
        self.expect(TokenType::Newline, "Expected newline after for colon")?;
        let body = self.parse_block()?;
        Ok(Statement::For { iterator, start, end, body })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        while self.current_token().token_type == TokenType::Newline { self.advance(); }

        match &self.current_token().token_type {
            TokenType::Print => {
                self.advance();
                self.expect(TokenType::LParen, "Expected '(' after 'print'")?;
                let arg = self.parse_expression()?;
                self.expect(TokenType::RParen, "Expected ')' after print argument")?;
                Ok(Statement::Print(arg))
            },
            TokenType::Val | TokenType::Var => self.parse_declaration(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::For => self.parse_for_statement(),
            TokenType::Def => self.parse_function_definition(),
            TokenType::Return => {
                self.advance();
                let expr = if self.current_token().token_type == TokenType::Newline || self.current_token().token_type == TokenType::EOF { None }
                    else { Some(self.parse_expression()?) };
                Ok(Statement::Return(expr))
            },
            _ => {
                let target = self.parse_expression()?;
                if self.current_token().token_type == TokenType::Assign {
                    self.advance();
                    let value = self.parse_expression()?;
                    Ok(Statement::Assignment { target, value })
                } else {
                    Ok(Statement::Expression(target))
                }
            }
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut statements = Vec::new();
        while self.current_token().token_type != TokenType::EOF {
            if self.current_token().token_type == TokenType::Newline {
                self.advance();
                continue;
            }
            statements.push(self.parse_statement()?);
        }
        Ok(Program { statements })
    }
}
