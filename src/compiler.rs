use crate::ast::*;
use std::collections::HashMap;
use anyhow::{anyhow, Result};

pub fn compile_and_run(program: Program) -> Result<()> {
    let mut compiler = Compiler::new();
    compiler.compile_and_execute(program)?;
    Ok(())
}

pub fn compile_to_file(program: Program, output: &std::path::Path) -> Result<()> {
    let mut compiler = Compiler::new();
    compiler.compile_to_file(program, output)?;
    Ok(())
}

struct Compiler {
    globals: HashMap<String, Value>,
    functions: HashMap<String, Function>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),
    Array(Vec<Value>),
    Null,
    Function(String),
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => {
                // Format float, removing unnecessary decimals
                if f.fract() == 0.0 {
                    format!("{:.1}", f)
                } else {
                    f.to_string()
                }
            }
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Char(c) => c.to_string(),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Value::Null => "null".to_string(),
            Value::Function(name) => format!("<fn {}>", name),
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            globals: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn compile_and_execute(&mut self, program: Program) -> Result<()> {
        // First pass: collect function definitions
        for item in &program.items {
            if let Item::Function(func) = item {
                self.functions.insert(func.name.clone(), func.clone());
            }
        }

        // Second pass: execute
        for item in program.items {
            match item {
                Item::Function(_) => {}, // Already registered
                Item::Expression(expr) => {
                    self.eval_expression(&expr, &mut HashMap::new())?;
                }
                _ => {},
            }
        }

        Ok(())
    }

    fn compile_to_file(&mut self, program: Program, output: &std::path::Path) -> Result<()> {
        // Placeholder: For now, just compile and run
        self.compile_and_execute(program)?;
        std::fs::write(output, "# XP compiled output\n")?;
        Ok(())
    }

    fn eval_expression(&mut self, expr: &Expression, locals: &mut HashMap<String, Value>) -> Result<Value> {
        match expr {
            Expression::Literal(lit) => Ok(self.eval_literal(lit)),
            Expression::Identifier(name) => {
                if let Some(val) = locals.get(name) {
                    Ok(val.clone())
                } else if let Some(val) = self.globals.get(name) {
                    Ok(val.clone())
                } else {
                    Err(anyhow!("Undefined variable: {}", name))
                }
            }
            Expression::Binary(op, left, right) => {
                self.eval_binary(*op.clone(), left, right, locals)
            }
            Expression::Unary(op, expr) => {
                self.eval_unary(op.clone(), expr, locals)
            }
            Expression::Call(func_expr, args) => {
                self.eval_call(func_expr, args, locals)
            }
            Expression::Array(elements) => {
                let mut array = Vec::new();
                for elem in elements {
                    array.push(self.eval_expression(elem, locals)?);
                }
                Ok(Value::Array(array))
            }
            Expression::Conditional(cond, then_expr, else_expr) => {
                let cond_val = self.eval_expression(cond, locals)?;
                if cond_val.is_truthy() {
                    self.eval_expression(then_expr, locals)
                } else {
                    self.eval_expression(else_expr, locals)
                }
            }
            Expression::Index(array_expr, idx_expr) => {
                let array = self.eval_expression(array_expr, locals)?;
                let idx = self.eval_expression(idx_expr, locals)?;
                if let (Value::Array(arr), Value::Integer(i)) = (array, idx) {
                    if i < 0 || i as usize >= arr.len() {
                        return Err(anyhow!("Index out of bounds: {}", i));
                    }
                    Ok(arr[i as usize].clone())
                } else {
                    Err(anyhow!("Invalid array indexing"))
                }
            }
            Expression::Block(statements) => {
                let mut result = Value::Null;
                for stmt in statements {
                    result = self.eval_statement(stmt, locals)?;
                }
                Ok(result)
            }
            Expression::Assignment(left, right) => {
                let value = self.eval_expression(right, locals)?;
                if let Expression::Identifier(name) = &**left {
                    locals.insert(name.clone(), value.clone());
                    Ok(value)
                } else {
                    Err(anyhow!("Invalid assignment target"))
                }
            }
            _ => Err(anyhow!("Unsupported expression type")),
        }
    }

    fn eval_statement(&mut self, stmt: &Statement, locals: &mut HashMap<String, Value>) -> Result<Value> {
        match stmt {
            Statement::Expression(expr) => self.eval_expression(expr, locals),
            Statement::Let(let_stmt) => {
                let value = self.eval_expression(&let_stmt.value, locals)?;
                locals.insert(let_stmt.name.clone(), value.clone());
                Ok(value)
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    self.eval_expression(e, locals)
                } else {
                    Ok(Value::Null)
                }
            }
            Statement::If(if_stmt) => {
                let cond = self.eval_expression(&if_stmt.condition, locals)?;
                if cond.is_truthy() {
                    self.eval_statement(&if_stmt.then_branch, locals)
                } else if let Some(else_branch) = &if_stmt.else_branch {
                    self.eval_statement(else_branch, locals)
                } else {
                    Ok(Value::Null)
                }
            }
            Statement::Block(statements) => {
                let mut result = Value::Null;
                for stmt in statements {
                    result = self.eval_statement(stmt, locals)?;
                }
                Ok(result)
            }
            _ => Ok(Value::Null),
        }
    }

    fn eval_literal(&self, lit: &Literal) -> Value {
        match lit {
            Literal::Integer(i) => Value::Integer(*i),
            Literal::Float(f) => Value::Float(*f),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Char(c) => Value::Char(*c),
            Literal::Bool(b) => Value::Bool(*b),
            Literal::Null => Value::Null,
        }
    }

    fn eval_binary(
        &mut self,
        op: BinaryOp,
        left: &Expression,
        right: &Expression,
        locals: &mut HashMap<String, Value>,
    ) -> Result<Value> {
        let left_val = self.eval_expression(left, locals)?;
        let right_val = self.eval_expression(right, locals)?;

        match op {
            BinaryOp::Add => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + b as f64)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                _ => Err(anyhow!("Cannot add these types")),
            },
            BinaryOp::Subtract => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a - b as f64)),
                _ => Err(anyhow!("Cannot subtract these types")),
            },
            BinaryOp::Multiply => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a * b as f64)),
                _ => Err(anyhow!("Cannot multiply these types")),
            },
            BinaryOp::Divide => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => {
                    if b == 0 {
                        Err(anyhow!("Division by zero"))
                    } else {
                        Ok(Value::Integer(a / b))
                    }
                }
                (Value::Float(a), Value::Float(b)) => {
                    if b == 0.0 {
                        Err(anyhow!("Division by zero"))
                    } else {
                        Ok(Value::Float(a / b))
                    }
                }
                (Value::Integer(a), Value::Float(b)) => {
                    if b == 0.0 {
                        Err(anyhow!("Division by zero"))
                    } else {
                        Ok(Value::Float(a as f64 / b))
                    }
                }
                (Value::Float(a), Value::Integer(b)) => {
                    if b == 0 {
                        Err(anyhow!("Division by zero"))
                    } else {
                        Ok(Value::Float(a / b as f64))
                    }
                }
                _ => Err(anyhow!("Cannot divide these types")),
            },
            BinaryOp::Modulo => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => {
                    if b == 0 {
                        Err(anyhow!("Division by zero"))
                    } else {
                        Ok(Value::Integer(a % b))
                    }
                }
                _ => Err(anyhow!("Cannot modulo these types")),
            },
            BinaryOp::Equal => Ok(Value::Bool(self.values_equal(&left_val, &right_val))),
            BinaryOp::NotEqual => Ok(Value::Bool(!self.values_equal(&left_val, &right_val))),
            BinaryOp::Less => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a < b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Bool(a < (b as f64))),
                _ => Err(anyhow!("Cannot compare these types")),
            },
            BinaryOp::LessEqual => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a <= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Bool((a as f64) <= b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Bool(a <= (b as f64))),
                _ => Err(anyhow!("Cannot compare these types")),
            },
            BinaryOp::Greater => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a > b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Bool((a as f64) > b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Bool(a > (b as f64))),
                _ => Err(anyhow!("Cannot compare these types")),
            },
            BinaryOp::GreaterEqual => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a >= b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
                (Value::Integer(a), Value::Float(b)) => Ok(Value::Bool((a as f64) >= b)),
                (Value::Float(a), Value::Integer(b)) => Ok(Value::Bool(a >= (b as f64))),
                _ => Err(anyhow!("Cannot compare these types")),
            },
            BinaryOp::LogicalAnd => Ok(Value::Bool(left_val.is_truthy() && right_val.is_truthy())),
            BinaryOp::LogicalOr => Ok(Value::Bool(left_val.is_truthy() || right_val.is_truthy())),
            BinaryOp::BitwiseAnd => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a & b)),
                _ => Err(anyhow!("Cannot perform bitwise AND on these types")),
            },
            BinaryOp::BitwiseOr => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a | b)),
                _ => Err(anyhow!("Cannot perform bitwise OR on these types")),
            },
            BinaryOp::BitwiseXor => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a ^ b)),
                _ => Err(anyhow!("Cannot perform bitwise XOR on these types")),
            },
            BinaryOp::LeftShift => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a << b)),
                _ => Err(anyhow!("Cannot perform left shift on these types")),
            },
            BinaryOp::RightShift => match (left_val, right_val) {
                (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a >> b)),
                _ => Err(anyhow!("Cannot perform right shift on these types")),
            },
        }
    }

    fn eval_unary(
        &mut self,
        op: UnaryOp,
        expr: &Expression,
        locals: &mut HashMap<String, Value>,
    ) -> Result<Value> {
        let val = self.eval_expression(expr, locals)?;
        match op {
            UnaryOp::Negate => match val {
                Value::Integer(i) => Ok(Value::Integer(-i)),
                Value::Float(f) => Ok(Value::Float(-f)),
                _ => Err(anyhow!("Cannot negate this type")),
            },
            UnaryOp::Not => Ok(Value::Bool(!val.is_truthy())),
            UnaryOp::BitwiseNot => match val {
                Value::Integer(i) => Ok(Value::Integer(!i)),
                _ => Err(anyhow!("Cannot bitwise NOT this type")),
            },
            _ => Err(anyhow!("Unsupported unary operation")),
        }
    }

    fn eval_call(
        &mut self,
        func_expr: &Expression,
        args: &[Expression],
        locals: &mut HashMap<String, Value>,
    ) -> Result<Value> {
        // Handle built-in functions
        if let Expression::Identifier(name) = func_expr {
            match name.as_str() {
                "print" => {
                    for (i, arg) in args.iter().enumerate() {
                        let val = self.eval_expression(arg, locals)?;
                        if i > 0 {
                            print!(" ");
                        }
                        print!("{}", val.to_string());
                    }
                    println!();
                    return Ok(Value::Null);
                }
                _ => {},
            }

            // User-defined functions
            if let Some(func) = self.functions.get(name).cloned() {
                let mut func_locals = HashMap::new();

                // Bind arguments to parameters
                if args.len() != func.params.len() {
                    return Err(anyhow!("Function {} expects {} arguments, got {}",
                        name, func.params.len(), args.len()));
                }

                for (param, arg) in func.params.iter().zip(args.iter()) {
                    let arg_val = self.eval_expression(arg, locals)?;
                    func_locals.insert(param.name.clone(), arg_val);
                }

                // Execute function body
                match func.body {
                    FunctionBody::Expression(expr) => {
                        self.eval_expression(&expr, &mut func_locals)
                    }
                    FunctionBody::Block(stmts) => {
                        let mut result = Value::Null;
                        for stmt in stmts {
                            result = self.eval_statement(&stmt, &mut func_locals)?;
                        }
                        Ok(result)
                    }
                }
            } else {
                Err(anyhow!("Undefined function: {}", name))
            }
        } else {
            Err(anyhow!("Invalid function call"))
        }
    }

    fn values_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Char(a), Value::Char(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
