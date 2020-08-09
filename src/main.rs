use std::error::Error;

use language::tokenizer::Tokenizer;

use language::example_language::EXAMPLE_LANGUAGE_RULES;

use language::instructions_language::{StackLanguage, INSTRUCTION_LANGUAGE_RULES};

fn main() -> Result<(), Box<dyn Error>> {
    let to_tokenize = String::from(
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
"#,
    );

    let assembly = String::from(
        r#"
PUSH 0
LOOP:
PEEK
PUSH 1
ADD
STORE 0
LOAD 0
PUSH 100000
EQ
BR DONE
LOAD 0
JMP LOOP
DONE:
PUSH 69420
PEEK
HLT
"#,
    );

    println!("{}", to_tokenize.len());

    let tokenizer = Tokenizer::new(EXAMPLE_LANGUAGE_RULES.clone(), &to_tokenize);
    for tok in tokenizer {
        println!("{:?}", tok);
    }

    println!("{}", assembly.len());

    let tokenizer = Tokenizer::new(INSTRUCTION_LANGUAGE_RULES.clone(), &assembly);
    let instructions = tokenizer.collect::<Vec<_>>();
    let mut stack_parser = StackLanguage::new(instructions);
    while stack_parser.step().is_some() {}

    Ok(())
}
