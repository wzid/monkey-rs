use super::*;

#[test]
fn test_next_token_basic() {
    let input = "=+(){},;";

    let tests = vec![
        token![=],
        token![+],
        token!['('],
        token![')'],
        token!['{'],
        token!['}'],
        token![,],
        token![;],
    ];

    let mut lexer = Lexer::new(input);

    for expect in tests {
        let tok = lexer.next_token();

        assert_eq!(expect, tok);
    }
}

#[test]
fn test_next_token_program() {
    let input = r#"
        let five = 5;
        
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        "foobar";
        "foo bar";
        "#;

    let tests = vec![
        token![LET],
        token![IDENT("five".to_string())],
        token![=],
        token![INT(5)],
        token![;],
        token![LET],
        token![IDENT("ten".to_string())],
        token![=],
        token![INT(10)],
        token![;],
        token![LET],
        token![IDENT("add".to_string())],
        token![=],
        token![FN],
        token!['('],
        token![IDENT("x".to_string())],
        token![,],
        token![IDENT("y".to_string())],
        token![')'],
        token!['{'],
        token![IDENT("x".to_string())],
        token![+],
        token![IDENT("y".to_string())],
        token![;],
        token!['}'],
        token![;],
        token![LET],
        token![IDENT("result".to_string())],
        token![=],
        token![IDENT("add".to_string())],
        token!['('],
        token![IDENT("five".to_string())],
        token![,],
        token![IDENT("ten".to_string())],
        token![')'],
        token![;],
        token![STR("foobar".to_string())],
        token![;],
        token![STR("foo bar".to_string())],
    ];

    let mut lexer = Lexer::new(input);

    for expect in tests {
        let tok = lexer.next_token();

        assert_eq!(expect, tok);
    }
}

#[test]
fn test_next_token_reserved() {
    let input = r#"
        if (5 < 10) {
            return true;
        } else {
            return false;
        }"#;

    let tests = vec![
        token![IF],
        token!['('],
        token![INT(5)],
        token![<],
        token![INT(10)],
        token![')'],
        token!['{'],
        token![RETURN],
        token![TRUE],
        token![;],
        token!['}'],
        token![ELSE],
        token!['{'],
        token![RETURN],
        token![FALSE],
        token![;],
        token!['}'],
    ];

    let mut lexer = Lexer::new(input);

    for expect in tests {
        let tok = lexer.next_token();

        assert_eq!(expect, tok);
    }
}

#[test]
fn test_next_token_operators() {
    let input = r#"
        !-/*5
        5 < 10 > 5"#;

    let tests = vec![
        token![!],
        token![-],
        token![/],
        token![*],
        token![INT(5)],
        token![INT(5)],
        token![<],
        token![INT(10)],
        token![>],
        token![INT(5)],
    ];

    let mut lexer = Lexer::new(input);

    for expect in tests {
        let tok = lexer.next_token();

        assert_eq!(expect, tok);
    }
}

#[test]
fn test_next_token_double_char() {
    let input = r#"
        10 == 10;
        10 != 9;"#;

    let tests = vec![
        token![INT(10)],
        token![==],
        token![INT(10)],
        token![;],
        token![INT(10)],
        token![!=],
        token![INT(9)],
        token![;],
    ];

    let mut lexer = Lexer::new(input);

    for expect in tests {
        let tok = lexer.next_token();

        assert_eq!(expect, tok);
    }
}
