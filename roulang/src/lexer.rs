use crate::ast::Token;

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    let mut in_multiline_comment = false;

    for line in source.lines() {
        let mut line = line.trim();

        if in_multiline_comment {
            if line.ends_with("#/") {
                in_multiline_comment = false;
            }
            continue;
        } else if line.starts_with("#/") {
            if !line.ends_with("#/") || line.len() <= 2 {
                in_multiline_comment = true;
            }
            continue;
        }

        if let Some(pos) = line.find('#') {
            line = &line[..pos].trim_end();
        }

        if line.is_empty() {
            continue;
        }

        if line == "iniciar</>" {
            tokens.push(Token::Iniciar);
            continue;
        }
        if line == "</>" {
            tokens.push(Token::Fin);
            continue;
        }
        if line.starts_with("imprimir(") {
            tokens.push(Token::Imprimir);
            if let Some(fin) = line.find(')') {
                let contenido = &line["imprimir(".len()..fin];
                tokens.push(Token::Texto(contenido.to_string()));
                continue;
            } else {
                return Err("Error: imprimir sin ')'".into());
            }
        }
        if line.starts_with("var ") {
            tokens.push(Token::Var);
            let resto = &line["var ".len()..];
            tokenize_expresion(resto, &mut tokens)?;
            continue;
        }
        if line.starts_with("mutar ") {
            tokens.push(Token::Mutar);
            let resto = &line["mutar ".len()..];
            tokenize_expresion(resto, &mut tokens)?;
            continue;
        }

        return Err(format!("Token desconocido o sintaxis no soportada: {}", line));
    }

    Ok(tokens)
}

fn tokenize_expresion(texto: &str, tokens: &mut Vec<Token>) -> Result<(), String> {
    let mut i = 0;
    let chars: Vec<char> = texto.chars().collect();
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c.is_alphabetic() || c == '_' {
            let start = i;
            i += 1;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let palabra: String = chars[start..i].iter().collect();
            tokens.push(Token::Identificador(palabra));
            continue;
        }
        if c.is_digit(10) {
            let start = i;
            i += 1;
            let mut tiene_punto = false;
            while i < chars.len() && (chars[i].is_digit(10) || (!tiene_punto && chars[i] == '.')) {
                if chars[i] == '.' {
                    tiene_punto = true;
                }
                i += 1;
            }
            let numero: String = chars[start..i].iter().collect();
            tokens.push(Token::Numero(numero));
            continue;
        }
        if c == '"' {
            i += 1;
            let start = i;
            while i < chars.len() && chars[i] != '"' {
                i += 1;
            }
            if i >= chars.len() {
                return Err("Error: cadena no cerrada".to_string());
            }
            let cadena: String = chars[start..i].iter().collect();
            i += 1;
            tokens.push(Token::Cadena(cadena));
            continue;
        }
        match c {
            '+' | '-' | '*' | '/' | '%' | '=' => {
                tokens.push(Token::Operador(c));
                i += 1;
            }
            '(' => {
                tokens.push(Token::ParenAbre);
                i += 1;
            }
            ')' => {
                tokens.push(Token::ParenCierra);
                i += 1;
            }
            ';' => {
                tokens.push(Token::PuntoYComa);
                i += 1;
            }
            ',' => {
                tokens.push(Token::Coma);
                i += 1;
            }
            _ => return Err(format!("Carácter no reconocido en expresión: '{}'", c)),
        }
    }
    Ok(())
}
