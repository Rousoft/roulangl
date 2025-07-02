#[derive(Debug)]
pub enum Token {
    Iniciar,
    Fin,
    Var,
    Mutar,
    Imprimir,
    Texto(String),
}

#[derive(Debug)]
pub enum Instruccion {
    Var(String, Valor),      // nombre, valor
    Mutar(String, Valor),    // nombre, valor
    Imprimir(String),
}

#[derive(Debug, Clone)]
pub enum Valor {
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
}
