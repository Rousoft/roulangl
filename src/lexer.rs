use crate::ast::Token;

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    let mut in_multiline_comment = false;

    for line in source.lines() {
        let mut line = line.trim();

        // Manejar comentario multilínea que empieza con "#/" y termina con "#/"
        if in_multiline_comment {
            if line.ends_with("#/") {
                in_multiline_comment = false;
            }
            continue; // ignorar líneas dentro del comentario multilínea
        } else if line.starts_with("#/") {
            if !line.ends_with("#/") || line.len() <= 2 {
                // comentario multilínea inicia
                in_multiline_comment = true;
            }
            continue; // ignorar esta línea de comentario multilínea
        }

        // Eliminar comentario inline que empieza con '#', solo si no es multilínea
        if let Some(pos) = line.find('#') {
            line = &line[..pos].trim_end();
        }

        if line.is_empty() {
            continue;
        }

        // Detectar iniciar</>
        if line.starts_with("iniciar</>") {
            tokens.push(Token::Iniciar);
            continue;
        }

        // Detectar </> final
        if line.starts_with("</>") {
            tokens.push(Token::Fin);
            continue;
        }

        // Variables mutables
        if line.starts_with("mutar ") {
            tokens.push(Token::Mutar);
            let resto = line["mutar ".len()..].trim();
            tokens.push(Token::Texto(resto.to_string()));
            continue;
        }

        // Variables inmutables
        if line.starts_with("var ") {
            tokens.push(Token::Var);
            let resto = line["var ".len()..].trim();
            tokens.push(Token::Texto(resto.to_string()));
            continue;
        }

        // imprimir(...)
        if line.starts_with("imprimir(") {
            tokens.push(Token::Imprimir);
            if let Some(fin) = line.find(')') {
                let contenido = &line["imprimir(".len()..fin];
                tokens.push(Token::Texto(contenido.to_string()));
            } else {
                return Err("Error: imprimir sin ')'".into());
            }
            continue;
        }

        return Err(format!("Token desconocido o sintaxis no soportada: {}", line));
    }

    Ok(tokens)
}
