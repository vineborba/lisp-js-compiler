use crate::{
    ast::lisp::{CallExpression, Literal, Program, Symbol},
    tokenizer::Token,
};

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut current: usize = 0;
    let mut ast = Program::new();
    let program_root = walk(&tokens, &mut current)?;
    let mut program_body = vec![program_root];
    ast.set_body(&mut program_body);
    Ok(ast)
}

fn walk(tokens: &Vec<Token>, current: &mut usize) -> Result<Symbol, String> {
    let mut token = &tokens[*current];
    match &token {
        Token::Number(token) => {
            *current += 1;
            Ok(Symbol::NumberLiteral(Literal::new(token.value.clone())))
        }
        Token::Paren(_) => {
            *current += 1;
            token = &tokens[*current];

            let mut expression = Symbol::CallExpression(CallExpression::new(token.value().clone()));
            *current += 1;
            token = &tokens[*current];
            while token.value() != String::from(')') {
                match &mut expression {
                    Symbol::CallExpression(expression) => {
                        expression.add_param(walk(tokens, current)?);
                        token = &tokens[*current];
                    }
                    _ => return Err(String::from("Invalid expression")),
                }
            }
            Ok(expression)
        }
        _ => unreachable!(),
    }
}
