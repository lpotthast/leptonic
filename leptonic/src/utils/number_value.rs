// Upstream: react-stately/src/utils/number.ts @ 99e6102368
//! Numeric value types of number fields: all primitive integers and floats.

use std::fmt::{Debug, Display};

use fixed_decimal::Decimal;
use leptos::prelude::*;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Generic over the value type (react-aria: JS `number`, an `f64`). Reason: integers are stepped
//   and clamped exactly, also beyond 2^53 and for 128-bit types, and a field's bounds default to
//   its type's. `None` replaces `NaN` for "no value".
//
// =============================================================================

/// A number a number field can hold: every primitive integer and float type.
///
/// Integers are exact (checked arithmetic, saturating at the type's bounds); floats follow
/// react-aria's rounding to the step's precision.
pub trait NumberValue:
    Copy + PartialEq + PartialOrd + Debug + Display + Send + Sync + 'static
{
    /// The type's smallest and largest value, for integers. Floats are unbounded.
    const BOUNDS: Option<(Self, Self)>;
    const ZERO: Self;
    const ONE: Self;
    /// Whether the type has no fraction digits.
    const IS_INTEGER: bool;

    /// `None` on overflow.
    fn checked_add(self, rhs: Self) -> Option<Self>;
    /// `None` on overflow.
    fn checked_sub(self, rhs: Self) -> Option<Self>;

    /// The value rounded to the nearest multiple of `step` from `min` (or zero), then clamped to
    /// `min..=max` (react-aria's `snapValueToStep`). A non-positive step only clamps.
    #[must_use]
    fn snap_to_step(self, min: Option<Self>, max: Option<Self>, step: Self) -> Self;

    /// The exact decimal value; `None` for infinite and NaN floats.
    fn to_decimal(self) -> Option<Decimal>;

    /// The value of `decimal`, saturating at an integer type's bounds (a typed `300` is a `u8`'s
    /// `255`, which the field then clamps as react-aria clamps a JS number). `None` for fraction
    /// digits in integer types and for decimals beyond a float's range.
    fn from_decimal(decimal: &Decimal) -> Option<Self>;

    /// An approximation as `f64`, for APIs that take one (e.g. `aria-valuenow`).
    fn to_f64(self) -> f64;

    /// The value closest to `value` (rounded for integer types); `None` if the type can't hold it.
    fn from_f64(value: f64) -> Option<Self> {
        let value = if Self::IS_INTEGER {
            value.round()
        } else {
            value
        };
        value
            .to_decimal()
            .and_then(|decimal| Self::from_decimal(&decimal))
    }

    /// The value clamped to `min..=max`.
    #[must_use]
    fn clamp_to(self, min: Option<Self>, max: Option<Self>) -> Self {
        let value = match min {
            Some(min) if self < min => min,
            _ => self,
        };
        match max {
            Some(max) if value > max => max,
            _ => value,
        }
    }

    /// `min`, else the type's lower bound.
    fn lower_bound(min: Option<Self>) -> Option<Self> {
        min.or(Self::BOUNDS.map(|(lower, _)| lower))
    }

    /// `max`, else the type's upper bound.
    fn upper_bound(max: Option<Self>) -> Option<Self> {
        max.or(Self::BOUNDS.map(|(_, upper)| upper))
    }
}

/// The digits of a decimal without fraction digits, and whether it is negative.
fn integer_digits(decimal: &Decimal) -> Option<(String, bool)> {
    let mut decimal = decimal.clone();
    decimal.absolute.trim_end();
    let is_negative = decimal.sign == fixed_decimal::Sign::Negative;
    (decimal.absolute.nonzero_magnitude_end() >= 0 || decimal.absolute.is_zero())
        .then(|| (decimal.to_string(), is_negative))
}

