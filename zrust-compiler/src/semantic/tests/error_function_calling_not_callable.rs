//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Analyzer;
use crate::semantic::Element;
use crate::semantic::Error as SemanticError;
use crate::semantic::Place;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let another = false;
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionCallOnNotCallable(
        Location::new(4, 24),
        Element::Place(Place::new("another".to_owned())),
    )));

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
