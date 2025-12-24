use std::{iter::Peekable, slice::Iter};

use crate::{
    chord::note::{Modifier, Note, NoteLiteral},
    parsing::{
        ast::Ast,
        lexer::Lexer,
        parser_error::ParserError,
        token::{Token, TokenType},
    },
};

pub enum ParserContext {
    None,
    Sus,
    /// Active when an Omit token is found
    OmitStart,
    /// Active when context == OmitStart AND a comma is found
    OmitActive,
}

pub struct NewParser {
    lexer: Lexer,
    errors: Vec<ParserError>,
    ast: Ast,
    op_count: i16,
}

impl NewParser {
    pub fn new() -> NewParser {
        NewParser {
            lexer: Lexer::new(),
            errors: Vec::new(),
            ast: Ast::default(),
            op_count: 0,
        }
    }

    pub fn parse(&mut self, input: &str) {
        let binding = self.lexer.scan_tokens(input);
        let mut tokens = binding.iter().peekable();
    }

    fn read_root(&mut self, tokens: &mut Peekable<Iter<Token>>) {
        match self.expect_note(tokens) {
            Some(note) => self.ast.root = note,
            None => self.errors.push(ParserError::MissingRootNote),
        }
    }

    fn expect_note(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Option<Note> {
        match &tokens.next()?.token_type {
            TokenType::Note(n) => {
                let modifier = self.match_modifier(tokens);
                Some(Note::new(NoteLiteral::from_string(n), modifier))
            }
            _ => None,
        }
    }

    fn match_modifier(&self, tokens: &mut Peekable<Iter<Token>>) -> Option<Modifier> {
        match tokens.peek().map(|t| &t.token_type) {
            Some(TokenType::Flat) => {
                tokens.next();
                Some(Modifier::Flat)
            }
            Some(TokenType::Sharp) => {
                tokens.next();
                Some(Modifier::Sharp)
            }
            _ => None,
        }
    }

    fn expect_peek(&self, expected: TokenType, tokens: &mut Peekable<Iter<Token>>) -> bool {
        matches!(tokens.peek(), Some(token) if token.token_type == expected)
    }
}
