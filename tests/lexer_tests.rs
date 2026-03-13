/*
 * Copyright (c) 2025 Raphael Amorim
 *
 * This file is part of flash, which is licensed
 * under GNU General Public License v3.0.
 */

use flash::lexer::{Lexer, TokenKind};

#[test]
fn test_lexer_basic_tokens() {
    let mut lexer = Lexer::new("echo hello");

    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hello".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_lexer_operators() {
    let mut lexer = Lexer::new("| && || ; & < > >>");

    assert_eq!(lexer.next_token().kind, TokenKind::Pipe);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::And);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Or);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Semicolon);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Background);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Less);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Great);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::DGreat);
}

#[test]
fn test_lexer_quotes() {
    let mut lexer = Lexer::new(r#""hello world" 'single quoted'"#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hello world".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::SingleQuote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("single quoted".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::SingleQuote);
}

#[test]
fn test_lexer_variable_expansion() {
    let mut lexer = Lexer::new("$HOME ${USER} $1 $@");

    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("HOME".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("USER".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("1".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("@".to_string()));
}

#[test]
fn test_lexer_command_substitution() {
    let mut lexer = Lexer::new("$(echo hello) `date`");

    assert_eq!(lexer.next_token().kind, TokenKind::CmdSubst);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hello".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Backtick);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("date".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Backtick);
}

#[test]
fn test_lexer_arithmetic_expansion() {
    let mut lexer = Lexer::new("$((1 + 2))");

    assert_eq!(lexer.next_token().kind, TokenKind::ArithSubst);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("1".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("+".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("2".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
}

#[test]
fn test_lexer_keywords() {
    let mut lexer = Lexer::new("if then elif else fi for while until do done function case esac");

    assert_eq!(lexer.next_token().kind, TokenKind::If);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Then);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Elif);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Else);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Fi);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::For);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::While);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Until);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Do);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Done);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Function);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Case);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Esac);
}

#[test]
fn test_lexer_comments() {
    let mut lexer = Lexer::new("echo hello # this is a comment\necho world");

    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hello".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Comment);
    assert_eq!(lexer.next_token().kind, TokenKind::Newline);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("world".to_string())
    );
}

#[test]
fn test_lexer_newlines() {
    let mut lexer = Lexer::new("echo\n\nworld");

    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Newline);
    assert_eq!(lexer.next_token().kind, TokenKind::Newline);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("world".to_string())
    );
}

#[test]
fn test_lexer_braces() {
    let mut lexer = Lexer::new("{ echo hello; }");

    assert_eq!(lexer.next_token().kind, TokenKind::LBrace);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hello".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Semicolon);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
}

#[test]
fn test_lexer_parentheses() {
    let mut lexer = Lexer::new("(echo hello)");

    assert_eq!(lexer.next_token().kind, TokenKind::LParen);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hello".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
}

#[test]
fn test_lexer_assignment() {
    let mut lexer = Lexer::new("VAR=value");

    assert_eq!(lexer.next_token().kind, TokenKind::Word("VAR".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Assignment);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("value".to_string())
    );
}

#[test]
fn test_lexer_extended_glob() {
    let mut lexer = Lexer::new("?(pattern) *(pattern) +(pattern) @(pattern) !(pattern)");

    assert_eq!(lexer.next_token().kind, TokenKind::ExtGlob('?'));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("pattern".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);

    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::ExtGlob('*'));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("pattern".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);

    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::ExtGlob('+'));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("pattern".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);

    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::ExtGlob('@'));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("pattern".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);

    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::ExtGlob('!'));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("pattern".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
}

