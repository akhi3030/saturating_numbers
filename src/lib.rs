//! This crate implements a generic type: SaturatingNumber<T> which can be used
//! to define saturating arthmetic on the underlying integer types. It then also
//! exposes SaturatingU32, SaturatingU64, SaturatingU128 type aliases.

use std::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    fmt,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

pub trait HasSaturatingAdd {
    fn do_saturating_add(&self, rhs: Self) -> Self;
}

pub trait HasSaturatingSub {
    fn do_saturating_sub(&self, rhs: Self) -> Self;
}

pub trait HasSaturatingMul {
    fn do_saturating_mul(&self, rhs: Self) -> Self;
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub struct SaturatingNumber<T>(T);

impl<T> From<T> for SaturatingNumber<T> {
    fn from(input: T) -> Self {
        Self(input)
    }
}

impl<T: Add<Output = T> + HasSaturatingAdd> Add for SaturatingNumber<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0.do_saturating_add(rhs.0))
    }
}

impl<T: Add<Output = T> + HasSaturatingAdd> AddAssign for SaturatingNumber<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.do_saturating_add(rhs.0)
    }
}

impl<T: Sub<Output = T> + HasSaturatingSub> Sub for SaturatingNumber<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0.do_saturating_sub(rhs.0))
    }
}

impl<T: Sub<Output = T> + HasSaturatingSub> SubAssign for SaturatingNumber<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.do_saturating_sub(rhs.0)
    }
}

impl<T: Mul<Output = T> + HasSaturatingMul> Mul for SaturatingNumber<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0.do_saturating_mul(rhs.0))
    }
}

impl<T: Mul<Output = T> + HasSaturatingMul> MulAssign for SaturatingNumber<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.0.do_saturating_mul(rhs.0)
    }
}

impl<T: fmt::Debug> fmt::Debug for SaturatingNumber<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl HasSaturatingAdd for u32 {
    fn do_saturating_add(&self, rhs: Self) -> Self {
        self.saturating_add(rhs)
    }
}

impl HasSaturatingSub for u32 {
    fn do_saturating_sub(&self, rhs: Self) -> Self {
        self.saturating_sub(rhs)
    }
}

impl HasSaturatingMul for u32 {
    fn do_saturating_mul(&self, rhs: Self) -> Self {
        self.saturating_mul(rhs)
    }
}

pub type SaturatingU32 = SaturatingNumber<u32>;

impl HasSaturatingAdd for u64 {
    fn do_saturating_add(&self, rhs: Self) -> Self {
        self.saturating_add(rhs)
    }
}

impl HasSaturatingSub for u64 {
    fn do_saturating_sub(&self, rhs: Self) -> Self {
        self.saturating_sub(rhs)
    }
}

impl HasSaturatingMul for u64 {
    fn do_saturating_mul(&self, rhs: Self) -> Self {
        self.saturating_mul(rhs)
    }
}

pub type SaturatingU64 = SaturatingNumber<u64>;

impl HasSaturatingAdd for u128 {
    fn do_saturating_add(&self, rhs: Self) -> Self {
        self.saturating_add(rhs)
    }
}

impl HasSaturatingSub for u128 {
    fn do_saturating_sub(&self, rhs: Self) -> Self {
        self.saturating_sub(rhs)
    }
}

impl HasSaturatingMul for u128 {
    fn do_saturating_mul(&self, rhs: Self) -> Self {
        self.saturating_mul(rhs)
    }
}

pub type SaturatingU128 = SaturatingNumber<u128>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(
            SaturatingU64::from(0) + SaturatingU64::from(0),
            SaturatingU64::from(0)
        );
        assert_eq!(
            SaturatingU64::from(0) + SaturatingU64::from(std::u64::MAX),
            SaturatingU64::from(std::u64::MAX)
        );
        assert_eq!(
            SaturatingU64::from(std::u64::MAX) + SaturatingU64::from(std::u64::MAX),
            SaturatingU64::from(std::u64::MAX)
        );
        assert_eq!(
            SaturatingU64::from(std::u64::MAX) + SaturatingU64::from(10),
            SaturatingU64::from(std::u64::MAX)
        );
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(
            SaturatingU64::from(0) - SaturatingU64::from(0),
            SaturatingU64::from(0)
        );
        assert_eq!(
            SaturatingU64::from(0) - SaturatingU64::from(std::u64::MAX),
            SaturatingU64::from(0)
        );
        assert_eq!(
            SaturatingU64::from(std::u64::MAX) - SaturatingU64::from(std::u64::MAX),
            SaturatingU64::from(0)
        );
        assert_eq!(
            SaturatingU64::from(std::u64::MAX) - SaturatingU64::from(10),
            SaturatingU64::from(std::u64::MAX - 10)
        );
        assert_eq!(
            SaturatingU64::from(0) - SaturatingU64::from(10),
            SaturatingU64::from(0)
        );
        assert_eq!(
            SaturatingU64::from(10) - SaturatingU64::from(20),
            SaturatingU64::from(0)
        );
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(
            SaturatingU64::from(0) * SaturatingU64::from(0),
            SaturatingU64::from(0)
        );
        assert_eq!(
            SaturatingU64::from(0) * SaturatingU64::from(std::u64::MAX),
            SaturatingU64::from(0)
        );
        assert_eq!(
            SaturatingU64::from(std::u64::MAX) * SaturatingU64::from(std::u64::MAX),
            SaturatingU64::from(std::u64::MAX)
        );
        assert_eq!(
            SaturatingU64::from(std::u64::MAX) * SaturatingU64::from(10),
            SaturatingU64::from(std::u64::MAX)
        );
    }
}
