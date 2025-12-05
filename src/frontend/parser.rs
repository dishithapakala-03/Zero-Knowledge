use crate::language::ast::*;
use crate::language::types::*;
use crate::FCMCError;
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    symbol_table: HashMap<String, Type>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            symbol_table: HashMap::new(),
        }
    }
    
    pub fn parse_program(&mut self) -> Result<Program, FCMCError> {
        let mut functions = Vec::new();
        let mut constraints = Vec::new();
        
        while !self.is_at_end() {
            match self.peek().kind {
                TokenKind::Fn => {
                    functions.push(self.parse_function()?);
                }
                TokenKind::Constraint => {
                    constraints.push(self.parse_constraint()?);
                }
                TokenKind::Struct => {
                    // Parse struct definition
                    self.parse_struct()?;
                }
                _ => {
                    return Err(FCMCError::ParseError(
                        format!("Unexpected token at program level: {:?}", self.peek())
                    ));
                }
            }
        }
        
        Ok(Program {
            functions,
            constraints,
            entry_point: "main".to_string(),
        })
    }
    
    fn parse_function(&mut self) -> Result<Function, FCMCError> {
        self.consume(TokenKind::Fn, "Expected 'fn'")?;
        
        let name = match self.consume_identifier()? {
            Some(ident) => ident,
            None => return Err(FCMCError::ParseError("Expected function name".to_string())),
        };
        
        self.consume(TokenKind::LParen, "Expected '('")?;
        
        // Parse parameters
        let mut params = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                let param_name = match self.consume_identifier()? {
                    Some(ident) => ident,
                    None => break,
                };
                
                self.consume(TokenKind::Colon, "Expected ':' after parameter name")?;
                
                let param_type = self.parse_type()?;
                params.push((param_name, param_type));
                
                if !self.check(TokenKind::Comma) {
                    break;
                }
                self.advance(); // Consume comma
            }
        }
        
        self.consume(TokenKind::RParen, "Expected ')'")?;
        
        // Parse return type
        let return_type = if self.check(TokenKind::Arrow) {
            self.advance(); // Consume '->'
            self.parse_type()?
        } else {
            Type::Unit
        };
        
        self.consume(TokenKind::LBrace, "Expected '{'")?;
        
        // Parse function body
        let body = self.parse_block()?;
        
        self.consume(TokenKind::RBrace, "Expected '}'")?;
        
        Ok(Function {
            name,
            params,
            return_type,
            body,
            is_public: name == "main", // main function is public by default
        })
    }
    
    fn parse_block(&mut self) -> Result<Vec<Statement>, FCMCError> {
        let mut statements = Vec::new();
        
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        Ok(statements)
    }
    
    fn parse_statement(&mut self) -> Result<Statement, FCMCError> {
        match self.peek().kind {
            TokenKind::Let => self.parse_let_statement(),
            TokenKind::If => self.parse_if_statement(),
            TokenKind::For => self.parse_for_statement(),
            TokenKind::Return => self.parse_return_statement(),
            TokenKind::Assert => self.parse_assert_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    
    fn parse_let_statement(&mut self) -> Result<Statement, FCMCError> {
        self.consume(TokenKind::Let, "Expected 'let'")?;
        
        let name = match self.consume_identifier()? {
            Some(ident) => ident,
            None => return Err(FCMCError::ParseError("Expected variable name".to_string())),
        };
        
        let var_type = if self.check(TokenKind::Colon) {
            self.advance(); // Consume ':'
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.consume(TokenKind::Equals, "Expected '='")?;
        
        let value = self.parse_expression()?;
        self.consume(TokenKind::Semicolon, "Expected ';'")?;
        
        // Add to symbol table for type inference
        if let Some(t) = var_type {
            self.symbol_table.insert(name.clone(), t.clone());
        }
        
        Ok(Statement::Let {
            name,
            var_type,
            value,
        })
    }
    
    fn parse_expression(&mut self) -> Result<Expression, FCMCError> {
        self.parse_assignment()
    }
    
    fn parse_assignment(&mut self) -> Result<Expression, FCMCError> {
        let expr = self.parse_equality()?;
        
        if self.check(TokenKind::Equals) {
            self.advance(); // Consume '='
            let value = self.parse_assignment()?;
            Ok(Expression::Assignment(Box::new(expr), Box::new(value)))
        } else {
            Ok(expr)
        }
    }
    
    fn parse_equality(&mut self) -> Result<Expression, FCMCError> {
        let mut expr = self.parse_comparison()?;
        
        while self.check(TokenKind::EqualsEquals) || self.check(TokenKind::BangEquals) {
            let operator = self.advance().kind;
            let right = self.parse_comparison()?;
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: match operator {
                    TokenKind::EqualsEquals => BinaryOp::Eq,
                    TokenKind::BangEquals => BinaryOp::Ne,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_comparison(&mut self) -> Result<Expression, FCMCError> {
        let mut expr = self.parse_term()?;
        
        while self.check(TokenKind::Less)
            || self.check(TokenKind::LessEquals)
            || self.check(TokenKind::Greater)
            || self.check(TokenKind::GreaterEquals)
        {
            let operator = self.advance().kind;
            let right = self.parse_term()?;
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: match operator {
                    TokenKind::Less => BinaryOp::Lt,
                    TokenKind::LessEquals => BinaryOp::Le,
                    TokenKind::Greater => BinaryOp::Gt,
                    TokenKind::GreaterEquals => BinaryOp::Ge,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_term(&mut self) -> Result<Expression, FCMCError> {
        let mut expr = self.parse_factor()?;
        
        while self.check(TokenKind::Plus) || self.check(TokenKind::Minus) {
            let operator = self.advance().kind;
            let right = self.parse_factor()?;
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: match operator {
                    TokenKind::Plus => BinaryOp::Add,
                    TokenKind::Minus => BinaryOp::Sub,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_factor(&mut self) -> Result<Expression, FCMCError> {
        let mut expr = self.parse_unary()?;
        
        while self.check(TokenKind::Star)
            || self.check(TokenKind::Slash)
            || self.check(TokenKind::Percent)
        {
            let operator = self.advance().kind;
            let right = self.parse_unary()?;
            
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: match operator {
                    TokenKind::Star => BinaryOp::Mul,
                    TokenKind::Slash => BinaryOp::Div,
                    TokenKind::Percent => BinaryOp::Mod,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn parse_unary(&mut self) -> Result<Expression, FCMCError> {
        if self.check(TokenKind::Minus) || self.check(TokenKind::Bang) {
            let operator = self.advance().kind;
            let right = self.parse_unary()?;
            
            Ok(Expression::Unary {
                operator: match operator {
                    TokenKind::Minus => UnaryOp::Neg,
                    TokenKind::Bang => UnaryOp::Not,
                    _ => unreachable!(),
                },
                expr: Box::new(right),
            })
        } else {
            self.parse_primary()
        }
    }
    
    fn parse_primary(&mut self) -> Result<Expression, FCMCError> {
        match self.peek().kind {
            TokenKind::Number => {
                let value = self.advance().lexeme.clone();
                Ok(Expression::Literal(Literal::Number(value)))
            }
            TokenKind::Identifier => {
                let name = self.advance().lexeme.clone();
                if self.check(TokenKind::LParen) {
                    self.parse_function_call(name)
                } else {
                    Ok(Expression::Variable(name))
                }
            }
            TokenKind::LParen => {
                self.advance(); // Consume '('
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RParen, "Expected ')'")?;
                Ok(expr)
            }
            TokenKind::LBracket => self.parse_array(),
            _ => Err(FCMCError::ParseError(
                format!("Unexpected token in expression: {:?}", self.peek())
            )),
        }
    }
    
    fn parse_function_call(&mut self, name: String) -> Result<Expression, FCMCError> {
        self.consume(TokenKind::LParen, "Expected '('")?;
        
        let mut args = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.check(TokenKind::Comma) {
                    break;
                }
                self.advance(); // Consume comma
            }
        }
        
        self.consume(TokenKind::RParen, "Expected ')'")?;
        
        Ok(Expression::FunctionCall {
            name,
            args,
        })
    }
    
    fn parse_array(&mut self) -> Result<Expression, FCMCError> {
        self.consume(TokenKind::LBracket, "Expected '['")?;
        
        let mut elements = Vec::new();
        if !self.check(TokenKind::RBracket) {
            loop {
                elements.push(self.parse_expression()?);
                if !self.check(TokenKind::Comma) {
                    break;
                }
                self.advance(); // Consume comma
            }
        }
        
        self.consume(TokenKind::RBracket, "Expected ']'")?;
        
        Ok(Expression::Array(elements))
    }
    
    fn parse_type(&mut self) -> Result<Type, FCMCError> {
        match self.peek().kind {
            TokenKind::Field => {
                self.advance();
                Ok(Type::Field)
            }
            TokenKind::Bool => {
                self.advance();
                Ok(Type::Bool)
            }
            TokenKind::U32 => {
                self.advance();
                Ok(Type::U32)
            }
            TokenKind::Identifier => {
                let name = self.advance().lexeme.clone();
                if self.check(TokenKind::LBracket) {
                    self.advance(); // Consume '['
                    let size = match self.parse_expression()? {
                        Expression::Literal(Literal::Number(n)) => n.parse().unwrap_or(0),
                        _ => return Err(FCMCError::ParseError("Expected array size".to_string())),
                    };
                    self.consume(TokenKind::RBracket, "Expected ']'")?;
                    Ok(Type::Array(Box::new(Type::from_name(&name)?), size))
                } else {
                    Type::from_name(&name)
                }
            }
            _ => Err(FCMCError::ParseError(
                format!("Expected type, found: {:?}", self.peek())
            )),
        }
    }
    
    fn parse_if_statement(&mut self) -> Result<Statement, FCMCError> {
        self.consume(TokenKind::If, "Expected 'if'")?;
        
        let condition = self.parse_expression()?;
        
        self.consume(TokenKind::LBrace, "Expected '{'")?;
        let then_branch = self.parse_block()?;
        self.consume(TokenKind::RBrace, "Expected '}'")?;
        
        let else_branch = if self.check(TokenKind::Else) {
            self.advance(); // Consume 'else'
            if self.check(TokenKind::LBrace) {
                self.consume(TokenKind::LBrace, "Expected '{'")?;
                let block = self.parse_block()?;
                self.consume(TokenKind::RBrace, "Expected '}'")?;
                Some(block)
            } else if self.check(TokenKind::If) {
                Some(vec![self.parse_if_statement()?])
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    
    fn parse_for_statement(&mut self) -> Result<Statement, FCMCError> {
        self.consume(TokenKind::For, "Expected 'for'")?;
        
        let var_name = match self.consume_identifier()? {
            Some(ident) => ident,
            None => return Err(FCMCError::ParseError("Expected loop variable".to_string())),
        };
        
        self.consume(TokenKind::In, "Expected 'in'")?;
        
        let start = self.parse_expression()?;
        self.consume(TokenKind::Range, "Expected '..'")?;
        let end = self.parse_expression()?;
        
        self.consume(TokenKind::LBrace, "Expected '{'")?;
        let body = self.parse_block()?;
        self.consume(TokenKind::RBrace, "Expected '}'")?;
        
        Ok(Statement::For {
            var_name,
            start,
            end,
            body,
        })
    }
    
    fn parse_constraint(&mut self) -> Result<Constraint, FCMCError> {
        self.consume(TokenKind::Constraint, "Expected 'constraint'")?;
        
        let name = match self.consume_identifier()? {
            Some(ident) => ident,
            None => return Err(FCMCError::ParseError("Expected constraint name".to_string())),
        };
        
        self.consume(TokenKind::LParen, "Expected '('")?;
        
        let mut params = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                let param_name = match self.consume_identifier()? {
                    Some(ident) => ident,
                    None => break,
                };
                
                self.consume(TokenKind::Colon, "Expected ':'")?;
                let param_type = self.parse_type()?;
                params.push((param_name, param_type));
                
                if !self.check(TokenKind::Comma) {
                    break;
                }
                self.advance(); // Consume comma
            }
        }
        
        self.consume(TokenKind::RParen, "Expected ')'")?;
        
        self.consume(TokenKind::LBrace, "Expected '{'")?;
        let body = self.parse_expression()?;
        self.consume(TokenKind::RBrace, "Expected '}'")?;
        
        Ok(Constraint {
            name,
            params,
            body,
        })
    }
    
    // Helper methods
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.position += 1;
        }
        &self.tokens[self.position - 1]
    }
    
    fn check(&self, kind: TokenKind) -> bool {
        !self.is_at_end() && self.peek().kind == kind
    }
    
    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<&Token, FCMCError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(FCMCError::ParseError(message.to_string()))
        }
    }
    
    fn consume_identifier(&mut self) -> Result<Option<String>, FCMCError> {
        if self.check(TokenKind::Identifier) {
            Ok(Some(self.advance().lexeme.clone()))
        } else {
            Ok(None)
        }
    }
}
