use crate::ast::{Expression, Program, Statement};

// get quoted argument for functions
fn extract_quoted_literal(input: &str) -> Result<String, String> {
    let input = input.trim();
    if input.starts_with('"') && input.ends_with('"') && input.len() >= 2 {
        Ok(input[1..input.len() - 1].to_string())
    } else {
        Err(format!("Expected quoted string, got: {input}"))
    }
}

pub fn parse(input: &str) -> Result<Program, String> {
    let mut statements = Vec::new();
    let mut session_name: Option<String> = None; // safe session name and avoid multiple ones

    for line in input.lines() {
        let line = line.trim();

        // skip shebang, comments and empty lines
        if line.starts_with("#!") || line.starts_with("//") || line.is_empty() {
            continue;
        }

        if let Some(rest) = line.strip_prefix("bash") {
            let value = extract_quoted_literal(rest)?;
            statements.push(Statement::Bash(Expression::Literal(value)));
        } else if let Some(rest) = line.strip_prefix("session") {
            let value = extract_quoted_literal(rest)?;

            if session_name.is_some() {
                return Err("Only one 'session' statement allowed.".to_string());
            }
            session_name = Some(value.clone());

            statements.push(Statement::Session(Expression::Literal(value)));
        } else if line == "attach" {
            // attach to previously declared session name
            match &session_name {
                Some(name) => {
                    statements.push(Statement::Attach(Expression::Literal(name.clone())));
                }
                None => {
                    return Err("Attach statement found but no session declared yet.".to_string());
                }
            }
        } else {
            return Err(format!("Unknown statement: {line}"));
        }
    }

    Ok(Program { statements })
}

