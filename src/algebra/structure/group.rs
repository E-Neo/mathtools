use crate::algebra::{
    operation::{Associative, Closure, Identity, Invertible, Op},
    structure::Monoid,
};

/// A group is a [Monoid](trait.Monoid.html) each of whose elements is invertible.
pub trait Group<O: Op + Closure + Associative + Identity + Invertible>: Monoid<O> {
    fn id() -> Self {
        Monoid::id()
    }

    fn op(self, rhs: Self) -> Self {
        Monoid::op(self, rhs)
    }

    fn inv(self) -> Self;
}
