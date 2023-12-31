use std::str::FromStr;

use crate::bitset::{impl_element_for_int_newtype, Element};

// =============================================================================

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Digit(u8);

super::macros::impl_bounded_int_newtype! { Digit = u8 < 9 }

impl_element_for_int_newtype! { Digit = u8 < 9 in u16 }

#[derive(Debug)]
pub struct ParseDigitError;

impl FromStr for Digit {
    type Err = ParseDigitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u8>()
            .ok()
            .and_then(|num| num.checked_sub(1))
            .and_then(Digit::new)
            .ok_or(ParseDigitError)
    }
}

impl std::fmt::Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

impl std::fmt::Debug for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Digit({self})")
    }
}
