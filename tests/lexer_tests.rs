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
    assert_eq!(lexer.next_token().kind, TokenKind::Word("do".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("done".to_string()));
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
fn test_do_is_word_outside_loop_header() {
    let mut lexer = Lexer::new("cd do");

    assert_eq!(lexer.next_token().kind, TokenKind::Word("cd".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("do".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_do_is_keyword_in_loop_header() {
    let mut lexer = Lexer::new("for i in 1; do echo $i; done");

    assert_eq!(lexer.next_token().kind, TokenKind::For);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("i".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::In);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("1".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Semicolon);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Do);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("i".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Semicolon);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Done);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
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
fn test_lexer_comma_brace_expansion_tokens() {
    let mut lexer = Lexer::new("echo {1,2}");

    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::LBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("1,2".to_string()));
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
fn test_assignment_after_command_is_plain_word() {
    let mut lexer = Lexer::new("echo foo=");

    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("foo=".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_multiple_leading_assignments_before_command() {
    let mut lexer = Lexer::new("FOO=1 BAR=2 echo");

    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Assignment);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("1".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("BAR".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Assignment);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("2".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
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
    assert_eq!(lexer.next_token().kind, TokenKind::LBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("1,2,3".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
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

#[test]
fn test_double_quote_dollar_expansion() {
    // echo "hello $FOO" should lex out the dollar sign and FOO
    let mut lexer = Lexer::new(r#"echo "hello $FOO""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
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
    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_double_quote_dollar_at_start() {
    // "$FOO" - dollar at the start of a double-quoted string
    let mut lexer = Lexer::new(r#""$FOO""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_double_quote_param_expansion() {
    // "${FOO}" inside double quotes
    let mut lexer = Lexer::new(r#""${FOO}""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_double_quote_param_expansion_with_text() {
    // "hello ${FOO} world" - param expansion with surrounding text
    let mut lexer = Lexer::new(r#""hello ${FOO} world""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("hello ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word(" world".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_double_quote_cmd_substitution_with_text() {
    // "result: $(echo hello)" - command substitution with leading text
    let mut lexer = Lexer::new(r#""result: $(echo hello)""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("result: ".to_string())
    );
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
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_double_quote_nested_cmd_substitution() {
    let mut lexer = Lexer::new(r#"echo "$( foo1 $(bar) )" && baz"#);

    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::CmdSubst);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("foo1".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::CmdSubst);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("bar".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::And);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("baz".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_double_quote_nested_cmd_substitution_with_surrounding_text() {
    let mut lexer = Lexer::new(r#"echo "prefix $( foo1 $(bar) ) suffix""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("prefix ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::CmdSubst);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("foo1".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::CmdSubst);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("bar".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word(" suffix".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_double_quote_nested_arith_substitution_in_cmd_substitution() {
    let mut lexer = Lexer::new(r#"echo "$( foo1 $((1+2)) )" && baz"#);

    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::CmdSubst);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("foo1".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::ArithSubst);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("1+2".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::And);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("baz".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_double_quote_nested_arith_substitution_with_surrounding_text() {
    let mut lexer = Lexer::new(r#"echo "prefix $( foo1 $((1+2)) ) suffix""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Word("echo".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("prefix ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::CmdSubst);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("foo1".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::ArithSubst);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("1+2".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RParen);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word(" suffix".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_double_quote_arith_substitution() {
    // "$((1 + 2))" - arithmetic substitution inside double quotes
    let mut lexer = Lexer::new(r#""$((1 + 2))""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
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
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_double_quote_backtick_substitution() {
    // "`date`" inside double quotes - backtick command substitution
    let mut lexer = Lexer::new(r#""`date`""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::Backtick);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("date".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Backtick);
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_double_quote_escaped_dollar() {
    // "\$FOO" - escaped dollar should be literal
    let mut lexer = Lexer::new(r#""\$FOO""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    // \$ is an escape sequence - backslash and $ are literal
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("\\$FOO".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_single_quote_no_expansion() {
    // Single quotes should NOT expand $, backticks, etc.
    let mut lexer = Lexer::new(r#"'$FOO'"#);

    assert_eq!(lexer.next_token().kind, TokenKind::SingleQuote);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("$FOO".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::SingleQuote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_double_rparen_token_kind() {
    // DoubleRParen is defined for downstream dependents and is not emitted by the lexer.
    // Verify the variant exists and its derived traits work correctly.
    let token = TokenKind::DoubleRParen;
    assert_eq!(token, TokenKind::DoubleRParen);
    assert_ne!(token, TokenKind::RParen);
    let cloned = token.clone();
    assert_eq!(cloned, TokenKind::DoubleRParen);
    assert!(format!("{:?}", token).contains("DoubleRParen"));
}

#[test]
fn test_bracket_followed_by_single_quote() {
    // "a ['" should parse as: word, whitespace, word([), singlequote
    let mut lexer = Lexer::new("a ['");

    assert_eq!(lexer.next_token().kind, TokenKind::Word("a".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("[".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::SingleQuote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_bracket_followed_by_double_quote() {
    // "a [\"" should parse as: word, whitespace, word([), quote
    let mut lexer = Lexer::new("a [\"");

    assert_eq!(lexer.next_token().kind, TokenKind::Word("a".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Whitespace(" ".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("[".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_param_expansion_length() {
    // ${#parameter} - length of parameter
    let mut lexer = Lexer::new("${#FOO}");

    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("#".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_param_expansion_prefix_removal() {
    // ${parameter#word} - remove smallest prefix matching word
    let mut lexer = Lexer::new("${FOO#prefix}");

    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Word("#".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("prefix".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_param_expansion_prefix_removal_longest() {
    // ${parameter##word} - remove largest prefix matching word
    let mut lexer = Lexer::new("${FOO##prefix}");

    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Word("##".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("prefix".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_param_expansion_substitution() {
    // ${parameter/pattern/string} - replace first occurrence of pattern with string
    let mut lexer = Lexer::new("${FOO/pattern/string}");

    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Word("/".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("pattern".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("/".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("string".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_param_expansion_global_substitution() {
    // ${parameter//pattern/string} - replace all occurrences of pattern with string
    let mut lexer = Lexer::new("${FOO//pattern/string}");

    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Word("//".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("pattern".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("/".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("string".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_param_expansion_anchored_prefix_substitution() {
    // ${parameter/#pattern/string} - replace pattern anchored at start
    let mut lexer = Lexer::new("${FOO/#pattern/string}");

    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Word("/".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Word("#".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("pattern".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("/".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("string".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_param_expansion_anchored_suffix_substitution() {
    // ${parameter/%pattern/string} - replace pattern anchored at end
    let mut lexer = Lexer::new("${FOO/%pattern/string}");

    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Word("/".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Word("%".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("pattern".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("/".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("string".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_param_expansion_anchored_prefix_substitution_double_quoted() {
    // "${parameter/#pattern/string}" - same but double-quoted
    let mut lexer = Lexer::new(r#""${FOO/#pattern/string}""#);

    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::ParamExpansion);
    assert_eq!(lexer.next_token().kind, TokenKind::Word("FOO".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Word("/".to_string()));
    assert_eq!(lexer.next_token().kind, TokenKind::Word("#".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("pattern".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::Word("/".to_string()));
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("string".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::RBrace);
    assert_eq!(lexer.next_token().kind, TokenKind::Quote);
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_dollar_env_var_path() {
    // $HOME/foo should lex as three tokens: $, HOME, /foo
    let mut lexer = Lexer::new("$HOME/foo");

    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("HOME".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("/foo".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}

#[test]
fn test_dollar_env_var_dot() {
    // $HOME.FOO should lex as three tokens: $, HOME, .FOO
    let mut lexer = Lexer::new("$HOME.FOO");

    assert_eq!(lexer.next_token().kind, TokenKind::Dollar);
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word("HOME".to_string())
    );
    assert_eq!(
        lexer.next_token().kind,
        TokenKind::Word(".FOO".to_string())
    );
    assert_eq!(lexer.next_token().kind, TokenKind::EOF);
}
