use std::iter::Peekable;

use std::error::Error;

use std::collections::HashMap;

use regex::Regex;


#[derive(Clone, Copy, Debug)]
pub enum Token {
    Plus,
    Minus,
    Mul,
    Div,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    If,
    Else,
    While,
    Int,
    Float,
    Var,
    Let,
    Equal,
    CompareEquals,
    CompareGreater,
    CompareGreaterEquals,
    CompareLess,
    CompareLessEquals
}

#[derive(Debug)]
pub struct TokenRule {
    pattern: Regex,
    token: Option<Token>,
}

impl TokenRule {
    pub fn new(pattern: &str, token: Option<Token>) -> Result<Self, Box<dyn Error>> {
        Ok(
            Self {
                pattern: Regex::new(pattern)?,
                token: token
            }
        )
    }
}

pub struct Tokenizer<'a> {
    rules: Vec<TokenRule>,
    token_start: usize,
    source: &'a str
}

impl<'a> Tokenizer<'a> {
    pub fn new(rules: Vec<TokenRule>, source: &'a str) -> Self {
        Self {
            rules,
            token_start: 0,
            source
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.token_start >= self.source.len() {
            return None
        }

        let best_match: Option<(usize, Option<Token>)> = self.rules
            .iter()
            .filter_map(|rule| {
                rule.pattern.find_at(self.source, self.token_start)
                    .filter(|matched| {
                        matched.start() == self.token_start
                    })
                    .map(|matched| {
                        (matched.end(), rule.token)
                    })
            })
            .max_by(|(x, _), (y, _)| {
                x.cmp(y)
            });

        if let Some((match_end, maybe_token)) = best_match {
            self.token_start = match_end;
            while !self.source.is_char_boundary(self.token_start) {
                self.token_start += 1;
            }
            
            maybe_token.or_else(|| {
                self.next()
            })
        } else {
            let context_upper_index = usize::min(self.token_start+50, self.source.len());
            println!("Syntax error: {}", &self.source[self.token_start..context_upper_index]);
            None
        }
    }
}
