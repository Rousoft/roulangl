use crate::ast::{Instruccion, Valor, Expresion, Operador};
use std::collections::HashMap;

pub fn run(instrucciones: &[Instruccion]) {
    let mut variables: HashMap<String, Valor> = HashMap::new();
    let mut mutables: HashMap<String, bool> = HashMap::new();

    for instruccion in instrucciones {
        match instruccion {
            Instruccion::Var(nombre, expr) => {
                if !variables.contains_key(nombre) {
                    match evaluar_expr(expr, &variables) {
                        Ok(valor) => {
                            variables.insert(nombre.clone(), valor);
                            mutables.insert(nombre.clone(), false);
                        }
                        Err(e) => eprintln!("Error al evaluar variable '{}': {}", nombre, e),
                    }
                } else {
                    eprintln!("Error: variable '{}' ya declarada", nombre);
                }
            }
            Instruccion::Mutar(nombre, expr) => {
                if !variables.contains_key(nombre) {
                    match evaluar_expr(expr, &variables) {
                        Ok(valor) => {
                            variables.insert(nombre.clone(), valor);
                            mutables.insert(nombre.clone(), true);
                        }
                        Err(e) => eprintln!("Error al evaluar variable '{}': {}", nombre, e),
                    }
                } else {
                    if *mutables.get(nombre).unwrap_or(&false) {
                        match evaluar_expr(expr, &variables) {
                            Ok(valor) => {
                                variables.insert(nombre.clone(), valor);
                            }
                            Err(e) => eprintln!("Error al evaluar variable '{}': {}", nombre, e),
                        }
                    } else {
                        eprintln!("Error: variable '{}' es inmutable", nombre);
                    }
                }
            }
            Instruccion::Imprimir(texto) => {
                let texto = texto.trim();

                if texto.starts_with('"') && texto.ends_with('"') {
                    let inner = &texto[1..texto.len()-1];
                    let output = expandir_variables(inner, &variables);
                    println!("{}", output);
                } else {
                    if let Some(val) = variables.get(texto) {
                        println!("{}", valor_a_string(val));
                    } else {
                        println!("{}", texto);
                    }
                }
            }
        }
    }
}

fn evaluar_expr(expr: &Expresion, vars: &HashMap<String, Valor>) -> Result<Valor, String> {
    match expr {
        Expresion::Valor(v) => Ok(v.clone()),
        Expresion::Variable(nombre) => {
            vars.get(nombre).cloned().ok_or_else(|| format!("Variable '{}' no definida", nombre))
        }
        Expresion::BinOp { izquierda, operador, derecha } => {
            let val_izq = evaluar_expr(izquierda, vars)?;
            let val_der = evaluar_expr(derecha, vars)?;
            operar(&val_izq, operador, &val_der)
        }
    }
}

fn operar(a: &Valor, op: &Operador, b: &Valor) -> Result<Valor, String> {
    match op {
        Operador::Suma => operar_sumar(a, b),
        Operador::Resta => operar_restar(a, b),
        Operador::Multiplicacion => operar_multiplicar(a, b),
        Operador::Division => operar_dividir(a, b),
        Operador::Modulo => operar_modulo(a, b),
    }
}

fn operar_sumar(a: &Valor, b: &Valor) -> Result<Valor, String> {
    match (a, b) {
        (Valor::Int(x), Valor::Int(y)) => Ok(Valor::Int(x + y)),
        (Valor::Int(x), Valor::Float(y)) => Ok(Valor::Float(*x as f64 + y)),
        (Valor::Float(x), Valor::Int(y)) => Ok(Valor::Float(x + *y as f64)),
        (Valor::Float(x), Valor::Float(y)) => Ok(Valor::Float(x + y)),
        (Valor::String(x), Valor::String(y)) => Ok(Valor::String(format!("{}{}", x, y))),
        _ => Err("Error: suma no soportada entre estos tipos".into()),
    }
}

fn operar_restar(a: &Valor, b: &Valor) -> Result<Valor, String> {
    match (a, b) {
        (Valor::Int(x), Valor::Int(y)) => Ok(Valor::Int(x - y)),
        (Valor::Int(x), Valor::Float(y)) => Ok(Valor::Float(*x as f64 - y)),
        (Valor::Float(x), Valor::Int(y)) => Ok(Valor::Float(x - *y as f64)),
        (Valor::Float(x), Valor::Float(y)) => Ok(Valor::Float(x - y)),
        _ => Err("Error: resta no soportada entre estos tipos".into()),
    }
}

fn operar_multiplicar(a: &Valor, b: &Valor) -> Result<Valor, String> {
    match (a, b) {
        (Valor::Int(x), Valor::Int(y)) => Ok(Valor::Int(x * y)),
        (Valor::Int(x), Valor::Float(y)) => Ok(Valor::Float(*x as f64 * y)),
        (Valor::Float(x), Valor::Int(y)) => Ok(Valor::Float(x * *y as f64)),
        (Valor::Float(x), Valor::Float(y)) => Ok(Valor::Float(x * y)),
        _ => Err("Error: multiplicación no soportada entre estos tipos".into()),
    }
}

fn operar_dividir(a: &Valor, b: &Valor) -> Result<Valor, String> {
    match (a, b) {
        (_, Valor::Int(0)) | (_, Valor::Float(0.0)) => Err("Error: división por cero".into()),
        (Valor::Int(x), Valor::Int(y)) => Ok(Valor::Float(*x as f64 / *y as f64)),
        (Valor::Int(x), Valor::Float(y)) => Ok(Valor::Float(*x as f64 / *y)),
        (Valor::Float(x), Valor::Int(y)) => Ok(Valor::Float(*x / *y as f64)),
        (Valor::Float(x), Valor::Float(y)) => Ok(Valor::Float(*x / *y)),
        _ => Err("Error: división no soportada entre estos tipos".into()),
    }
}

fn operar_modulo(a: &Valor, b: &Valor) -> Result<Valor, String> {
    match (a, b) {
        (Valor::Int(x), Valor::Int(y)) => {
            if *y == 0 {
                Err("Error: módulo por cero".into())
            } else {
                Ok(Valor::Int(x % y))
            }
        }
        _ => Err("Error: módulo solo soportado para enteros".into()),
    }
}

fn expandir_variables(texto: &str, vars: &std::collections::HashMap<String, Valor>) -> String {
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
