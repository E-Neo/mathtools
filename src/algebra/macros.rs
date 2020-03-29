macro_rules! impl_magma {
    ($O:ty, |$self:ident, $rhs:ident| $op:expr; $($T:ty),*) => {
        $(impl $crate::algebra::structure::AbstractMagma<$O> for $T {
            fn op(&$self, $rhs: &Self) -> Self { $op }
        })*
    }
}

macro_rules! impl_semigroup {
    ($O:ty, |$self:ident, $rhs:ident| $op:expr; $($T:ty),*) => {
        $(
            impl_magma!($O, |$self, $rhs| $op; $T);
            impl $crate::algebra::structure::AbstractSemigroup<$O> for $T {}
        )*
    }
}

macro_rules! impl_monoid {
    ($O:ty, |$self:ident, $rhs:ident| $op:expr, $id:expr; $($T:ty),*) => {
        $(
            impl_semigroup!($O, |$self, $rhs| $op; $T);
            impl $crate::algebra::structure::AbstractMonoid<$O> for $T {
                fn id() -> Self { $id }
            }
        )*
    }
}

macro_rules! impl_group {
    ($O:ty, |$self:ident, $rhs:ident| $op:expr, $id:expr,
     |$self_inv:ident| $inv:expr; $($T:ty),*) => {
        $(
            impl_monoid!($O, |$self, $rhs| $op, $id; $T);
            impl $crate::algebra::structure::AbstractGroup<$O> for $T {
                fn inv(&$self_inv) -> Self { $inv }
            }
        )*
    }
}

macro_rules! impl_abelian_group {
    ($O:ty, |$self:ident, $rhs:ident| $op:expr, $id:expr,
     |$self_inv:ident| $inv:expr; $($T:ty),*) => {
        $(
            impl_group!($O, |$self, $rhs| $op, $id, |$self_inv| $inv; $T);
            impl $crate::algebra::structure::AbstractAbelianGroup<$O> for $T {}
        )*
    }
}

macro_rules! impl_ring {
    ($Add:ty, |$self_add:ident, $rhs_add:ident| $add:expr, $zero:expr,
     |$self_neg:ident| $neg:expr,
     $Mul:ty, |$self_mul:ident, $rhs_mul:ident| $mul:expr, $one:expr;
     $($T:ty),*) => {
        $(
            impl_abelian_group!($Add, |$self_add, $rhs_add| $add, $zero, |$self_neg| $neg; $T);
            impl_monoid!($Mul, |$self_mul, $rhs_mul| $mul, $one; $T);
            impl $crate::algebra::structure::AbstractRing<$Add, $Mul> for $T {}
        )*
    }
}

macro_rules! impl_commutative_ring {
    ($Add:ty, |$self_add:ident, $rhs_add:ident| $add:expr, $zero:expr,
     |$self_neg:ident| $neg:expr,
     $Mul:ty, |$self_mul:ident, $rhs_mul:ident| $mul:expr, $one:expr;
     $($T:ty),*) => {
        $(
            impl_ring!($Add, |$self_add, $rhs_add| $add, $zero, |$self_neg| $neg,
                       $Mul, |$self_mul, $rhs_mul| $mul, $one; $T);
            impl $crate::algebra::structure::AbstractCommutativeRing<$Add, $Mul> for $T {}
        )*
    }
}

macro_rules! impl_field {
    ($Add:ty, |$self_add:ident, $rhs_add:ident| $add:expr, $zero:expr,
     |$self_neg:ident| $neg:expr,
     $Mul:ty, |$self_mul:ident, $rhs_mul:ident| $mul:expr, $one:expr,
     |$self_recip:ident| $recip:expr;
     $($T:ty),*) => {
        $(
            impl_commutative_ring!($Add, |$self_add, $rhs_add| $add, $zero, |$self_neg| $neg,
                                   $Mul, |$self_mul, $rhs_mul| $mul, $one;
                                   $T);
            impl $crate::algebra::structure::AbstractField<$Add, $Mul> for $T {
                fn recip(&$self_recip) -> Self { $recip }
            }
        )*
    };
}
