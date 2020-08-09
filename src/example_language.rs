#[macro_use]
use crate::tokenizer::{Token, TokenRule, TokenParseRule};

#[derive(Clone, Debug)]
pub enum ExampleLanguageToken {
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
    Int(i32),
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
    Comma,
    Return,
}

impl Token for ExampleLanguageToken {}

lazy_static! {
    pub static ref EXAMPLE_LANGUAGE_RULES: Vec<TokenRule<ExampleLanguageToken>> = vec![
        TokenRule::new(r"let", TokenParseRule::Constant(ExampleLanguageToken::Let)),
        TokenRule::new(
            r"while",
            TokenParseRule::Constant(ExampleLanguageToken::While)
        ),
        TokenRule::new(r"if", TokenParseRule::Constant(ExampleLanguageToken::If)),
        TokenRule::new(
            r"else",
            TokenParseRule::Constant(ExampleLanguageToken::Else)
        ),
        TokenRule::new(r"for", TokenParseRule::Constant(ExampleLanguageToken::For)),
        TokenRule::new(
            r"return",
            TokenParseRule::Constant(ExampleLanguageToken::Return)
        ),
        TokenRule::new(
            r"0|([1-9][0-9]*)",
            TokenParseRule::Map(|val| { ExampleLanguageToken::Int(val.parse().unwrap()) })
        ),
        TokenRule::new(
            r"(0|([1-9][0-9]*))\.[0-9]+",
            TokenParseRule::Map(|val| { ExampleLanguageToken::Float(val.parse().unwrap()) })
        ),
        TokenRule::new(r"\s+", TokenParseRule::Ignore),
        TokenRule::new(r",", TokenParseRule::Constant(ExampleLanguageToken::Comma)),
        TokenRule::new(r"\+", TokenParseRule::Constant(ExampleLanguageToken::Plus)),
        TokenRule::new(
            r"\+\+",
            TokenParseRule::Constant(ExampleLanguageToken::Increment)
        ),
        TokenRule::new(r"\-", TokenParseRule::Constant(ExampleLanguageToken::Minus)),
        TokenRule::new(
            r"\-\-",
            TokenParseRule::Constant(ExampleLanguageToken::Decrement)
        ),
        TokenRule::new(r"\*", TokenParseRule::Constant(ExampleLanguageToken::Mul)),
        TokenRule::new(r"/", TokenParseRule::Constant(ExampleLanguageToken::Div)),
        TokenRule::new(
            r"\(",
            TokenParseRule::Constant(ExampleLanguageToken::OpenParen)
        ),
        TokenRule::new(
            r"\)",
            TokenParseRule::Constant(ExampleLanguageToken::CloseParen)
        ),
        TokenRule::new(
            r"\{",
            TokenParseRule::Constant(ExampleLanguageToken::OpenBrace)
        ),
        TokenRule::new(
            r"\}",
            TokenParseRule::Constant(ExampleLanguageToken::CloseBrace)
        ),
        TokenRule::new(
            r";",
            TokenParseRule::Constant(ExampleLanguageToken::Semicolon)
        ),
        TokenRule::new(r"=", TokenParseRule::Constant(ExampleLanguageToken::Equal)),
        TokenRule::new(
            r"==",
            TokenParseRule::Constant(ExampleLanguageToken::CompareEquals)
        ),
        TokenRule::new(
            r">",
            TokenParseRule::Constant(ExampleLanguageToken::CompareGreater)
        ),
        TokenRule::new(
            r">=",
            TokenParseRule::Constant(ExampleLanguageToken::CompareGreaterEquals)
        ),
        TokenRule::new(
            r"<",
            TokenParseRule::Constant(ExampleLanguageToken::CompareLess)
        ),
        TokenRule::new(
            r"<=",
            TokenParseRule::Constant(ExampleLanguageToken::CompareLessEquals)
        ),
        TokenRule::new(r"//.*", TokenParseRule::Ignore),
        TokenRule::new(r"/\*[\s\S]*?\*/", TokenParseRule::Ignore),
        TokenRule::new(
            r#"".*""#,
            TokenParseRule::Map(|val| {
                ExampleLanguageToken::String(val[1..val.len() - 1].to_string())
            })
        ),
        TokenRule::new(
            r"[A-Za-z_]+",
            TokenParseRule::Map(|val| { ExampleLanguageToken::Var(val.to_string()) })
        ),
    ]
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .unwrap();
}
