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

    for (source, expected) in cases {
        println!("\n\n=====================\n\n");
        println!("source: '{}'", source);
        println!("expected: '{:?}'", expected);
        assert_tokens(source, expected);

        println!("PASSED");
        println!("\n\n=====================\n\n");
    }
}