macro_rules! impl_integer {
    ($($t:ty),*) => {$(
        impl NumberValue for $t {
            const BOUNDS: Option<(Self, Self)> = Some((<$t>::MIN, <$t>::MAX));
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const IS_INTEGER: bool = true;

            fn checked_add(self, rhs: Self) -> Option<Self> {
                <$t>::checked_add(self, rhs)
            }

            fn checked_sub(self, rhs: Self) -> Option<Self> {
                <$t>::checked_sub(self, rhs)
            }

            fn snap_to_step(self, min: Option<Self>, max: Option<Self>, step: Self) -> Self {
                if step <= 0 {
                    return self.clamp_to(min, max);
                }
                // `(a - b) mod step` without computing `a - b`, which may overflow.
                let difference_mod = |a: Self, b: Self| {
                    let (a, b) = (a.rem_euclid(step), b.rem_euclid(step));
                    if a >= b { a - b } else { step - (b - a) }
                };
                let base = min.unwrap_or(0);
                // As JavaScript's `%`, the remainder has the sign of `self - base`.
                let below = self < base;
                let remainder = if below {
                    (step - difference_mod(self, base)) % step
                } else {
                    difference_mod(self, base)
                };
                let toward_base = || {
                    if below { self.checked_add(remainder) } else { self.checked_sub(remainder) }
                };
                let snapped = if remainder >= step - remainder {
                    // Halfway or more: away from the base, unless that overflows.
                    let away = if below {
                        self.checked_sub(step - remainder)
                    } else {
                        self.checked_add(step - remainder)
                    };
                    away.or_else(toward_base)
                } else {
                    toward_base()
                }
                .unwrap_or(self);

                match (min, max) {
                    (Some(min), _) if snapped < min => min,
                    // The largest step from `min` not above `max`.
                    (Some(min), Some(max)) if snapped > max => max - difference_mod(max, min),
                    (None, Some(max)) if snapped > max => max - max.rem_euclid(step),
                    _ => snapped,
                }
            }

            fn to_decimal(self) -> Option<Decimal> {
                Some(Decimal::from(self))
            }

            fn from_decimal(decimal: &Decimal) -> Option<Self> {
                let (digits, is_negative) = integer_digits(decimal)?;
                // Only out-of-range values fail to parse: the digits are an integer.
                Some(digits.parse().unwrap_or(if is_negative { <$t>::MIN } else { <$t>::MAX }))
            }

            #[allow(clippy::cast_precision_loss, clippy::cast_lossless)]
            fn to_f64(self) -> f64 {
                self as f64
            }
        }
    )*};
}

impl_integer!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

impl NumberValue for f64 {
    const BOUNDS: Option<(Self, Self)> = None;
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const IS_INTEGER: bool = false;

    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(super::math::handle_decimal_operation(
            super::math::DecimalOperation::Add,
            self,
            rhs,
        ))
        .filter(|sum| sum.is_finite())
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(super::math::handle_decimal_operation(
            super::math::DecimalOperation::Subtract,
            self,
            rhs,
        ))
        .filter(|difference| difference.is_finite())
    }

    fn snap_to_step(self, min: Option<Self>, max: Option<Self>, step: Self) -> Self {
        super::math::snap_value_to_step(self, min, max, step)
    }

    fn to_decimal(self) -> Option<Decimal> {
        // `Display` prints the shortest representation that round-trips, never with an exponent.
        self.is_finite()
            .then(|| Decimal::try_from_str(&self.to_string()).ok())
            .flatten()
    }

    fn from_decimal(decimal: &Decimal) -> Option<Self> {
        decimal
            .to_string()
            .parse()
            .ok()
            .filter(|v: &f64| v.is_finite())
    }

    fn to_f64(self) -> f64 {
        self
    }
}

impl NumberValue for f32 {
    const BOUNDS: Option<(Self, Self)> = None;
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const IS_INTEGER: bool = false;

    fn checked_add(self, rhs: Self) -> Option<Self> {
        f32::from_decimal(&f64::from(self).checked_add(f64::from(rhs))?.to_decimal()?)
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        f32::from_decimal(&f64::from(self).checked_sub(f64::from(rhs))?.to_decimal()?)
    }

    fn snap_to_step(self, min: Option<Self>, max: Option<Self>, step: Self) -> Self {
        // Through the shortest decimal representations, so that `0.1_f32` is 0.1, not 0.10000000149.
        let widen = |value: f32| value.to_string().parse::<f64>().unwrap_or(f64::from(value));
        let snapped = super::math::snap_value_to_step(
            widen(self),
            min.map(widen),
            max.map(widen),
            widen(step),
        );
        snapped
            .to_decimal()
            .and_then(|decimal| f32::from_decimal(&decimal))
            .unwrap_or(self)
    }

    fn to_decimal(self) -> Option<Decimal> {
        self.is_finite()
            .then(|| Decimal::try_from_str(&self.to_string()).ok())
            .flatten()
    }

