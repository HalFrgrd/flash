use flash::lexer::{Lexer, TokenKind};

#[test]
fn test_trace_param_expansion() {
    let inputs = vec![
        "${USER}",
        "${parameter#word}",
        "${parameter/#pattern/string}",
        r#""${parameter/#pattern/string}""#,
    ];
    for input in inputs {
        println!("Input: {:?}", input);
        let mut lexer = Lexer::new(input);
        loop {
            let tok = lexer.next_token();
            println!("  {:?}", tok.kind);
            if tok.kind == TokenKind::EOF {
                break;
            }
        }
    }
}
