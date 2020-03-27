use crate::algebra::operation::{Closure, Op};

/// A Magma is a set with a binary operation.
pub trait Magma<O: Op + Closure>: Sized {
    fn op(self, rhs: Self) -> Self;
}
