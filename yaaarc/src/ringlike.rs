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
    grouplike::{AbelianGroup, CommutativeMagma, Group, Monoid, Quasigroup},
    operators::{Plus, Times},
};

/// A ring.
///
/// That is, a set which has two [`crate::operators::BinaryOperator`]s: one called addition
/// ([`Plus`]) which forms an [`AbelianGroup`] and the other called multiplication ([`Times`]) which
/// forms a [`Monoid`]. The additive identity is called *zero* and the multiplicative identity is
/// called *one*. We usually denote these as 0 and 1. Some authors call 1 the *unity*. We usually
/// give addition the familiar symbol `+` and leave multiplication implied. If we want to be
/// explicit, we write `⋅` for multiplication.
///
/// Sometimes, rather than calling it 'multiplication', authors call it '(ring) product' to avoid
/// confusion with multiplication of numbers in the sets naturals, rationals, etc. However, since
/// we're already using 'addition' for the group operator, we might as well also use the word
/// 'multiplication'.
///
/// <div class="warning">
///
/// Note, using this definition, multiplication has an identity called 1, sometimes called the
/// *unity* of the ring, and addition has inverses (otherwise it would be a
/// [`crate::grouplike::CommutativeMonoid`]). Rings without these two properties are sometimes
/// called a *rng* (pronounced 'rung') and *rig* respectively.
///
/// </div>
///
/// Alternatively, a ring *R* obeys the ring axioms (these axioms are numbered differently by
/// different authors), for all *x*, *y*, *z* ∈ *R*:
///
/// * (A0) Addition is closed, *x* + *y* ∈ *R*.
/// * (A1) Addition is associative, (*x* + *y*) + *z* = *x* + (*y* + *z*).
/// * (A2) Addition has an identity called 0 (zero), ∃ 0 ∈ *R* s.t. *x* + 0 = *x* = 0 + *x*.
/// * (A3) Addition is commutative, *x* + *y* = *y* + *x*.
/// * (A4) Additive inverses exist, ∃ *x'* s.t. *x* + *x'* = 0 = *x'* + *x*.
/// * (M0) Multiplication is closed,  *xy* ∈ *R*.
/// * (M1) Multiplication is associative, (*xy*)*z* = *x*(*yz*).
/// * (M2) Multiplication has an identity called 1 (one), ∃ 1 ∈ *R* s.t. *x*1 = a = 1*x*.
/// * (D)  Multiplication is distributive over addition, *x*(*y* + *z*) = *xy* + *xz* and (*y* +
///        *z*)*x* = *yx* + *zx*.
///
/// We also introduce functions to compute the inverse of a ring element, if it exists. We call an
/// invertible element a *unit* which form the *group of units*; it's a group under the
/// multiplication operation of the ring.
///
/// We need the [`Sized`] trait because we are now introducing functions which may compute a value,
/// such as the [`Ring::inverse_mul`] of an element which may or may not exist.
///
/// # Example
///
/// Here we implement the zero ring. It is, in fact, also a commutative ring, but it's a convenient
/// structure to keep the example brief.
///
/// ```rust
/// use yaaarc::{
///     grouplike::{
///         AbelianGroup,
///         CommutativeMagma,
///         Magma,
///         Monoid,
///         Quasigroup,
///         Semigroup,
///         UnitalMagma},
///     operators::{BinaryOperator, Plus, Times},
///     ringlike::Ring,
/// };
///
///
/// // Note how we only have one constructor. Thus, this type is isomorphic to the unit type, which
/// // is what we want.
/// #[derive(PartialEq)]
/// struct ZeroRing;
///
/// impl BinaryOperator<Plus> for ZeroRing {
///     fn op(&self, rhs: Self) -> Self {
///         Self
///     }
///     fn op_assign(&mut self, rhs: Self) {
///         // We don't do anything since we cannot change the value of the unit type.
///         ()
///     }
/// }
///
/// impl BinaryOperator<Times> for ZeroRing {
///     fn op(&self, rhs: Self) -> Self {
///         Self
///     }
///     fn op_assign(&mut self, rhs: Self) {
///         ()
///     }
/// }
///
/// impl Magma<Plus> for ZeroRing {}
/// impl Semigroup<Plus> for ZeroRing {}
/// impl UnitalMagma<Plus> for ZeroRing {
///     const IDENTITY: Self = Self;
/// }
/// impl Quasigroup<Plus> for ZeroRing {
///     fn inverse(&self) -> Self {
///         Self
///     }
/// }
/// impl CommutativeMagma<Plus> for ZeroRing {}
/// impl AbelianGroup<Plus> for ZeroRing {}
///
/// impl Magma<Times> for ZeroRing {}
/// impl Semigroup<Times> for ZeroRing {}
/// impl UnitalMagma<Times> for ZeroRing {
///     const IDENTITY: Self = Self;
/// }
/// impl Monoid<Times> for ZeroRing {}
///
/// impl Ring for ZeroRing {
///     const ZERO: Self = Self;
///     const ONE: Self = Self;
///
///     fn left_inverse_mul(&self) -> Option<Self> {
///         Some(ZeroRing)
///     }
///
///     fn right_inverse_mul(&self) -> Option<Self> {
///         Some(ZeroRing)
///     }
///
///     fn inverse_mul(&self) -> Option<Self> {
///         Some(ZeroRing)
///     }
/// }
/// ```
pub trait Ring: AbelianGroup<Plus> + Monoid<Times> + Sized {
    const ZERO: Self;
    const ONE: Self;

