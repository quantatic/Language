use std::error::Error;

use language::tokenizer::{Token, TokenRule, TokenParseRule, Tokenizer};

use language::example_language::EXAMPLE_LANGUAGE_RULES;

use language::instructions_language::{INSTRUCTION_LANGUAGE_RULES, StackLanguage};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let mut to_tokenize = String::from(
r#"
/**
    This is an example "for" loop, which is properly documented.
*/
for(let i = 0; i < 50; i++) {
    let my_string = "Some string"; // Here was declare some string
    my_string = "some string data"; // Here we update the string
    let some_float = 5 - (0.1 * 0.5); // assign some float
//    let some_invalid = .5;         // This would be invalid if this line were uncommented
    return 5;                      // Here we return some value
}

int main(int c, double d) {
    printf("%c, %d", c, d);
}
"#   );

    let mut assembly = String::from(
r#"
PUSH 0
PEEK
PUSH 1
ADD
STORE 0
LOAD 0
PUSH 99999
EQ
BR 11
LOAD 0
JMP 1
PUSH 69420
PEEK
HLT
"#);

    println!("{}", to_tokenize.len());
    
    let tokenizer = Tokenizer::new(EXAMPLE_LANGUAGE_RULES.clone(), &to_tokenize);
    for tok in tokenizer {
        println!("{:?}", tok);
    }

    println!("{}", assembly.len());
    
    let tokenizer = Tokenizer::new(INSTRUCTION_LANGUAGE_RULES.clone(), &assembly);
    let instructions = tokenizer.collect::<Vec<_>>();
    let mut stack_parser = StackLanguage::new(instructions);
    while let Some(_) = stack_parser.step() { };
    
    Ok(())
}
