use crate::ast::{Expression, Program, Statement};
use std::process::Command;

pub fn run(program: Program) {
    for stmt in program.statements {
        match stmt {
            Statement::Bash(expr) => {
                let Expression::Literal(val) = expr;
                let output = Command::new("bash")
                    .arg("-c") // run string as command
                    .arg(&val)
                    .output()
                    .expect("Failed to run bash command");

                print!("{}", String::from_utf8_lossy(&output.stdout));
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }

            Statement::Session(expr) => {
                let Expression::Literal(val) = expr;
                let output = Command::new("tmux")
                    .arg("new-session")
                    .arg("-d") // run detached
                    .arg("-s")
                    .arg(val) // name of the session
                    .output()
                    .expect("Failed to start tmux session");

                print!("{}", String::from_utf8_lossy(&output.stdout));
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }

            Statement::Attach(expr) => {
                let Expression::Literal(val) = expr;
                let status = Command::new("tmux")
                    .arg("attach")
                    .arg("-t")
                    .arg(val) // name of the session
                    .status()
                    .expect("Failed to attach to tmux session");

                if !status.success() {
                    eprintln!("tmux attach failed with code {:?}", status.code());
                }
            }
        }
    }
}
