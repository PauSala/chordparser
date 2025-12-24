use std::{iter::Peekable, slice::Iter};

use crate::{
    chord::{
        intervals::Interval,
        note::{Modifier, Note, NoteLiteral},
    },
    parsing::{
        ast::Ast,
        expression::Exp,
        expressions::{AddExp, ExtensionExp, OmitExp, SusExp},
        lexer::Lexer,
        parser_error::ParserError,
        token::{Token, TokenType},
    },
};

/// This is used to handle X(omit/add a,b) cases.
/// An omit/add modifier inside a parenthesis changes context to Omit(false)/Add(false).  
/// When a comma is encountered, if a context exits it is changed to true.    
/// This allows for handling subsequent tokens assuming this context.  
/// So in C7(omit3,5), the 5 is assumed as an omit, but in C7(omit3 5) it is not.
/// When parents are closed the context is reset to None.  
/// Commas with no context are ignored.  
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Context {
    None,
    Sus,
    Group(GroupContext),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GroupContext {
    kind: GroupKind,
    active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GroupKind {
    Omit,
    Add,
}

impl Context {
    fn start_group(kind: GroupKind) -> Self {
        Context::Group(GroupContext {
            kind,
            active: false,
        })
    }

    fn on_comma(self) -> Self {
        match self {
            Context::Group(mut group) => {
                group.active = true;
                Context::Group(group)
            }
            Context::Sus => Context::None,
            Context::None => Context::None,
        }
    }
}

pub struct NewParser {
    lexer: Lexer,
    errors: Vec<ParserError>,
    ast: Ast,
    open_parent_count: i16,
    context: Context,
}

impl NewParser {
    pub fn new() -> NewParser {
        NewParser {
            lexer: Lexer::new(),
            errors: Vec::new(),
            ast: Ast::default(),
            open_parent_count: 0,
            context: Context::None,
        }
    }

    pub fn parse(&mut self, input: &str) {
        let binding = self.lexer.scan_tokens(input);
        let mut tokens = binding.iter().peekable();
        self.read_root(&mut tokens);
        self.read_tokens(&mut tokens);
    }

    fn read_root(&mut self, tokens: &mut Peekable<Iter<Token>>) {
        match self.expect_note(tokens) {
            Some(note) => self.ast.root = note,
            None => self.errors.push(ParserError::MissingRootNote),
        }
    }

    fn read_tokens(&mut self, tokens: &mut Peekable<Iter<Token>>) {
        let mut next = tokens.next();
        while next.is_some() {
            self.process_token(next.unwrap(), tokens);
            next = tokens.next();
        }
    }

    fn process_token(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) {
        match &token.token_type {
            TokenType::Note(_) => self.note(token),
            TokenType::Sharp => self.modifier(tokens, Modifier::Sharp, token),
            TokenType::Flat => self.modifier(tokens, Modifier::Flat, token),
            TokenType::Aug => (),
            TokenType::Dim => (),
            TokenType::HalfDim => (),
            TokenType::Extension(ext) => (),
            TokenType::Add => (),
            TokenType::Omit => (),
            TokenType::Alt => (),
            TokenType::Sus => (),
            TokenType::Minor => (),
            TokenType::Hyphen => (),
            TokenType::Maj => (),
            TokenType::Maj7 => (),
            TokenType::Slash => (),
            TokenType::LParent => (),
            TokenType::RParent => (),
            TokenType::Comma => (),
            TokenType::Bass => (),
            TokenType::Illegal => self.errors.push(ParserError::IllegalToken(token.pos)),
            TokenType::Eof => (),
        }
    }

    /// Since the root is already read, another note token must error
    fn note(&mut self, token: &Token) {
        self.errors.push(ParserError::UnexpectedNote(token.pos));
    }

    fn modifier(&mut self, tokens: &mut Peekable<Iter<Token>>, modifier: Modifier, token: &Token) {
        if self.expect_extension(tokens) {
            let alt = tokens
                .next()
                .expect("expect_extension guarantees that a next token exist");
            if let TokenType::Extension(a) = &alt.token_type {
                let mut id = modifier.to_string();
                id.push_str(a);
                let interval = Interval::from_chord_notation(&id);
                if let Some(int) = interval {
                    self.add_interval(int, token.pos);
                } else {
                    self.errors
                        .push(ParserError::InvalidExtension(token.pos + 1));
                }
            }
        } else {
            self.errors.push(ParserError::UnexpectedModifier(token.pos));
        }
    }

    fn add_interval(&mut self, int: Interval, pos: usize) {
        match self.context {
            Context::Sus => match int {
                Interval::MinorSecond
                | Interval::MajorSecond
                | Interval::PerfectFourth
                | Interval::AugmentedFourth => {
                    self.ast.expressions.push(Exp::Sus(SusExp::new(int)));
                    self.context = Context::None;
                }
                _ => {
                    self.ast
                        .expressions
                        .push(Exp::Sus(SusExp::new(Interval::PerfectFourth)));
                    self.context = Context::None;
                    self.ast
                        .expressions
                        .push(Exp::Extension(ExtensionExp::new(int, pos)));
                }
            },
            Context::Group(g) if g.active => match g.kind {
                GroupKind::Omit => {
                    self.ast.expressions.push(Exp::Omit(OmitExp::new(int, pos)));
                }
                GroupKind::Add => {
                    self.ast.expressions.push(Exp::Add(AddExp::new(int, pos)));
                }
            },
            _ => {
                // This is for the C4 as Csus case
                if int == Interval::PerfectFourth {
                    self.ast.expressions.push(Exp::Sus(SusExp::new(int)));
                }
                // But #4 is not allowed
                if int == Interval::AugmentedFourth {
                    self.errors.push(ParserError::InvalidExtension(pos));
                } else {
                    self.ast
                        .expressions
                        .push(Exp::Extension(ExtensionExp::new(int, pos)));
                }
            }
        }
    }

    fn expect_extension(&self, tokens: &mut Peekable<Iter<Token>>) -> bool {
        matches!(tokens.peek(), Some(t) if matches!(t.token_type, TokenType::Extension(_)))
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
