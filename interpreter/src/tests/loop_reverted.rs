//!
//! The interpreter tests.
//!

#![cfg(test)]

use crate::Interpreter;

#[test]
fn test() {
    let input = r#"
inputs {}

let mut sum: u64 = 0;
for i in 10..=0 {
    sum = sum + i;
};

require(sum == 55 as u64);
"#;

    let expected = Ok(());

    let result =
        Interpreter::default().interpret(parser::parse(input.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}
