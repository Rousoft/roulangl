use crate::ast::{Token, Instruccion, Expresion, Valor, Operador};

pub fn parse(tokens: &[Token]) -> Result<Vec<Instruccion>, String> {
    let mut instrucciones = Vec::new();
    let mut dentro_bloque = false;
    let mut inicio_encontrado = false;
    let mut fin_encontrado = false;

    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Iniciar => {
                if inicio_encontrado {
                    return Err("Error: múltiples bloques iniciar</> detectados".into());
                }
                inicio_encontrado = true;
                dentro_bloque = true;
                i += 1;
            }
            Token::Fin => {
                if !inicio_encontrado {
                    return Err("Error: bloque </> sin bloque iniciar</>".into());
                }
                if fin_encontrado {
                    return Err("Error: múltiples bloques </> detectados".into());
                }
                fin_encontrado = true;
                dentro_bloque = false;
                i += 1;
            }
            Token::Var => {
                if !dentro_bloque {
                    return Err("Error: 'var' solo dentro de iniciar</> ... </>".into());
                }
                let (inst, fin) = parse_declaracion_var_mutar(&tokens, i+1)?;
                instrucciones.push(inst);
                i = fin;
            }
            Token::Mutar => {
                if !dentro_bloque {
                    return Err("Error: 'mutar' solo dentro de iniciar</> ... </>".into());
                }
                let (inst, fin) = parse_declaracion_var_mutar(&tokens, i+1)?;
                instrucciones.push(inst);
                i = fin;
            }
            Token::Imprimir => {
                if !dentro_bloque {
                    return Err("Error: 'imprimir' solo dentro de iniciar</> ... </>".into());
                }
                if i + 1 >= tokens.len() {
                    return Err("Error: imprimir sin contenido".into());
                }
                if let Token::Texto(texto) = &tokens[i + 1] {
                    instrucciones.push(Instruccion::Imprimir(texto.clone()));
                    i += 2;
                } else {
                    return Err("Error: imprimir sin texto".into());
                }
            }
            _ => return Err("Token inesperado".into()),
        }
    }

    if !inicio_encontrado {
        return Err("Error: no se encontró el bloque obligatorio iniciar</>".into());
    }
    if !fin_encontrado {
        return Err("Error: no se encontró el bloque obligatorio </>".into());
    }

    Ok(instrucciones)
}

fn parse_declaracion_var_mutar(tokens: &[Token], mut pos: usize) -> Result<(Instruccion, usize), String> {
    // Esperamos: Identificador = expresión ;
    if pos >= tokens.len() {
        return Err("Error: declaración incompleta".into());
    }
    let nombre = match &tokens[pos] {
        Token::Identificador(id) => id.clone(),
        _ => return Err("Error: se esperaba identificador en declaración".into()),
    };
    pos += 1;

    if pos >= tokens.len() {
        return Err("Error: declaración incompleta, falta '='".into());
    }
    match &tokens[pos] {
        Token::Operador('=') => pos +=1,
        _ => return Err("Error: se esperaba '=' en declaración".into()),
    }

    // Parsear expresión desde pos hasta encontrar punto y coma
    let (expr, siguiente_pos) = parse_expresion(tokens, pos)?;

    // El siguiente token debe ser ;
    if siguiente_pos >= tokens.len() {
        return Err("Error: falta ';' al final de declaración".into());
    }
    match &tokens[siguiente_pos] {
        Token::PuntoYComa => {},
        _ => return Err("Error: falta ';' al final de declaración".into()),
    }

    // Determinar si es var o mutar:
    let instruccion = if tokens[pos-2] == Token::Var {
        Instruccion::Var(nombre, expr)
    } else {
        Instruccion::Mutar(nombre, expr)
    };

    Ok((instruccion, siguiente_pos +1))
}

// Parseo recursivo simple con precedencia para expresiones aritméticas
fn parse_expresion(tokens: &[Token], pos: usize) -> Result<(Expresion, usize), String> {
    parse_expr_precedence(tokens, pos, 0)
}

// Precedencia de operadores (más alto = mayor precedencia)
fn get_precedencia(op: char) -> usize {
    match op {
        '+' | '-' => 1,
        '*' | '/' | '%' => 2,
        _ => 0,
    }
}

// Parseo con precedencia (recursivo)
fn parse_expr_precedence(tokens: &[Token], pos: usize, min_prec: usize) -> Result<(Expresion, usize), String> {
    let mut pos = pos; // mutable local para avanzar

    // Parseamos el lado izquierdo (factor)
    let (mut izquierda, mut pos) = parse_factor(tokens, pos)?;

    loop {
        if pos >= tokens.len() {
            break;
        }
        // Verificamos si hay operador y su precedencia
        let op = match &tokens[pos] {
            Token::Operador(c) => *c,
            _ => break,
        };
        let prec = get_precedencia(op);
        if prec < min_prec {
            break;
        }
        pos += 1;

        // Parseamos el lado derecho con precedencia superior
        let (derecha, siguiente_pos) = parse_expr_precedence(tokens, pos, prec +1)?;
        pos = siguiente_pos;

        izquierda = Expresion::BinOp {
            izquierda: Box::new(izquierda),
            operador: match op {
                '+' => Operador::Suma,
                '-' => Operador::Resta,
                '*' => Operador::Multiplicacion,
                '/' => Operador::Division,
                '%' => Operador::Modulo,
                _ => return Err(format!("Operador desconocido: {}", op)),
            },
            derecha: Box::new(derecha),
        };
    }
    Ok((izquierda, pos))
}

// Parsear un factor: número, variable, o expresión entre paréntesis
fn parse_factor(tokens: &[Token], pos: usize) -> Result<(Expresion, usize), String> {
    if pos >= tokens.len() {
        return Err("Error: expresión incompleta".into());
    }
    match &tokens[pos] {
        Token::Numero(n) => {
            // parsear int o float
            if let Ok(i) = n.parse::<i64>() {
                Ok((Expresion::Valor(Valor::Int(i)), pos+1))
            } else if let Ok(f) = n.parse::<f64>() {
                Ok((Expresion::Valor(Valor::Float(f)), pos+1))
            } else {
                Err(format!("Número inválido: {}", n))
            }
        }
        Token::Cadena(s) => Ok((Expresion::Valor(Valor::String(s.clone())), pos+1)),
        Token::Identificador(id) => Ok((Expresion::Variable(id.clone()), pos+1)),
        Token::ParenAbre => {
            let (expr, siguiente_pos) = parse_expresion(tokens, pos+1)?;
            if siguiente_pos >= tokens.len() {
                return Err("Error: falta ')'".into());
            }
            match &tokens[siguiente_pos] {
                Token::ParenCierra => Ok((expr, siguiente_pos + 1)),
                _ => Err("Error: falta ')'".into()),
            }
        }
        _ => Err(format!("Token inesperado en expresión: {:?}", tokens[pos])),
    }
}
