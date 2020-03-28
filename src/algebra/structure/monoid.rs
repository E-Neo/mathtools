use crate::algebra::{
    operation::{Associative, Closure, Identity, Op},
    structure::AbstractSemigroup,
};

/// A Monoid is a Semigroup with identity.
pub trait AbstractMonoid<O: Op + Closure + Associative + Identity>: AbstractSemigroup<O> {
    fn op(&self, rhs: &Self) -> Self {
        AbstractSemigroup::op(self, rhs)
    }

    fn id() -> Self;
}
