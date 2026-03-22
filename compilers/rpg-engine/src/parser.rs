use crate::{Result, RgssError, vm::Value};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // 关键字
    If,
    Elsif,
    Else,
    End,
    While,
    For,
    In,
    Do,
    Begin,
    Rescue,
    Ensure,
    Return,
    Break,
    Next,
    Redo,
    Retry,
    Def,
    Class,
    Module,
    Self,
    Super,
    Nil,
    True,
    False,
    And,
    Or,
    Not,
    
    // 操作符
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModulusAssign,
    
    // 标点符号
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Colon,
    Semicolon,
    QuestionMark,
    ExclamationMark,
    
    // 标识符
    Identifier(String),
    
    // 字面量
    Integer(i32),
    Float(f32),
    String(String),
    Symbol(String),
    
    // 其他
    Newline,
    EOF,
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return Ok(Token::EOF);
        }

        let current_char = self.input.chars().nth(self.position).unwrap();

        match current_char {
            '+' => {
                self.position += 1;
                self.column += 1;
                if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == '=' {
                    self.position += 1;
                    self.column += 1;
                    Ok(Token::PlusAssign)
                } else {
                    Ok(Token::Plus)
                }
            }
            '-' => {
                self.position += 1;
                self.column += 1;
                if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == '=' {
                    self.position += 1;
                    self.column += 1;
                    Ok(Token::MinusAssign)
                } else {
                    Ok(Token::Minus)
                }
            }
            '*' => {
                self.position += 1;
                self.column += 1;
                if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == '=' {
                    self.position += 1;
                    self.column += 1;
                    Ok(Token::MultiplyAssign)
                } else {
                    Ok(Token::Multiply)
                }
            }
            '/' => {
                self.position += 1;
                self.column += 1;
                if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == '=' {
                    self.position += 1;
                    self.column += 1;
                    Ok(Token::DivideAssign)
                } else {
                    Ok(Token::Divide)
                }
            }
            '%' => {
                self.position += 1;
                self.column += 1;
                if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == '=' {
                    self.position += 1;
                    self.column += 1;
                    Ok(Token::ModulusAssign)
                } else {
                    Ok(Token::Modulus)
                }
            }
            '=' => {
                self.position += 1;
                self.column += 1;
                if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == '=' {
                    self.position += 1;
                    self.column += 1;
                    Ok(Token::Equal)
                } else {
                    Ok(Token::Assign)
                }
            }
            '!' => {
                self.position += 1;
                self.column += 1;
                if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == '=' {
                    self.position += 1;
                    self.column += 1;
                    Ok(Token::NotEqual)
                } else {
                    Ok(Token::ExclamationMark)
                }
            }
            '<' => {
                self.position += 1;
                self.column += 1;
                if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == '=' {
                    self.position += 1;
                    self.column += 1;
                    Ok(Token::LessThanOrEqual)
                } else {
                    Ok(Token::LessThan)
                }
            }
            '>' => {
                self.position += 1;
                self.column += 1;
                if self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() == '=' {
                    self.position += 1;
                    self.column += 1;
                    Ok(Token::GreaterThanOrEqual)
                } else {
                    Ok(Token::GreaterThan)
                }
            }
            '(' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::LeftParen)
            }
            ')' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::RightParen)
            }
            '[' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::LeftBracket)
            }
            ']' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::RightBracket)
            }
            '{' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::LeftBrace)
            }
            '}' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::RightBrace)
            }
            ',' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::Comma)
            }
            '.' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::Dot)
            }
            ':' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::Colon)
            }
            ';' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::Semicolon)
            }
            '?' => {
                self.position += 1;
                self.column += 1;
                Ok(Token::QuestionMark)
            }
            '\n' => {
                self.position += 1;
                self.line += 1;
                self.column = 1;
                Ok(Token::Newline)
            }
            '"' => {
                self.position += 1;
                self.column += 1;
                self.parse_string()
            }
            '' => {
                self.position += 1;
                self.column += 1;
                self.parse_symbol()
            }
            '0'..='9' => {
                self.parse_number()
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                self.parse_identifier()
            }
            _ => {
                Err(RgssError::ParserError(format!("Unexpected character '{}' at line {}, column {}", current_char, self.line, self.column)))
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            match current_char {
                ' ' | '\t' | '\r' => {
                    self.position += 1;
                    self.column += 1;
                }
                '#' => {
                    // 跳过注释
                    while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap() != '\n' {
                        self.position += 1;
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn parse_string(&mut self) -> Result<Token> {
        let mut string = String::new();
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            if current_char == '"' {
                self.position += 1;
                self.column += 1;
                break;
            } else if current_char == '\\' && self.position + 1 < self.input.len() {
                // 处理转义字符
                self.position += 1;
                self.column += 1;
                let next_char = self.input.chars().nth(self.position).unwrap();
                match next_char {
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    'r' => string.push('\r'),
                    '"' => string.push('"'),
                    '\\' => string.push('\\'),
                    _ => string.push(next_char),
                }
                self.position += 1;
                self.column += 1;
            } else {
                string.push(current_char);
                self.position += 1;
                self.column += 1;
            }
        }
        Ok(Token::String(string))
    }

    fn parse_symbol(&mut self) -> Result<Token> {
        let mut symbol = String::new();
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            if current_char == '\'' {
                self.position += 1;
                self.column += 1;
                break;
            } else {
                symbol.push(current_char);
                self.position += 1;
                self.column += 1;
            }
        }
        Ok(Token::Symbol(symbol))
    }

    fn parse_number(&mut self) -> Result<Token> {
        let mut number = String::new();
        let mut is_float = false;
        
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            match current_char {
                '0'..='9' => {
                    number.push(current_char);
                    self.position += 1;
                    self.column += 1;
                }
                '.' => {
                    if is_float {
                        return Err(RgssError::ParserError(format!("Invalid number format at line {}, column {}", self.line, self.column)));
                    }
                    is_float = true;
                    number.push(current_char);
                    self.position += 1;
                    self.column += 1;
                }
                _ => {
                    break;
                }
            }
        }
        
        if is_float {
            match number.parse::<f32>() {
                Ok(value) => Ok(Token::Float(value)),
                Err(_) => Err(RgssError::ParserError(format!("Invalid float format at line {}, column {}", self.line, self.column))),
            }
        } else {
            match number.parse::<i32>() {
                Ok(value) => Ok(Token::Integer(value)),
                Err(_) => Err(RgssError::ParserError(format!("Invalid integer format at line {}, column {}", self.line, self.column))),
            }
        }
    }

    fn parse_identifier(&mut self) -> Result<Token> {
        let mut identifier = String::new();
        
        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();
            match current_char {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    identifier.push(current_char);
                    self.position += 1;
                    self.column += 1;
                }
                _ => {
                    break;
                }
            }
        }
        
        // 检查是否是关键字
        match identifier.as_str() {
            "if" => Ok(Token::If),
            "elsif" => Ok(Token::Elsif),
            "else" => Ok(Token::Else),
            "end" => Ok(Token::End),
            "while" => Ok(Token::While),
            "for" => Ok(Token::For),
            "in" => Ok(Token::In),
            "do" => Ok(Token::Do),
            "begin" => Ok(Token::Begin),
            "rescue" => Ok(Token::Rescue),
            "ensure" => Ok(Token::Ensure),
            "return" => Ok(Token::Return),
            "break" => Ok(Token::Break),
            "next" => Ok(Token::Next),
            "redo" => Ok(Token::Redo),
            "retry" => Ok(Token::Retry),
            "def" => Ok(Token::Def),
            "class" => Ok(Token::Class),
            "module" => Ok(Token::Module),
            "self" => Ok(Token::Self),
            "super" => Ok(Token::Super),
            "nil" => Ok(Token::Nil),
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            "and" => Ok(Token::And),
            "or" => Ok(Token::Or),
            "not" => Ok(Token::Not),
            _ => Ok(Token::Identifier(identifier)),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(Value),
    Identifier(String),
    Assignment(String, Box<Expression>),
    BinaryOp(Box<Expression>, Token, Box<Expression>),
    UnaryOp(Token, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
    MethodCall(Box<Expression>, String, Vec<Expression>),
    IfExpression(Box<Expression>, Vec<Expression>, Option<Vec<Expression>>, Option<Vec<Expression>>),
    WhileExpression(Box<Expression>, Vec<Expression>),
    ForExpression(String, Box<Expression>, Vec<Expression>),
    Block(Vec<Expression>),
    ReturnExpression(Option<Box<Expression>>),
    BreakExpression,
    NextExpression,
    RedoExpression,
    RetryExpression,
    ClassDefinition(String, Option<String>, Vec<Expression>),
    ModuleDefinition(String, Vec<Expression>),
    FunctionDefinition(String, Vec<String>, Vec<Expression>),
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(input: &str) -> Result<Self> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;
        Ok(Self {
            lexer,
            current_token,
        })
    }

    pub fn parse(&mut self) -> Result<Vec<Expression>> {
        let mut expressions = Vec::new();
        while self.current_token != Token::EOF {
            // 跳过换行符
            while self.current_token == Token::Newline {
                self.current_token = self.lexer.next_token()?;
            }
            if self.current_token != Token::EOF {
                match self.parse_expression() {
                    Ok(expr) => expressions.push(expr),
                    Err(RgssError::ParserError(e)) if e.contains("Unexpected end of file") => break,
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(expressions)
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_statement()
    }

    fn parse_statement(&mut self) -> Result<Expression> {
        // 跳过换行符
        while self.current_token == Token::Newline {
            self.current_token = self.lexer.next_token()?;
        }
        
        if self.current_token == Token::EOF {
            return Err(RgssError::ParserError(format!("Unexpected end of file at line {}, column {}", self.lexer.line, self.lexer.column)));
        }
        
        match &self.current_token {
            Token::If => self.parse_if_expression(),
            Token::While => self.parse_while_expression(),
            Token::For => self.parse_for_expression(),
            Token::Begin => self.parse_begin_expression(),
            Token::Return => self.parse_return_expression(),
            Token::Break => self.parse_break_expression(),
            Token::Next => self.parse_next_expression(),
            Token::Redo => self.parse_redo_expression(),
            Token::Retry => self.parse_retry_expression(),
            Token::Class => self.parse_class_definition(),
            Token::Module => self.parse_module_definition(),
            Token::Def => self.parse_function_definition(),
            _ => self.parse_assignment(),
        }
    }

    fn parse_if_expression(&mut self) -> Result<Expression> {
        self.consume(Token::If)?;
        let condition = self.parse_expression()?;
        
        let mut body = Vec::new();
        while self.current_token != Token::End && self.current_token != Token::Elsif && self.current_token != Token::Else {
            body.push(self.parse_expression()?);
        }
        
        let mut elsif_conditions = Vec::new();
        let mut elsif_bodies = Vec::new();
        
        while self.current_token == Token::Elsif {
            self.consume(Token::Elsif)?;
            let elsif_condition = self.parse_expression()?;
            elsif_conditions.push(elsif_condition);
            
            let mut elsif_body = Vec::new();
            while self.current_token != Token::End && self.current_token != Token::Elsif && self.current_token != Token::Else {
                elsif_body.push(self.parse_expression()?);
            }
            elsif_bodies.push(elsif_body);
        }
        
        let else_body = if self.current_token == Token::Else {
            self.consume(Token::Else)?;
            let mut else_body = Vec::new();
            while self.current_token != Token::End {
                else_body.push(self.parse_expression()?);
            }
            Some(else_body)
        } else {
            None
        };
        
        self.consume(Token::End)?;
        
        Ok(Expression::IfExpression(Box::new(condition), body, Some(elsif_bodies), else_body))
    }

    fn parse_while_expression(&mut self) -> Result<Expression> {
        self.consume(Token::While)?;
        let condition = self.parse_expression()?;
        
        let mut body = Vec::new();
        while self.current_token != Token::End {
            body.push(self.parse_expression()?);
        }
        
        self.consume(Token::End)?;
        
        Ok(Expression::WhileExpression(Box::new(condition), body))
    }

    fn parse_for_expression(&mut self) -> Result<Expression> {
        self.consume(Token::For)?;
        let variable = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(RgssError::ParserError(format!("Expected identifier after 'for' at line {}, column {}", self.lexer.line, self.lexer.column)));
        };
        self.consume(Token::Identifier(variable.clone()))?;
        
        self.consume(Token::In)?;
        let collection = self.parse_expression()?;
        
        let mut body = Vec::new();
        while self.current_token != Token::End {
            body.push(self.parse_expression()?);
        }
        
        self.consume(Token::End)?;
        
        Ok(Expression::ForExpression(variable, Box::new(collection), body))
    }

    fn parse_begin_expression(&mut self) -> Result<Expression> {
        self.consume(Token::Begin)?;
        
        let mut body = Vec::new();
        while self.current_token != Token::Rescue && self.current_token != Token::Ensure && self.current_token != Token::End {
            body.push(self.parse_expression()?);
        }
        
        let mut rescue_clauses = Vec::new();
        while self.current_token == Token::Rescue {
            self.consume(Token::Rescue)?;
            let mut rescue_body = Vec::new();
            while self.current_token != Token::Rescue && self.current_token != Token::Ensure && self.current_token != Token::End {
                rescue_body.push(self.parse_expression()?);
            }
            rescue_clauses.push(rescue_body);
        }
        
        let ensure_body = if self.current_token == Token::Ensure {
            self.consume(Token::Ensure)?;
            let mut ensure_body = Vec::new();
            while self.current_token != Token::End {
                ensure_body.push(self.parse_expression()?);
            }
            Some(ensure_body)
        } else {
            None
        };
        
        self.consume(Token::End)?;
        
        // 简化处理，实际需要更复杂的错误处理逻辑
        Ok(Expression::Block(body))
    }

    fn parse_return_expression(&mut self) -> Result<Expression> {
        self.consume(Token::Return)?;
        let value = if self.current_token != Token::Newline && self.current_token != Token::End {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        Ok(Expression::ReturnExpression(value))
    }

    fn parse_break_expression(&mut self) -> Result<Expression> {
        self.consume(Token::Break)?;
        Ok(Expression::BreakExpression)
    }

    fn parse_next_expression(&mut self) -> Result<Expression> {
        self.consume(Token::Next)?;
        Ok(Expression::NextExpression)
    }

    fn parse_redo_expression(&mut self) -> Result<Expression> {
        self.consume(Token::Redo)?;
        Ok(Expression::RedoExpression)
    }

    fn parse_retry_expression(&mut self) -> Result<Expression> {
        self.consume(Token::Retry)?;
        Ok(Expression::RetryExpression)
    }

    fn parse_class_definition(&mut self) -> Result<Expression> {
        self.consume(Token::Class)?;
        let name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(RgssError::ParserError(format!("Expected class name at line {}, column {}", self.lexer.line, self.lexer.column)));
        };
        self.consume(Token::Identifier(name.clone()))?;
        
        let superclass = if self.current_token == Token::LessThan {
            self.consume(Token::LessThan)?;
            if let Token::Identifier(superclass_name) = &self.current_token {
                let superclass = superclass_name.clone();
                self.consume(Token::Identifier(superclass.clone()))?;
                Some(superclass)
            } else {
                return Err(RgssError::ParserError(format!("Expected superclass name at line {}, column {}", self.lexer.line, self.lexer.column)));
            }
        } else {
            None
        };
        
        let mut body = Vec::new();
        while self.current_token != Token::End {
            body.push(self.parse_expression()?);
        }
        
        self.consume(Token::End)?;
        
        Ok(Expression::ClassDefinition(name, superclass, body))
    }

    fn parse_module_definition(&mut self) -> Result<Expression> {
        self.consume(Token::Module)?;
        let name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(RgssError::ParserError(format!("Expected module name at line {}, column {}", self.lexer.line, self.lexer.column)));
        };
        self.consume(Token::Identifier(name.clone()))?;
        
        let mut body = Vec::new();
        while self.current_token != Token::End {
            body.push(self.parse_expression()?);
        }
        
        self.consume(Token::End)?;
        
        Ok(Expression::ModuleDefinition(name, body))
    }

    fn parse_function_definition(&mut self) -> Result<Expression> {
        self.consume(Token::Def)?;
        let name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            return Err(RgssError::ParserError(format!("Expected function name at line {}, column {}", self.lexer.line, self.lexer.column)));
        };
        self.consume(Token::Identifier(name.clone()))?;
        
        let mut parameters = Vec::new();
        if self.current_token == Token::LeftParen {
            self.consume(Token::LeftParen)?;
            while self.current_token != Token::RightParen {
                if let Token::Identifier(param) = &self.current_token {
                    parameters.push(param.clone());
                    self.consume(Token::Identifier(param.clone()))?;
                } else {
                    return Err(RgssError::ParserError(format!("Expected parameter name at line {}, column {}", self.lexer.line, self.lexer.column)));
                }
                if self.current_token == Token::Comma {
                    self.consume(Token::Comma)?;
                }
            }
            self.consume(Token::RightParen)?;
        }
        
        let mut body = Vec::new();
        while self.current_token != Token::End {
            body.push(self.parse_expression()?);
        }
        
        self.consume(Token::End)?;
        
        Ok(Expression::FunctionDefinition(name, parameters, body))
    }

    fn parse_assignment(&mut self) -> Result<Expression> {
        let left = self.parse_primary()?;
        
        if let Token::Assign = self.current_token {
            self.consume(Token::Assign)?;
            let right = self.parse_expression()?;
            
            if let Expression::Identifier(name) = left {
                Ok(Expression::Assignment(name, Box::new(right)))
            } else {
                Err(RgssError::ParserError(format!("Expected identifier on left side of assignment at line {}, column {}", self.lexer.line, self.lexer.column)))
            }
        } else {
            Ok(left)
        }
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        match &self.current_token {
            Token::Integer(value) => {
                self.consume(Token::Integer(*value))?;
                Ok(Expression::Literal(Value::Integer(*value)))
            }
            Token::Float(value) => {
                self.consume(Token::Float(*value))?;
                Ok(Expression::Literal(Value::Float(*value)))
            }
            Token::String(value) => {
                self.consume(Token::String(value.clone()))?;
                Ok(Expression::Literal(Value::String(value.clone())))
            }
            Token::Symbol(value) => {
                self.consume(Token::Symbol(value.clone()))?;
                Ok(Expression::Literal(Value::String(value.clone())))
            }
            Token::Nil => {
                self.consume(Token::Nil)?;
                Ok(Expression::Literal(Value::Nil))
            }
            Token::True => {
                self.consume(Token::True)?;
                Ok(Expression::Literal(Value::Boolean(true)))
            }
            Token::False => {
                self.consume(Token::False)?;
                Ok(Expression::Literal(Value::Boolean(false)))
            }
            Token::Identifier(name) => {
                let name_clone = name.clone();
                self.consume(Token::Identifier(name_clone.clone()))?;
                
                if self.current_token == Token::LeftParen {
                    // 函数调用
                    self.consume(Token::LeftParen)?;
                    let mut arguments = Vec::new();
                    while self.current_token != Token::RightParen {
                        arguments.push(self.parse_expression()?);
                        if self.current_token == Token::Comma {
                            self.consume(Token::Comma)?;
                        }
                    }
                    self.consume(Token::RightParen)?;
                    Ok(Expression::FunctionCall(name_clone, arguments))
                } else {
                    Ok(Expression::Identifier(name_clone))
                }
            }
            Token::LeftParen => {
                self.consume(Token::LeftParen)?;
                let expr = self.parse_expression()?;
                self.consume(Token::RightParen)?;
                Ok(expr)
            }
            _ => {
                Err(RgssError::ParserError(format!("Unexpected token {:?} at line {}, column {}", self.current_token, self.lexer.line, self.lexer.column)))
            }
        }
    }

    fn consume(&mut self, expected: Token) -> Result<()> {
        if self.current_token == expected {
            self.current_token = self.lexer.next_token()?;
            Ok(())
        } else {
            Err(RgssError::ParserError(format!("Expected {:?}, got {:?} at line {}, column {}", expected, self.current_token, self.lexer.line, self.lexer.column)))
        }
    }
}

pub fn parse_script(script: &str) -> Result<Vec<Expression>> {
    let mut parser = Parser::new(script)?;
    parser.parse()
}

pub fn validate_script_syntax(script: &str) -> Result<()> {
    let mut parser = Parser::new(script)?;
    parser.parse()?;
    Ok(())
}