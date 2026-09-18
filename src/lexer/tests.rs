use super::*;

fn lex(source: &str) -> Vec<Spanned> {
    Lexer::new(source).tokenize()
}

fn assert_tokens(source: &str, expected: &[(TokenType, usize)]) {
    let expected: Vec<Spanned> = expected
        .iter()
        .map(|(token, line)| Spanned {
            token: *token,
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