    fn from_decimal(decimal: &Decimal) -> Option<Self> {
        decimal
            .to_string()
            .parse()
            .ok()
            .filter(|v: &f32| v.is_finite())
    }

    fn to_f64(self) -> f64 {
        f64::from(self)
    }
}

/// A number prop of a component generic over its number type: a number or any signal of one.
///
/// Unlike `#[prop(into)] Signal<T>`, it lets the compiler infer `T` from what is passed
/// (`value=3.0`, `value=rw_signal`): Leptos also converts any value into a `Signal<Option<_>>`,
/// which leaves `T` open. Its conversions accept only [`NumberValue`]s. Closures don't convert
/// (a closure type could also be a number type, as far as the compiler knows): pass
/// `Signal::derive(move || ..)`.
#[derive(Debug, Clone, Copy)]
pub struct NumberSignal<T: NumberValue>(Signal<T>);

impl<T: NumberValue> NumberSignal<T> {
    #[must_use]
    pub fn into_signal(self) -> Signal<T> {
        self.0
    }
}

impl<T: NumberValue> From<T> for NumberSignal<T> {
    fn from(value: T) -> Self {
        Self(Signal::stored(value))
    }
}

impl<T: NumberValue> From<Signal<T>> for NumberSignal<T> {
    fn from(signal: Signal<T>) -> Self {
        Self(signal)
    }
}

impl<T: NumberValue> From<ReadSignal<T>> for NumberSignal<T> {
    fn from(signal: ReadSignal<T>) -> Self {
        Self(signal.into())
    }
}

impl<T: NumberValue> From<RwSignal<T>> for NumberSignal<T> {
    fn from(signal: RwSignal<T>) -> Self {
        Self(signal.into())
    }
}

impl<T: NumberValue> From<Memo<T>> for NumberSignal<T> {
    fn from(memo: Memo<T>) -> Self {
        Self(memo.into())
    }
}

/// An optional number prop of a component generic over its number type: a number, an `Option` of
/// one, or any signal of them. As [`NumberSignal`], it lets the compiler infer `T`.
#[derive(Debug, Clone, Copy)]
pub struct OptionalNumberSignal<T: NumberValue>(Signal<Option<T>>);

impl<T: NumberValue> OptionalNumberSignal<T> {
    #[must_use]
    pub fn into_signal(self) -> Signal<Option<T>> {
        self.0
    }
}

impl<T: NumberValue> From<T> for OptionalNumberSignal<T> {
    fn from(value: T) -> Self {
        Self(Signal::stored(Some(value)))
    }
}

impl<T: NumberValue> From<Option<T>> for OptionalNumberSignal<T> {
    fn from(value: Option<T>) -> Self {
        Self(Signal::stored(value))
    }
}

impl<T: NumberValue> From<Signal<Option<T>>> for OptionalNumberSignal<T> {
    fn from(signal: Signal<Option<T>>) -> Self {
        Self(signal)
    }
}

impl<T: NumberValue> From<ReadSignal<Option<T>>> for OptionalNumberSignal<T> {
    fn from(signal: ReadSignal<Option<T>>) -> Self {
        Self(signal.into())
    }
}

impl<T: NumberValue> From<RwSignal<Option<T>>> for OptionalNumberSignal<T> {
    fn from(signal: RwSignal<Option<T>>) -> Self {
        Self(signal.into())
    }
}

impl<T: NumberValue> From<Memo<Option<T>>> for OptionalNumberSignal<T> {
    fn from(memo: Memo<Option<T>>) -> Self {
        Self(memo.into())
    }
}

impl<T: NumberValue> From<Signal<T>> for OptionalNumberSignal<T> {
    fn from(signal: Signal<T>) -> Self {
        Self(Signal::derive(move || Some(signal.get())))
    }
}

impl<T: NumberValue> From<ReadSignal<T>> for OptionalNumberSignal<T> {
    fn from(signal: ReadSignal<T>) -> Self {
        Self(Signal::derive(move || Some(signal.get())))
    }
}

impl<T: NumberValue> From<RwSignal<T>> for OptionalNumberSignal<T> {
    fn from(signal: RwSignal<T>) -> Self {
        Self(Signal::derive(move || Some(signal.get())))
    }
}