    /// The left multiplicative inverse.
    ///
    /// Returns [`None`] if there doesn't exist one, or [`Some`] *i* where for the given *a* ∈ *R*,
    /// *ia* = 1.
    fn left_inverse_mul(&self) -> Option<Self>;

    /// The right multiplicative inverse.
    ///
    /// Returns [`None`] if there doesn't exist one, or [`Some`] *i* where for the given *a* ∈ *R*,
    /// *ai* = 1.
    fn right_inverse_mul(&self) -> Option<Self>;

    /// The two-sided multiplicative inverse.
    ///
    /// Returns [`None`] if there doesn't exist one, or [`Some`] *i* where for the given *a* ∈ *R*,
    /// *ai* = *ia* = 1.
    ///
    /// If we have, for some *a*, *b*, *c* ∈ *R* that *ab* = 1 = *bc*, then *b* = *c*, which allows us to
    /// talk about *the* two-sided inverse, if it exists.
    fn inverse_mul(&self) -> Option<Self>;

    /// A convenience function to call the additive inverse function (i.e.
    /// [`Quasigroup<Plus>::inverse`]).
    fn inverse_add(&self) -> Self {
        <Self as Quasigroup<Plus>>::inverse(&self)
    }

    /// Checks whether the given element is a unit.
    fn is_unit(&self) -> bool {
        self.inverse_mul().is_some()
    }
}

/// A division ring, a ring where the nonzero elements form a group under multiplication.
///
/// Formally, *R* is a ring, and the group of units is exactly the ring without zero. Alternatively,
/// for all *r* ∈ *R* ∖ 0, there exists a unique *i* ∈ *R* ∖ 0 s.t. *xi* = *ix* = 1. We sometimes
/// write this as *x*⁻¹.
///
/// Since multiplication forms an abelian group for all elements *except* zero, the only time when
/// the [`Ring::inverse_mul`] function returns [`None`] is when it is called on zero.
///
/// # Safety
///
/// It is undefined behaviour if [`Ring::ZERO`] is used as the input to [`DivisionRing::div_right`]
/// or [`DivisionRing::div_left`]! The div function need only support nonzero inputs.
pub trait DivisionRing: Ring + Group<Times> {
    /// Given a `rhs`, returns `self` (`rhs`¹).
    ///
    /// # Safety
    ///
    /// It is undefined behaviour if [`Ring::ZERO`] is used as the input! The
    /// [`DivisionRing::div_right`] function need only
    /// support nonzero inputs.
    fn div_right(&self, rhs: Self) -> Self;

    /// Given a `rhs`, returns `rhs`⁻¹ `self`.
    ///
    /// # Safety
    ///
    /// It is undefined behaviour if [`Ring::ZERO`] is used as the input! The
    /// [`DivisionRing::div_right`] function need only support nonzero inputs.
    fn div_left(&self, rhs: Self) -> Self;
}

