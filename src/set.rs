use std::ops::{BitAnd, BitOr, BitXor, Not, Sub};

use crate::sudoku::digit::Digit;

pub trait Element: Copy + Sized {
    type SetRepr: Copy
        + Eq
        + BitAnd<Output = Self::SetRepr>
        + BitOr<Output = Self::SetRepr>
        + BitXor<Output = Self::SetRepr>;

    const EMPTY: <Self::SetRepr as BitAnd>::Output;
    const FULL: Self::SetRepr;

    fn into_set_repr(self) -> Self::SetRepr;
    fn into_set(self) -> Set<Self> {
        Set::from_bits(self.into_set_repr())
    }
}

impl Element for Digit {
    type SetRepr = u16;

    const EMPTY: <Self::SetRepr as BitAnd>::Output = 0;
    const FULL: Self::SetRepr = 0b111111111;

    fn into_set_repr(self) -> Self::SetRepr {
        1 << self.as_index()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Set<E: Element> {
    data: E::SetRepr,
}

impl<E: Element> Set<E>
where
    Self: Copy,
{
    pub const EMPTY: Self = Self { data: E::EMPTY };
    pub const FULL: Self = Self { data: E::FULL };

    fn from_bits(bits: E::SetRepr) -> Self {
        Self { data: bits }
    }

    pub fn is_empty(&self) -> bool {
        self.data == E::EMPTY
    }

    pub fn is_nonempty(&self) -> bool {
        self.data != E::EMPTY
    }

    pub fn contains(&self, element: E) -> bool {
        (*self & element).is_nonempty()
    }
}

impl<E: Element> BitAnd for Set<E> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_bits(self.data & rhs.data)
    }
}

impl<E: Element> BitAnd<E> for Set<E> {
    type Output = Self;

    fn bitand(self, rhs: E) -> Self::Output {
        self & rhs.into_set()
    }
}

impl<E: Element> BitOr for Set<E> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_bits(self.data | rhs.data)
    }
}

impl<E: Element> BitOr<E> for Set<E> {
    type Output = Self;

    fn bitor(self, rhs: E) -> Self::Output {
        self | rhs.into_set()
    }
}

impl<E: Element> BitXor for Set<E> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::from_bits(self.data ^ rhs.data)
    }
}

impl<E: Element> BitXor<E> for Set<E> {
    type Output = Self;

    fn bitxor(self, rhs: E) -> Self::Output {
        self ^ rhs.into_set()
    }
}

impl<E: Element> Not for Set<E> {
    type Output = Self;

    fn not(self) -> Self::Output {
        self ^ Set::FULL
    }
}

impl<E: Element> Sub for Set<E> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self & !rhs
    }
}

impl<E: Element> Sub<E> for Set<E> {
    type Output = Self;

    fn sub(self, rhs: E) -> Self::Output {
        self - rhs.into_set()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Iter<E: Element>(E::SetRepr);

impl Iterator for Iter<Digit> {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == Digit::EMPTY {
            None
        } else {
            let bit = self.0.trailing_zeros() as usize;
            self.0 ^= 1 << bit;
            Digit::from_index_checked(bit)
        }
    }
}

impl IntoIterator for Set<Digit> {
    type Item = Digit;
    type IntoIter = Iter<Digit>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.data)
    }
}



