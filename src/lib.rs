#[macro_use]
extern crate lazy_static;

pub mod tokenizer;
pub mod example_language;
pub mod instructions_language;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
