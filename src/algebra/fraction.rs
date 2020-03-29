use crate::algebra::{
    operation::{Addition, Multiplication},
    structure::{AbstractCommutativeRing, AbstractField},
};

pub trait Term: Clone + AbstractCommutativeRing<Addition, Multiplication> {}
impl<T: Clone + AbstractCommutativeRing<Addition, Multiplication>> Term for T {}

pub trait AbstractFraction<T: Term>: AbstractField<Addition, Multiplication> {
    fn numer(&self) -> &T;

    fn denom(&self) -> &T;

    fn zero() -> Self {
        AbstractField::zero()
    }

    fn add(&self, rhs: &Self) -> Self {
        AbstractField::add(self, rhs)
    }

    fn neg(&self) -> Self {
        AbstractField::neg(self)
    }

    fn one() -> Self {
        AbstractField::one()
    }

    fn mul(&self, rhs: &Self) -> Self {
        AbstractField::mul(self, rhs)
    }

    fn recip(&self) -> Self {
        AbstractField::recip(self)
    }
}

#[derive(Debug)]
pub struct Fraction<T: Term> {
    numer: T,
    denom: T,
}

impl<T: Term> Fraction<T> {
    pub fn new(numer: T, denom: T) -> Self {
        Fraction { numer, denom }
    }
}

macro_rules! impl_magma {
    ($O:ty, |$self:ident, $rhs:ident| $op:expr) => {
        impl<T: Term> $crate::algebra::structure::AbstractMagma<$O> for Fraction<T> {
            fn op(&$self, $rhs: &Self) -> Self {
                $op
            }
        }
    }
}

macro_rules! impl_semigroup {
    ($O:ty, |$self:ident, $rhs:ident| $op:expr) => {
        impl_magma!($O, |$self, $rhs| $op);
        impl<T: Term> $crate::algebra::structure::AbstractSemigroup<$O> for Fraction<T> {}
    };
}

macro_rules! impl_monoid {
    ($O:ty, |$self:ident, $rhs:ident| $op:expr, $id:expr) => {
        impl_semigroup!($O, |$self, $rhs| $op);
        impl<T: Term> $crate::algebra::structure::AbstractMonoid<$O> for Fraction<T> {
            fn id() -> Self {
                $id
            }
        }
    };
}

macro_rules! impl_group {
    ($O:ty, |$self_op:ident, $rhs_op:ident| $op:expr, $id:expr,
     |$self_inv:ident| $inv:expr) => {
        impl_monoid!($O, |$self_op, $rhs_op| $op, $id);
        impl<T: Term> $crate::algebra::structure::AbstractGroup<$O> for Fraction<T> {
            fn inv(&$self_inv) -> Self {
                $inv
            }
        }
    };
}

macro_rules! impl_abelian_group {
    ($O:ty, |$self_op:ident, $rhs_op:ident| $op:expr, $id:expr,
     |$self_inv:ident| $inv:expr) => {
        impl_group!($O, |$self_op, $rhs_op| $op, $id, |$self_inv| $inv);
        impl<T: Term> $crate::algebra::structure::AbstractAbelianGroup<$O> for Fraction<T> {}
    };
}

macro_rules! impl_ring {
    ($Add:ty, |$self_add:ident, $rhs_add:ident| $add:expr, $zero:expr,
     |$self_neg:ident| $neg:expr,
     $Mul:ty, |$self_mul:ident, $rhs_mul:ident| $mul:expr, $one:expr) => {
        impl_abelian_group!($Add, |$self_add, $rhs_add| $add, $zero, |$self_neg| $neg);
        impl_monoid!($Mul, |$self_mul, $rhs_mul| $mul, $one);
        impl<T: Term> $crate::algebra::structure::AbstractRing<$Add, $Mul> for Fraction<T> {}
    };
}

macro_rules! impl_commutative_ring {
    ($Add:ty, |$self_add:ident, $rhs_add:ident| $add:expr, $zero:expr,
     |$self_neg:ident| $neg:expr,
     $Mul:ty, |$self_mul:ident, $rhs_mul:ident| $mul:expr, $one:expr) => {
        impl_ring!(
            $Add,
            |$self_add, $rhs_add| $add,
            $zero,
            |$self_neg| $neg,
            $Mul,
            |$self_mul, $rhs_mul| $mul,
            $one
        );
        impl<T: Term> $crate::algebra::structure::AbstractCommutativeRing<$Add, $Mul>
            for Fraction<T>
        {
        }
    };
}

macro_rules! impl_field {
    ($Add:ty, |$self_add:ident, $rhs_add:ident| $add:expr, $zero:expr,
     |$self_neg:ident| $neg:expr,
     $Mul:ty, |$self_mul:ident, $rhs_mul:ident| $mul:expr, $one:expr,
     |$self_recip:ident| $recip:expr) => {
        impl_commutative_ring!(
            $Add,
            |$self_add, $rhs_add| $add,
            $zero,
            |$self_neg| $neg,
            $Mul,
            |$self_mul, $rhs_mul| $mul,
            $one
        );
        impl<T: Term> $crate::algebra::structure::AbstractField<$Add, $Mul> for Fraction<T> {
            fn recip(&$self_recip) -> Self {
                $recip
            }
        }
    };
}

impl_field!(
    Addition,
    |self, rhs| Fraction::new(
        AbstractCommutativeRing::add(
            &AbstractCommutativeRing::mul(&self.numer, &rhs.denom),
            &AbstractCommutativeRing::mul(&self.denom, &rhs.numer),
        ),
        AbstractCommutativeRing::mul(&self.denom, &rhs.denom),
    ),
    Fraction::new(
        AbstractCommutativeRing::zero(),
        AbstractCommutativeRing::one(),
    ),
    |self| Fraction::new(
        AbstractCommutativeRing::neg(&self.numer),
        self.denom.clone()
    ),
    Multiplication,
    |self, rhs| Fraction::new(
        AbstractCommutativeRing::mul(&self.numer, &rhs.numer),
        AbstractCommutativeRing::mul(&self.denom, &rhs.denom)
    ),
    Fraction::new(
        AbstractCommutativeRing::one(),
        AbstractCommutativeRing::one(),
    ),
    |self| Fraction::new(self.denom.clone(), self.numer.clone())
);

impl<T: Term> AbstractFraction<T> for Fraction<T> {
    fn numer(&self) -> &T {
        &self.numer
    }

    fn denom(&self) -> &T {
        &self.denom
    }
}
