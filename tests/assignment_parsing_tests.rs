/*
 * Copyright (c) 2025 Raphael Amorim
 *
 * This file is part of flash, which is licensed
 * under GNU General Public License v3.0.
 */

use flash::lexer::Lexer;
use flash::parser::{Node, Parser};

fn parse_script(script: &str) -> Node {
    let lexer = Lexer::new(script);
    let mut parser = Parser::new(lexer);
    parser.parse_script()
}

#[test]
fn trailing_equals_stays_in_command_argument() {
    let ast = parse_script("echo foo=");

    match ast {
        Node::List { statements, .. } => {
            assert_eq!(statements.len(), 1);
            match &statements[0] {
                Node::Command { name, args, .. } => {
                    assert_eq!(name, "echo");
                    assert_eq!(args, &vec!["foo=".to_string()]);
                }
                _ => panic!("expected command node"),
            }
        }
        _ => panic!("expected list node"),
    }
}

#[test]
fn inline_equals_value_stays_in_command_argument() {
    let ast = parse_script("echo foo=bar");

    match ast {
        Node::List { statements, .. } => {
            assert_eq!(statements.len(), 1);
            match &statements[0] {
                Node::Command { name, args, .. } => {
                    assert_eq!(name, "echo");
                    assert_eq!(args, &vec!["foo=bar".to_string()]);
                }
                _ => panic!("expected command node"),
            }
        }
        _ => panic!("expected list node"),
    }
}

#[test]
fn leading_assignments_are_kept_before_command() {
    let ast = parse_script("FOO=1 BAR=2 echo");

    match ast {
        Node::List {
            statements,
            operators,
        } => {
            assert_eq!(operators, vec!["".to_string(), "".to_string()]);
            assert_eq!(statements.len(), 3);

            match &statements[0] {
                Node::Assignment { name, value } => {
                    assert_eq!(name, "FOO");
                    assert_eq!(value.as_ref(), &Node::StringLiteral("1".to_string()));
                }
                _ => panic!("expected first assignment"),
            }

            match &statements[1] {
                Node::Assignment { name, value } => {
                    assert_eq!(name, "BAR");
                    assert_eq!(value.as_ref(), &Node::StringLiteral("2".to_string()));
                }
                _ => panic!("expected second assignment"),
            }

            match &statements[2] {
                Node::Command { name, args, .. } => {
                    assert_eq!(name, "echo");
                    assert!(args.is_empty());
                }
                _ => panic!("expected command node"),
            }
        }
        _ => panic!("expected list node"),
    }
}
