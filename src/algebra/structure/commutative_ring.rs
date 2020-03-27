use crate::algebra::{
    operation::{Associative, Closure, Commutative, Distributive, Identity, Invertible, Op},
    structure::Ring,
};

/// A CommutativeRing is a [Ring](trait.Ring.html) where the multiplication is commutative.
pub trait CommutativeRing<
    Add: Op + Closure + Associative + Identity + Invertible + Commutative,
    Mul: Op + Closure + Associative + Identity + Distributive<Add> + Commutative,
>: Ring<Add, Mul>
{
    fn zero() -> Self {
        Ring::zero()
    }

    fn add(self, rhs: Self) -> Self {
        Ring::add(self, rhs)
    }

    fn neg(self) -> Self {
        Ring::neg(self)
    }

    fn one() -> Self {
        Ring::one()
    }

    fn mul(self, rhs: Self) -> Self {
        Ring::mul(self, rhs)
    }
}
