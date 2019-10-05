//!
//! The type variant.
//!

use std::collections::BTreeMap;
use std::fmt;

use serde_derive::Serialize;

use crate::syntax::TypeVariant;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "name")]
pub enum Variant {
    Unit,
    Boolean,
    IntegerUnsigned {
        bitlength: usize,
    },
    IntegerSigned {
        bitlength: usize,
    },
    Field,
    Array {
        type_variant: Box<Self>,
        size: usize,
    },
    Tuple {
        type_variants: Vec<Self>,
    },
    Structure {
        identifier: String,
        fields: BTreeMap<String, Self>,
    },
    Alias {
        identifier: String,
    },
}

impl Variant {
    pub fn new_unit() -> Self {
        Self::Unit
    }

    pub fn new_boolean() -> Self {
        Self::Boolean
    }

    pub fn new_integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    pub fn new_integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }

    pub fn new_field() -> Self {
        Self::Field
    }

    pub fn new_array(type_variant: TypeVariant, size: usize) -> Self {
        Self::Array {
            type_variant: Box::new(type_variant),
            size,
        }
    }

    pub fn new_tuple(type_variants: Vec<TypeVariant>) -> Self {
        Self::Tuple { type_variants }
    }

    pub fn new_structure(identifier: String, fields: BTreeMap<String, Self>) -> Self {
        Self::Structure { identifier, fields }
    }

    pub fn new_alias(identifier: String) -> Self {
        Self::Alias { identifier }
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Boolean => write!(f, "bool"),
            Self::IntegerUnsigned { bitlength } => write!(f, "u{}", bitlength),
            Self::IntegerSigned { bitlength } => write!(f, "i{}", bitlength),
            Self::Field => write!(f, "field"),
            Self::Array { type_variant, size } => write!(f, "[{}; {}]", type_variant, size),
            Self::Tuple { type_variants } => write!(
                f,
                "({})",
                type_variants
                    .iter()
                    .map(|type_variant| type_variant.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Structure { identifier, fields } => write!(
                f,
                "{} {{ {} }}",
                identifier,
                fields
                    .iter()
                    .map(|(identiifer, type_variant)| format!("{}: {}", identiifer, type_variant))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Alias { identifier } => write!(f, "{}", identifier),
        }
    }
}
