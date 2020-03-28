use crate::algebra::{
    operation::{Associative, Closure, Commutative, Distributive, Identity, Invertible, Op},
    structure::AbstractRing,
};

/// A CommutativeRing is a Ring where the multiplication is commutative.
pub trait AbstractCommutativeRing<
    Add: Op + Closure + Associative + Identity + Invertible + Commutative,
    Mul: Op + Closure + Associative + Identity + Distributive<Add> + Commutative,
>: AbstractRing<Add, Mul>
{
    fn zero() -> Self {
        AbstractRing::zero()
    }

    fn add(&self, rhs: &Self) -> Self {
        AbstractRing::add(self, rhs)
    }

    fn neg(&self) -> Self {
        AbstractRing::neg(self)
    }

    fn one() -> Self {
        AbstractRing::one()
    }

    fn mul(&self, rhs: &Self) -> Self {
        AbstractRing::mul(self, rhs)
    }
}
