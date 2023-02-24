/// implements methods for a newtype wrapper around a bounded integer
macro_rules! impl_bounded_int_newtype {
    ($Ty:ident = $Repr:ident < $MAX:literal) => {
        impl $Ty {
            pub const fn new_unchecked(value: $Repr) -> Self {
                Self(value)
            }

            pub fn new(value: $Repr) -> Option<Self> {
                if value < $MAX {
                    Some(Self(value))
                } else {
                    None
                }
            }

            pub fn list() -> impl Iterator<Item = Self> {
                (0..$MAX).map(Self::new_unchecked)
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
