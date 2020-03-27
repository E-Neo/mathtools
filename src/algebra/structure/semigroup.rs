use crate::algebra::{
    operation::{Associative, Closure, Op},
    structure::Magma,
};

/// A Semigroup is a [Magma](trait.Magma.html) where the operation is associative.
pub trait Semigroup<O: Op + Closure + Associative>: Magma<O> {
    fn op(self, rhs: Self) -> Self {
        Magma::op(self, rhs)
    }
}
