use std::error::Error;

use language::tokenizer::{Token, TokenRule, Tokenizer};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let rules = vec![
        TokenRule::new(r"let", Some(Token::Let)),
        TokenRule::new(r"while", Some(Token::While)),
        TokenRule::new(r"if", Some(Token::If)),
        TokenRule::new(r"else", Some(Token::Else)),
        TokenRule::new(r"[1-9][0-9]*", Some(Token::Int)),
        TokenRule::new(r"(0|[1-9][0-9]*).[0-9]+", Some(Token::Float)),
        TokenRule::new(r"\s+", None),
        TokenRule::new(r"\+", Some(Token::Plus)),
        TokenRule::new(r"\-", Some(Token::Minus)),
        TokenRule::new(r"\*", Some(Token::Mul)),
        TokenRule::new(r"/", Some(Token::Div)),
        TokenRule::new(r"\(", Some(Token::OpenParen)),
        TokenRule::new(r"\)", Some(Token::CloseParen)),
        TokenRule::new(r"\{", Some(Token::OpenBrace)),
        TokenRule::new(r"\}", Some(Token::CloseBrace)),
        TokenRule::new(r";", Some(Token::Semicolon)),
        TokenRule::new(r"=", Some(Token::Equal)),
        TokenRule::new(r"==", Some(Token::CompareEquals)),
        TokenRule::new(r">", Some(Token::CompareGreater)),
        TokenRule::new(r">=", Some(Token::CompareGreaterEquals)),
        TokenRule::new(r"<", Some(Token::CompareLess)),
        TokenRule::new(r"<=", Some(Token::CompareLessEquals)),
        TokenRule::new(r"[A-Za-z_]+", Some(Token::Var)),
    ]
        .into_iter()
        .rev()
        .collect::<Result<Vec<_>, _>>()?;

    let mut to_tokenize = String::from(
r"
while(a > 5) {
    let i = 1.069;
    if(i == 1.05) {
        let new_var = 45444;
    } else {
        let new_var = 12349867;
    }
}
"   );

    println!("{}", to_tokenize.len());
    
    let tokenizer = Tokenizer::new(rules, &to_tokenize);
    for tok in tokenizer {
        println!("{:?}", tok);
    }

    Ok(())
}
