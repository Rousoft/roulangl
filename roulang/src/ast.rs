#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Iniciar,
    Fin,
    Var,
    Mutar,
    Imprimir,
    Identificador(String),
    Numero(String),
    Operador(char),
    ParenAbre,
    ParenCierra,
    PuntoYComa,
    Coma,              // Agregado para coma ','
    Texto(String),
    Cadena(String),
}

#[derive(Debug, Clone)]
pub enum Valor {
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
}

#[derive(Debug, Clone, Copy)]
pub enum Operador {
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Modulo,
}

#[derive(Debug, Clone)]
pub enum Expresion {
    Valor(Valor),
    Variable(String),
    BinOp {
        izquierda: Box<Expresion>,
        operador: Operador,
        derecha: Box<Expresion>,
    },
}

#[derive(Debug)]
pub enum Instruccion {
    Var(String, Expresion),
    Mutar(String, Expresion),
    Imprimir(String),
}
