use std::{
    cmp::Ordering,
    marker::PhantomData,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Sub}, iter::Sum,
};

// =============================================================================

// operations/methods needed for type of `bits`
// - zero
// - single
// - count_ones
// - trailing_zeros
// - sub (one) (ones in last N bits)
//
// - equals (zero)
// - bitand (self & other) + assign (self &= other)
// - bitor assign (self |= other)
// - not (!single(index))

pub trait BitsRepr:
    Copy
    + Eq
    + Default
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + Not<Output = Self>
{
    /// maximum number of bits capable of representing
    const SIZE: usize = std::mem::size_of::<Self>() * 8;

    /// value with all bits set to zero
    const ZERO: Self;

    /// value with only the bit at `index` set to one
    ///
    /// Safety:
    /// caller must ensure that `index < Self::SIZE`
    fn single(index: usize) -> Self;

    /// value with the first `n` bits set to one
    ///
    /// Safety:
    /// caller must ensure that `n <= Self::SIZE`
    fn n_ones(n: usize) -> Self;

    /// number of bits set to one
    fn count_ones(self) -> usize;

    /// number of trailing zeros / index of first bit set to one
    fn trailing_zeros(self) -> usize;

    /// number of leading zeros
    fn leading_zeros(self) -> usize;
}

macro_rules! impl_bits_repr_for_int {
    ($Type:ty) => {
        impl BitsRepr for $Type {
            const ZERO: Self = 0;

            fn single(index: usize) -> Self {
                1 << index
            }

            fn n_ones(n: usize) -> Self {
                (1 << n) - 1
            }

            fn count_ones(self) -> usize {
                self.count_ones() as usize
            }

            fn trailing_zeros(self) -> usize {
                self.trailing_zeros() as usize
            }

            fn leading_zeros(self) -> usize {
                self.leading_zeros() as usize
            }
        }
    };
}

impl_bits_repr_for_int! { u8 }
impl_bits_repr_for_int! { u16 }
impl_bits_repr_for_int! { u32 }
impl_bits_repr_for_int! { u64 }
impl_bits_repr_for_int! { u128 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct U64array<const N: usize>([u64; N]);

impl<const N: usize> Default for U64array<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> BitAndAssign for U64array<N> {
    fn bitand_assign(&mut self, other: Self) {
        for (x, y) in self.0.iter_mut().zip(other.0.iter()) {
            *x &= *y;
        }
    }
}

impl<const N: usize> BitAnd for U64array<N> {
    type Output = Self;

    fn bitand(mut self, other: Self) -> Self::Output {
        self &= other;
        self
    }
}

impl<const N: usize> BitOrAssign for U64array<N> {
    fn bitor_assign(&mut self, other: Self) {
        for (x, y) in self.0.iter_mut().zip(other.0.iter()) {
            *x |= *y;
        }
    }
}

impl<const N: usize> BitOr for U64array<N> {
    type Output = Self;

    fn bitor(mut self, other: Self) -> Self::Output {
        self |= other;
        self
    }
}

impl<const N: usize> Not for U64array<N> {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        for x in self.0.iter_mut() {
            *x = !*x;
        }

        self
    }
}

impl<const N: usize> BitsRepr for U64array<N> {
    const SIZE: usize = u64::SIZE * N;

    const ZERO: Self = U64array([0; N]);

    fn single(index: usize) -> Self {
        let mut bits = Self::ZERO;

        bits.0[index / 64] = u64::single(index % 64);

        bits
    }

    fn n_ones(n: usize) -> Self {
        let mut bits = Self::ZERO;

        let n_full = n / 64;

        for i in 0..n_full {
            bits.0[i] = u64::MAX;
        }

        let n_rem = n % 64;

        if n_rem > 0 {
            bits.0[n_full] = u64::n_ones(n_rem);
        }

        bits
    }

    fn count_ones(self) -> usize {
        self.0.map(|x| x.count_ones() as usize).iter().sum()
    }

    fn trailing_zeros(self) -> usize {
        for (i, x) in self.0.iter().enumerate() {
            if *x != 0 {
                return i * 64 + x.trailing_zeros() as usize;
            }
        }

        64 * N
    }

    fn leading_zeros(self) -> usize {
        for (i, x) in self.0.iter().enumerate().rev() {
            if *x != 0 {
                return i * 64 + x.leading_zeros() as usize;
            }
        }

        64 * N
    }
}

// =============================================================================

