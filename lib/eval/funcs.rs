use super::{value::Value, EvaluatorErr};

use std::io::{stdin, stdout, Write};

pub type BuiltInFunctionType = fn(Vec<Value>) -> Result<Value, EvaluatorErr>;

pub fn get_function(name: &str) -> Option<BuiltInFunctionType> {
    match name {
        "len" => Some(len),
        "println" => Some(println),
        "input" => Some(input),
        _ => None,
    }
}

fn len(args: Vec<Value>) -> Result<Value, EvaluatorErr> {
    if args.len() == 1 {
        match args.first().unwrap() {
            Value::String(s) => Ok(Value::Integer(s.len() as i64)),
            _ => Err(format!("Cannot take length of {}", args.first().unwrap())),
        }
    } else {
        return Err(format!(
            "len() takes 1 argument, {} arguments given",
            args.len()
        ));
    }
}

fn println(args: Vec<Value>) -> Result<Value, EvaluatorErr> {
    let result = args
        .iter()
        .map(|arg| arg.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{result}");

    Ok(Value::Null)
}

fn input(args: Vec<Value>) -> Result<Value, EvaluatorErr> {
    if args.len() == 1 {
        match args.first().unwrap() {
            Value::String(s) => {
                print!("{s}");
                stdout().flush().unwrap();

                let mut input = String::new();

                stdin().read_line(&mut input).expect("Failed to read line");

                // Remove trailing new line character and trim whitespace
                input = input.trim().to_string();

                if let Ok(parsed_int) = input.parse::<i64>() {
                    Ok(Value::Integer(parsed_int))
                } else {
                    Ok(Value::String(input))
                }
            }
            _ => Err(format!("Cannot take input of {}", args.first().unwrap())),
        }
    } else {
        return Err(format!(
            "input() takes 1 argument, {} arguments given",
            args.len()
        ));
    }
}
