use crate::algebra::{
    operation::{Addition, Multiplication},
    structure::AbstractField,
};
pub use rug::Float;

pub trait Real: AbstractField<Addition, Multiplication> {}

macro_rules! impl_real {
    ($Add:ty, |$self_add:ident, $rhs_add:ident| $add:expr, $zero:expr,
     |$self_neg:ident| $neg:expr,
     $Mul:ty, |$self_mul:ident, $rhs_mul:ident| $mul:expr, $one:expr,
     |$self_recip:ident| $recip:expr;
     $($T:ty),*) => {
        $(
            impl_field!($Add, |$self_add, $rhs_add| $add, $zero, |$self_neg| $neg,
                        $Mul, |$self_mul, $rhs_mul| $mul, $one, |$self_recip| $recip;
                        $T);
            impl Real for $T {}
        )*
    }
}

impl_real!(Addition, |self, rhs| self + rhs, 0., |self| -self,
           Multiplication, |self, rhs| self * rhs, 1., |self| 1. / self;
           f32, f64);

impl_real!(
    Addition,
    |self, rhs| Float::with_val(std::cmp::max(self.prec(), rhs.prec()), self + rhs),
    Float::with_val(64, 0),
    |self| Float::with_val(self.prec(), -self),
    Multiplication,
    |self, rhs| Float::with_val(std::cmp::max(self.prec(), rhs.prec()), self * rhs),
    Float::with_val(64, 1),
    |self| Float::with_val(self.prec(), 1 / self);
    Float
);
