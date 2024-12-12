/* Copyright 2024 Charlotte Ausel

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License. */

//! Ringlike structures have two binary operators with distributivity.
//!
//! The distributive property sets ringlike structures apart from [`crate::latticelike`]s, which
//! have absorption rather than distributivity. The most studied of these is the ring, which this
//! module is named after (though the field is possibly just as extensively studied). We usually
//! call these two operators "addition" and "multiplication".
//!
//! We don't give semiring or near-ring implementations because these vary from author to author.

use crate::{
    grouplike::{AbelianGroup, CommutativeMonoid, Monoid},
    operators::{Plus, Times},
};

/// A ring.
///
/// That is, a set which has two [`crate::operators::BinaryOperator`]s: one called addition
/// ([`Plus`]) which forms an [`AbelianGroup`] and the other called multiplication ([`Times`])
/// which forms a [`Monoid`]. The additive identity is called *zero* and the multiplicative
/// identity is called *one*. We usually denote these as 0 and 1. Some authors call 1 the
/// *unity*. We usually give addition the familiar symbol `+` and leave multiplication implied. If
/// we want to be explicit, we write `⋅` for multiplication.
///
/// Sometimes, rather than calling it 'multiplication', authors call it '(ring) product' to avoid
/// confusion with multiplication of numbers in the sets naturals, rationals, etc. However, since
/// we're already using 'addition' for the group operator, we might as well also use the word
/// 'multiplication'.
///
/// <div class="warning">
///
/// Note, using this definition, multiplication has an identity called 1, sometimes called the
/// *unity* of the ring, and addition has inverses (otherwise it would be a [`CommutativeMonoid`]).
/// Rings without these two properties are sometimes called a *rng* (pronounced 'rung') and *rig*
/// respectively.
///
/// </div>
///
/// Alternatively, a ring *R* obeys the ring axioms (these axioms are numbered differently by
/// different authors), for all *x*, *y*, *z* ∈ *R*:
///
/// - (A0) Addition is closed, *a* + *b* ∈ *R*.
/// - (A1) Addition is associative, (*a* + *b*) + *c* = *a* + (*b* + *c*).
/// - (A2) Addition has an identity called 0 (zero), ∃ 0 ∈ *R* s.t. *a* + 0 = a = 0 + *a*.
/// - (A3) Addition is commutative, *a* + *b* = *b* + *a*.
/// - (M0) Multiplication is closed,  *ab* ∈ *R*.
/// - (M1) Multiplication is associative, (*ab*)*c* = *a*(*bc*).
/// - (M2) Multiplication has an identity called 1 (one), ∃ 1 ∈ *R* s.t. *a*1 = a = 1*a*.
/// - (D)  Multiplication is distributive over addition, *a*(*b* + *c*) = *ab* + *ac*
///        and (*b* + *c*)*a* = *ba* + *ca*.
///
/// We are primarily interested in [`CommutativeRing`]s, so see that struct's documentation for an
/// example of implementing the ring trait. An example of a noncommutative ring is the ring of n×m
/// matrices where addition and multiplication is the familiar matrix addition and multiplication.
pub trait Ring: AbelianGroup<Plus> + Monoid<Times> {
    const ZERO: Self;
    const ONE: Self;
}

/// A commutative ring.
///
/// Although addition is already commutative, multiplication is not necessarily commutative. If it
/// *is* commutative, then the ring is called a *commutative ring*.
///
/// Rather than multiplication simply being a [`Monoid`], it is now a [`CommutativeMonoid`].
/// Alternatively, we add the additional axiom:
///
/// - (M3) Multiplication is commutative, *ab* = *ba*.
///
/// # Example
///
/// ```rust
/// use yaaarc::{
///     grouplike::{
///         AbelianGroup,
///         CommutativeMagma,
///         CommutativeMonoid,
///         Magma,
///         Quasigroup,
///         Semigroup,
///         UnitalMagma},
///     operators::{BinaryOperator, Plus, Times},
///     ringlike::CommutativeRing,
/// };
///
/// struct Mod4(u8);
///
/// impl BinaryOperator<Plus> for Mod4 {
///     fn op(&self, rhs: Self) -> Self {
///         Mod4((self.0 + rhs.0) % 4)
///     }
///     fn op_assign(&mut self, rhs: Self) {
///         self.0 += rhs.0;
///         self.0 %= 4;
///     }
/// }
///
/// impl BinaryOperator<Times> for Mod4 {
///     fn op(&self, rhs: Self) -> Self {
///         Mod4((self.0 * rhs.0) % 4)
///     }
///     fn op_assign(&mut self, rhs: Self) {
///         self.0 *= rhs.0;
///         self.0 %= 4;
///     }
/// }
///
/// impl Magma<Plus> for Mod4 {}
/// impl Semigroup<Plus> for Mod4 {}
/// impl UnitalMagma<Plus> for Mod4 {
///     const IDENTITY: Self = Mod4(0);
/// }
/// impl Quasigroup<Plus> for Mod4 {
///     fn inverse(&self) -> Self {
///         Mod4(4 - self.0)
///     }
/// }
/// impl CommutativeMagma<Plus> for Mod4 {}
/// impl AbelianGroup<Plus> for Mod4 {}
///
/// impl Magma<Times> for Mod4 {}
/// impl Semigroup<Times> for Mod4 {}
/// impl UnitalMagma<Times> for Mod4 {
///     const IDENTITY: Self = Mod4(1);
/// }
/// impl CommutativeMagma<Times> for Mod4 {}
/// impl CommutativeMonoid<Times> for Mod4 {}
///
/// impl CommutativeRing for Mod4 {
///     const ZERO: Self = Mod4(0);
///     const ONE: Self = Mod4(1);
/// }
/// ```
pub trait CommutativeRing: AbelianGroup<Plus> + CommutativeMonoid<Times> {
    const ZERO: Self;
    const ONE: Self;
}
