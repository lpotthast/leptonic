//! A share of a whole, from 0 to 1 (convention C13: the unit is in the type).

/// A share of a whole: 0 (nothing) to 1 (all), e.g. a progress bar's progress through its range.
/// Construction clamps; NaN becomes 0.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Fraction(f64);

impl Fraction {
    /// Nothing: 0.
    pub const ZERO: Self = Self(0.0);
    /// All: 1.
    pub const ONE: Self = Self(1.0);

    /// `value`, clamped to 0..=1 (NaN: 0).
    #[must_use]
    pub fn new(value: f64) -> Self {
        if value.is_nan() {
            Self::ZERO
        } else {
            Self(value.clamp(0.0, 1.0))
        }
    }

    /// The share, from 0 to 1.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// The share in percent, from 0 to 100 (e.g. for a CSS width).
    #[must_use]
    pub fn as_percent(self) -> f64 {
        self.0 * 100.0
    }
}

impl From<Fraction> for f64 {
    fn from(fraction: Fraction) -> Self {
        fraction.get()
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn clamps_and_converts() {
        assert_that!(Fraction::new(0.25).as_percent()).is_equal_to(25.0);
        assert_that!(Fraction::new(1.5)).is_equal_to(Fraction::ONE);
        assert_that!(Fraction::new(-1.0)).is_equal_to(Fraction::ZERO);
        assert_that!(Fraction::new(f64::NAN)).is_equal_to(Fraction::ZERO);
    }
}
