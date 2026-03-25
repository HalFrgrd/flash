use flash::lexer::{Lexer, TokenKind};

#[test]
fn test_trace_detail() {
    let input = "${USER}";
    let mut lexer = Lexer::new(input);
    // Check state after constructor
    println!("Initial: pos={}", lexer.position);
    
    // Token 1: ParamExpansion
    let tok = lexer.next_token();
    println!("After ParamExpansion: pos={}, ch_at_pos={:?}", lexer.position, input.chars().nth(lexer.position));
    assert_eq!(tok.kind, TokenKind::ParamExpansion);
    
    // Token 2: Word("USER")
    let tok = lexer.next_token();
    println!("After Word(USER): pos={}, ch_at_pos={:?}", lexer.position, input.chars().nth(lexer.position));
    assert_eq!(tok.kind, TokenKind::Word("USER".to_string()));
    
    // Token 3: RBrace
    let tok = lexer.next_token();
    println!("After RBrace: pos={}", lexer.position);
    assert_eq!(tok.kind, TokenKind::RBrace);
}
