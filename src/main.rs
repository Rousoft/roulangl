mod lexer;
mod parser;
mod interpreter;
mod ast;

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: cargo run archivo.rlg");
        eprintln!("Ejemplo: cargo run ejemplo.rlg");
        return;
    }

    let filename = &args[1];

    if !filename.ends_with(".rlg") {
        eprintln!("Error: El archivo debe tener extensión .rlg");
        return;
    }

    let source = fs::read_to_string(filename)
        .expect("No se pudo leer el archivo");

    // 1. Lexer
    let tokens = lexer::tokenize(&source)
        .expect("Error en el lexer");

    // 2. Parser
    let ast = parser::parse(&tokens)
        .expect("Error en el parser");

    // 3. Ejecutar
    interpreter::run(&ast);
}
