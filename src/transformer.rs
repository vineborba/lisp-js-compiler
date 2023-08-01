use std::{cell::RefCell, rc::Rc};

use crate::ast::{
    js::{
        CallExpression, ExpressionStatement, Identifier, NumericLiteral, Program as JsProgram,
        Symbol as JsSymbol,
    },
    lisp::{Program as LispProgram, Symbol as LispSymbol},
};

pub fn transform(lisp_ast: LispProgram) -> JsProgram {
    let mut ast = JsProgram::new();
    let position = traverse(lisp_ast);
    ast.set_body(Rc::clone(&position));
    ast
}

fn traverse(lisp_ast: LispProgram) -> Rc<RefCell<Vec<JsSymbol>>> {
    let position: Rc<RefCell<Vec<JsSymbol>>> = Rc::new(RefCell::new(vec![]));
    walk_node(
        &LispSymbol::Program(lisp_ast),
        &mut Rc::clone(&position),
        None,
    );
    position
}

fn walk_nodes(
    nodes: &Vec<LispSymbol>,
    position: &mut Rc<RefCell<Vec<JsSymbol>>>,
    parent: Option<&LispSymbol>,
) {
    nodes
        .iter()
        .for_each(|node| walk_node(node, &mut Rc::clone(&position), parent))
}

fn walk_node(
    node: &LispSymbol,
    position: &mut Rc<RefCell<Vec<JsSymbol>>>,
    parent: Option<&LispSymbol>,
) {
    match node {
        LispSymbol::NumberLiteral(node) => {
            position
                .borrow_mut()
                .push(JsSymbol::NumericLiteral(NumericLiteral::new(
                    node.value.clone(),
                )));
        }
        LispSymbol::CallExpression(node) => {
            let callee = Identifier::new(node.name.clone());
            let expression = CallExpression::new(callee);

            let prev_position = position.clone();
            let mut position = expression.arguments.clone();
            if let Some(parent) = parent {
                match parent {
                    LispSymbol::CallExpression(_) => {
                        prev_position
                            .borrow_mut()
                            .push(JsSymbol::CallExpression(expression));
                    }
                    _ => {
                        let expression = ExpressionStatement::new(expression);
                        prev_position
                            .borrow_mut()
                            .push(JsSymbol::ExpressionStatement(expression));
                    }
                }
            } else {
                prev_position
                    .borrow_mut()
                    .push(JsSymbol::CallExpression(expression));
            }
            walk_nodes(
                &node.params,
                &mut position,
                Some(&LispSymbol::CallExpression(node.clone())),
            )
        }
        LispSymbol::Program(p) => walk_nodes(&p.body, position, Some(&node)),
    }
}
