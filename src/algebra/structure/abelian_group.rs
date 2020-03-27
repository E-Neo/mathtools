use crate::algebra::{
    operation::{Associative, Closure, Commutative, Identity, Invertible, Op},
    structure::Group,
};

/// A AbelianGroup is a [Group](trait.Group.html) where the operation is commutative.
pub trait AbelianGroup<O: Op + Closure + Associative + Identity + Invertible + Commutative>:
    Group<O>
{
    fn id() -> Self {
        Group::id()
    }

    fn op(self, rhs: Self) -> Self {
        Group::op(self, rhs)
    }

    fn inv(self) -> Self {
        Group::inv(self)
    }
}
