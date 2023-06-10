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
    let tests = vec![
        ("let x = 5;", "x", "5"),
        ("let y = true;", "y", "true"),
        ("let foobar = y;", "foobar", "y"),
    ];

    for (input, expected_ident, expected_value) in tests {
        let program = setup_and_validate(input, 1);

        let stmt = program.statments.first().unwrap();

        test_let_statement(stmt, expected_ident, expected_value);
    }
}

fn test_let_statement(smt: &Statement, expected_name: &str, expected_value: &str) {
    match smt {
        Statement::LetStatement { ident, value } => match ident {
            Token::Ident(name) => {
                assert_eq!(expected_name, name, "ident not '{}'. got={}", expected_name, name );
                assert_eq!(expected_value, value.to_string(), "value not '{}'. got={}", expected_value, value)
            },
            _ => panic!("ident not Token::Ident. got={}", ident),
        },
        _ => panic!("smt not Statement::LetStatement. got={}", smt),
    }
}

#[test]
fn test_parse_return_statements() {
    let tests = vec![
        ("return 5;", "5"),
        ("return x;", "x"),
    ];

    for (input, expected_value) in tests {
        let program = setup_and_validate(input, 1);

        let stmt = program.statments.first().unwrap();

        match stmt {
            Statement::ReturnStatement(return_value) => {
                assert_eq!(expected_value, return_value.to_string(), "return_value not '{}'. got={}", expected_value, return_value)
            },
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
        Statement::ExpressionStatement(expr) => test_identifier(expr, "foobar"),
        _ => panic!("stmt not Statement::ExpressionStatement. got={}", stmt),
    }
}

fn test_identifier(expr: &Expression, value: &str) {
    match expr {
        Expression::IdentifierExpression(ident) => {
            assert_eq!(ident, value, "ident not '{}'. got={}", value, ident)
        }
        _ => panic!("expr not Expression::IdentifierExpression. got={}", expr),
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
        Statement::ExpressionStatement(expr) => test_infix_expression(expr, "5", "+", "4089"),
        _ => panic!("stmt not Statement::ExpressionStatement. got={}", stmt),
    }
}

fn test_infix_expression(
    expr: &Expression,
    expect_left: &str,
    expect_op: &str,
    expect_right: &str,
) {
    match expr {
        Expression::InfixExpression {
            left,
            op_token,
            right,
        } => {
            assert_eq!(
                expect_left,
                left.to_string(),
                "left not '{}'. got={}",
                expect_left,
                left
            );
            assert_eq!(
                op_token.to_string(),
                expect_op.to_string(),
                "op_token not '{}'. got={}",
                expect_op,
                op_token
            );
            assert_eq!(
                expect_right,
                right.to_string(),
                "right not '{}'. got={}",
                expect_right,
                right
            );
        }
        _ => panic!("expr not Expression::InfixExpression. got={}", expr),
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
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("(3 < 5) == true", "((3 < 5) == true)"),
        ("a * (b / c)", "(a * (b / c))"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
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
            panic!(
                "parser has {} errors. testcase: {}",
                parser.errors.len(),
                input
            );
        }

        let actual = program.to_string();
        assert_eq!(actual, expected, "expected={}, got={}", expected, actual);
    }
}

fn test_boolean_expression(expr: &Expression, expect_value: &bool) {
    match expr {
        Expression::BooleanExpression(b) => {
            assert_eq!(b, expect_value, "value not '{}'. got={}", expect_value, b)
        }
        _ => panic!("expr not Expression::BooleanExpression. got={}", expr),
    }
}

#[test]
fn test_parse_boolean_expression() {
    let tests = vec![("true;", true), ("false;", false)];

    for (input, expected) in tests {
        let program = setup_and_validate(input, 1);

        let stmt = program.statments.first().unwrap();

        match stmt {
            Statement::ExpressionStatement(expr) => test_boolean_expression(expr, &expected),
            _ => panic!("stmt not Statement::ExpressionStatement. got={}", stmt),
        }
    }
}

#[test]
fn test_parse_infix_boolean_expressions() {
    let tests = vec![
        ("true == false;", true, "==", false),
        ("true != false;", true, "!=", false),
        ("false == false;", false, "==", false),
    ];

    for (input, first, op, second) in tests {
        let program = setup_and_validate(input, 1);

        let stmt = program.statments.first().unwrap();

        match stmt {
            Statement::ExpressionStatement(expr) => {
                test_infix_expression(expr, &first.to_string(), op, &second.to_string())
            }
            _ => panic!("stmt not Statement::ExpressionStatement. got={}", stmt),
        }
    }
}

fn test_block_statements(stmt: &Statement, expect_len: usize, expect: Vec<&str>) {
    match stmt {
        Statement::BlockStatement(statements) => {
            assert_eq!(
                statements.len(),
                expect_len,
                "statements does not contain {} statements. got={:?}",
                expect_len,
                statements
            );

            for (i, test) in expect.iter().enumerate() {
                let stmt = &statements[i];
                assert_eq!(stmt.to_string(), *test, "stmt not '{}'. got={}", test, stmt);
            }
        }
        _ => panic!("stmt not Statement::BlockStatement. got={}", stmt),
    }
}

#[test]
fn test_parse_if_expression() {
    let input = "if x < y { x }";

    let program = setup_and_validate(input, 1);

    let stmt = program.statments.first().unwrap();

    match stmt {
        Statement::ExpressionStatement(expr) => match expr {
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                test_infix_expression(condition, "x", "<", "y");

                test_block_statements(&consequence, 1, vec!["x"]);

                assert_eq!(
                    alternative, &None,
                    "alternative is not None. got={:?}",
                    alternative
                );
            }
            _ => panic!("expr not Expression::IfExpression. got={}", expr),
        },
        _ => panic!("stmt not Statement::ExpressionStatement. got={}", stmt),
    }
}

