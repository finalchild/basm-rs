use core::ops::*;

// While the number of traits for numbers is not too many, the number of methods isn't. Thus, we
// plan to only add methods when we need it for implementations of internal functions.
pub trait PrimUint:
    Sized
    + Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Shl<Output = Self>
    + Shr<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + ShlAssign
    + ShrAssign
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
    + Eq
    + Ord
{
    fn is_zero(self) -> bool;
    fn trailing_zeros(self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
}

macro_rules! define_primitive_uint {
    ($t:ty) => {
        impl PrimUint for $t {
            fn is_zero(self) -> bool {
                self == 0
            }
            fn trailing_zeros(self) -> Self {
                self.trailing_zeros() as $t
            }
            fn wrapping_sub(self, rhs: Self) -> Self {
                self.wrapping_sub(rhs)
            }
        }
    };
}

define_primitive_uint!(u8);
define_primitive_uint!(u16);
define_primitive_uint!(u32);
define_primitive_uint!(u64);
define_primitive_uint!(u128);
define_primitive_uint!(usize);
