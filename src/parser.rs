use crate::ast::{Expression, Program, Statement};

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

    for line in input.lines() {
        let line = line.trim();

        // shebang (#!)
        if line.starts_with("#!") {
            continue;
        }

        // comments (//)
        if line.starts_with("//") {
            continue;
        }

        if let Some(rest) = line.strip_prefix("bash") {
            let value = extract_quoted_literal(rest)?;
            statements.push(Statement::Bash(Expression::Literal(value)));
        } else if let Some(rest) = line.strip_prefix("session") {
            let value = extract_quoted_literal(rest)?;
            statements.push(Statement::Session(Expression::Literal(value)));
        } else if let Some(rest) = line.strip_prefix("attach") {
            let value = extract_quoted_literal(rest)?;
            statements.push(Statement::Attach(Expression::Literal(value)));
        } else if !line.is_empty() {
            return Err(format!("Unknown statement: {line}"));
        }
    }

    Ok(Program { statements })
}
