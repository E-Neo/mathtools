use crate::algebra::{
    operation::{Associative, Closure, Commutative, Distributive, Identity, Invertible, Op},
    structure::{AbelianGroup, Monoid},
};

/// A Ring is an [AbelianGroup](trait.AbelianGroup.html) under addition,
/// a [Monoid](trait.Monoid.html) under multiplication,
/// and multiplication is distributive with respect to addition.
pub trait Ring<
    Add: Op + Closure + Associative + Identity + Invertible + Commutative,
    Mul: Op + Closure + Associative + Identity + Distributive<Add>,
>: AbelianGroup<Add> + Monoid<Mul>
{
    fn zero() -> Self {
        AbelianGroup::id()
    }

    fn add(self, rhs: Self) -> Self {
        AbelianGroup::op(self, rhs)
    }

    fn neg(self) -> Self {
        AbelianGroup::inv(self)
    }

    fn one() -> Self {
        Monoid::<Mul>::id()
    }

    fn mul(self, rhs: Self) -> Self {
        Monoid::<Mul>::op(self, rhs)
    }
}
