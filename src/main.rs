use std::io::{stdin, stdout, Write};

use monkey_lib::{
    lexer,
    parser::{self, program::Program},
    Monkey,
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

    let mut monkey = Monkey::default();


    if let Some(file) = args.file {
        let contents = fs::read_to_string(file).expect("Unable to find or read file");

        if let Ok(program) = lex_and_parse(contents) {
            evaluate(program, &mut monkey);
        }
    } else {
        println!("Monkey v1.0");
        println!("REPL Mode");

        loop {
            let input = get_input();

            if let Ok(program) = lex_and_parse(input) {
                evaluate(program, &mut monkey);
            }
        }
    }
}

fn get_input() -> String {
    print!(">> ");
    stdout().flush().unwrap();

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    input
}

fn evaluate(program: Program, monkey: &mut Monkey) {
    if let Err(msg) = monkey.eval(program) {
        println!("Error: {msg}");
    }
}

fn lex_and_parse(input: String) -> Result<Program, ()> {
    let lexer = Lexer::new(input.as_str());

    let mut parser = parser::Parser::new(lexer);

    let program = parser.parse_program();

    if !parser.errors.is_empty() {
        print_parse_errors(&parser.errors);
        return Err(());
    }

    Ok(program)
}

fn print_parse_errors(errors: &Vec<String>) {
    println!("Woops! We ran into some monkey business here!");
    println!(" parser errors: ");
    for err in errors {
        println!("\t{err}");
    }
}
