use crate::ast::{Program, Statement, Expression};

pub fn parse(input: &str) -> Result<Program, String> {
    let mut statements = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        // Skip shebang (#!)
        if line.starts_with("#!") {
            continue;
        }

        if let Some(rest) = line.strip_prefix("session ") {
            // trim_matches takes chars or closures, not strings!
            let value = rest.trim().trim_matches('"').to_string();
            let expr = Expression::Literal(value);
            statements.push(Statement::Session(expr));
        } else if !line.is_empty() {
            return Err(format!("Unknown statement: {line}"));
        }
    }

    Ok(Program { statements })
}

