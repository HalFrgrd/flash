use flash::lexer::{Lexer, TokenKind};

#[test]
fn test_trace_param_expansion2() {
    let input = "${USER}";
    println!("Testing: {:?}", input);
    let mut lexer = Lexer::new(input);
    // Print position info with each token
    loop {
        let pos = lexer.position;
        let tok = lexer.next_token();
        println!("  pos={}, token={:?}", pos, tok.kind);
        if tok.kind == TokenKind::EOF {
            break;
        }
    }
}
