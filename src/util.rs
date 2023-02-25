use itertools::{Combinations, Itertools};

use crate::bitset::{BitSet, BitsRepr, Element};

// =============================================================================

pub struct TryIntoArrayError;

type Result<T> = std::result::Result<T, TryIntoArrayError>;

pub trait TryIntoArray<T> {
    fn try_into_array<const N: usize>(self) -> Result<[T; N]>;
    fn try_singleton(self) -> Result<T>
    where
        Self: Sized,
    {
        self.try_into_array::<1>().map(|[x]| x)
    }
}

impl<T> TryIntoArray<T> for Vec<T> {
    fn try_into_array<const N: usize>(self) -> Result<[T; N]> {
        self.try_into().map_err(|_| TryIntoArrayError)
    }
}

impl<T: Clone> TryIntoArray<T> for &Vec<T> {
    fn try_into_array<const N: usize>(self) -> Result<[T; N]> {
        self.iter().cloned().collect_vec().try_into_array()
    }
}

impl<E: Element, B: BitsRepr> TryIntoArray<E> for BitSet<E, B> {
    fn try_into_array<const N: usize>(self) -> Result<[E; N]> {
        self.iter().collect_vec().try_into_array()
    }
}

// -----------------------------------------------------------------------------

pub struct ArrayCombinations<I: Iterator, const N: usize> {
    iter: Combinations<I>,
}

impl<I: Iterator, const N: usize> Iterator for ArrayCombinations<I, N>
where
    I::Item: Clone,
{
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|vec| vec.try_into().ok())
    }
}

pub trait IterArrayCombinations: Iterator + Sized {
    fn array_combinations<const N: usize>(self) -> ArrayCombinations<Self, N>;
}

impl<I> IterArrayCombinations for I
where
    I: Iterator + Sized,
    I::Item: Clone,
{
    fn array_combinations<const N: usize>(self) -> ArrayCombinations<Self, N> {
        ArrayCombinations {
            iter: self.combinations(N),
        }
    }
}
