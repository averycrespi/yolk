use num_traits::cast::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

use crate::error::ParseError;

use std::convert::TryInto;
use std::i64;
use std::str::FromStr;

const SCALE: i128 = 10_000;
const MAX: i128 = i64::MAX as i128;
const MIN: i128 = i64::MIN as i128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YololNumber {
    inner: i128,
}

impl YololNumber {
    pub fn zero() -> Self {
        YololNumber { inner: 0 }
    }

    pub fn one() -> Self {
        YololNumber { inner: SCALE }
    }

    pub fn to_f64(&self) -> f64 {
        (self.inner / SCALE) as f64
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.to_f64())
    }

    pub fn add(&self, other: Self) -> Self {
        YololNumber::from(self.inner.saturating_add(other.inner))
    }

    pub fn sub(&self, other: Self) -> Self {
        YololNumber::from(self.inner.saturating_sub(other.inner))
    }

    pub fn mul(&self, other: Self) -> Self {
        YololNumber::from((self.inner / SCALE).saturating_mul(other.inner / SCALE))
    }

    pub fn div(&self, other: Self) -> Self {
        YololNumber::from((self.inner / SCALE) / (other.inner) / SCALE)
    }

    pub fn modulus(&self, other: Self) -> Self {
        YololNumber::from((self.inner / SCALE) % (other.inner / SCALE))
    }

    pub fn pow(&self, other: Self) -> Self {
        YololNumber::from(
            (self.inner / SCALE).saturating_pow((other.inner / SCALE).try_into().unwrap()),
        )
    }
}

impl From<i128> for YololNumber {
    fn from(value: i128) -> Self {
        if value < MIN {
            YololNumber { inner: MIN }
        } else if value > MAX {
            YololNumber { inner: MAX }
        } else {
            YololNumber { inner: value }
        }
    }
}

impl From<bool> for YololNumber {
    fn from(value: bool) -> Self {
        if value {
            YololNumber::one()
        } else {
            YololNumber::zero()
        }
    }
}

impl FromStr for YololNumber {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Decimal::from_str(s) {
            Ok(decimal) => match (decimal * Decimal::from_i128(SCALE).unwrap()).to_i128() {
                Some(integer) => Ok(YololNumber::from(integer)),
                None => Err(ParseError::BadYololNumber(s.to_string())),
            },
            Err(e) => Err(ParseError::BadYololNumber(e.to_string())),
        }
    }
}
