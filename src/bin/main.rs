use std::io::{stdout, stdin, Write};

use monkey_rs::{lexer, token};
extern crate monkey_rs;

use lexer::Lexer;

fn main() {

    println!("Monkey v1.0");
    println!("REPL Mode");
    
    loop {
        print_token_input();
    }
}

fn print_token_input() {
    print!(">> ");
    stdout().flush().unwrap();
    
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    let mut lexer = Lexer::new(input.as_str());

    loop {
        match lexer.next_token() {
            token![EOF] => break,
            token => println!("{:?}", token)
        }
    }
}