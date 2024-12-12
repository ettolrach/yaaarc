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

//! Group-like algebraic structures. These structures have just one closed binary operator.
//!
//! Note that all of these structures are *total*, meaning that their binary operator is defined
//! for all inputs.
//!
//! # Safety
//!
//! A lot of these properties, namely associativity and commutativity, are not guaranteed at the
//! type level! For that we would need dependent types. Thus, if, say, a [`Monoid`] is not
//! associative and is passed into a function which expects a [`Monoid`], then the function may
//! panic.
use crate::operators::BinaryOperator;

/// A set which has a closed [`BinaryOperator`] that is defined for all inputs.
pub trait Magma<O>: BinaryOperator<O> {}

/// A [`Magma`] with associativity.
///
/// Formally, for all *x*, *y*, *z* ∈ *X*, we have *(xy)z* = *x(yz)*.
///
/// # Safety
///
/// Associativity is not guaranteed at the type level! See the note at the beginning of this module
/// for more details.
///
/// # Example
///
/// ```rust
/// use yaaarc::{operators::BinaryOperator, grouplike::{Magma, Semigroup}};
///
/// // The empty type.
/// enum Empty {}
///
/// impl BinaryOperator<()> for Empty {
///     fn op(&self, rhs: Self) -> Self {
///         panic!("Can never be called.")
///     }
///     fn op_assign(&mut self, rhs: Self) {
///         panic!("Can never be called.")
///     }
/// }
///
/// impl Magma<()> for Empty {}
/// impl Semigroup<()> for Empty {}
pub trait Semigroup<O>: Magma<O> {}

/// A [`Magma`] with invertibility.
///
/// Formally, for all *x* ∈ *X*, there exists a *y* ∈ *X* such that *xy* = *yx* = *i*, where *i* is
/// the identity in *X*.
///
/// # Example
///
/// The integers with subtraction form a quasigroup. We'll panic if we exceed [`isize::MAX`] or
/// [`isize::MIN`].
///
/// ```rust
/// use yaaarc::{operators::BinaryOperator, grouplike::{Magma, Quasigroup}};
///
/// struct Z(isize);
///
/// impl BinaryOperator<()> for Z {
///     fn op(&self, rhs: Self) -> Self {
///         Z(self.0 - rhs.0)
///     }
///     fn op_assign(&mut self, rhs: Self) {
///         self.0 -= rhs.0;
///     }
/// }
///
/// impl Magma<()> for Z {}
/// impl Quasigroup<()> for Z {
///     fn inverse(&self) -> Self {
///         Z(-self.0)
///     }
/// }
/// ```
pub trait Quasigroup<O>: Magma<O> {
    /// Finds the inverse for the `self` element in the set.
    fn inverse(&self) -> Self;
}

/// A [`Magma`] with identity.
///
/// Formally, there exists an *i* ∈ *X* such that for all *x* ∈ *X*, we have *xi* = *ix* = *x*.
pub trait UnitalMagma<O>: Magma<O> {
    const IDENTITY: Self;
}

/// A commutative [`Magma`].
///
/// Formally, for all *x*, *y* ∈ *X*, we have *xy* = *yx*.
///
/// Commutativity is not guaranteed at the type level! See the note at the beginning of this module
/// for more details.
pub trait CommutativeMagma<O>: Magma<O> {}

/// A [`Semigroup`] with identity. That is, a [`Semigroup`] that is also a [`UnitalMagma`].
///
/// # Example
///
/// From maths, we know that the naturals with addition form a monoid. So let's implement
/// semigroups for [`usize`] with the expectation that if we exceed [`usize::MAX`], we panic. We
/// could alternatively use a bigint struct and instead panic when we run out of memory.
///
/// ```rust
/// use yaaarc::{operators::BinaryOperator, grouplike::{Magma, Semigroup, UnitalMagma, Monoid}};
///
/// struct Nat(usize);
///
/// impl BinaryOperator<()> for Nat {
///     fn op(&self, rhs: Self) -> Self {
///         Nat(self.0 + rhs.0)
///     }
///     fn op_assign(&mut self, rhs: Self) {
///         self.0 += rhs.0;
///     }
/// }
///
/// impl Magma<()> for Nat {}
/// impl Semigroup<()> for Nat {}
/// impl UnitalMagma<()> for Nat {
///     const IDENTITY: Self = Self(0);
/// }
///
/// impl Monoid<()> for Nat {}
/// ```
pub trait Monoid<O>: Semigroup<O> + UnitalMagma<O> {}

/// An associative [`Semigroup`]. That is, a [`Semigroup`] that's also a [`Quasigroup`].
pub trait AssociativeQuasigroup<O>: Semigroup<O> + Quasigroup<O> {}

/// A [`Quasigroup`] that has an identity. That is, a [`Quasigroup`] that's also a [`UnitalMagma`].
pub trait Loop<O>: Quasigroup<O> + UnitalMagma<O> {}

/// A [`Monoid`] that is also commutative.
pub trait CommutativeMonoid<O>: Semigroup<O> + UnitalMagma<O> + CommutativeMagma<O> {}

/// A [`Monoid`] with invertibility. Alternatively, a structure which is a [`Semigroup`],
/// [`UnitalMagma`], and [`Quasigroup`].
///
/// Formally, for all *x* ∈ *X*, there exists a *y* ∈ *X* such that *xy* = *yx* = *i*, where *i* is
/// the identity in *X*.
pub trait Group<O>: Semigroup<O> + UnitalMagma<O> + Quasigroup<O> {}

/// A commutative [`Group`].
///
/// # Example
///
/// The integers with addition form an abelian group. We'll panic if we exceed [`isize::MAX`] or
/// [`isize::MIN`].
///
/// ```rust
/// use yaaarc::{
///     operators::BinaryOperator,
///     grouplike::{
///         AbelianGroup,
///         CommutativeMagma,
///         Magma,
///         Quasigroup,
///         Semigroup,
///         UnitalMagma,
///     },
/// };
///
/// struct Z(isize);
///
/// // For this example, we can just use the unit type as the generic because we don't intend to
/// // use it for rings or lattices. But if we did, we would provide a more meaningful type here.
/// impl BinaryOperator<()> for Z {
///     fn op(&self, rhs: Self) -> Self {
///         Z(self.0 - rhs.0)
///     }
///     fn op_assign(&mut self, rhs: Self) {
///         self.0 -= rhs.0;
///     }
/// }
///
/// impl Magma<()> for Z {}
/// impl Quasigroup<()> for Z {
///     fn inverse(&self) -> Self {
///         Z(-self.0)
///     }
/// }
/// impl Semigroup<()> for Z {}
/// impl UnitalMagma<()> for Z {
///     const IDENTITY: Self = Z(0);
/// }
/// impl CommutativeMagma<()> for Z {}
/// impl AbelianGroup<()> for Z {}
/// ```
pub trait AbelianGroup<O>:
    Semigroup<O> + UnitalMagma<O> + Quasigroup<O> + CommutativeMagma<O>
{
}
