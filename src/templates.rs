use tera::{Tera, Value, Result as TeraResult};
use std::collections::HashMap;

fn format_score(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    match value {
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                if f.fract() == 0.0 {
                    Ok(Value::String(format!("{}", f as i64)))
                } else {
                    Ok(Value::String(format!("{}", f)))
                }
            } else {
                Ok(value.clone())
            }
        }
        _ => Ok(value.clone()),
    }
}

pub fn init_tera() -> Tera {
    match Tera::new("templates/**/*.html") {
        Ok(mut t) => {
            t.register_filter("format_score", format_score);
            t
        },
        Err(e) => {
            eprintln!("Template parsing error(s): {}", e);
            std::process::exit(1);
        }
    }
}