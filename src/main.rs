use std::process;

fn main() {
    let input = "(prod 192 (
        add 2 (
            sub 4 3
        )
    )";
    println!("LISP input:\n{}", &input);
    match lisp_js_compiler::compile(input) {
        Ok(output) => {
            println!("JS Output:\n{}", output);
        }
        Err(e) => {
            eprintln!("Failed to compile program: {}", e);
            process::exit(1);
        }
    };
}
