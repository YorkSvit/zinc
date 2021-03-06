//!
//! The semantic analyzer standard library `std::convert::from_bits_unsigned` function element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Clone)]
pub struct Function {
    builtin_identifier: BuiltinIdentifier,
    identifier: &'static str,
}

impl Function {
    pub const ARGUMENT_INDEX_BITS: usize = 0;
    pub const ARGUMENT_COUNT: usize = 1;

    pub fn new(builtin_identifier: BuiltinIdentifier) -> Self {
        Self {
            builtin_identifier,
            identifier: "from_bits_unsigned",
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        self.builtin_identifier
    }

    pub fn call(self, actual_elements: Vec<Element>) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let r#type = match element {
                Element::Value(value) => value.r#type(),
                Element::Constant(constant) => constant.r#type(),
                element => {
                    return Err(Error::argument_not_evaluable(
                        self.identifier.to_owned(),
                        index + 1,
                        element.to_string(),
                    ))
                }
            };
            actual_params.push(r#type);
        }

        let return_type = match actual_params.get(Self::ARGUMENT_INDEX_BITS) {
            Some(Type::Array { r#type, size }) => match (r#type.deref(), *size) {
                (Type::Boolean, size)
                    if crate::BITLENGTH_BYTE <= size
                        && size <= crate::BITLENGTH_MAX_INT
                        && size % crate::BITLENGTH_BYTE == 0 =>
                {
                    Type::integer_unsigned(size)
                }
                (r#type, size) => {
                    return Err(Error::argument_type(
                        self.identifier.to_owned(),
                        "bits".to_owned(),
                        Self::ARGUMENT_INDEX_BITS + 1,
                        format!(
                            "[bool; N], {} <= N <= {}, N % {} == 0",
                            crate::BITLENGTH_BYTE,
                            crate::BITLENGTH_MAX_INT,
                            crate::BITLENGTH_BYTE
                        ),
                        format!("[{}; {}]", r#type, size),
                    ))
                }
            },
            Some(r#type) => {
                return Err(Error::argument_type(
                    self.identifier.to_owned(),
                    "bits".to_owned(),
                    Self::ARGUMENT_INDEX_BITS + 1,
                    format!(
                        "[bool; N], {} <= N <= {}, N % {} == 0",
                        crate::BITLENGTH_BYTE,
                        crate::BITLENGTH_MAX_INT,
                        crate::BITLENGTH_BYTE
                    ),
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(Error::argument_count(
                    self.identifier.to_owned(),
                    Self::ARGUMENT_COUNT,
                    actual_params.len(),
                ))
            }
        };

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::argument_count(
                self.identifier.to_owned(),
                Self::ARGUMENT_COUNT,
                actual_params.len(),
            ));
        }

        Ok(return_type)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn std::convert::{}(bits: [bool; N]) -> u{{N}}",
            self.identifier
        )
    }
}
