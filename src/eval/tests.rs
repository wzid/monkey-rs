use crate::{
    lexer::Lexer,
    parser::{ast::Ast, Parser},
};

use super::{eval, value::Value};

fn test_eval(input: &str) -> Value {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    assert!(parser.errors.is_empty(), "Parser had errors");

    eval(&Ast::Program(program))
}

#[test]
fn test_eval_integer_expression() {
    let tests = vec![
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5+5+5+5", 20),
        ("5+5+5+5-10", 10),
        ("2*2*2*2*2", 32),
        ("3 * (3 *3)+10", 37),
        ("(5+10*2+15/3)*2+-10", 50),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_integer_object(evaluated, expected);
    }
}

fn test_integer_object(value: Value, expected: i64) {
    match value {
        Value::Integer(i) => assert_eq!(expected, i, "value is not {}. got {}", expected, i),
        _ => panic!("value is not an Integer. got {:?}", value),
    }
}

#[test]
fn test_eval_boolean_expression() {
    let tests = vec![
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("false != true", true),
        ("false == false", true),
        ("true == false", false),
        ("(1 > 2) == false", true),
        ("(1 < 2) == false", false),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_boolean_object(evaluated, expected);
    }
}

fn test_boolean_object(value: Value, expected: bool) {
    match value {
        Value::Boolean(b) => assert_eq!(expected, b, "value is not {}. got {}", expected, b),
        _ => panic!("value is not an Boolean. got {:?}", value),
    }
}

#[test]
fn test_bang_operator() {
    let tests = vec![
        ("!true", false),
        ("!false", true),
        ("!!!true", false),
        ("!5", false),
        ("!!5", true),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_boolean_object(evaluated, expected);
    }
}

#[test]
fn test_if_else_expressions() {
    let tests = vec![
        ("if true { 10 }", Some(10)),
        ("if false { 10 }", None),
        ("if 1 > 2 { 10 }", None),
        ("if 1 < 2 { 10 }", Some(10)),
        ("if 1 > 2 { 10 } else { 20 }", Some(20)),
        ("if 1 < 2 { 10 } else { 20 }", Some(10)),
    ];


    for (input , expected) in tests {
        let evaluated = test_eval(input);

        match expected {
            Some(i) => test_integer_object(evaluated, i),
            None => assert!(evaluated.is_null(), "value is not Null. got {:?}", evaluated)
        }
    }
}

#[test]
fn test_return_statements() {
    let tests = vec![
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2*5; 9;", 10),
        ("9; return 2*5; 9;", 10),
        ("if 10 > 1 { if 10 > 1 { return 10; } return 1; }", 10),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_integer_object(evaluated, expected);
    }
}