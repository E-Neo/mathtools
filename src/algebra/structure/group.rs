use crate::algebra::{
    operation::{Associative, Closure, Identity, Invertible, Op},
    structure::AbstractMonoid,
};

/// A group is a Monoid each of whose elements is invertible.
pub trait AbstractGroup<O: Op + Closure + Associative + Identity + Invertible>:
    AbstractMonoid<O>
{
    fn op(&self, rhs: &Self) -> Self {
        AbstractMonoid::op(self, rhs)
    }

    fn id() -> Self {
        AbstractMonoid::id()
    }

    fn inv(&self) -> Self;
}
