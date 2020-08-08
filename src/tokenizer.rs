use std::error::Error;

use std::fmt::Debug;

use regex::Regex;

pub trait Token: Clone + Debug { }

#[derive(Clone)]
pub enum TokenParseRule<T: Token> {
    Ignore,
    Constant(T),
    Map(fn(&str) -> T),
}

#[derive(Clone)]
pub struct TokenRule<T: Token> {
    pattern: Regex,
    parse_rule: TokenParseRule<T>
}

impl<T: Token> TokenRule<T> {
    pub fn new(pattern: &str, parse_rule: TokenParseRule<T>) -> Result<Self, Box<dyn Error>> {
        Ok(
            Self {
                pattern: Regex::new(pattern)?,
                parse_rule
            }
        )
    }
}

pub struct Tokenizer<'a, T: Token> {
    rules: Vec<TokenRule<T>>,
    token_start: usize,
    source: &'a str
}

impl<'a, T: Token> Tokenizer<'a, T> {
    pub fn new(mut rules: Vec<TokenRule<T>>, source: &'a str) -> Self {
        // Rules earlier in the list are given higher priority. Max prioritizes elements
        // later in the list, so we must reverse the rules given to us.
        rules.reverse();
        Self {
            rules,
            token_start: 0,
            source
        }
    }
}

impl<'a, T: Token> Iterator for Tokenizer<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.token_start >= self.source.len() {
            return None
        }

        let best_match: Option<(usize, TokenParseRule<T>)> = self.rules
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
            println!("Syntax error here: ->{}", &self.source[self.token_start..context_upper_index]);
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
