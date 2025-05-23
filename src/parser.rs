use crate::ast::{Expression, Program, Statement};

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
            let value = rest.trim().trim_matches('"').to_string();
            let expr = Expression::Literal(value);
            statements.push(Statement::Bash(expr));
        } else if let Some(rest) = line.strip_prefix("session") {
            let value = rest.trim().trim_matches('"').to_string();
            let expr = Expression::Literal(value);
            statements.push(Statement::Session(expr));
        } else if let Some(rest) = line.strip_prefix("attach") {
            let value = rest.trim().trim_matches('"').to_string();
            let expr = Expression::Literal(value);
            statements.push(Statement::Attach(expr));
        } else if !line.is_empty() {
            return Err(format!("Unknown statement: {line}"));
        }
    }

    Ok(Program { statements })
}
