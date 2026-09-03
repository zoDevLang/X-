use crate::ast::*;
use crate::lexer::token::{Token, TokenKind};
use anyhow::{anyhow, Result};
use std::collections::VecDeque;

pub fn parse(tokens: Vec<Token>) -> Result<Program> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: VecDeque::from(tokens),
        }
    }

    fn parse_program(&mut self) -> Result<Program> {
        let mut items = Vec::new();

        while !self.is_eof() {
            self.skip_semicolons();
            if self.is_eof() {
                break;
            }

            items.push(self.parse_item()?);
            self.skip_semicolons();
        }

        Ok(Program { items })
    }

    fn parse_item(&mut self) -> Result<Item> {
        match &self.peek().kind {
            TokenKind::Fn => Ok(Item::Function(self.parse_function()?)),
            TokenKind::Struct => Ok(Item::Struct(self.parse_struct()?)),
            TokenKind::Class => Ok(Item::Class(self.parse_class()?)),
            TokenKind::Enum => Ok(Item::Enum(self.parse_enum()?)),
            TokenKind::Interface => Ok(Item::Interface(self.parse_interface()?)),
            TokenKind::Module => Ok(Item::Module(self.parse_module()?)),
            TokenKind::Import => Ok(Item::Import(self.parse_import()?)),
            _ => Ok(Item::Expression(self.parse_expression()?)),
        }
    }

    fn parse_function(&mut self) -> Result<Function> {
        let position = self.peek().position.clone();
        let is_async = if self.check(TokenKind::Async) {
            self.advance();
            true
        } else {
            false
        };

        self.consume(TokenKind::Fn, "Expected 'fn'")?;
        let name = self.parse_identifier()?;

        self.consume(TokenKind::LParen, "Expected '('")?;
        let params = self.parse_parameters()?;
        self.consume(TokenKind::RParen, "Expected ')'")?;

        let return_type = if self.check(TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(TokenKind::FatArrow, "Expected '=>'")?;

        let body = if self.check(TokenKind::LBrace) {
            self.advance();
            let statements = self.parse_block()?;
            self.consume(TokenKind::RBrace, "Expected '}'")?;
            FunctionBody::Block(statements)
        } else {
            let expr = self.parse_expression()?;
            FunctionBody::Expression(Box::new(expr))
        };

        Ok(Function {
            name,
            params,
            return_type,
            body,
            is_async,
            position,
        })
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>> {
        let mut params = Vec::new();

        if !self.check(TokenKind::RParen) {
            loop {
                let is_mutable = if self.check(TokenKind::Mut) {
                    self.advance();
                    true
                } else {
                    false
                };

                let name = self.parse_identifier()?;
                self.consume(TokenKind::Colon, "Expected ':' in parameter")?;
                let param_type = self.parse_type()?;

                params.push(Parameter {
                    name,
                    param_type,
                    is_mutable,
                });

                if !self.check(TokenKind::Comma) {
                    break;
                }
                self.advance();
            }
        }

        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Type> {
        let mut base_type = self.parse_base_type()?;

        loop {
            if self.check(TokenKind::LBracket) {
                self.advance();
                self.consume(TokenKind::RBracket, "Expected ']'")?;
                base_type = Type::Array(Box::new(base_type));
            } else if self.check(TokenKind::Star) {
                self.advance();
                base_type = Type::Pointer(Box::new(base_type));
            } else if self.check(TokenKind::Amp) {
                self.advance();
                base_type = Type::Reference(Box::new(base_type));
            } else {
                break;
            }
        }

        Ok(base_type)
    }

    fn parse_base_type(&mut self) -> Result<Type> {
        let token = self.peek();
        match &token.kind {
            TokenKind::Identifier => {
                let name = self.parse_identifier()?;
                if self.check(TokenKind::Less) {
                    self.advance();
                    let mut args = vec![self.parse_type()?];
                    while self.check(TokenKind::Comma) {
                        self.advance();
                        args.push(self.parse_type()?);
                    }
                    self.consume(TokenKind::Greater, "Expected '>'")?;
                    Ok(Type::Generic(name, args))
                } else {
                    Ok(Type::Custom(name))
                }
            }
            TokenKind::Fn => {
                self.advance();
                self.consume(TokenKind::LParen, "Expected '('")?;
                let mut params = Vec::new();
                if !self.check(TokenKind::RParen) {
                    loop {
                        params.push(self.parse_type()?);
                        if !self.check(TokenKind::Comma) {
                            break;
                        }
                        self.advance();
                    }
                }
                self.consume(TokenKind::RParen, "Expected ')'")?;
                self.consume(TokenKind::Colon, "Expected ':' in function type")?;
                let return_type = Box::new(self.parse_type()?);
                Ok(Type::Function(params, return_type))
            }
            _ => {
                let type_str = format!("{:?}", token.kind);
                self.advance();
                match type_str.as_str() {
                    "Int" => Ok(Type::Int),
                    "UInt" => Ok(Type::UInt),
                    "Int8" => Ok(Type::Int8),
                    "Int16" => Ok(Type::Int16),
                    "Int32" => Ok(Type::Int32),
                    "Int64" => Ok(Type::Int64),
                    "UInt8" => Ok(Type::UInt8),
                    "UInt16" => Ok(Type::UInt16),
                    "UInt32" => Ok(Type::UInt32),
                    "UInt64" => Ok(Type::UInt64),
                    "Float" => Ok(Type::Float),
                    "Float32" => Ok(Type::Float32),
                    "Float64" => Ok(Type::Float64),
                    "Bool" => Ok(Type::Bool),
                    "Char" => Ok(Type::Char),
                    "String" => Ok(Type::String),
                    "Byte" => Ok(Type::Byte),
                    "Void" => Ok(Type::Void),
                    _ => Err(anyhow!("Unknown type: {}", type_str)),
                }
            }
        }
    }

    fn parse_struct(&mut self) -> Result<Struct> {
        let position = self.peek().position.clone();
        self.consume(TokenKind::Struct, "Expected 'struct'")?;
        let name = self.parse_identifier()?;
        self.consume(TokenKind::LBrace, "Expected '{'")?;

        let mut fields = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            let field_name = self.parse_identifier()?;
            self.consume(TokenKind::Colon, "Expected ':'")?;
            let field_type = self.parse_type()?;
            fields.push(StructField {
                name: field_name,
                field_type,
                is_mutable: false,
            });

            if self.check(TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        self.consume(TokenKind::RBrace, "Expected '}'")?;

        Ok(Struct {
            name,
            fields,
            position,
        })
    }

    fn parse_class(&mut self) -> Result<Class> {
        let position = self.peek().position.clone();
        self.consume(TokenKind::Class, "Expected 'class'")?;
        let name = self.parse_identifier()?;
        self.consume(TokenKind::LBrace, "Expected '{'")?;

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            if self.check(TokenKind::Fn) {
                methods.push(self.parse_function()?);
            } else {
                let is_mutable = self.check(TokenKind::Mut);
                if is_mutable {
                    self.advance();
                }
                let field_name = self.parse_identifier()?;
                self.consume(TokenKind::Colon, "Expected ':'")?;
                let field_type = self.parse_type()?;
                fields.push(ClassField {
                    name: field_name,
                    field_type,
                    is_mutable,
                });
            }

            self.skip_semicolons();
        }

        self.consume(TokenKind::RBrace, "Expected '}'")?;

        Ok(Class {
            name,
            fields,
            methods,
            position,
        })
    }

    fn parse_enum(&mut self) -> Result<Enum> {
        let position = self.peek().position.clone();
        self.consume(TokenKind::Enum, "Expected 'enum'")?;
        let name = self.parse_identifier()?;
        self.consume(TokenKind::LBrace, "Expected '{'")?;

        let mut variants = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            variants.push(self.parse_identifier()?);
            if self.check(TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        self.consume(TokenKind::RBrace, "Expected '}'")?;

        Ok(Enum {
            name,
            variants,
            position,
        })
    }

    fn parse_interface(&mut self) -> Result<Interface> {
        let position = self.peek().position.clone();
        self.consume(TokenKind::Interface, "Expected 'interface'")?;
        let name = self.parse_identifier()?;
        self.consume(TokenKind::LBrace, "Expected '{'")?;

        let mut methods = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            self.consume(TokenKind::Fn, "Expected 'fn'")?;
            let method_name = self.parse_identifier()?;
            self.consume(TokenKind::LParen, "Expected '('")?;
            let params = self.parse_parameters()?;
            self.consume(TokenKind::RParen, "Expected ')'")?;
            self.consume(TokenKind::Colon, "Expected ':'")?;
            let return_type = self.parse_type()?;

            methods.push(FunctionSignature {
                name: method_name,
                params,
                return_type,
            });

            self.skip_semicolons();
        }

        self.consume(TokenKind::RBrace, "Expected '}'")?;

        Ok(Interface {
            name,
            methods,
            position,
        })
    }

    fn parse_module(&mut self) -> Result<Module> {
        self.consume(TokenKind::Module, "Expected 'module'")?;
        let name = self.parse_identifier()?;
        self.skip_semicolons();

        let mut items = Vec::new();
        while !self.is_eof() && !self.check(TokenKind::Module) {
            items.push(self.parse_item()?);
            self.skip_semicolons();
        }

        Ok(Module { name, items })
    }

    fn parse_import(&mut self) -> Result<Import> {
        self.consume(TokenKind::Import, "Expected 'import'")?;
        let module = self.parse_identifier()?;

        let mut items = None;
        if self.check(TokenKind::Dot) {
            self.advance();
            self.consume(TokenKind::LBrace, "Expected '{'")?;
            let mut import_items = Vec::new();
            loop {
                import_items.push(self.parse_identifier()?);
                if !self.check(TokenKind::Comma) {
                    break;
                }
                self.advance();
            }
            self.consume(TokenKind::RBrace, "Expected '}'")?;
            items = Some(import_items);
        }

        let mut alias = None;
        if self.check(TokenKind::As) {
            self.advance();
            alias = Some(self.parse_identifier()?);
        }

        Ok(Import {
            module,
            items,
            alias,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            self.skip_semicolons();
            if self.check(TokenKind::RBrace) {
                break;
            }
            statements.push(self.parse_statement()?);
            self.skip_semicolons();
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match &self.peek().kind {
            TokenKind::Let => {
                self.advance();
                let is_const = false;
                let name = self.parse_identifier()?;
                let declared_type = if self.check(TokenKind::Colon) {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                self.consume(TokenKind::FatArrow, "Expected '=>'")?;
                let value = self.parse_expression()?;
                Ok(Statement::Let(Let {
                    name,
                    declared_type,
                    value,
                    is_mutable: false,
                    is_const,
                }))
            }
            TokenKind::Mut => {
                self.advance();
                let name = self.parse_identifier()?;
                let declared_type = if self.check(TokenKind::Colon) {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                self.consume(TokenKind::FatArrow, "Expected '=>'")?;
                let value = self.parse_expression()?;
                Ok(Statement::Let(Let {
                    name,
                    declared_type,
                    value,
                    is_mutable: true,
                    is_const: false,
                }))
            }
            TokenKind::Const => {
                self.advance();
                let name = self.parse_identifier()?;
                let declared_type = if self.check(TokenKind::Colon) {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                self.consume(TokenKind::FatArrow, "Expected '=>'")?;
                let value = self.parse_expression()?;
                Ok(Statement::Let(Let {
                    name,
                    declared_type,
                    value,
                    is_mutable: false,
                    is_const: true,
                }))
            }
            TokenKind::If => {
                self.advance();
                let condition = self.parse_expression()?;
                self.consume(TokenKind::LBrace, "Expected '{'")?;
                let then_stmts = self.parse_block()?;
                self.consume(TokenKind::RBrace, "Expected '}'")?;

                let else_branch = if self.check(TokenKind::Else) {
                    self.advance();
                    if self.check(TokenKind::If) {
                        Some(Box::new(self.parse_statement()?))
                    } else {
                        self.consume(TokenKind::LBrace, "Expected '{'")?;
                        let else_stmts = self.parse_block()?;
                        self.consume(TokenKind::RBrace, "Expected '}'")?;
                        Some(Box::new(Statement::Block(else_stmts)))
                    }
                } else {
                    None
                };

                Ok(Statement::If(If {
                    condition,
                    then_branch: Box::new(Statement::Block(then_stmts)),
                    else_branch,
                }))
            }
            TokenKind::While => {
                self.advance();
                let condition = self.parse_expression()?;
                self.consume(TokenKind::LBrace, "Expected '{'")?;
                let body_stmts = self.parse_block()?;
                self.consume(TokenKind::RBrace, "Expected '}'")?;
                Ok(Statement::While(While {
                    condition,
                    body: Box::new(Statement::Block(body_stmts)),
                }))
            }
            TokenKind::For => {
                self.advance();
                let variable = self.parse_identifier()?;
                self.consume(TokenKind::In, "Expected 'in'")?;
                let iterable = self.parse_expression()?;
                self.consume(TokenKind::LBrace, "Expected '{'")?;
                let body_stmts = self.parse_block()?;
                self.consume(TokenKind::RBrace, "Expected '}'")?;
                Ok(Statement::For(For {
                    variable,
                    iterable,
                    body: Box::new(Statement::Block(body_stmts)),
                }))
            }
            TokenKind::Loop => {
                self.advance();
                self.consume(TokenKind::LBrace, "Expected '{'")?;
                let body_stmts = self.parse_block()?;
                self.consume(TokenKind::RBrace, "Expected '}'")?;
                Ok(Statement::Loop(Loop {
                    body: Box::new(Statement::Block(body_stmts)),
                }))
            }
            TokenKind::Break => {
                self.advance();
                Ok(Statement::Break)
            }
            TokenKind::Continue => {
                self.advance();
                Ok(Statement::Continue)
            }
            TokenKind::Return => {
                self.advance();
                let expr = if self.check(TokenKind::Semicolon)
                    || self.check(TokenKind::RBrace)
                    || self.is_eof()
                {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                Ok(Statement::Return(expr))
            }
            TokenKind::LBrace => {
                self.advance();
                let stmts = self.parse_block()?;
                self.consume(TokenKind::RBrace, "Expected '}'")?;
                Ok(Statement::Block(stmts))
            }
            _ => Ok(Statement::Expression(self.parse_expression()?)),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_ternary()
    }

    fn parse_ternary(&mut self) -> Result<Expression> {
        let mut expr = self.parse_logical_or()?;

        if self.check(TokenKind::Question) {
            self.advance();
            let then_expr = self.parse_expression()?;
            self.consume(TokenKind::Colon, "Expected ':' in ternary")?;
            let else_expr = self.parse_expression()?;
            expr = Expression::Conditional(
                Box::new(expr),
                Box::new(then_expr),
                Box::new(else_expr),
            );
        }

        Ok(expr)
    }

    fn parse_logical_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_logical_and()?;

        while self.check(TokenKind::PipePipe) {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expression::Binary(BinaryOp::LogicalOr, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_bitwise_or()?;

        while self.check(TokenKind::AmpAmp) {
            self.advance();
            let right = self.parse_bitwise_or()?;
            left = Expression::Binary(BinaryOp::LogicalAnd, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_bitwise_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_bitwise_xor()?;

        while self.check(TokenKind::Pipe) {
            self.advance();
            let right = self.parse_bitwise_xor()?;
            left = Expression::Binary(BinaryOp::BitwiseOr, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_bitwise_xor(&mut self) -> Result<Expression> {
        let mut left = self.parse_bitwise_and()?;

        while self.check(TokenKind::Caret) {
            self.advance();
            let right = self.parse_bitwise_and()?;
            left = Expression::Binary(BinaryOp::BitwiseXor, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_bitwise_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_equality()?;

        while self.check(TokenKind::Amp) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::Binary(BinaryOp::BitwiseAnd, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expression> {
        let mut left = self.parse_comparison()?;

        while let Some(op) = match &self.peek().kind {
            TokenKind::EqualEqual => Some(BinaryOp::Equal),
            TokenKind::BangEqual => Some(BinaryOp::NotEqual),
            _ => None,
        } {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expression::Binary(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression> {
        let mut left = self.parse_shift()?;

        while let Some(op) = match &self.peek().kind {
            TokenKind::Less => Some(BinaryOp::Less),
            TokenKind::LessEqual => Some(BinaryOp::LessEqual),
            TokenKind::Greater => Some(BinaryOp::Greater),
            TokenKind::GreaterEqual => Some(BinaryOp::GreaterEqual),
            _ => None,
        } {
            self.advance();
            let right = self.parse_shift()?;
            left = Expression::Binary(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_shift(&mut self) -> Result<Expression> {
        let mut left = self.parse_additive()?;

        while let Some(op) = match &self.peek().kind {
            TokenKind::LeftShift => Some(BinaryOp::LeftShift),
            TokenKind::RightShift => Some(BinaryOp::RightShift),
            _ => None,
        } {
            self.advance();
            let right = self.parse_additive()?;
            left = Expression::Binary(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expression> {
        let mut left = self.parse_multiplicative()?;

        while let Some(op) = match &self.peek().kind {
            TokenKind::Plus => Some(BinaryOp::Add),
            TokenKind::Minus => Some(BinaryOp::Subtract),
            _ => None,
        } {
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expression::Binary(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression> {
        let mut left = self.parse_unary()?;

        while let Some(op) = match &self.peek().kind {
            TokenKind::Star => Some(BinaryOp::Multiply),
            TokenKind::Slash => Some(BinaryOp::Divide),
            TokenKind::Percent => Some(BinaryOp::Modulo),
            _ => None,
        } {
            self.advance();
            let right = self.parse_unary()?;
            left = Expression::Binary(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression> {
        match &self.peek().kind {
            TokenKind::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Unary(UnaryOp::Negate, Box::new(expr)))
            }
            TokenKind::Bang => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Unary(UnaryOp::Not, Box::new(expr)))
            }
            TokenKind::Tilde => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Unary(UnaryOp::BitwiseNot, Box::new(expr)))
            }
            TokenKind::Star => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Unary(UnaryOp::Dereference, Box::new(expr)))
            }
            TokenKind::Amp => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expression::Unary(UnaryOp::Reference, Box::new(expr)))
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression> {
        let mut expr = self.parse_primary()?;

        loop {
            match &self.peek().kind {
                TokenKind::LParen => {
                    self.advance();
                    let args = self.parse_arguments()?;
                    self.consume(TokenKind::RParen, "Expected ')'")?;
                    expr = Expression::Call(Box::new(expr), args);
                }
                TokenKind::LBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.consume(TokenKind::RBracket, "Expected ']'")?;
                    expr = Expression::Index(Box::new(expr), Box::new(index));
                }
                TokenKind::Dot => {
                    self.advance();
                    let field = self.parse_identifier()?;
                    expr = Expression::Field(Box::new(expr), field);
                }
                TokenKind::Increment => {
                    self.advance();
                    expr = Expression::PostfixOp(Box::new(expr), PostfixOp::Increment);
                }
                TokenKind::Decrement => {
                    self.advance();
                    expr = Expression::PostfixOp(Box::new(expr), PostfixOp::Decrement);
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        let token = self.peek();
        match &token.kind {
            TokenKind::Integer => {
                let value = token.value.as_ref().unwrap().parse::<i64>()?;
                self.advance();
                Ok(Expression::Literal(Literal::Integer(value)))
            }
            TokenKind::Float => {
                let value = token.value.as_ref().unwrap().parse::<f64>()?;
                self.advance();
                Ok(Expression::Literal(Literal::Float(value)))
            }
            TokenKind::String => {
                let value = token.value.as_ref().unwrap().clone();
                self.advance();
                Ok(Expression::Literal(Literal::String(value)))
            }
            TokenKind::Char => {
                let value = token.value.as_ref().unwrap().chars().next().unwrap();
                self.advance();
                Ok(Expression::Literal(Literal::Char(value)))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expression::Literal(Literal::Bool(true)))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expression::Literal(Literal::Bool(false)))
            }
            TokenKind::Null => {
                self.advance();
                Ok(Expression::Literal(Literal::Null))
            }
            TokenKind::Identifier => {
                let name = self.parse_identifier()?;
                Ok(Expression::Identifier(name))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenKind::RParen, "Expected ')'")?;
                Ok(expr)
            }
            TokenKind::LBracket => {
                self.advance();
                let mut elements = Vec::new();
                if !self.check(TokenKind::RBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if !self.check(TokenKind::Comma) {
                            break;
                        }
                        self.advance();
                    }
                }
                self.consume(TokenKind::RBracket, "Expected ']'")?;
                Ok(Expression::Array(elements))
            }
            _ => Err(anyhow!(
                "Unexpected token in expression: {:?} at {}",
                token.kind,
                token.position
            )),
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression>> {
        let mut args = Vec::new();

        if !self.check(TokenKind::RParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.check(TokenKind::Comma) {
                    break;
                }
                self.advance();
            }
        }

        Ok(args)
    }

    fn parse_identifier(&mut self) -> Result<String> {
        let token = self.peek();
        match &token.kind {
            TokenKind::Identifier => {
                let name = token.value.as_ref().unwrap().clone();
                self.advance();
                Ok(name)
            }
            _ => Err(anyhow!("Expected identifier, got {:?}", token.kind)),
        }
    }

    fn peek(&self) -> Token {
        self.tokens.front().cloned().unwrap_or_else(|| {
            Token::new(
                TokenKind::Eof,
                crate::lexer::position::Position::new(),
            )
        })
    }

    fn advance(&mut self) -> Token {
        self.tokens
            .pop_front()
            .unwrap_or_else(|| Token::new(TokenKind::Eof, crate::lexer::position::Position::new()))
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<()> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            Err(anyhow!("{} at {}", message, self.peek().position))
        }
    }

    fn skip_semicolons(&mut self) {
        while self.check(TokenKind::Semicolon) {
            self.advance();
        }
    }

    fn is_eof(&self) -> bool {
        self.peek().is_eof()
    }
}
