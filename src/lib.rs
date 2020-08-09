#[macro_use]
extern crate lazy_static;

pub mod example_language;
pub mod instructions_language;
pub mod tokenizer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