pub trait Element: Copy + Eq {
    /// representation of bits used for a set of this element type
    type Repr: BitsRepr;

    /// maximum number of elements of this type
    const MAX: usize;

    /// index of this element in the set
    fn index(self) -> usize;

    /// element from index
    fn from_index(index: usize) -> Self;
}

macro_rules! impl_element_for_int {
    ($Type:ty) => {
        impl Element for $Type {
            type Repr = u64;

            const MAX: usize = std::mem::size_of::<Self>() * 8;

            fn index(self) -> usize {
                self as usize
            }

            fn from_index(index: usize) -> Self {
                index as Self
            }
        }
    };
}

impl_element_for_int! { u8 }
impl_element_for_int! { u16 }
impl_element_for_int! { u32 }
impl_element_for_int! { u64 }

macro_rules! impl_element_for_int_newtype {
    ($Ty:ty = $Inner:ident < $MAX:literal in $Repr:ty) => {
        impl Element for $Ty {
            type Repr = $Repr;
            const MAX: usize = $MAX;
            fn index(self) -> usize {
                self.0 as usize
            }
            fn from_index(index: usize) -> Self {
                Self(index as $Inner)
            }
        }
    };
}

pub(crate) use impl_element_for_int_newtype;

// =============================================================================

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BitSet<E: Element, B: BitsRepr> {
    bits: B,
    _phantom: PhantomData<E>,
}

pub type Set<E> = BitSet<E, <E as Element>::Repr>;

impl<E: Element, B: BitsRepr> BitSet<E, B> {
    /// creates bitset from bits.
    ///
    /// Safety:
    /// caller must ensure that all bits above `E::MAX` are zero.
    pub fn with_bits(bits: B) -> Self {
        assert!(
            E::MAX <= B::SIZE,
            "too many elements for bitset ({} > {})",
            E::MAX,
            B::SIZE
        );

        Self {
            bits,
            _phantom: PhantomData,
        }
    }

    /// creates empty bitset.
    pub fn new() -> Self {
        Self::with_bits(B::ZERO)
    }

    /// creates full bitset
    pub fn full() -> Self {
        Self::with_bits(B::n_ones(E::MAX))
    }

    /// creates bitset with a single element.
    pub fn singleton(element: E) -> Self {
        Self::with_bits(B::single(element.index()))
    }

    /// iterator over elements in the set.
    pub fn iter(&self) -> Iter<E, B> {
        Iter::with_bits(self.bits)
    }

    pub fn map<E2: Element, F: Fn(E) -> E2>(&self, f: F) -> BitSet<E2, <E2 as Element>::Repr> {
        let mut bits = <E2 as Element>::Repr::ZERO;

        for e in self.iter() {
            bits |= <E2 as Element>::Repr::single(f(e).index());
        }

        BitSet::with_bits(bits)
    }

    /// returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.bits.count_ones()
    }

    /// returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.bits == B::ZERO
    }

    /// returns `true` if the set contains any elements.
    pub fn is_nonempty(&self) -> bool {
        self.bits != B::ZERO
    }

    /// clears the set, removing all elements.
    pub fn clear(&mut self) {
        self.bits = B::ZERO;
    }

    /// returns `true` if the set contains the specified element.
    pub fn contains(&self, element: E) -> bool {
        self.bits & B::single(element.index()) != B::ZERO
    }

    /// returns `true` if the set is a subset of another,
    /// i.e., `other` contains at least all the values in `self`.
    ///
    /// could maybe be replaced with PartialOrd `leq`?
    pub fn is_subset(&self, other: &Self) -> bool {
        self.bits & other.bits == self.bits
    }

    /// returns `true` if the set is a superset of another,
    /// i.e., `self` contains at least all the values in `other`.
    ///
    /// could maybe be replaced with PartialOrd `geq`?
    pub fn is_superset(&self, other: &Self) -> bool {
        self.bits & other.bits == other.bits
    }

    /// adds an element to the set.
    pub fn insert(&mut self, element: E) {
        assert!(element.index() < B::SIZE);

        self.bits |= B::single(element.index())
    }

    /// removes an element from the set.
    pub fn remove(&mut self, element: E) {
        self.bits &= !B::single(element.index());
    }
}

// =============================================================================

