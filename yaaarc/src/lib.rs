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

//! Yet Another Abstract Algebra Rust Crate.
//!
//! I have some free time so I want to try to implement my own abstract algebra library with the
//! eventual goal of writing an algorithm to compute
//! [Gröbner basis](https://en.wikipedia.org/wiki/Gr%C3%B6bner_basis).
//!
//! Throughout, we will be referring to "sets". By that, we mean types. Specifically, the difference
//! between sets and types doesn't matter here. A type which can implement a ring will still model a
//! set.

pub mod grouplike;
pub mod latticelike;
pub mod operators;
pub mod ringlike;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