#[test]
fn test_parse_if_else_expression() {
    let input = "if x { x } else { y } ";

    let program = setup_and_validate(input, 1);

    let stmt = program.statments.first().unwrap();

    match stmt {
        Statement::ExpressionStatement(expr) => match expr {
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                test_identifier(condition, "x");

                test_block_statements(&consequence, 1, vec!["x"]);

                match alternative {
                    Some(alt) => test_block_statements(alt, 1, vec!["y"]),
                    None => panic!("alternative is None. got={:?}", alternative),
                }
            }
            _ => panic!("expr not Expression::IfExpression. got={}", expr),
        },
        _ => panic!("stmt not Statement::ExpressionStatement. got={}", stmt),
    }
}

#[test]
fn test_parse_function_expression() {
    let input = "fn(x, y) { x + y; }";

    let program = setup_and_validate(input, 1);

    let stmt = program.statments.first().unwrap();

    match stmt {
        Statement::ExpressionStatement(expr) => match expr {
            Expression::FunctionExpression { parameters, body } => {
                assert_eq!(
                    parameters.len(),
                    2,
                    "parameters does not contain 2 parameters. got={:?}",
                    parameters
                );

                assert_eq!(parameters[0].to_string(), "x");
                assert_eq!(parameters[1].to_string(), "y");

                test_block_statements(body, 1, vec!["(x + y)"]);
            }
            _ => panic!("expr not Expression::FunctionExpression. got={}", expr),
        },
        _ => panic!("stmt not Statement::ExpressionStatement. got={}", stmt),
    }
}

#[test]
fn test_parse_function_call() {
    let input = "add(1, 2 * 3, 4 + 5);";

    let program = setup_and_validate(input, 1);

    let stmt = program.statments.first().unwrap();

    match stmt {
        Statement::ExpressionStatement(expr) => match expr {
            Expression::CallExpression {
                function,
                arguments,
            } => {
                test_identifier(function, "add");

                assert_eq!(
                    arguments.len(),
                    3,
                    "arguments does not contain 3 arguments. got={:?}",
                    arguments
                );

                test_integer_expression(&arguments[0], &1);
                test_infix_expression(&arguments[1], "2", "*", "3");
                test_infix_expression(&arguments[2], "4", "+", "5");
            }
            _ => panic!("expr not Expression::CallExpression. got={}", expr),
        },
        _ => panic!("stmt not Statement::ExpressionStatement. got={}", stmt),
    }
}