impl<E: Element, B: BitsRepr> IntoIterator for BitSet<E, B> {
    type Item = E;
    type IntoIter = Iter<E, B>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<E: Element, B: BitsRepr> {
    bits: B,
    _phantom: PhantomData<E>,
}

impl<E: Element, B: BitsRepr> Iter<E, B> {
    fn with_bits(bits: B) -> Self {
        Self {
            bits,
            _phantom: PhantomData,
        }
    }
}

impl<E: Element, B: BitsRepr> Iterator for Iter<E, B> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == B::ZERO {
            None
        } else {
            let index = self.bits.trailing_zeros();

            self.bits &= !B::single(index);
            // ^ removes last bit. could replace with following:
            //
            // let t = self.bits & (0 as u64).wrapping_sub(self.bits);
            // self.bits ^= t;
            //
            // maybe faster because no shift?

            Some(E::from_index(index))
        }
    }
}

// =============================================================================

impl<E: Element, B: BitsRepr> FromIterator<E> for BitSet<E, B> {
    fn from_iter<I: IntoIterator<Item = E>>(iter: I) -> Self {
        let mut set = Self::new();

        for element in iter {
            set.insert(element);
        }

        set
    }
}

impl<E: Element, B: BitsRepr> From<&[E]> for BitSet<E, B> {
    fn from(slice: &[E]) -> Self {
        slice.iter().copied().collect()
    }
}

// =============================================================================

// operations
// - union (add, bitor)
// - intersection (~mul, bitand)
// - difference (sub, ?)
// - complement (not, bitnot)

impl<E: Element, B: BitsRepr> BitOr for BitSet<E, B> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::with_bits(self.bits | rhs.bits)
    }
}

impl<E: Element, B: BitsRepr> BitOrAssign for BitSet<E, B> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl<E: Element, B: BitsRepr> BitAnd for BitSet<E, B> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::with_bits(self.bits & rhs.bits)
    }
}

impl<E: Element, B: BitsRepr> Sub for BitSet<E, B> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::with_bits(self.bits & !rhs.bits)
    }
}

impl<E: Element, B: BitsRepr> Sum for BitSet<E, B> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(), |acc, set| acc | set)
    }
}

impl<'a, E: Element, B: BitsRepr> Sum<&'a Self> for BitSet<E, B> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::new(), |acc, &set| acc | set)
    }
}


// =============================================================================

impl<E: Element, B: BitsRepr> PartialOrd for BitSet<E, B> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.is_subset(other) {
            Some(Ordering::Less)
        } else if self.is_superset(other) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

// =============================================================================

mod fmt {
    use std::fmt::*;

    use super::*;

    impl<E, B> Binary for BitSet<E, B>
    where
        E: Element,
        B: BitsRepr + Binary,
    {
        fn fmt(&self, f: &mut Formatter) -> Result {
            let mut lz = self.bits.leading_zeros() - (B::SIZE - E::MAX);

            if self.is_empty() && lz > 0 {
                lz -= 1;
            }

            write!(
                f,
                "{}{:b}",
                "0".repeat(lz),
                self.bits
            )
        }
    }
}

// =============================================================================

#[test]
fn alternate_remove_last_one() {
    let a: u8 = 0b0011_1000;

    let neg_a = 0u8.wrapping_sub(a);

    let t = a & neg_a;

    let a_xor_t = a ^ t;

    println!("      a = {a:08b}");
    println!("  neg_a = {neg_a:08b}");
    println!("      t = {t:08b}");
    println!("a_xor_t = {a_xor_t:08b}");
}

#[test]
fn full_set_constructor() {
    const N: usize = 3;

    let a: u8 = (1 << N) - 1;

    println!("a = {a:08b}");
}

#[test]
fn temp() {
    let size = std::mem::size_of::<U64array<3>>();

    println!("size = {size}");
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Elt(u8);

    impl_element_for_int_newtype! { Elt = u8 < 8 in u8 }

    #[test]
    fn insert_contains() {
        let mut set = Set::new();

        for i in 0..8 {
            assert!(!set.contains(Elt(i)));
        }

        set.insert(Elt(0));

        assert!(set.contains(Elt(0)));

        for i in 1..8 {
            assert!(!set.contains(Elt(i)));
        }
    }

    #[test]
    fn subset() {
        let ps = (0..4)
            .powerset()
            .map(|v| v.into_iter().map(Elt).collect::<Set<Elt>>())
            .collect_vec();

        for (s, t) in ps.iter().tuple_combinations() {
            println!("{s:b} <= {t:b} = {}", s <= t);
        }
    }
}
