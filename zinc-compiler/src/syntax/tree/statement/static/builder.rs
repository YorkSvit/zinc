//!
//! The static statement builder.
//!

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::Identifier;
use crate::syntax::StaticStatement;
use crate::syntax::Type;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    r#type: Option<Type>,
    expression: Option<Expression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_type(&mut self, value: Type) {
        self.r#type = Some(value);
    }

    pub fn set_expression(&mut self, value: Expression) {
        self.expression = Some(value);
    }

    pub fn finish(mut self) -> StaticStatement {
        StaticStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.identifier.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "identifier"
                )
            }),
            self.r#type.take().unwrap_or_else(|| {
                panic!("{}{}", crate::syntax::PANIC_BUILDER_REQUIRES_VALUE, "type")
            }),
            self.expression.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "expression"
                )
            }),
        )
    }
}
