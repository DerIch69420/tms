#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Session(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Literal(String),
}