impl<T: NumberValue> From<Memo<T>> for OptionalNumberSignal<T> {
    fn from(memo: Memo<T>) -> Self {
        Self(Signal::derive(move || Some(memo.get())))
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn decimal(s: &str) -> Decimal {
        Decimal::try_from_str(s).unwrap()
    }

    #[test]
    fn integers_snap_like_react_aria() {
        assert_that!(7_i32.snap_to_step(None, None, 5)).is_equal_to(5);
        assert_that!(8_i32.snap_to_step(None, None, 5)).is_equal_to(10);
        // Halfway rounds away from the base (JavaScript `%` keeps the dividend's sign).
        assert_that!((-7_i32).snap_to_step(None, None, 2)).is_equal_to(-8);
        assert_that!((-6_i32).snap_to_step(None, None, 4)).is_equal_to(-8);
        assert_that!(4_i32.snap_to_step(Some(1), None, 2)).is_equal_to(5);
        assert_that!(12_i32.snap_to_step(Some(1), Some(10), 2)).is_equal_to(9);
        assert_that!(12_i32.snap_to_step(None, Some(10), 4)).is_equal_to(8);
        assert_that!((-3_i32).snap_to_step(Some(0), None, 2)).is_equal_to(0);
    }

    #[test]
    fn integers_snap_at_the_type_bounds_without_overflow() {
        assert_that!(u8::MAX.snap_to_step(None, None, 2)).is_equal_to(254);
        assert_that!(i128::MAX.snap_to_step(Some(i128::MIN), None, 10))
            .is_less_or_equal_to(i128::MAX);
        assert_that!(u128::MAX.snap_to_step(None, None, 1)).is_equal_to(u128::MAX);
        assert_that!(i64::MIN.snap_to_step(None, None, 3)).is_greater_or_equal_to(i64::MIN);
        assert_that!(5_u64.snap_to_step(Some(1), Some(u64::MAX), 2)).is_equal_to(5);
    }

    #[test]
    fn floats_snap_to_the_step_precision() {
        assert_that!(0.31_f64.snap_to_step(None, None, 0.1)).is_equal_to(0.3);
        assert_that!(1.04_f64.snap_to_step(Some(0.0), Some(1.0), 0.1)).is_equal_to(1.0);
        assert_that!(0.31_f32.snap_to_step(None, None, 0.1)).is_equal_to(0.3);
    }

    #[test]
    fn integers_are_exact_beyond_f64() {
        let big = u64::MAX - 1;
        assert_that!(big.checked_add(1)).is_equal_to(Some(u64::MAX));
        assert_that!(big.checked_add(2)).is_none();
        assert_that!(u64::from_decimal(&big.to_decimal().unwrap())).is_equal_to(Some(big));
        assert_that!(i128::from_decimal(&decimal(
            "-170141183460469231731687303715884105728"
        )))
        .is_equal_to(Some(i128::MIN));
    }

    #[test]
    fn integers_reject_fractions_and_saturate_out_of_range_values() {
        assert_that!(i32::from_decimal(&decimal("1.5"))).is_none();
        assert_that!(i32::from_decimal(&decimal("2.00"))).is_equal_to(Some(2));
        assert_that!(u8::from_decimal(&decimal("256"))).is_equal_to(Some(255));
        assert_that!(u8::from_decimal(&decimal("-1"))).is_equal_to(Some(0));
        assert_that!(u8::from_decimal(&decimal("0"))).is_equal_to(Some(0));
        assert_that!(i8::from_decimal(&decimal("-1000"))).is_equal_to(Some(-128));
        assert_that!(u128::from_decimal(&decimal(&format!(
            "1{}",
            "0".repeat(40)
        ))))
        .is_equal_to(Some(u128::MAX));
        assert_that!(i64::from_decimal(&decimal("-0"))).is_equal_to(Some(0));
    }

    #[test]
    fn floats_round_trip_through_decimals() {
        assert_that!(0.1_f64.to_decimal().map(|d| d.to_string()))
            .is_equal_to(Some("0.1".to_owned()));
        assert_that!(0.1_f32.to_decimal().map(|d| d.to_string()))
            .is_equal_to(Some("0.1".to_owned()));
        assert_that!(f64::NAN.to_decimal()).is_none();
        assert_that!(f64::from_decimal(&decimal("-12.5"))).is_equal_to(Some(-12.5));
    }

    #[test]
    fn float_arithmetic_avoids_binary_rounding_errors() {
        assert_that!(0.1_f64.checked_add(0.2)).is_equal_to(Some(0.3));
        assert_that!(f64::MAX.checked_add(f64::MAX)).is_none();
    }
}
