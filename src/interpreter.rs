use crate::ast::{Expression, Program, Statement};
use std::process::Command;

pub fn run(program: Program) {
    for stmt in program.statements {
        match stmt {
            Statement::Session(expr) => {
                let Expression::Literal(val) = expr;
                Command::new("tmux")
                    .arg("new-session")
                    .arg("-d") // run detached
                    .arg("-s")
                    .arg(val) // name of the session
                    .output()
                    .expect("Failed to start tmux session");
            }
        }
    }
}
