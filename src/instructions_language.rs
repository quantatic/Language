#[macro_use]

use crate::tokenizer::{Token, TokenRule, TokenParseRule};

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum InstructionToken {
    Push(f32),
    Pop,
    Peek,
    Add,
    Sub,
    Mul,
    Div,
    Jmp(usize),
    Branch(usize),
    Equals,
    Store(usize),
    Load(usize),
    Halt
}

impl Token for InstructionToken { }

lazy_static! {
    pub static ref INSTRUCTION_LANGUAGE_RULES: Vec<TokenRule<InstructionToken>> = vec![
        TokenRule::new(r"PUSH (0|\-?[1-9][0-9]*)", TokenParseRule::Map(|val| {
            InstructionToken::Push(val[5..].parse().unwrap())
        })),
        TokenRule::new(r"POP", TokenParseRule::Constant(InstructionToken::Pop)),
        TokenRule::new(r"PEEK", TokenParseRule::Constant(InstructionToken::Peek)),
        TokenRule::new(r"ADD", TokenParseRule::Constant(InstructionToken::Add)),
        TokenRule::new(r"SUB", TokenParseRule::Constant(InstructionToken::Sub)),
        TokenRule::new(r"MUL", TokenParseRule::Constant(InstructionToken::Mul)),
        TokenRule::new(r"DIV", TokenParseRule::Constant(InstructionToken::Div)),
        TokenRule::new(r"JMP [0-9]+", TokenParseRule::Map(|val| {
            InstructionToken::Jmp(val[4..].parse().unwrap())
        })),
        TokenRule::new(r"BR [0-9]+", TokenParseRule::Map(|val| {
            InstructionToken::Branch(val[3..].parse().unwrap())
        })),
        TokenRule::new(r"EQ", TokenParseRule::Constant(InstructionToken::Equals)),
        TokenRule::new(r"STORE [0-9]+", TokenParseRule::Map(|val| {
            InstructionToken::Store(val[6..].parse().unwrap())
        })),
        TokenRule::new(r"LOAD [0-9]+", TokenParseRule::Map(|val| {
            InstructionToken::Load(val[5..].parse().unwrap())
        })),
        TokenRule::new(r"HLT", TokenParseRule::Constant(InstructionToken::Halt)),
        TokenRule::new(r"\s*\n", TokenParseRule::Ignore)
    ]
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
}

pub struct StackLanguage {
    stack: Vec<f32>,
    locals: HashMap<usize, f32>,
    instructions: Vec<InstructionToken>,
    ip: usize
}

impl StackLanguage {
    pub fn new(instructions: Vec<InstructionToken>) -> Self {
        StackLanguage {
            stack: Vec::new(),
            locals: HashMap::new(),
            instructions,
            ip: 0
        }
    }

    pub fn step(&mut self) -> Option<()> {
        if self.ip >= self.instructions.len() {
            return None
        }

        let next_instruction = &self.instructions[self.ip];
        self.ip += 1;

        match *next_instruction {
            InstructionToken::Push(val) => self.stack.push(val),
            InstructionToken::Pop => {
                self.stack.pop().unwrap();
            },
            InstructionToken::Peek => {
                let top_of_stack = self.stack.last().unwrap();
                println!("Top of stack: {}", top_of_stack);
            },
            InstructionToken::Add => {
                let top_val = self.stack.pop().unwrap();
                let second_val = self.stack.pop().unwrap();
                self.stack.push(second_val + top_val);
            },
            InstructionToken::Sub => {
                let top_val = self.stack.pop().unwrap();
                let second_val = self.stack.pop().unwrap();
                self.stack.push(second_val - top_val);
            },
            InstructionToken::Mul => {
                let top_val = self.stack.pop().unwrap();
                let second_val = self.stack.pop().unwrap();
                self.stack.push(second_val * top_val);
            },
            InstructionToken::Div => {
                let top_val = self.stack.pop().unwrap();
                let second_val = self.stack.pop().unwrap();
                self.stack.push(second_val / top_val);
            },
            InstructionToken::Jmp(addr) => {
                self.ip = addr;
            },
            InstructionToken::Branch(addr) => {
                if self.stack.pop().unwrap() != 0.0 {
                    self.ip = addr;
                }
            },
            InstructionToken::Equals => {
                let top_val = self.stack.pop().unwrap();
                let second_val = self.stack.pop().unwrap();
                if second_val == top_val {
                    self.stack.push(1.0)
                } else {
                    self.stack.push(0.0)
                }
            },
            InstructionToken::Store(idx) => {
                let to_save = self.stack.pop().unwrap();
                self.locals.insert(idx, to_save);
            },
            InstructionToken::Load(idx) => {
                let loaded_val = *self.locals.get(&idx).unwrap();
                self.stack.push(loaded_val)
            }
            InstructionToken::Halt => {
                return None
            }
        };

        Some(())
    }
}
