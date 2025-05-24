#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Bash(Expression),
    Session(Expression),
    Attach(Expression),
    Window(Expression, Expression),
}

#[derive(Debug)]
pub enum Expression {
    Literal(String),
}
