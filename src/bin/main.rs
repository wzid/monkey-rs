use std::io::{stdout, stdin, Write};

use monkey_rs::{lexer, token, parser::Parser};
extern crate monkey_rs;

use lexer::Lexer;

fn main() {

    println!("Monkey v1.0");
    println!("REPL Mode");
    
    loop {
        print_parser_output();
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
            token => println!("{}", token)
        }
    }
}

// Parser REPL
fn print_parser_output() {
    print!(">> ");
    stdout().flush().unwrap();
    
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    let lexer = Lexer::new(input.as_str());

    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    if parser.errors.len() != 0 {
        println!("Woops! We ran into some monkey business here!");
        println!(" parser errors: ");
        for error in parser.errors {
            println!("\t{}", error);
        }
    } else {
        println!("{:?}", program);
    }
}