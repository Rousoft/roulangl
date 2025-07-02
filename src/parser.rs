use crate::ast::{Token, Instruccion, Valor};

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
                if i + 1 >= tokens.len() {
                    return Err("Error: var sin contenido".into());
                }
                if let Token::Texto(texto) = &tokens[i + 1] {
                    let inst = parse_var(texto)?;
                    instrucciones.push(inst);
                    i += 2;
                } else {
                    return Err("Error: var sin texto".into());
                }
            }
            Token::Mutar => {
                if i + 1 >= tokens.len() {
                    return Err("Error: mutar sin contenido".into());
                }
                if let Token::Texto(texto) = &tokens[i + 1] {
                    let inst = parse_mutar(texto)?;
                    instrucciones.push(inst);
                    i += 2;
                } else {
                    return Err("Error: mutar sin texto".into());
                }
            }
            Token::Imprimir => {
                if !dentro_bloque {
                    return Err("Error: 'imprimir' solo permitido dentro de iniciar</> ... </>".into());
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

fn parse_var(texto: &str) -> Result<Instruccion, String> {
    let texto = texto.trim();
    if !texto.ends_with(';') {
        return Err(format!("Error: falta ';' al final de la declaración var: {}", texto));
    }
    let texto = &texto[..texto.len()-1]; // quitar ';'

    let parts: Vec<&str> = texto.split('=').map(|s| s.trim()).collect();
    if parts.len() != 2 {
        return Err(format!("Error en declaración var: {}", texto));
    }
    let nombre = parts[0].to_string();
    let valor = parse_valor(parts[1])?;
    Ok(Instruccion::Var(nombre, valor))
}

fn parse_mutar(texto: &str) -> Result<Instruccion, String> {
    let texto = texto.trim();
    if !texto.ends_with(';') {
        return Err(format!("Error: falta ';' al final de la declaración mutar: {}", texto));
    }
    let texto = &texto[..texto.len()-1]; // quitar ';'

    let parts: Vec<&str> = texto.split('=').map(|s| s.trim()).collect();
    if parts.len() != 2 {
        return Err(format!("Error en declaración mutar: {}", texto));
    }
    let nombre = parts[0].to_string();
    let valor = parse_valor(parts[1])?;
    Ok(Instruccion::Mutar(nombre, valor))
}

fn parse_valor(texto: &str) -> Result<Valor, String> {
    let texto = texto.trim();
    if texto.starts_with('"') && texto.ends_with('"') {
        Ok(Valor::String(texto[1..texto.len()-1].to_string()))
    } else if texto.starts_with('\'') && texto.ends_with('\'') {
        let chars: Vec<char> = texto[1..texto.len()-1].chars().collect();
        if chars.len() != 1 {
            return Err(format!("Error: char debe tener un solo caracter: {}", texto));
        }
        Ok(Valor::Char(chars[0]))
    } else if let Ok(i) = texto.parse::<i64>() {
        Ok(Valor::Int(i))
    } else if let Ok(f) = texto.parse::<f64>() {
        Ok(Valor::Float(f))
    } else {
        Err(format!("Valor no reconocido: {}", texto))
    }
}
