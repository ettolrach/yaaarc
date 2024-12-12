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

//! Traits for operators.
//!
//! Defines unary and binary operators which will be used for algebraic structures.

/// A closed unary operator.
///
/// Currently unused, but who knows, maybe that'll change?
pub trait UnaryOperator<Output> {
    fn unary_op(&self) -> Self;
    fn unary_op_assign(&mut self);
}

/// A closed binary operation that's defined for all inputs.
///
/// Formally, for all *x*, *y* ∈ *X* we have *xy* ∈ *X*.
///
/// We require a generic `O` here. This can be any type you wish. By default, we provide two unit
/// structs (these have no fields). [`Plus`] and [`Times`]. These two are used in traits like
/// [`crate::ringlike::Ring`] to allow two different binary operators to be implemented for the same type.
///
/// # Example
///
/// ```rust
/// use yaaarc::operators::BinaryOperator;
///
/// // We have to create a new struct here since the trait and type are foreign.
/// struct NewString(String);
///
/// // We don't care about what type we give to BinaryOperator, since we only want one binary
/// // operator on NewString.
/// impl BinaryOperator<()> for NewString {
/// 	fn op(&self, rhs: Self) -> Self {
/// 		NewString(format!("{}{}", self.0, rhs.0))
/// 	}
///     fn op_assign(&mut self, rhs: Self) {
///         self.0.push_str(rhs.0.as_str());
///     }
/// }
/// ```
pub trait BinaryOperator<O> {
    fn op(&self, rhs: Self) -> Self;
    fn op_assign(&mut self, rhs: Self);
}

/// A unit struct representing an additive operation, primarily used for [`crate::ringlike::Ring`].
pub struct Plus;

/// A unit struct representing a multiplicative operation, primarily used for [`crate::ringlike::Ring`].
pub struct Times;
