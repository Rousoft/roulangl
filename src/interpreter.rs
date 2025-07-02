use crate::ast::{Instruccion, Valor};
use std::collections::HashMap;

pub fn run(instrucciones: &[Instruccion]) {
    let mut variables: HashMap<String, Valor> = HashMap::new();
    let mut mutables: HashMap<String, bool> = HashMap::new();

    for instruccion in instrucciones {
        match instruccion {
            Instruccion::Var(nombre, valor) => {
                if !variables.contains_key(nombre) {
                    variables.insert(nombre.clone(), valor.clone());
                    mutables.insert(nombre.clone(), false);
                } else {
                    eprintln!("Error: variable '{}' ya declarada", nombre);
                }
            }
            Instruccion::Mutar(nombre, valor) => {
                if !variables.contains_key(nombre) {
                    variables.insert(nombre.clone(), valor.clone());
                    mutables.insert(nombre.clone(), true);
                } else {
                    if *mutables.get(nombre).unwrap_or(&false) {
                        variables.insert(nombre.clone(), valor.clone());
                    } else {
                        eprintln!("Error: variable '{}' es inmutable", nombre);
                    }
                }
            }
            Instruccion::Imprimir(texto) => {
                let texto = texto.trim();

                if texto.starts_with('"') && texto.ends_with('"') {
                    // Cadena con variables: quitar comillas y expandir
                    let inner = &texto[1..texto.len()-1];
                    let output = expandir_variables(inner, &variables);
                    println!("{}", output);
                } else {
                    // Puede ser nombre variable
                    if let Some(val) = variables.get(texto) {
                        println!("{}", valor_a_string(val));
                    } else {
                        // Texto literal (sin comillas)
                        println!("{}", texto);
                    }
                }
            }
        }
    }
}

fn expandir_variables(texto: &str, vars: &HashMap<String, Valor>) -> String {
    let mut result = String::new();
    let mut chars = texto.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' {
            let mut var_name = String::new();
            while let Some(&nc) = chars.peek() {
                if nc == '}' {
                    chars.next();
                    break;
                }
                var_name.push(nc);
                chars.next();
            }
            if let Some(val) = vars.get(&var_name) {
                result.push_str(&valor_a_string(val));
            } else {
                result.push_str(&format!("{{{}?}}", var_name));
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn valor_a_string(val: &Valor) -> String {
    match val {
        Valor::Int(i) => i.to_string(),
        Valor::Float(f) => f.to_string(),
        Valor::Char(c) => c.to_string(),
        Valor::String(s) => s.clone(),
    }
}
