use crate::ast::js::Symbol;

pub fn generate_code(node: Symbol) -> String {
    match node {
        Symbol::NumericLiteral(node) => node.value,
        Symbol::Identifier(node) => node.name,
        Symbol::CallExpression(node) => {
            let callee = generate_code(Symbol::Identifier(node.callee));
            let args = node
                .arguments
                .borrow()
                .iter()
                .map(|arg| generate_code(arg.clone()))
                .collect::<Vec<String>>()
                .join(", ");
            format!("{callee}({args})")
        }
        Symbol::ExpressionStatement(node) => {
            let code = generate_code(Symbol::CallExpression(node.expression));
            format!("{code};")
        }
        Symbol::Program(node) => node
            .body
            .borrow()
            .iter()
            .map(|b| generate_code(b.clone()))
            .collect::<Vec<String>>()
            .join("\n"),
    }
}
