use crate::algebra::{
    operation::{Addition, Multiplication},
    structure::AbstractCommutativeRing,
};
pub use rug::Integer as BigInt;

pub trait Integer: AbstractCommutativeRing<Addition, Multiplication> {}

macro_rules! impl_integer {
    ($Add:ty, |$self_add:ident, $rhs_add:ident| $add:expr, $zero:expr,
     |$self_neg:ident| $neg:expr,
     $Mul:ty, |$self_mul:ident, $rhs_mul:ident| $mul:expr, $one:expr;
     $($T:ty),*) => {
        $(
            impl_commutative_ring!($Add, |$self_add, $rhs_add| $add, $zero, |$self_neg| $neg,
                                   $Mul, |$self_mul, $rhs_mul| $mul, $one; $T);
            impl Integer for $T {}
        )*
    }
}

impl_integer!(
    Addition,
    |self, rhs| self + rhs,
    0,
    |self| -self,
    Multiplication,
    |self, rhs| self * rhs,
    1;
    i8, i16, i32, i64, i128, isize
);

impl_integer!(
    Addition,
    |self, rhs| BigInt::from(self + rhs),
    BigInt::from(0),
    |self| BigInt::from(-self),
    Multiplication,
    |self, rhs| BigInt::from(self * rhs),
    BigInt::from(1);
    BigInt
);
