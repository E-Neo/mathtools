/// An operation is a function which takes zero or more input values
/// (called operands) to a well-defined output value.
pub trait Op {}

/// a, b ∈ S ⇒ a ∘ b ∈ S
pub trait Closure {}

/// ∀ a, b, c ∈ S, (a ∘ b) ∘ c = a ∘ (b ∘ c)
pub trait Associative {}

/// ∀ a, b ∈ S, a ∘ b = b ∘ a
pub trait Commutative {}

/// ∃ e ∈ S, ∀ a ∈ S, e ∘ a = a ∘ e = a
pub trait Identity {}

/// ∃ e ∈ S, ∀ a ∈ S, ∃ a⁻¹ ∈ S such that a⁻¹ ∘ a = a ∘ a⁻¹ = e
pub trait Invertible {}

/// ∀ a, b, c ∈ S, a · (b + c) = (a · b) + (a · c), (b + c) · a = (b · a) + (c · a)
pub trait Distributive<Add: Op> {}

pub struct Addition;
impl Op for Addition {}
impl Closure for Addition {}
impl Associative for Addition {}
impl Commutative for Addition {}
impl Identity for Addition {}
impl Invertible for Addition {}

pub struct Multiplication;
impl Op for Multiplication {}
impl Closure for Multiplication {}
impl Associative for Multiplication {}
impl Commutative for Multiplication {}
impl Identity for Multiplication {}
impl Invertible for Multiplication {}
impl Distributive<Addition> for Multiplication {}
