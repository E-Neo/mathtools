use crate::algebra::{
    operation::{Associative, Closure, Commutative, Distributive, Identity, Invertible, Op},
    structure::{AbstractAbelianGroup, AbstractMonoid},
};

/// A Ring is an AbelianGroup under addition,
/// a Monoid under multiplication,
/// and multiplication is distributive with respect to addition.
pub trait AbstractRing<
    Add: Op + Closure + Associative + Identity + Invertible + Commutative,
    Mul: Op + Closure + Associative + Identity + Distributive<Add>,
>: AbstractAbelianGroup<Add> + AbstractMonoid<Mul>
{
    fn add(&self, rhs: &Self) -> Self {
        AbstractAbelianGroup::op(self, rhs)
    }

    fn zero() -> Self {
        AbstractAbelianGroup::id()
    }

    fn neg(&self) -> Self {
        AbstractAbelianGroup::inv(self)
    }

    fn mul(&self, rhs: &Self) -> Self {
        AbstractMonoid::<Mul>::op(self, rhs)
    }

    fn one() -> Self {
        AbstractMonoid::<Mul>::id()
    }
}
