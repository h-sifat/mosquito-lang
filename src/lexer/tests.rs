use super::*;

fn lex(source: &str) -> Vec<Spanned> {
    Lexer::new(source).tokenize()
}

fn assert_tokens(source: &str, expected: &[(TokenType, usize)]) {
    let expected: Vec<Spanned> = expected
        .iter()
        .map(|(token, line)| Spanned {
            token: token.clone(),
            line: *line,
        })
        .collect();

    let result = lex(source);
    println!("result: {:?}", result);

    assert_eq!(result, expected, "\nsource: {source:?}");
}

fn run_cases(cases: &[(&str, &[(TokenType, usize)])]) {
    println!("run cases!");

    for (source, expected) in cases {
        println!("\n\n=====================\n\n");
        println!("source: '{}'", source);
        println!("expected: '{:?}'", expected);
        assert_tokens(source, expected);

        println!("PASSED");
        println!("\n\n=====================\n\n");
    }
}

#[test]
fn punctuation_and_comments() {
    use TokenType::*;

    let cases: &[(&str, &[(TokenType, usize)])] = &[
        ("", &[(Eof, 1)]),
        ("();", &[(LParen, 1), (RParen, 1), (Semicolon, 1), (Eof, 1)]),
        (
            "( , ) [ ] { } . ;",
            &[
                (LParen, 1),
                (Comma, 1),
                (RParen, 1),
                (LBracket, 1),
                (RBracket, 1),
                (LBrace, 1),
                (RBrace, 1),
                (Dot, 1),
                (Semicolon, 1),
                (Eof, 1),
            ],
        ),
        (
            "// header\n(,);",
            &[
                (LParen, 2),
                (Comma, 2),
                (RParen, 2),
                (Semicolon, 2),
                (Eof, 2),
            ],
        ),
        (
            "(\n  ,\n)",
            &[(LParen, 1), (Comma, 2), (RParen, 3), (Eof, 3)],
        ),
        (
            "();  // trailing comment, no newline at all after it",
            &[(LParen, 1), (RParen, 1), (Semicolon, 1), (Eof, 1)],
        ),
        ("()", &[(LParen, 1), (RParen, 1), (Eof, 1)]),
    ];

    run_cases(cases);
}

#[test]
fn numbers() {
    use TokenType::*;

    let cases: &[(&str, &[(TokenType, usize)])] = &[
        ("0", &[(Number(0.0), 1), (Eof, 1)]),
        ("42", &[(Number(42.0), 1), (Eof, 1)]),
        ("3.14", &[(Number(3.14), 1), (Eof, 1)]),
        ("10.5", &[(Number(10.5), 1), (Eof, 1)]),
        // a trailing dot with no digit after it is NOT part of the number
        ("2.", &[(Number(2.0), 1), (Dot, 1), (Eof, 1)]),
        (
            "1;2",
            &[(Number(1.0), 1), (Semicolon, 1), (Number(2.0), 1), (Eof, 1)],
        ),
        (
            "(1, 2.5)",
            &[
                (LParen, 1),
                (Number(1.0), 1),
                (Comma, 1),
                (Number(2.5), 1),
                (RParen, 1),
                (Eof, 1),
            ],
        ),
        // line tracking still has to work when a number follows a comment
        ("// x\n7", &[(Number(7.0), 2), (Eof, 2)]),
    ];

    run_cases(cases);
}

#[test]
fn identifiers_and_keywords() {
    use TokenType::*;

    let cases: &[(&str, &[(TokenType, usize)])] = &[
        ("let", &[(Let, 1), (Eof, 1)]),
        ("const", &[(Const, 1), (Eof, 1)]),
        ("mut", &[(Mut, 1), (Eof, 1)]),
        ("function", &[(Function, 1), (Eof, 1)]),
        ("return", &[(Return, 1), (Eof, 1)]),
        ("class", &[(Class, 1), (Eof, 1)]),
        ("pub", &[(Pub, 1), (Eof, 1)]),
        ("init", &[(Init, 1), (Eof, 1)]),
        ("self", &[(SelfKw, 1), (Eof, 1)]),
        ("static", &[(Static, 1), (Eof, 1)]),
        ("get", &[(Get, 1), (Eof, 1)]),
        ("new", &[(New, 1), (Eof, 1)]),
        ("enum", &[(Enum, 1), (Eof, 1)]),
        ("match", &[(Match, 1), (Eof, 1)]),
        ("if", &[(If, 1), (Eof, 1)]),
        ("else", &[(Else, 1), (Eof, 1)]),
        ("while", &[(While, 1), (Eof, 1)]),
        ("for", &[(For, 1), (Eof, 1)]),
        ("typeof", &[(Typeof, 1), (Eof, 1)]),
        ("void", &[(Void, 1), (Eof, 1)]),
        // true/false are literal *values*, not bare keywords - same shape as Number(f64)
        ("true", &[(Bool(true), 1), (Eof, 1)]),
        ("false", &[(Bool(false), 1), (Eof, 1)]),
        ("x", &[(Ident("x".to_string()), 1), (Eof, 1)]),
        // maximal munch: "letter" is one identifier, NOT `Let` + `Ident("ter")`
        ("letter", &[(Ident("letter".to_string()), 1), (Eof, 1)]),
        ("x1", &[(Ident("x1".to_string()), 1), (Eof, 1)]),
        ("_foo", &[(Ident("_foo".to_string()), 1), (Eof, 1)]),
        ("if x", &[(If, 1), (Ident("x".to_string()), 1), (Eof, 1)]),
        // identifiers must stop cleanly at punctuation even with no space
        (
            "if(x)",
            &[
                (If, 1),
                (LParen, 1),
                (Ident("x".to_string()), 1),
                (RParen, 1),
                (Eof, 1),
            ],
        ),
    ];

    run_cases(cases);
}
