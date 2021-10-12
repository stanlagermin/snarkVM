// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use std::ops::Not;

pub trait BooleanTrait: Not {}

/// Representation of the zero value.
pub trait Zero {
    type Boolean: BooleanTrait;
    type Output;

    /// Returns a new zero constant.
    fn zero() -> Self;

    /// Returns `true` if `self` is zero.
    fn is_zero(&self) -> Self::Output;
}

/// Representation of the one value.
pub trait One {
    type Boolean: BooleanTrait;
    type Output;

    /// Returns a new one constant.
    fn one() -> Self;

    /// Returns `true` if `self` is one.
    fn is_one(&self) -> Self::Output;
}

/// Trait for equality comparisons.
pub trait Equal<Rhs: ?Sized = Self> {
    type Boolean: BooleanTrait;
    type Output;

    /// Returns `true` if `self` and `other` are equal.
    fn is_eq(&self, other: &Rhs) -> Self::Output;

    /// Returns `true` if `self` and `other` are *not* equal.
    fn is_neq(&self, other: &Rhs) -> Self::Output;
}

/// Binary operator for performing `a AND b`.
pub trait And<Rhs: ?Sized = Self> {
    type Boolean: BooleanTrait;
    type Output;

    fn and(&self, other: &Rhs) -> Self::Output;
}

/// Binary operator for performing `a OR b`.
pub trait Or<Rhs: ?Sized = Self> {
    type Boolean: BooleanTrait;
    type Output;

    fn or(&self, other: &Rhs) -> Self::Output;
}

/// Binary operator for performing `(NOT a) AND (NOT b)`.
pub trait Nor<Rhs: ?Sized = Self> {
    type Boolean: BooleanTrait;
    type Output;

    fn nor(&self, other: &Rhs) -> Self::Output;
}

/// Trait for ternary operations.
pub trait Ternary {
    type Boolean: BooleanTrait;
    type Output;

    fn ternary(condition: &Self::Boolean, a: &Self, b: &Self) -> Self::Output;
}

/// Unary operator for retrieving the doubled value.
pub trait Double {
    type Output;

    fn double(self) -> Self::Output;
}

/// Unary operator for retrieving the squared value.
pub trait Square {
    type Output;

    fn square(&self) -> Self::Output;
}

/// Unary operator for converting to bits.
pub trait ToBits {
    type Boolean: BooleanTrait;

    fn to_bits_le(&self) -> Vec<Self::Boolean>;

    fn to_bits_be(&self) -> Vec<Self::Boolean>;
}