/// A commutative ring.
///
/// Although addition is already commutative, multiplication is not necessarily commutative. If it
/// *is* commutative, then the ring is called a *commutative ring*.
///
/// Rather than multiplication simply being a [`Monoid`], it is now a
/// [`crate::grouplike::CommutativeMonoid`]. Alternatively, we add the additional axiom:
///
/// * (M3) Multiplication is commutative, *ab* = *ba*.
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
///         Monoid,
///         Quasigroup,
///         Semigroup,
///         UnitalMagma},
///     operators::{BinaryOperator, Plus, Times},
///     ringlike::{CommutativeRing, Ring},
/// };
///
/// #[derive(PartialEq)]
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
/// impl Monoid<Times> for Mod4 {}
///
/// impl Ring for Mod4 {
///     const ZERO: Self = Mod4(0);
///     const ONE: Self = Mod4(1);
///
///     fn left_inverse_mul(&self) -> Option<Self> {
///         match self.0 {
///             1 => Some(Mod4(1)),
///             3 => Some(Mod4(3)),
///             // We assume that no value above 3 will exist.
///             _ => None
///         }
///     }
///
///     fn right_inverse_mul(&self) -> Option<Self> {
///         // Because Z/4Z is commutative, we can just call left_inverse().
///         self.left_inverse_mul()
///     }
///
///     fn inverse_mul(&self) -> Option<Self> {
///         self.left_inverse_mul()
///     }
/// }
///
/// impl CommutativeRing for Mod4 {}
/// ```
pub trait CommutativeRing: Ring + CommutativeMagma<Times> {}

/// An integral domain.
///
/// First, we define a zero divisor: for a [`Ring`] *R*, we call an *x* ∈ *R* a *zero divisor* if
/// *x* ≠ 0 and exists a *y* ∈ *R* s.t. xy = 0. Some authors choose to include 0 as a zero divisor
/// and call this definition a *proper* zero divisor, but it's easier to exclude it, since we almost
/// always are interested in proper zero divisors only.
///
/// An integral domain is is a [`CommutativeRing`] where there are no zero divisors, or in other
/// words, where every nonzero element is cancellable. Alternatively, let *R* be an integral domain,
/// then for all *x*, *y* ∈ *R*, if *xy* = 0, then *x* = 0 or *y* = 0.
///
/// We can add this to our axioms of a [`CommutativeRing`], for all *x*, *y* ∈ *R*:
///
/// * (D) There are no zero divisors, if xy = 0, then x = 0 or y = 0.
pub trait IntegralDomain: CommutativeRing {
    /// The 'associates' equivalence relation, i.e. does `self` associate with `rhs`?
    ///
    /// The relation is *a* ~ *b* iff there exists a unit *u* s.t. *a* = *ub*.
    fn associates(&self, rhs: Self) -> bool;
}

/// A unique factorisation domain.
///
/// Every element has a unique prime factorisation (up to the order and taking associates).
pub trait UniqueFactorisationDomain: IntegralDomain {}

pub trait PrincipalIdealDomain: UniqueFactorisationDomain {}

/// A GCD Domain, a domain with a greatest common divisor function.
pub trait GCDDomain: PrincipalIdealDomain {
    /// Calculate the gcd of the element and another element `b`.
    fn gcd(&self, b: Self) -> Self;
}

pub trait EuclideanDomain: PrincipalIdealDomain {
    fn valuation(&self) -> usize;
}

/// A field.
///
/// There are a few different equivalent definitions. One states that a field is a commutative
/// [`DivisionRing`]. Another states that it is a [`EuclideanDomain`] where multiplication forms an
/// [`AbelianGroup`] (excluding zero), or in other words, a commutative ring where there exists a
/// multilplicative inverse for all elements *except* zero!
///
/// We can achieve this by adding this axiom to our commutative ring axioms:
///
/// * (M4) Multiplicative inverses exist for all nonzero elements, ∃ *x'* s.t. *xx'* = 0 = *x'x*.
///
/// Notice that this definition leaves implicit that a field must contain two different elements.
/// Specifically, [`Ring::ZERO`] and [`Ring::ONE`] must be distinct.
///
/// Since multiplication forms an abelian group for all elements *except* zero, the only time when
/// the [`Ring::inverse_mul`] function returns [`None`] is when it is called on zero.
pub trait Field: EuclideanDomain + DivisionRing {
    /// Given a `*rhs`, returns `self` (`rhs`⁻¹). In other words, it calculates `self` / `rhs`.
    /// 
    /// Since an implementation of this was already given in [`DivisionRing`], we can just call that
    /// function.
    ///
    /// # Safety
    ///
    /// It is undefined behaviour if [`Ring::ZERO`] is used as the input! The [`Field::div`] function need only
    /// support nonzero inputs.
    fn div(&self, rhs: Self) -> Self;
}
