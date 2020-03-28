use crate::algebra::{
    operation::{Associative, Closure, Commutative, Identity, Invertible, Op},
    structure::AbstractGroup,
};

/// A AbelianGroup is a Group where the operation is commutative.
pub trait AbstractAbelianGroup<O: Op + Closure + Associative + Identity + Invertible + Commutative>:
    AbstractGroup<O>
{
    fn op(&self, rhs: &Self) -> Self {
        AbstractGroup::op(self, rhs)
    }

    fn id() -> Self {
        AbstractGroup::id()
    }

    fn inv(&self) -> Self {
        AbstractGroup::inv(self)
    }
}
