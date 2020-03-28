use crate::algebra::operation::{Closure, Op};

/// A Magma is a set with a binary operation.
pub trait AbstractMagma<O: Op + Closure>: Sized {
    fn op(&self, rhs: &Self) -> Self;
}
