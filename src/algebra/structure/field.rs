use crate::algebra::{
    operation::{Associative, Closure, Commutative, Distributive, Identity, Invertible, Op},
    structure::CommutativeRing,
};

/// A Field is a [CommutativeRing](trait.CommutativeRing.html)
/// where each non-zero element is multiplicative invertible.
pub trait Field<
    Add: Op + Closure + Associative + Identity + Invertible + Commutative,
    Mul: Op + Closure + Associative + Identity + Distributive<Add> + Commutative + Invertible,
>: CommutativeRing<Add, Mul>
{
    fn zero() -> Self {
        CommutativeRing::zero()
    }

    fn add(self, rhs: Self) -> Self {
        CommutativeRing::add(self, rhs)
    }

    fn neg(self) -> Self {
        CommutativeRing::neg(self)
    }

    fn one() -> Self {
        CommutativeRing::one()
    }

    fn mul(self, rhs: Self) -> Self {
        CommutativeRing::mul(self, rhs)
    }

    fn recip(self) -> Self;
}
