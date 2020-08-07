use std::error::Error;

use language::tokenizer::{Token, TokenRule, TokenParseRule, Tokenizer};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let rules = vec![
        TokenRule::new(r"let", TokenParseRule::Constant(Token::Let)),
        TokenRule::new(r"while", TokenParseRule::Constant(Token::While)),
        TokenRule::new(r"if", TokenParseRule::Constant(Token::If)),
        TokenRule::new(r"else", TokenParseRule::Constant(Token::Else)),
        TokenRule::new(r"for", TokenParseRule::Constant(Token::For)),
        TokenRule::new(r"return", TokenParseRule::Constant(Token::Return)),
        TokenRule::new(r"0|[1-9][0-9]*", TokenParseRule::Map(|val| {
            Token::Int(val.parse().unwrap())
        })),
        TokenRule::new(r"(0|[1-9][0-9]*).[0-9]+", TokenParseRule::Map(|val| {
            Token::Float(val.parse().unwrap())
        })),
        TokenRule::new(r"\s+", TokenParseRule::Ignore),
        TokenRule::new(r"\+", TokenParseRule::Constant(Token::Plus)),
        TokenRule::new(r"\+\+", TokenParseRule::Constant(Token::Increment)),
        TokenRule::new(r"\-", TokenParseRule::Constant(Token::Minus)),
        TokenRule::new(r"\-\-", TokenParseRule::Constant(Token::Decrement)),
        TokenRule::new(r"\*", TokenParseRule::Constant(Token::Mul)),
        TokenRule::new(r"/", TokenParseRule::Constant(Token::Div)),
        TokenRule::new(r"\(", TokenParseRule::Constant(Token::OpenParen)),
        TokenRule::new(r"\)", TokenParseRule::Constant(Token::CloseParen)),
        TokenRule::new(r"\{", TokenParseRule::Constant(Token::OpenBrace)),
        TokenRule::new(r"\}", TokenParseRule::Constant(Token::CloseBrace)),
        TokenRule::new(r";", TokenParseRule::Constant(Token::Semicolon)),
        TokenRule::new(r"=", TokenParseRule::Constant(Token::Equal)),
        TokenRule::new(r"==", TokenParseRule::Constant(Token::CompareEquals)),
        TokenRule::new(r">", TokenParseRule::Constant(Token::CompareGreater)),
        TokenRule::new(r">=", TokenParseRule::Constant(Token::CompareGreaterEquals)),
        TokenRule::new(r"<", TokenParseRule::Constant(Token::CompareLess)),
        TokenRule::new(r"<=", TokenParseRule::Constant(Token::CompareLessEquals)),
        TokenRule::new(r"//.*", TokenParseRule::Ignore),
        TokenRule::new(r"/\*[\s\S]*?\*/", TokenParseRule::Ignore),
        TokenRule::new(r#"".*""#, TokenParseRule::Map(|val| {
            Token::String(val[1..val.len() - 1].to_string())
        })),
        TokenRule::new(r"[A-Za-z_]+", TokenParseRule::Map(|val| {
            Token::Var(val.to_string())
        })),
    ]
        .into_iter()
        .rev()
        .collect::<Result<Vec<_>, _>>()?;
    let mut to_tokenize = String::from(
r#"
/**
    This is an example "for" loop, which is properly documented.
*/
for(let i = 0; i < 50; i++) {
    let my_string = "Some string"; // Here was declare some string
    my_string = "";                // Here we update the string
    let some_float = 1.234;        // assign some float
//    let some_invalid = .5;         // This would be invalid if this line were uncommented
    return 5;                      // Here we return some value
}
"#   );

    println!("{}", to_tokenize.len());
    
    let tokenizer = Tokenizer::new(rules, &to_tokenize);
    for tok in tokenizer {
        println!("{:?}", tok);
    }

    Ok(())
}
