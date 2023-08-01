#[derive(Debug, Clone)]
pub enum Symbol {
    Program(Program),
    NumberLiteral(Literal),
    CallExpression(CallExpression),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Symbol>,
}

impl Program {
    pub fn new() -> Self {
        Self { body: vec![] }
    }

    pub fn set_body(&mut self, body: &mut Vec<Symbol>) {
        self.body.append(body);
    }
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: String,
}

impl Literal {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub name: String,
    pub params: Vec<Symbol>,
}

impl CallExpression {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: vec![],
        }
    }

    pub fn add_param(&mut self, param: Symbol) {
        self.params.push(param);
    }
}
