use std::error::Error;

use language::tokenizer::{Token, TokenRule, Tokenizer};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let rules = vec![
        TokenRule::new(r"let", Some(Token::Let)),
        TokenRule::new(r"while", Some(Token::While)),
        TokenRule::new(r"if", Some(Token::If)),
        TokenRule::new(r"else", Some(Token::Else)),
        TokenRule::new(r"for", Some(Token::For)),
        TokenRule::new(r"return", Some(Token::Return)),
        TokenRule::new(r"0|[1-9][0-9]*", Some(Token::Int)),
        TokenRule::new(r"(0|[1-9][0-9]*).[0-9]+", Some(Token::Float)),
        TokenRule::new(r"\s+", None),
        TokenRule::new(r"\+", Some(Token::Plus)),
        TokenRule::new(r"\+\+", Some(Token::Increment)),
        TokenRule::new(r"\-", Some(Token::Minus)),
        TokenRule::new(r"\-\-", Some(Token::Decrement)),
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
        TokenRule::new(r"//.*", None),
        TokenRule::new(r"/\*[\s\S]*?\*/", None),
        TokenRule::new(r#"".*""#, Some(Token::String)),
        TokenRule::new(r"[A-Za-z_]+", Some(Token::Var)),
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
//  let some_invalid = .5;         // This would be invalid if this line were uncommented
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
