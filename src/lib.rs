mod ast;
mod generator;
mod parser;
mod tokenizer;
mod transformer;

pub fn compile(input: &str) -> Result<String, String> {
    let tokens = tokenizer::tokenize(input)?;

    let lisp_ast = parser::parse(tokens)?;

    let js_ast = transformer::transform(lisp_ast);

    let code = generator::generate_code(ast::js::Symbol::Program(js_ast));

    Ok(code)
}
