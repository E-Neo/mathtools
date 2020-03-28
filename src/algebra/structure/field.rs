use crate::algebra::{
    operation::{Associative, Closure, Commutative, Distributive, Identity, Invertible, Op},
    structure::AbstractCommutativeRing,
};

/// A Field is a CommutativeRing where each non-zero element is multiplicative invertible.
pub trait AbstractField<
    Add: Op + Closure + Associative + Identity + Invertible + Commutative,
    Mul: Op + Closure + Associative + Identity + Distributive<Add> + Commutative + Invertible,
>: AbstractCommutativeRing<Add, Mul>
{
    fn add(&self, rhs: &Self) -> Self {
        AbstractCommutativeRing::add(self, rhs)
    }

    fn zero() -> Self {
        AbstractCommutativeRing::zero()
    }

    fn neg(&self) -> Self {
        AbstractCommutativeRing::neg(self)
    }

    fn mul(&self, rhs: &Self) -> Self {
        AbstractCommutativeRing::mul(self, rhs)
    }

    fn one() -> Self {
        AbstractCommutativeRing::one()
    }

    fn recip(&self) -> Self;
}
