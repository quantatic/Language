use std::error::Error;

use regex::Regex;

#[derive(Clone, Debug)]
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
    For,
    Int(u32),
    Float(f32),
    Var(String),
    Let,
    Equal,
    CompareEquals,
    CompareGreater,
    CompareGreaterEquals,
    CompareLess,
    CompareLessEquals,
    Increment,
    Decrement,
    String(String),
    Return
}

#[derive(Clone)]
pub enum TokenParseRule {
    Ignore,
    Constant(Token),
    Map(fn(&str) -> Token),
}

pub struct TokenRule {
    pattern: Regex,
    parse_rule: TokenParseRule
}

impl TokenRule {
    pub fn new(pattern: &str, parse_rule: TokenParseRule) -> Result<Self, Box<dyn Error>> {
        Ok(
            Self {
                pattern: Regex::new(pattern)?,
                parse_rule
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

        let best_match: Option<(usize, TokenParseRule)> = self.rules
            .iter()
            .filter_map(|rule| {
                rule.pattern.find_at(self.source, self.token_start)
                    .filter(|matched| {
                        matched.start() == self.token_start
                    })
                    .map(|matched| {
                        (matched.end(), rule.parse_rule.clone())
                    })
            })
            .max_by(|(x, _), (y, _)| {
                x.cmp(y)
            });

        if best_match.is_none() {
            let context_upper_index = usize::min(self.token_start+100, self.source.len());
            println!("Syntax error: {}", &self.source[self.token_start..context_upper_index]);
        }

        best_match.and_then(|(match_end, parse_rule)| {
            match parse_rule {
                TokenParseRule::Ignore => {
                    self.token_start = match_end;
                    self.next()
                },
                TokenParseRule::Constant(res) => {
                    self.token_start = match_end;
                    Some(res)
                },
                TokenParseRule::Map(map_fn) => {
                    let res = map_fn(&self.source[self.token_start..match_end]);
                    self.token_start = match_end;
                    Some(res)
                }
            }
        })
    }
}
