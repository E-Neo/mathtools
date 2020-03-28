use crate::algebra::{
    operation::{Associative, Closure, Op},
    structure::AbstractMagma,
};

/// A Semigroup is a Magma where the operation is associative.
pub trait AbstractSemigroup<O: Op + Closure + Associative>: AbstractMagma<O> {
    fn op(&self, rhs: &Self) -> Self {
        AbstractMagma::op(self, rhs)
    }
}
