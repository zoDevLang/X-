use crate::lexer::position::Position;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Class(Class),
    Enum(Enum),
    Interface(Interface),
    Module(Module),
    Import(Import),
    Expression(Expression),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: FunctionBody,
    pub is_async: bool,
    pub position: Position,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub is_mutable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Int,
    UInt,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float,
    Float32,
    Float64,
    Bool,
    Char,
    String,
    Byte,
    Void,
    Any,
    Never,
    Array(Box<Type>),
    Pointer(Box<Type>),
    Reference(Box<Type>),
    Custom(String),
    Generic(String, Vec<Type>),
    Function(Vec<Type>, Box<Type>), // params, return type
}

impl Type {
    pub fn as_str(&self) -> String {
        match self {
            Type::Int => "Int".to_string(),
            Type::UInt => "UInt".to_string(),
            Type::Int8 => "Int8".to_string(),
            Type::Int16 => "Int16".to_string(),
            Type::Int32 => "Int32".to_string(),
            Type::Int64 => "Int64".to_string(),
            Type::UInt8 => "UInt8".to_string(),
            Type::UInt16 => "UInt16".to_string(),
            Type::UInt32 => "UInt32".to_string(),
            Type::UInt64 => "UInt64".to_string(),
            Type::Float => "Float".to_string(),
            Type::Float32 => "Float32".to_string(),
            Type::Float64 => "Float64".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::Char => "Char".to_string(),
            Type::String => "String".to_string(),
            Type::Byte => "Byte".to_string(),
            Type::Void => "Void".to_string(),
            Type::Any => "Any".to_string(),
            Type::Never => "Never".to_string(),
            Type::Array(t) => format!("{}[]", t.as_str()),
            Type::Pointer(t) => format!("*{}", t.as_str()),
            Type::Reference(t) => format!("&{}", t.as_str()),
            Type::Custom(name) => name.clone(),
            Type::Generic(name, args) => {
                let args_str = args.iter().map(|t| t.as_str()).collect::<Vec<_>>().join(",");
                format!("{}\u{003c}{}\u{003e}", name, args_str)
            }
            Type::Function(params, ret) => {
                let params_str = params.iter().map(|t| t.as_str()).collect::<Vec<_>>().join(",");
                format!("fn({}):{}", params_str, ret.as_str())
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FunctionBody {
    Expression(Box<Expression>),
    Block(Vec<Statement>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Let(Let),
    Return(Option<Expression>),
    If(If),
    While(While),
    For(For),
    Loop(Loop),
    Break,
    Continue,
    Block(Vec<Statement>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Let {
    pub name: String,
    pub declared_type: Option<Type>,
    pub value: Expression,
    pub is_mutable: bool,
    pub is_const: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct If {
    pub condition: Expression,
    pub then_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct While {
    pub condition: Expression,
    pub body: Box<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct For {
    pub variable: String,
    pub iterable: Expression,
    pub body: Box<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Loop {
    pub body: Box<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Binary(BinaryOp, Box<Expression>, Box<Expression>),
    Unary(UnaryOp, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Index(Box<Expression>, Box<Expression>),
    Field(Box<Expression>, String),
    Array(Vec<Expression>),
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
    Lambda(Vec<Parameter>, Box<Expression>),
    Block(Vec<Statement>),
    Assignment(Box<Expression>, Box<Expression>),
    PostfixOp(Box<Expression>, PostfixOp),
    Pipeline(Vec<Expression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
    BitwiseNot,
    Dereference,
    Reference,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PostfixOp {
    Increment,
    Decrement,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
    pub position: Position,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructField {
    pub name: String,
    pub field_type: Type,
    pub is_mutable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub name: String,
    pub fields: Vec<ClassField>,
    pub methods: Vec<Function>,
    pub position: Position,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClassField {
    pub name: String,
    pub field_type: Type,
    pub is_mutable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
    pub position: Position,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Interface {
    pub name: String,
    pub methods: Vec<FunctionSignature>,
    pub position: Position,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    pub module: String,
    pub items: Option<Vec<String>>,
    pub alias: Option<String>,
}

impl Program {
    pub fn new() -> Self {
        Program { items: Vec::new() }
    }

    pub fn format(&self) -> String {
        self.items
            .iter()
            .map(|item| item.format())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl Item {
    pub fn format(&self) -> String {
        match self {
            Item::Function(f) => f.format(),
            Item::Struct(s) => s.format(),
            Item::Class(c) => c.format(),
            Item::Enum(e) => e.format(),
            Item::Interface(i) => i.format(),
            Item::Module(m) => m.format(),
            Item::Import(imp) => imp.format(),
            Item::Expression(expr) => expr.format(),
        }
    }
}

impl Function {
    pub fn format(&self) -> String {
        let params = self
            .params
            .iter()
            .map(|p| format!("{}:{}", p.name, p.param_type.as_str()))
            .collect::<Vec<_>>()
            .join(",");

        let return_type = self
            .return_type
            .as_ref()
            .map(|t| format!(":{}", t.as_str()))
            .unwrap_or_default();

        let async_str = if self.is_async { "async " } else { "" };

        match &self.body {
            FunctionBody::Expression(expr) => {
                format!("{}fn {}({}){}=>{}", async_str, self.name, params, return_type, expr.format())
            }
            FunctionBody::Block(stmts) => {
                let body = stmts
                    .iter()
                    .map(|s| format!("    {}", s.format()))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "{}fn {}({}){}=>{{\n{}\n}}",
                    async_str, self.name, params, return_type, body
                )
            }
        }
    }
}

impl Statement {
    pub fn format(&self) -> String {
        match self {
            Statement::Expression(e) => e.format(),
            Statement::Let(l) => {
                let type_str = l
                    .declared_type
                    .as_ref()
                    .map(|t| format!(":{}", t.as_str()))
                    .unwrap_or_default();
                let mut_str = if l.is_mutable { "mut " } else { "" };
                let const_str = if l.is_const { "const " } else { "" };
                format!("{}{}{}{}=>{}", const_str, mut_str, l.name, type_str, l.value.format())
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    format!("return {}", e.format())
                } else {
                    "return".to_string()
                }
            }
            Statement::If(i) => i.format(),
            Statement::While(w) => w.format(),
            Statement::For(f) => f.format(),
            Statement::Loop(_) => "loop { ... }".to_string(),
            Statement::Break => "break".to_string(),
            Statement::Continue => "continue".to_string(),
            Statement::Block(stmts) => {
                let body = stmts
                    .iter()
                    .map(|s| format!("    {}", s.format()))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{{\n{}\n}}", body)
            }
        }
    }
}

impl Expression {
    pub fn format(&self) -> String {
        match self {
            Expression::Literal(l) => l.format(),
            Expression::Identifier(name) => name.clone(),
            Expression::Binary(op, left, right) => {
                format!("{}{}{}", left.format(), op.format(), right.format())
            }
            Expression::Unary(op, expr) => format!("{}{}", op.format(), expr.format()),
            Expression::Call(func, args) => {
                let args_str = args
                    .iter()
                    .map(|a| a.format())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", func.format(), args_str)
            }
            Expression::Index(expr, idx) => format!("{}[{}]", expr.format(), idx.format()),
            Expression::Field(expr, field) => format!("{}.{}", expr.format(), field),
            Expression::Array(elems) => {
                let elems_str = elems
                    .iter()
                    .map(|e| e.format())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("[{}]", elems_str)
            }
            Expression::Conditional(cond, then_expr, else_expr) => {
                format!("{}?{}:{}", cond.format(), then_expr.format(), else_expr.format())
            }
            Expression::Lambda(params, expr) => {
                let params_str = params
                    .iter()
                    .map(|p| format!("{}:{}", p.name, p.param_type.as_str()))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("({}){}", params_str, expr.format())
            }
            Expression::Block(_) => "{ ... }".to_string(),
            Expression::Assignment(left, right) => {
                format!("{}={}", left.format(), right.format())
            }
            Expression::PostfixOp(expr, op) => format!("{}{}", expr.format(), op.format()),
            Expression::Pipeline(exprs) => {
                exprs
                    .iter()
                    .map(|e| e.format())
                    .collect::<Vec<_>>()
                    .join("|>")
            }
        }
    }
}

impl Literal {
    pub fn format(&self) -> String {
        match self {
            Literal::Integer(i) => i.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::String(s) => format!("\"{}\"",'s),
            Literal::Char(c) => format!("'{}'", c),
            Literal::Bool(b) => b.to_string(),
            Literal::Null => "null".to_string(),
        }
    }
}

impl BinaryOp {
    pub fn format(&self) -> String {
        match self {
            BinaryOp::Add => " + ",
            BinaryOp::Subtract => " - ",
            BinaryOp::Multiply => " * ",
            BinaryOp::Divide => " / ",
            BinaryOp::Modulo => " % ",
            BinaryOp::Equal => " == ",
            BinaryOp::NotEqual => " != ",
            BinaryOp::Less => " < ",
            BinaryOp::LessEqual => " <= ",
            BinaryOp::Greater => " > ",
            BinaryOp::GreaterEqual => " >= ",
            BinaryOp::LogicalAnd => " && ",
            BinaryOp::LogicalOr => " || ",
            BinaryOp::BitwiseAnd => " & ",
            BinaryOp::BitwiseOr => " | ",
            BinaryOp::BitwiseXor => " ^ ",
            BinaryOp::LeftShift => " << ",
            BinaryOp::RightShift => " >> ",
        }
        .to_string()
    }
}

impl UnaryOp {
    pub fn format(&self) -> String {
        match self {
            UnaryOp::Negate => "-",
            UnaryOp::Not => "!",
            UnaryOp::BitwiseNot => "~",
            UnaryOp::Dereference => "*",
            UnaryOp::Reference => "&",
        }
        .to_string()
    }
}

impl PostfixOp {
    pub fn format(&self) -> String {
        match self {
            PostfixOp::Increment => "+",
            PostfixOp::Decrement => "-",
        }
        .to_string()
    }
}

impl Struct {
    pub fn format(&self) -> String {
        let fields = self
            .fields
            .iter()
            .map(|f| format!("    {}:{}", f.name, f.field_type.as_str()))
            .collect::<Vec<_>>()
            .join("\n");
        format!("struct {} {{\n{}\n}}", self.name, fields)
    }
}

impl Class {
    pub fn format(&self) -> String {
        format!("class {} {{ ... }}", self.name)
    }
}

impl Enum {
    pub fn format(&self) -> String {
        let variants = self.variants.join(", ");
        format!("enum {} {{ {} }}", self.name, variants)
    }
}

impl Interface {
    pub fn format(&self) -> String {
        format!("interface {} {{ ... }}", self.name)
    }
}

impl Module {
    pub fn format(&self) -> String {
        format!("module {}", self.name)
    }
}

impl Import {
    pub fn format(&self) -> String {
        if let Some(ref items) = self.items {
            format!("import {}.{{}}", self.module)
        } else {
            format!("import {}", self.module)
        }
    }
}

impl If {
    pub fn format(&self) -> String {
        let else_str = self
            .else_branch
            .as_ref()
            .map(|b| format!(" else {}", b.format()))
            .unwrap_or_default();
        format!("if {}{{ {} }}{}", self.condition.format(), self.then_branch.format(), else_str)
    }
}

impl While {
    pub fn format(&self) -> String {
        format!("while {} {{ {} }}", self.condition.format(), self.body.format())
    }
}

impl For {
    pub fn format(&self) -> String {
        format!("for {} in {} {{ {} }}", self.variable, self.iterable.format(), self.body.format())
    }
}
