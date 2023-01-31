/// implements methods for a newtype wrapper around a bounded integer
macro_rules! impl_bounded_int_newtype {
    ($name:ident = $repr:ident < $bound:literal) => {
        impl $name {
            pub fn new(value: $repr) -> Option<Self> {
                (value < $bound).then_some(Self::new_unchecked(value))
            }

            pub(super) const fn new_unchecked(value: $repr) -> Self {
                Self(value)
            }

            pub fn list() -> impl Iterator<Item = Self> {
                (0..$bound).map(Self::new_unchecked)
            }

            pub fn as_index(self) -> usize {
                self.0 as usize
            }

            pub fn from_index(index: usize) -> Option<Self> {
                index.try_into().ok().and_then(Self::new)
            }

            pub(super) fn from_index_unchecked(index: usize) -> Self {
                Self::new_unchecked(index as $repr)
            }
        }
    };
}

pub(super) use impl_bounded_int_newtype;

trait BoundedIntNewtype: Sized {
    type Repr: Copy + Into<usize> + From<usize>;
    const BOUND: usize;

    fn new_unchecked(value: Self::Repr) -> Self;

    fn new(value: Self::Repr) -> Option<Self> {
        (value.into() < Self::BOUND).then_some(Self::new_unchecked(value))
    }

    fn from_index_unchecked(index: usize) -> Self {
        Self::new_unchecked(Self::Repr::from(index))
    }

    fn from_index(index: usize) -> Option<Self> {
        index.try_into().ok().and_then(Self::new)
    }
}
