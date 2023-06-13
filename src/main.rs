use std::io::{stdin, stdout, Write};

extern crate monkey_lib;
use monkey_lib::{
    eval::Evaluator,
    lexer,
    parser::{self, ast::Ast},
};

use clap::Parser;

use std::fs;

use lexer::Lexer;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The location of the monkey file
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(file) = args.file {
        let contents = fs::read_to_string(file).expect("Unable to find or read file");

        run(contents);
    } else {
        println!("Monkey v1.0");
        println!("REPL Mode");
        
        loop {
            let input = repl_input();
            run(input);
        }
    }
}

fn repl_input() -> String {
    print!(">> ");
    stdout().flush().unwrap();

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    input
}

fn run(input: String) {
    let lexer = Lexer::new(input.as_str());

    let mut parser = parser::Parser::new(lexer);

    let program = parser.parse_program();

    if parser.errors.len() != 0 {
        print_parse_errors(&parser.errors)
    } else {
        let result = Evaluator::eval(&Ast::Program(program));

        match result {
            Ok(v) => println!("{}", v),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn print_parse_errors(errors: &Vec<String>) {
    println!("Woops! We ran into some monkey business here!");
    println!(" parser errors: ");
    for err in errors {
        println!("\t{}", err);
    }
}
