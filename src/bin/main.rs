use std::io::{stdin, stdout, Write};

use monkey_rs::{lexer, parser::{Parser, ast::Ast}, token, eval::eval};
extern crate monkey_rs;

use lexer::Lexer;

fn main() {
    println!("Monkey v1.0");
    println!("REPL Mode");

    loop {
        run();
    }
}

// Final version
fn run() {
    print!(">> ");
    stdout().flush().unwrap();

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    let lexer = Lexer::new(input.as_str());

    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    if parser.errors.len() != 0 {
        print_parse_errors(&parser.errors)
    } else {
        let evaluated = eval(&Ast::Program(program));
        println!("{}", evaluated);
    }
}

fn print_parse_errors(errors: &Vec<String>) {
    println!("Woops! We ran into some monkey business here!");
    println!(" parser errors: ");
    for err in errors {
        println!("\t{}", err);
    }
}

// Lexer REPL
#[allow(dead_code)]
fn print_token_output() {
    print!(">> ");
    stdout().flush().unwrap();

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    let mut lexer = Lexer::new(input.as_str());

    loop {
        match lexer.next_token() {
            token![EOF] => break,
            token => println!("{}", token),
        }
    }
}

// Parser REPL
#[allow(dead_code)]
fn print_parser_output() {
    print!(">> ");
    stdout().flush().unwrap();

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    let lexer = Lexer::new(input.as_str());

    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    if parser.errors.len() != 0 {
        print_parse_errors(&parser.errors)
    } else {
        println!("{}", program);
    }
}
