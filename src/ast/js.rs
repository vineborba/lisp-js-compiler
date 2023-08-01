use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub enum Symbol {
    Program(Program),
    NumericLiteral(NumericLiteral),
    CallExpression(CallExpression),
    ExpressionStatement(ExpressionStatement),
    Identifier(Identifier),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Rc<RefCell<Vec<Symbol>>>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            body: Rc::new(RefCell::new(vec![])),
        }
    }

    pub fn set_body(&mut self, body: Rc<RefCell<Vec<Symbol>>>) {
        self.body = body;
    }
}

#[derive(Debug, Clone)]
pub struct NumericLiteral {
    pub value: String,
}

impl NumericLiteral {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub expression: CallExpression,
}

impl ExpressionStatement {
    pub fn new(expression: CallExpression) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub callee: Identifier,
    pub arguments: Rc<RefCell<Vec<Symbol>>>,
}

impl CallExpression {
    pub fn new(callee: Identifier) -> Self {
        Self {
            callee,
            arguments: Rc::new(RefCell::new(vec![])),
        }
    }
}
