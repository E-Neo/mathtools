use crate::algebra::{
    operation::{Associative, Closure, Identity, Op},
    structure::Semigroup,
};

/// A Monoid is a [Semigroup](trait.Semigroup.html) with identity.
pub trait Monoid<O: Op + Closure + Associative + Identity>: Semigroup<O> {
    fn id() -> Self;

    fn op(self, rhs: Self) -> Self {
        Semigroup::op(self, rhs)
    }
}