#[test]
fn test_lexer_double_semicolon() {
    let mut lexer = Lexer::new("case $var in pattern) echo hello ;; esac");

    assert_eq!(lexer.next_token().kind, TokenKind::Case);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("var".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::In);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("pattern".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hello".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::DoubleSemicolon);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Esac);
}

#[test]
fn test_lexer_position_tracking() {
    let mut lexer = Lexer::new("echo\nhello");

    let token1 = lexer.next_token();
    assert_eq!(token1.position.line, 1);
    assert_eq!(token1.position.column, 1);

    let token2 = lexer.next_token(); // newline
    assert_eq!(token2.position.line, 1);
    assert_eq!(token2.position.column, 5);

    let token3 = lexer.next_token();
    assert_eq!(token3.position.line, 2);
    assert_eq!(token3.position.column, 1);
}

#[test]
fn test_lexer_whitespace_handling() {
    let mut lexer = Lexer::new("  echo   hello  ");

    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace("  ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace("   ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hello".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace("  ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_lexer_empty_input() {
    let mut lexer = Lexer::new("");
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_lexer_only_whitespace() {
    let mut lexer = Lexer::new("   \t  \n  ");
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace("   \t  ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Newline);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace("  ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_lexer_mixed_quotes() {
    let mut lexer = Lexer::new(r#"echo "hello 'world'" 'goodbye "friend"'"#);

    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hello 'world'".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::SingleQuote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("goodbye \"friend\"".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::SingleQuote);
}

#[test]
fn test_lexer_escape_sequences() {
    let mut lexer = Lexer::new(r#"echo \$HOME \n \t \\"#);

    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("\\$HOME".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("\\n".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("\\t".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("\\\\".to_string()));
}

#[test]
fn test_lexer_complex_redirection() {
    let mut lexer = Lexer::new("cmd 2>&1 3< file 4>> log");

    assert_eq!(lexer.next_token().kind, TokenKind::Word("cmd".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("2".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Great);
    assert_eq!(lexer.next_token().kind, TokenKind::Background);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("1".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("3".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Less);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("file".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("4".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::DGreat);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("log".to_string()));
}

#[test]
fn test_lexer_glob_patterns() {
    let mut lexer = Lexer::new("ls *.txt [abc] {1,2,3}");

    assert_eq!(lexer.next_token().kind, TokenKind::Word("ls".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("*.txt".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("[abc]".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("{1,2,3}".to_string())
    );
}
#[test]
fn test_arith_command_token() {
    let mut lexer = Lexer::new("((");
    let token = lexer.next_token();

    println!("Token: {token:?}");

    // Check if it's an ArithCommand token
    match token.kind {
        TokenKind::ArithCommand => {
            println!("SUCCESS: Got ArithCommand token");
            assert_eq!(token.value, "((");
        }
        _ => {
            println!("FAILURE: Expected ArithCommand, got {:?}", token.kind);
            panic!("Expected ArithCommand token");
        }
    }
}

#[test]
fn test_arith_command_full() {
    let mut lexer = Lexer::new("(( 5 == 10 ))");

    println!("All tokens:");
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        println!("{token:?}");

        if token.kind == TokenKind::EOF {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    // Check that the first token is ArithCommand
    assert_eq!(tokens[0].kind, TokenKind::ArithCommand);
    assert_eq!(tokens[0].value, "((");
}

#[test]
fn test_gte_tokens() {
    let mut lexer = Lexer::new("(( 3 >= 5 ))");

    println!("All tokens:");
    loop {
        let token = lexer.next_token();
        println!("{token:?}");

        if token.kind == TokenKind::EOF {
            break;
        }
    }
}

#[test]
fn test_nested_arithmetic_tokens() {
    let mut lexer = Lexer::new("(( $((5 + 3)) > 7 ))");

    println!("All tokens for nested arithmetic:");
    loop {
        let token = lexer.next_token();
        println!("{token:?}");

        if token.kind == TokenKind::EOF {
            break;
        }
    }
}

#[test]
fn test_deeply_nested_tokens() {
    let mut lexer = Lexer::new("(( $((1 + $((2 * 3)))) == 7 ))");

    println!("All tokens for deeply nested:");
    loop {
        let token = lexer.next_token();
        println!("{token:?}");

        if token.kind == TokenKind::EOF {
            break;
        }
    }
}

// ---------------------------------------------------------------------------
// Double-quote expression lexing tests
// ---------------------------------------------------------------------------

/// `"$FOO"` should emit Dollar + the variable name, not a single Word.
#[test]
fn test_dquote_variable_simple() {
    let mut lexer = Lexer::new(r#""$FOO""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("FOO".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

/// `echo "hello $FOO"` – literal prefix before the expansion.
#[test]
fn test_dquote_variable_with_prefix() {
    let mut lexer = Lexer::new(r#"echo "hello $FOO""#);

    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("echo".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hello ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("FOO".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

/// `"$FOO bar"` – literal suffix after the expansion.
#[test]
fn test_dquote_variable_with_suffix() {
    let mut lexer = Lexer::new(r#""$FOO bar""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("FOO".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word(" bar".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

/// Multiple variable references inside the same double-quoted string.
#[test]
fn test_dquote_multiple_variables() {
    let mut lexer = Lexer::new(r#""$FOO $BAR""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("FOO".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("BAR".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

/// Special variable `$@` inside double quotes.
#[test]
fn test_dquote_special_variable_at() {
    let mut lexer = Lexer::new(r#""$@""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("@".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

/// `${VAR}` parameter expansion inside double quotes.
#[test]
fn test_dquote_param_expansion() {
    let mut lexer = Lexer::new(r#""${VAR}""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("VAR".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

/// `$(...)` command substitution inside double quotes.
#[test]
fn test_dquote_cmd_substitution() {
    let mut lexer = Lexer::new(r#""$(echo hi)""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::CmdSubst);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("echo".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hi".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

/// Arithmetic expansion `$((…))` inside double quotes.
#[test]
fn test_dquote_arith_substitution() {
    let mut lexer = Lexer::new(r#""$((1 + 2))""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::ArithSubst);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("1".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("+".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("2".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

/// Backtick command substitution inside double quotes.
#[test]
fn test_dquote_backtick_substitution() {
    let mut lexer = Lexer::new("\"` date `\"");

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::Backtick);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("date".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Backtick);
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

/// `\$` inside double quotes should produce a literal `$` in the Word content,
/// not trigger variable expansion.
#[test]
fn test_dquote_escaped_dollar() {
    let mut lexer = Lexer::new(r#""\$FOO""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("$FOO".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

/// `$` not followed by a name/expansion character stays literal inside double quotes.
#[test]
fn test_dquote_literal_dollar_no_expansion() {
    let mut lexer = Lexer::new(r#""$ foo""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    // The $ is followed by a space, so it is treated as literal content.
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("$ foo".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

/// Literal text with no expansions still works as a single Word.
#[test]
fn test_dquote_no_expansion() {
    let mut lexer = Lexer::new(r#""hello world""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hello world".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

/// `${VAR}` followed by literal text inside double quotes.
#[test]
fn test_dquote_param_expansion_with_suffix() {
    let mut lexer = Lexer::new(r#""${VAR} suffix""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("VAR".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word(" suffix".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}
