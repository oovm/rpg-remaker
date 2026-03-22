use crate::{Result, RgssError};
use std::collections::HashMap;

pub struct VmState {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Box<dyn Fn(&[Value]) -> Value>>,
}

impl Clone for VmState {
    fn clone(&self) -> Self {
        Self {
            variables: self.variables.clone(),
            functions: self.functions.clone(),
        }
    }
}

impl VmState {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_function(&self, name: &str) -> Option<&Box<dyn Fn(&[Value]) -> Value>> {
        self.functions.get(name)
    }

    pub fn register_function(&mut self, name: String, func: Box<dyn Fn(&[Value]) -> Value>) {
        self.functions.insert(name, func);
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Hash(HashMap<String, Value>),
}

pub struct RgssVm {
    state: VmState,
}

impl Clone for RgssVm {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

impl RgssVm {
    pub fn new() -> Self {
        Self {
            state: VmState::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<()>
    {
        self.register_builtin_functions();
        Ok(())
    }

    pub fn execute(&mut self, script: &str) -> Result<()>
    {
        // 解析脚本
        let expressions = crate::parser::parse_script(script)?;
        
        // 执行解析后的表达式
        for expr in expressions {
            self.execute_expression(&expr)?;
        }
        
        Ok(())
    }
    
    // 执行状态，用于处理循环控制
    struct ExecutionState {
        break_flag: bool,
        next_flag: bool,
        redo_flag: bool,
    }
    
    impl ExecutionState {
        fn new() -> Self {
            Self {
                break_flag: false,
                next_flag: false,
                redo_flag: false,
            }
        }
        
        fn reset(&mut self) {
            self.break_flag = false;
            self.next_flag = false;
            self.redo_flag = false;
        }
    }
    
    fn execute_expression(&mut self, expr: &crate::parser::Expression) -> Result<()>
    {
        let mut state = ExecutionState::new();
        self.execute_expression_with_state(expr, &mut state)
    }
    
    fn execute_expression_with_state(&mut self, expr: &crate::parser::Expression, state: &mut ExecutionState) -> Result<()>
    {
        match expr {
            crate::parser::Expression::Literal(_) => {
                // 字面量不需要执行
                Ok(())
            }
            crate::parser::Expression::Identifier(_) => {
                // 标识符不需要执行
                Ok(())
            }
            crate::parser::Expression::Assignment(name, value) => {
                let value_result = self.evaluate_expression(value)?;
                self.state.set_variable(name.clone(), value_result);
                Ok(())
            }
            crate::parser::Expression::FunctionCall(name, args) => {
                // 评估参数
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.evaluate_expression(arg)?);
                }
                
                // 调用函数
                if let Some(func) = self.state.get_function(name) {
                    func(&evaluated_args);
                }
                Ok(())
            }
            crate::parser::Expression::IfExpression(condition, body, elsif_conditions, else_body) => {
                let condition_result = self.evaluate_expression(condition)?;
                
                if self.is_truthy(&condition_result) {
                    // 执行 if 分支
                    for expr in body {
                        self.execute_expression_with_state(expr, state)?;
                        if state.break_flag || state.next_flag {
                            break;
                        }
                    }
                } else if let Some(elsif_conditions) = elsif_conditions {
                    // 执行 elsif 分支
                    for elsif_body in elsif_conditions {
                        for expr in elsif_body {
                            self.execute_expression_with_state(expr, state)?;
                            if state.break_flag || state.next_flag {
                                break;
                            }
                        }
                    }
                } else if let Some(else_body) = else_body {
                    // 执行 else 分支
                    for expr in else_body {
                        self.execute_expression_with_state(expr, state)?;
                        if state.break_flag || state.next_flag {
                            break;
                        }
                    }
                }
                Ok(())
            }
            crate::parser::Expression::WhileExpression(condition, body) => {
                while self.is_truthy(&self.evaluate_expression(condition)?) {
                    state.reset();
                    for expr in body {
                        self.execute_expression_with_state(expr, state)?;
                        if state.break_flag {
                            state.break_flag = false;
                            return Ok(());
                        }
                        if state.next_flag {
                            state.next_flag = false;
                            break;
                        }
                        if state.redo_flag {
                            state.redo_flag = false;
                            break;
                        }
                    }
                    if state.redo_flag {
                        continue;
                    }
                }
                Ok(())
            }
            crate::parser::Expression::ForExpression(variable, collection, body) => {
                // 简化处理，实际需要更复杂的逻辑
                // 这里假设 collection 是一个数组
                if let Value::Array(items) = self.evaluate_expression(collection)? {
                    for item in items {
                        self.state.set_variable(variable.clone(), item);
                        state.reset();
                        for expr in body {
                            self.execute_expression_with_state(expr, state)?;
                            if state.break_flag {
                                state.break_flag = false;
                                return Ok(());
                            }
                            if state.next_flag {
                                state.next_flag = false;
                                break;
                            }
                            if state.redo_flag {
                                state.redo_flag = false;
                                break;
                            }
                        }
                        if state.redo_flag {
                            continue;
                        }
                    }
                }
                Ok(())
            }
            crate::parser::Expression::Block(body) => {
                for expr in body {
                    self.execute_expression_with_state(expr, state)?;
                    if state.break_flag || state.next_flag {
                        break;
                    }
                }
                Ok(())
            }
            crate::parser::Expression::ReturnExpression(_) => {
                // 简化处理，实际需要处理函数返回
                Ok(())
            }
            crate::parser::Expression::BreakExpression => {
                state.break_flag = true;
                Ok(())
            }
            crate::parser::Expression::NextExpression => {
                state.next_flag = true;
                Ok(())
            }
            crate::parser::Expression::RedoExpression => {
                state.redo_flag = true;
                Ok(())
            }
            crate::parser::Expression::RetryExpression => {
                // 简化处理，实际需要处理异常重试
                Ok(())
            }
            crate::parser::Expression::ClassDefinition(_, _, body) => {
                // 简化处理，实际需要处理类定义
                for expr in body {
                    self.execute_expression_with_state(expr, state)?;
                    if state.break_flag || state.next_flag {
                        break;
                    }
                }
                Ok(())
            }
            crate::parser::Expression::ModuleDefinition(_, body) => {
                // 简化处理，实际需要处理模块定义
                for expr in body {
                    self.execute_expression_with_state(expr, state)?;
                    if state.break_flag || state.next_flag {
                        break;
                    }
                }
                Ok(())
            }
            crate::parser::Expression::FunctionDefinition(name, params, body) => {
                // 注册函数
                let vm = self.clone();
                self.state.register_function(name.clone(), Box::new(move |args| {
                    // 创建一个新的状态来执行函数
                    let mut func_vm = vm.clone();
                    
                    // 设置参数
                    for (i, param) in params.iter().enumerate() {
                        if i < args.len() {
                            func_vm.state.set_variable(param.clone(), args[i].clone());
                        } else {
                            func_vm.state.set_variable(param.clone(), Value::Nil);
                        }
                    }
                    
                    // 执行函数体
                    for expr in body {
                        if let Err(_) = func_vm.execute_expression(expr) {
                            return Value::Nil;
                        }
                    }
                    
                    Value::Nil
                }));
                Ok(())
            }
            _ => {
                Ok(())
            }
        }
    }
    
