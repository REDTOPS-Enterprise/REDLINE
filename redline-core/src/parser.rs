use crate::lexer::{Lexer, Token, TokenType}; // Imported Lexer
use crate::ast::{Program, Statement, Expression, Type, Literal, BinaryOperator, ClassMember};

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

    fn consume_if(&mut self, token_type: TokenType) -> bool {
        if self.current_token().token_type == token_type {
            self.advance();
            true
        } else {
            false
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
        match self.current_token().token_type {
            TokenType::Type(ty_str) => {
                match ty_str.as_str() {
                    "int" => { self.advance(); Ok(Type::Int) },
                    "float" => { self.advance(); Ok(Type::Float) },
                    "string" => { self.advance(); Ok(Type::String) },
                    "bool" => { self.advance(); Ok(Type::Bool) },
                    "void" => { self.advance(); Ok(Type::Void) },
                    "list" => {
                        self.advance();
                        self.expect(TokenType::LBracket, "Expected '[' after 'list'")?;
                        let inner_type = self.parse_type()?;
                        self.expect(TokenType::RBracket, "Expected ']' after list inner type")?;
                        Ok(Type::List(Box::new(inner_type)))
                    },
                    "dict" => {
                        self.advance();
                        self.expect(TokenType::LBracket, "Expected '[' after 'dict'")?;
                        let key_type = self.parse_type()?;
                        self.expect(TokenType::Comma, "Expected ',' after dictionary key type")?;
                        let value_type = self.parse_type()?;
                        self.expect(TokenType::RBracket, "Expected ']' after dictionary value type")?;
                        Ok(Type::Dict(Box::new(key_type), Box::new(value_type)))
                    },
                    _ => Err(self.error(format!("Unknown built-in type: {}", ty_str))),
                }
            },
            TokenType::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(Type::Class(name))
            }
            _ => Err(self.error(format!("Expected type identifier, got {:?}", self.current_token().token_type)))
        }
    }

    fn parse_expression_primary(&mut self) -> Result<Expression, ParserError> {
        let token = self.current_token();
        let mut expr = match &token.token_type {
            TokenType::FString(s) => {
                self.advance();
                let mut parts = Vec::new();
                let mut last_pos = 0;
                let chars: Vec<char> = s.chars().collect();
                let mut i = 0;

                while i < chars.len() {
                    if chars[i] == '{' {
                        // Add string literal before '{'
                        if i > last_pos {
                            let literal = s[last_pos..i].to_string();
                            parts.push(Expression::Literal(Literal::String(literal)));
                        }

                        // Find matching '}'
                        let start_expr = i + 1;
                        let mut brace_count = 1;
                        i += 1;
                        while i < chars.len() && brace_count > 0 {
                            if chars[i] == '{' { brace_count += 1; }
                            else if chars[i] == '}' { brace_count -= 1; }
                            i += 1;
                        }

                        if brace_count == 0 {
                            let expr_str = s[start_expr..i-1].to_string();
                            // Parse expression inside {}
                            let mut lexer = Lexer::new(expr_str);
                            let tokens = lexer.tokenize().map_err(|e| ParserError { message: e.message, line: token.line, column: token.column })?;
                            let mut parser = Parser::new(&tokens);
                            let expr = parser.parse_expression()?;

                            // Wrap in to_string()
                            let to_string_call = Expression::Call {
                                callee: Box::new(Expression::Identifier("to_string".to_string())),
                                args: vec![expr]
                            };
                            parts.push(to_string_call);
                            last_pos = i;
                        } else {
                            return Err(self.error("Unclosed '{' in f-string".to_string()));
                        }
                    } else {
                        i += 1;
                    }
                }

                // Add remaining string literal
                if last_pos < chars.len() {
                    let literal = s[last_pos..].to_string();
                    parts.push(Expression::Literal(Literal::String(literal)));
                }

                // Combine parts with '+'
                if parts.is_empty() {
                    Ok(Expression::Literal(Literal::String("".to_string())))
                } else {
                    let mut final_expr = parts[0].clone();
                    for j in 1..parts.len() {
                        final_expr = Expression::BinaryOp {
                            op: BinaryOperator::Add,
                            left: Box::new(final_expr),
                            right: Box::new(parts[j].clone())
                        };
                    }
                    Ok(final_expr)
                }
            },
            TokenType::New => {
                self.advance();
                if let TokenType::Ident(class_name) = self.current_token().token_type {
                    let class_name = class_name.clone();
                    self.advance();
                    self.expect(TokenType::LParen, "Expected '(' after class name in new expression")?;
                    let mut args = Vec::new();
                    if !self.consume_if(TokenType::RParen) {
                        loop {
                            args.push(self.parse_expression()?);
                            if !self.consume_if(TokenType::Comma) { break; }
                        }
                        self.expect(TokenType::RParen, "Expected ')' after new expression arguments")?;
                    }
                    Ok(Expression::New { class_name, args })
                } else {
                    Err(self.error("Expected class name after 'new'".to_string()))
                }
            },
            TokenType::This => { self.advance(); Ok(Expression::This) },
            TokenType::Int(n) => { self.advance(); Ok(Expression::Literal(Literal::Int(*n))) },
            TokenType::Float(n) => { self.advance(); Ok(Expression::Literal(Literal::Float(*n))) },
            TokenType::Str(s) => { self.advance(); Ok(Expression::Literal(Literal::String(s.clone()))) },
            TokenType::True => { self.advance(); Ok(Expression::Literal(Literal::Bool(true))) },
            TokenType::False => { self.advance(); Ok(Expression::Literal(Literal::Bool(false))) },
            TokenType::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expression::Identifier(name))
            },
            TokenType::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenType::RParen, "Expected ')' after parenthesized expression")?;
                Ok(expr)
            },
            TokenType::LBracket => self.parse_list_literal(),
            TokenType::LBrace => {
                self.advance();
                let mut entries = Vec::new();

                // Consume optional newline and indent after '{'
                self.consume_if(TokenType::Newline);
                self.consume_if(TokenType::Indent);

                if !self.consume_if(TokenType::RBrace) {
                    loop {
                        // Consume optional newlines/indent before key
                        self.consume_if(TokenType::Newline);
                        self.consume_if(TokenType::Indent);

                        let key = self.parse_expression()?;
                        self.expect(TokenType::Colon, "Expected ':' after dictionary key")?;
                        let value = self.parse_expression()?;
                        entries.push((key, value));

                        if !self.consume_if(TokenType::Comma) {
                            // Consume optional newline before '}'
                            self.consume_if(TokenType::Newline);
                            break;
                        }
                        // Consume optional newline after ','
                        self.consume_if(TokenType::Newline);
                    }
                    // Consume optional dedent before '}'
                    self.consume_if(TokenType::Dedent);
                    self.expect(TokenType::RBrace, "Expected '}' after dictionary entries")?;
                }
                Ok(Expression::DictLiteral(entries))
            },
            _ => Err(self.error(format!("Expected a primary expression, got {:?}", token.token_type))),
        }?;

        loop {
            if self.consume_if(TokenType::LParen) {
                let mut args = Vec::new();
                if !self.consume_if(TokenType::RParen) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.consume_if(TokenType::Comma) { break; }
                    }
                    self.expect(TokenType::RParen, "Expected ')' after function arguments")?;
                }
                expr = Expression::Call { callee: Box::new(expr), args };
            } else if self.consume_if(TokenType::LBracket) {
                let index = self.parse_expression()?;
                self.expect(TokenType::RBracket, "Expected ']' after index expression")?;
                expr = Expression::Index { list: Box::new(expr), index: Box::new(index) };
            } else if self.consume_if(TokenType::Dot) {
                if let TokenType::Ident(name) = self.current_token().token_type {
                    let name = name.clone();
                    self.advance();
                    expr = Expression::Get { object: Box::new(expr), name };
                } else {
                    return Err(self.error("Expected identifier after '.'".to_string()));
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_list_literal(&mut self) -> Result<Expression, ParserError> {
        self.expect(TokenType::LBracket, "Expected '[' to start a list literal")?;
        let mut elements = Vec::new();
        if self.current_token().token_type != TokenType::RBracket {
            loop {
                elements.push(self.parse_expression()?);
                if !self.consume_if(TokenType::Comma) { break; }
            }
        }
        self.expect(TokenType::RBracket, "Expected ']' to end a list literal")?;
        Ok(Expression::ListLiteral(elements))
    }

    fn get_precedence(token_type: &TokenType) -> u8 {
        match token_type {
            TokenType::Dot => 7,
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
            self.advance();

            let right = self.parse_expression_binop(precedence + 1)?;
            left = Expression::BinaryOp { op: self.token_to_binary_op(&op_token.token_type)?, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_expression_binop(0)
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, ParserError> {
        self.expect(TokenType::Indent, "Expected indentation for block")?;
        let mut statements = Vec::new();
        while self.current_token().token_type != TokenType::Dedent && self.current_token().token_type != TokenType::EOF {
            while self.consume_if(TokenType::Newline) {}
            if self.current_token().token_type == TokenType::Dedent { break; }
            statements.push(self.parse_statement()?);
        }
        self.expect(TokenType::Dedent, "Expected dedent to end block")?;
        Ok(statements)
    }

    fn parse_class_block(&mut self) -> Result<Vec<ClassMember>, ParserError> {
        self.expect(TokenType::Indent, "Expected indentation for class body")?;
        let mut members = Vec::new();
        while self.current_token().token_type != TokenType::Dedent && self.current_token().token_type != TokenType::EOF {
            while self.consume_if(TokenType::Newline) {}
            if self.current_token().token_type == TokenType::Dedent { break; }

            let is_public = self.consume_if(TokenType::Pub);
            match self.current_token().token_type {
                TokenType::Val | TokenType::Var => {
                    let decl = self.parse_declaration(is_public)?;
                    members.push(ClassMember::Variable(decl));
                }
                TokenType::Def => {
                    let method = self.parse_function_definition(is_public)?;
                    if let Statement::FunctionDefinition { ref name, .. } = method {
                        if name == "init" {
                            members.push(ClassMember::Constructor(method));
                        } else {
                            members.push(ClassMember::Method(method));
                        }
                    }
                }
                _ => return Err(self.error("Expected 'val', 'var', or 'def' in class body".to_string())),
            }
        }
        self.expect(TokenType::Dedent, "Expected dedent to end class body")?;
        Ok(members)
    }

    fn parse_declaration(&mut self, is_public: bool) -> Result<Statement, ParserError> {
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
        Ok(Statement::Declaration { is_public, is_mutable, name, data_type, initializer })
    }

    fn parse_function_definition(&mut self, is_public: bool) -> Result<Statement, ParserError> {
        self.expect(TokenType::Def, "Expected 'def'")?;
        let name = if let TokenType::Ident(n) = &self.current_token().token_type { n.clone() }
            else { return Err(self.error("Expected function name after 'def'".to_string())); };
        self.advance();

        self.expect(TokenType::LParen, "Expected '(' after function name")?;
        let mut params = Vec::new();
        if !self.consume_if(TokenType::RParen) {
            loop {
                let param_name = if let TokenType::Ident(n) = &self.current_token().token_type { n.clone() }
                    else { return Err(self.error("Expected parameter name".to_string())); };
                self.advance();
                self.expect(TokenType::Colon, "Expected ':' after parameter name")?;
                let param_type = self.parse_type()?;
                params.push((param_name, param_type));
                if !self.consume_if(TokenType::Comma) { break; }
            }
            self.expect(TokenType::RParen, "Expected ')' after parameters")?;
        }

        let return_type = if self.consume_if(TokenType::Arrow) {
            self.parse_type()?
        } else {
            Type::Void
        };

        self.expect(TokenType::Colon, "Expected ':' after function signature")?;
        self.expect(TokenType::Newline, "Expected newline after function definition")?;
        let body = self.parse_block()?;
        Ok(Statement::FunctionDefinition { is_public, name, params, return_type, body })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::If, "Expected 'if'")?;
        let condition = self.parse_expression()?;
        self.expect(TokenType::Colon, "Expected ':' after if condition")?;
        self.expect(TokenType::Newline, "Expected newline after if colon")?;
        let consequence = self.parse_block()?;
        let mut alternative = None;
        if self.consume_if(TokenType::Else) {
            self.expect(TokenType::Colon, "Expected ':' after 'else'")?;
            self.expect(TokenType::Newline, "Expected newline after else colon")?;
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

    fn parse_import_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::Import, "Expected 'import'")?;
        if let TokenType::Str(path) = self.current_token().token_type {
            let path = path.clone();
            self.advance();
            Ok(Statement::Import(path))
        } else {
            Err(self.error("Expected string literal for import path".to_string()))
        }
    }

    fn parse_class_statement(&mut self, is_public: bool) -> Result<Statement, ParserError> {
        self.expect(TokenType::Class, "Expected 'class'")?;
        let name = if let TokenType::Ident(n) = &self.current_token().token_type { n.clone() }
            else { return Err(self.error("Expected class name".to_string())); };
        self.advance();
        self.expect(TokenType::Colon, "Expected ':' after class name")?;
        self.expect(TokenType::Newline, "Expected newline after class definition")?;
        let members = self.parse_class_block()?;
        Ok(Statement::Class { is_public, name, members })
    }

    fn parse_try_catch_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect(TokenType::Try, "Expected 'try'")?;
        self.expect(TokenType::Colon, "Expected ':' after 'try'")?;
        self.expect(TokenType::Newline, "Expected newline after 'try:'")?;
        let try_block = self.parse_block()?;

        self.expect(TokenType::Catch, "Expected 'catch'")?;
        let catch_var = if let TokenType::Ident(name) = &self.current_token().token_type {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err(self.error("Expected identifier for catch variable".to_string()));
        };
        self.expect(TokenType::Colon, "Expected ':' after catch variable")?;
        self.expect(TokenType::Newline, "Expected newline after 'catch ...:'")?;
        let catch_block = self.parse_block()?;

        Ok(Statement::TryCatch { try_block, catch_var, catch_block })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        while self.consume_if(TokenType::Newline) {}

        match self.current_token().token_type {
            TokenType::Import => self.parse_import_statement(),
            TokenType::Class => self.parse_class_statement(false),
            TokenType::Try => self.parse_try_catch_statement(),
            TokenType::Break => {
                self.advance();
                Ok(Statement::Break)
            },
            TokenType::Continue => {
                self.advance();
                Ok(Statement::Continue)
            },
            TokenType::Print => {
                self.advance();
                self.expect(TokenType::LParen, "Expected '(' after 'print'")?;
                let arg = self.parse_expression()?;
                self.expect(TokenType::RParen, "Expected ')' after print argument")?;
                Ok(Statement::Print(arg))
            },
            TokenType::Pub => {
                self.advance();
                match self.current_token().token_type {
                    TokenType::Val | TokenType::Var => self.parse_declaration(true),
                    TokenType::Def => self.parse_function_definition(true),
                    TokenType::Class => self.parse_class_statement(true),
                    _ => Err(self.error("Expected 'val', 'var', 'def', or 'class' after 'pub'".to_string())),
                }
            },
            TokenType::Val | TokenType::Var => self.parse_declaration(false),
            TokenType::Def => self.parse_function_definition(false),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::For => self.parse_for_statement(),
            TokenType::Return => {
                self.advance();
                let expr = if self.current_token().token_type == TokenType::Newline || self.current_token().token_type == TokenType::EOF { None }
                    else { Some(self.parse_expression()?) };
                Ok(Statement::Return(expr))
            },
            _ => {
                let target = self.parse_expression()?;
                if self.consume_if(TokenType::Assign) {
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
            if self.consume_if(TokenType::Newline) { continue; }
            statements.push(self.parse_statement()?);
        }
        Ok(Program { statements })
    }
}
