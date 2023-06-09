use crate::{lexer::Lexer, token::Token};

use super::{
    ast::{Expression, Statement},
    program::Program,
    Parser,
};

fn test_parser_errors(parser: &Parser) {
    if parser.errors.is_empty() {
        return;
    }

    println!("parser has {} errors", parser.errors.len());
    for error in &parser.errors {
        println!("parser error: {}", error);
    }
    panic!("parser has {} errors", parser.errors.len());
}

fn setup_and_validate(input: &str, expected_statements: usize) -> Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    test_parser_errors(&parser);

    assert_eq!(
        program.statments.len(),
        expected_statements,
        "program.statements does not contain {} statements. got={:?}",
        expected_statements,
        program.statments
    );

    program
}

#[test]
fn test_parse_let_statements() {
    let input = r#"
    let x = 5;
    let y = 10;
    let foobar = 838383;
    "#;

    let program = setup_and_validate(input, 3);

    let tests = vec!["x", "y", "foobar"];

    for (i, test) in tests.iter().enumerate() {
        let stmt = &program.statments[i];
        test_let_statement(stmt, test);
    }
}

fn test_let_statement(smt: &Statement, name: &str) {
    match smt {
        Statement::LetStatement { ident, value: _ } => match ident {
            Token::Ident(s) => assert_eq!(s, name, "ident not '{}'. got={}", name, s),
            _ => panic!("ident not Token::Ident. got={}", ident),
        },
        _ => panic!("smt not Statement::LetStatement. got={}", smt),
    }
}

#[test]
fn test_parse_return_statements() {
    let input = r#"
    return 5;
    return 10;"#;

    let program = setup_and_validate(input, 2);

    for stmt in &program.statments {
        match stmt {
            Statement::ReturnStatement(_) => (),
            _ => panic!("stmt not Statement::ReturnStatement. got={}", stmt),
        }
    }
}

#[test]
fn test_parse_ident_expression() {
    let input = "foobar;";

    let program = setup_and_validate(input, 1);

    let stmt = program.statments.first().unwrap();
    match stmt {
        Statement::ExpressionStatement(expr) => match expr {
            Expression::IdentifierExpression(ident) => {
                assert_eq!(ident, "foobar", "ident not 'foobar'. got={}", ident)
            }
            _ => panic!("expr not Expression::Ident. got={}", expr),
        },
        _ => panic!("stmt not Statement::ExpressionStatement. got={}", stmt),
    }
}

#[test]
fn test_parse_integer_expression() {
    let input = "5;";

    let program = setup_and_validate(input, 1);

    let stmt = program.statments.first().unwrap();

    match stmt {
        Statement::ExpressionStatement(expr) => test_integer_expression(expr, &5),
        _ => panic!("stmt not Statement::ExpressionStatement. got={}", stmt),
    }
}

fn test_integer_expression(expr: &Expression, expect_value: &i64) {
    match expr {
        Expression::IntExpression(i) => {
            assert_eq!(i, expect_value, "value not '{}'. got={}", expect_value, i)
        }
        _ => panic!("expr not Expression::IntExpression. got={}", expr),
    }
}

#[test]
fn test_parse_prefix_expression() {
    let input = "-15;";

    let program = setup_and_validate(input, 1);

    let stmt = program.statments.first().unwrap();

    match stmt {
        Statement::ExpressionStatement(expr) => match expr {
            Expression::PrefixExpression { op_token, right } => {
                assert_eq!(
                    op_token,
                    &Token::Minus,
                    "op_token not '-'. got={}",
                    op_token
                );

                test_integer_expression(right, &15);
            }
            _ => panic!("expr not Expression::PrefixExpression. got={}", expr),
        },
        _ => panic!("stmt not Statement::ExpressionStatement. got={}", stmt),
    }
}

#[test]
fn test_parse_infix_expression() {
    let input = "5 + 4089;";

    let program = setup_and_validate(input, 1);

    let stmt = program.statments.first().unwrap();

    match stmt {
        Statement::ExpressionStatement(expr) => match expr {
            Expression::InfixExpression {
                left,
                op_token,
                right,
            } => {
                assert_eq!(op_token, &Token::Plus, "op_token not '+'. got={}", op_token);

                test_integer_expression(left, &5);
                test_integer_expression(right, &4089);
            }
            _ => panic!("expr not Expression::PrefixExpression. got={}", expr),
        },
        _ => panic!("stmt not Statement::ExpressionStatement. got={}", stmt),
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
    ];

    for (input, expected) in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        if !parser.errors.is_empty() {
            println!("parser has {} errors", parser.errors.len());
            for error in &parser.errors {
                println!("parser error: {}", error);
            }
            panic!("parser has {} errors. testcase: {}", parser.errors.len(), input);
        }
    

        let actual = program.to_string();
        assert_eq!(actual, expected, "expected={}, got={}", expected, actual);
    }
}