    fn evaluate_expression(&self, expr: &crate::parser::Expression) -> Result<Value>
    {
        match expr {
            crate::parser::Expression::Literal(value) => {
                Ok(value.clone())
            }
            crate::parser::Expression::Identifier(name) => {
                if let Some(value) = self.state.get_variable(name) {
                    Ok(value.clone())
                } else {
                    Ok(Value::Nil)
                }
            }
            crate::parser::Expression::BinaryOp(left, op, right) => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.evaluate_binary_op(&left_val, op, &right_val)
            }
            crate::parser::Expression::UnaryOp(op, expr) => {
                let val = self.evaluate_expression(expr)?;
                self.evaluate_unary_op(op, &val)
            }
            crate::parser::Expression::FunctionCall(name, args) => {
                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.evaluate_expression(arg)?);
                }
                if let Some(func) = self.state.get_function(name) {
                    Ok(func(&evaluated_args))
                } else {
                    Ok(Value::Nil)
                }
            }
            _ => {
                Ok(Value::Nil)
            }
        }
    }
    
    fn evaluate_binary_op(&self, left: &Value, op: &crate::parser::Token, right: &Value) -> Result<Value> {
        match (left, op, right) {
            (Value::Integer(l), crate::parser::Token::Plus, Value::Integer(r)) => {
                Ok(Value::Integer(l + r))
            }
            (Value::Integer(l), crate::parser::Token::Minus, Value::Integer(r)) => {
                Ok(Value::Integer(l - r))
            }
            (Value::Integer(l), crate::parser::Token::Multiply, Value::Integer(r)) => {
                Ok(Value::Integer(l * r))
            }
            (Value::Integer(l), crate::parser::Token::Divide, Value::Integer(r)) => {
                if r == 0 {
                    Err(RgssError::VmError("Division by zero".to_string()))
                } else {
                    Ok(Value::Integer(l / r))
                }
            }
            (Value::Integer(l), crate::parser::Token::Modulus, Value::Integer(r)) => {
                if r == 0 {
                    Err(RgssError::VmError("Modulus by zero".to_string()))
                } else {
                    Ok(Value::Integer(l % r))
                }
            }
            (Value::Float(l), crate::parser::Token::Plus, Value::Float(r)) => {
                Ok(Value::Float(l + r))
            }
            (Value::Float(l), crate::parser::Token::Minus, Value::Float(r)) => {
                Ok(Value::Float(l - r))
            }
            (Value::Float(l), crate::parser::Token::Multiply, Value::Float(r)) => {
                Ok(Value::Float(l * r))
            }
            (Value::Float(l), crate::parser::Token::Divide, Value::Float(r)) => {
                if r == 0.0 {
                    Err(RgssError::VmError("Division by zero".to_string()))
                } else {
                    Ok(Value::Float(l / r))
                }
            }
            (Value::Integer(l), crate::parser::Token::Plus, Value::Float(r)) => {
                Ok(Value::Float(*l as f32 + r))
            }
            (Value::Float(l), crate::parser::Token::Plus, Value::Integer(r)) => {
                Ok(Value::Float(l + *r as f32))
            }
            (Value::Integer(l), crate::parser::Token::Minus, Value::Float(r)) => {
                Ok(Value::Float(*l as f32 - r))
            }
            (Value::Float(l), crate::parser::Token::Minus, Value::Integer(r)) => {
                Ok(Value::Float(l - *r as f32))
            }
            (Value::Integer(l), crate::parser::Token::Multiply, Value::Float(r)) => {
                Ok(Value::Float(*l as f32 * r))
            }
            (Value::Float(l), crate::parser::Token::Multiply, Value::Integer(r)) => {
                Ok(Value::Float(l * *r as f32))
            }
            (Value::Integer(l), crate::parser::Token::Divide, Value::Float(r)) => {
                if r == 0.0 {
                    Err(RgssError::VmError("Division by zero".to_string()))
                } else {
                    Ok(Value::Float(*l as f32 / r))
                }
            }
            (Value::Float(l), crate::parser::Token::Divide, Value::Integer(r)) => {
                if *r == 0 {
                    Err(RgssError::VmError("Division by zero".to_string()))
                } else {
                    Ok(Value::Float(l / *r as f32))
                }
            }
            (Value::String(l), crate::parser::Token::Plus, Value::String(r)) => {
                Ok(Value::String(format!("{}{}", l, r)))
            }
            (Value::String(l), crate::parser::Token::Plus, other) => {
                Ok(Value::String(format!("{}{:?}", l, other)))
            }
            (other, crate::parser::Token::Plus, Value::String(r)) => {
                Ok(Value::String(format!("{:?}{}", other, r)))
            }
            (l, crate::parser::Token::Equal, r) => {
                Ok(Value::Boolean(self.compare_values(l, r) == std::cmp::Ordering::Equal))
            }
            (l, crate::parser::Token::NotEqual, r) => {
                Ok(Value::Boolean(self.compare_values(l, r) != std::cmp::Ordering::Equal))
            }
            (l, crate::parser::Token::LessThan, r) => {
                Ok(Value::Boolean(self.compare_values(l, r) == std::cmp::Ordering::Less))
            }
            (l, crate::parser::Token::LessThanOrEqual, r) => {
                let order = self.compare_values(l, r);
                Ok(Value::Boolean(order == std::cmp::Ordering::Less || order == std::cmp::Ordering::Equal))
            }
            (l, crate::parser::Token::GreaterThan, r) => {
                Ok(Value::Boolean(self.compare_values(l, r) == std::cmp::Ordering::Greater))
            }
            (l, crate::parser::Token::GreaterThanOrEqual, r) => {
                let order = self.compare_values(l, r);
                Ok(Value::Boolean(order == std::cmp::Ordering::Greater || order == std::cmp::Ordering::Equal))
            }
            _ => {
                Ok(Value::Nil)
            }
        }
    }
    
    fn evaluate_unary_op(&self, op: &crate::parser::Token, value: &Value) -> Result<Value> {
        match (op, value) {
            (crate::parser::Token::Minus, Value::Integer(i)) => {
                Ok(Value::Integer(-i))
            }
            (crate::parser::Token::Minus, Value::Float(f)) => {
                Ok(Value::Float(-f))
            }
            (crate::parser::Token::ExclamationMark, v) => {
                Ok(Value::Boolean(!self.is_truthy(v)))
            }
            _ => {
                Ok(Value::Nil)
            }
        }
    }
    
    fn compare_values(&self, left: &Value, right: &Value) -> std::cmp::Ordering {
        match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => l.cmp(r),
            (Value::Float(l), Value::Float(r)) => l.partial_cmp(r).unwrap_or(std::cmp::Ordering::Equal),
            (Value::Integer(l), Value::Float(r)) => (*l as f32).partial_cmp(r).unwrap_or(std::cmp::Ordering::Equal),
            (Value::Float(l), Value::Integer(r)) => l.partial_cmp(&(*r as f32)).unwrap_or(std::cmp::Ordering::Equal),
            (Value::String(l), Value::String(r)) => l.cmp(r),
            (Value::Boolean(l), Value::Boolean(r)) => l.cmp(r),
            _ => std::cmp::Ordering::Equal,
        }
    }
    
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Boolean(false) => false,
            Value::Integer(0) => false,
            Value::Float(0.0) => false,
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }

    pub fn get_state(&self) -> &VmState {
        &self.state
    }

    fn register_builtin_functions(&mut self) {
        // 注册内置函数
        self.state.register_function("print".to_string(), Box::new(|args| {
            for arg in args {
                match arg {
                    Value::String(s) => println!("{}", s),
                    Value::Integer(i) => println!("{}", i),
                    Value::Float(f) => println!("{}", f),
                    Value::Boolean(b) => println!("{}", b),
                    _ => println!("{:?}", arg),
                }
            }
            Value::Nil
        }));

        self.state.register_function("rand".to_string(), Box::new(|args| {
            if let Some(Value::Integer(max)) = args.first() {
                Value::Integer(rand::random::<i32>() % max)
            } else {
                Value::Integer(rand::random::<i32>())
            }
        }));
    }
}
