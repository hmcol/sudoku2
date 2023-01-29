use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Digit(u8);

impl Digit {
    fn new_unchecked(value: u8) -> Self {
        Self(value)
    }

    pub fn new_checked(value: u8) -> Option<Self> {
        match value {
            0..=8 => Some(Self::new_unchecked(value)),
            _ => None,
        }
    }

    pub fn list() -> impl Iterator<Item = Self> {
        (0..9).map(Self::new_unchecked)
    }

    pub fn as_index(self) -> usize {
        self.0 as usize
    }

    fn from_index_unchecked(index: usize) -> Self {
        Self::new_unchecked(index as u8)
    }

    pub fn from_index_checked(index: usize) -> Option<Self> {
        index.try_into().ok().and_then(Self::new_checked)
    }
}

#[derive(Debug)]
pub struct ParseDigitError;

impl FromStr for Digit {
    type Err = ParseDigitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u8>()
            .ok()
            .and_then(|num| num.checked_sub(1))
            .and_then(Digit::new_checked)
            .ok_or(ParseDigitError)
    }
}

impl std::fmt::Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}
