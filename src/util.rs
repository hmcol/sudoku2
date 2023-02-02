use std::collections::HashSet;

use itertools::Itertools;

pub struct TryIntoArrayError;

// =============================================================================

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

impl<T> TryIntoArray<T> for HashSet<T> {
    fn try_into_array<const N: usize>(self) -> Result<[T; N]> {
        self.into_iter().collect_vec().try_into_array()
    }
}

impl<T: Clone> TryIntoArray<T> for &HashSet<T> {
    fn try_into_array<const N: usize>(self) -> Result<[T; N]> {
        self.iter().cloned().collect_vec().try_into_array()
    }
}
