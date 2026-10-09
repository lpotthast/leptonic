// Upstream: react-stately/src/utils/number.ts @ 99e6102368
// Upstream: react-stately/test/utils/number.test.ts @ 99e6102368
//
// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `snap_value_to_step` takes the bounds as `Option`s (react-aria: `number | undefined`).
// - `clamp` is Rust's `f64::clamp`.
//
// ## ADDITIONS
// - `percentage_in_range`, and `handle_decimal_operation` (react-aria: inside
//   `useNumberFieldState`).
//
// =============================================================================

pub(crate) fn percentage_in_range(min: f64, max: f64, value: f64) -> f64 {
    let range = max - min;
    if range == 0.0 {
        0.0
    } else {
        (value - min) / range
    }
}

/// The number of digits after the decimal point of `value`'s shortest representation.
#[allow(clippy::cast_possible_truncation)]
fn decimal_precision(value: f64) -> u32 {
    let s = value.to_string();
    s.find('.').map_or(0, |i| (s.len() - i - 1) as u32)
}

/// `value` rounded to one digit more than `step` has (react-aria's `roundToStepPrecision`):
/// removes floating-point noise such as `0.30000000000000004`. Rust prints no exponents, so the
/// decimal point gives the precision.
#[must_use]
pub fn round_to_step_precision(value: f64, step: f64) -> f64 {
    let step_string = step.to_string();
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let precision = step_string
        .find('.')
        .map_or(0, |point| (step_string.len() - point) as i32);
    if precision > 0 {
        let pow = 10_f64.powi(precision);
        (value * pow).round() / pow
    } else {
        value
    }
}

/// `value` snapped to the closest multiple of `step` from `min` (or 0), kept within `min` and
/// `max` (react-aria's `snapValueToStep`; above `max`, the last step not past it). A step that
/// isn't positive and finite only clamps.
#[must_use]
pub fn snap_value_to_step(value: f64, min: Option<f64>, max: Option<f64>, step: f64) -> f64 {
    if step <= 0.0 || !step.is_finite() {
        let value = min.map_or(value, |min| value.max(min));
        return max.map_or(value, |max| value.min(max));
    }
    let remainder = (value - min.unwrap_or(0.0)) % step;
    let mut snapped = round_to_step_precision(
        if remainder.abs() * 2.0 >= step {
            value + remainder.signum() * (step - remainder.abs())
        } else {
            value - remainder
        },
        step,
    );
    match (min, max) {
        (Some(min), _) if snapped < min => snapped = min,
        (Some(min), Some(max)) if snapped > max => {
            snapped = round_to_step_precision((max - min) / step, step)
                .floor()
                .mul_add(step, min);
        }
        (None, Some(max)) if snapped > max => {
            snapped = round_to_step_precision(max / step, step).floor() * step;
        }
        _ => {}
    }
    round_to_step_precision(snapped, step)
}

/// `value` rounded to `digits` decimal digits (react-aria's `toFixedNumber`, base 10).
#[must_use]
pub fn to_fixed_number(value: f64, digits: i32) -> f64 {
    let pow = 10_f64.powi(digits);
    (value * pow).round() / pow
}

/// An operation of [`handle_decimal_operation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecimalOperation {
    Add,
    Subtract,
}

/// Performs a decimal arithmetic operation (add or subtract) with precision handling.
///
/// Avoids floating-point errors by computing in integer space when possible.
/// For example, `handle_decimal_operation(DecimalOperation::Add, 0.1, 0.2)` returns `0.3`
/// instead of the naive `0.30000000000000004`.
///
/// react-aria's `handleDecimalOperation` (`useNumberFieldState.ts`, with the operator as a
/// `'+' | '-'` string).
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn handle_decimal_operation(op: DecimalOperation, value1: f64, value2: f64) -> f64 {
    let precision = decimal_precision(value1).max(decimal_precision(value2));
    let multiplier = 10_f64.powi(precision as i32);

    let int1 = (value1 * multiplier).round();
    let int2 = (value2 * multiplier).round();

    let result = match op {
        DecimalOperation::Add => int1 + int2,
        DecimalOperation::Subtract => int1 - int2,
    };

    result / multiplier
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::{
        DecimalOperation, handle_decimal_operation, round_to_step_precision, snap_value_to_step,
        to_fixed_number,
    };

    #[test]
    fn round_to_step_precision_removes_floating_point_noise() {
        assert_that!(round_to_step_precision(0.300_000_000_000_000_04, 0.1)).is_equal_to(0.3);
        assert_that!(round_to_step_precision(0.123_456, 0.1)).is_equal_to(0.12);
        assert_that!(round_to_step_precision(47.3, 1.0)).is_equal_to(47.3);
    }

    #[test]
    fn snap_value_to_step_snaps_and_clamps() {
        assert_that!(snap_value_to_step(47.0, Some(0.0), Some(100.0), 10.0)).is_equal_to(50.0);
        assert_that!(snap_value_to_step(44.0, Some(0.0), Some(100.0), 10.0)).is_equal_to(40.0);
        // Relative to the minimum.
        assert_that!(snap_value_to_step(52.0, Some(5.0), Some(95.0), 10.0)).is_equal_to(55.0);
        assert_that!(snap_value_to_step(150.0, Some(0.0), Some(100.0), 10.0)).is_equal_to(100.0);
        assert_that!(snap_value_to_step(-50.0, Some(0.0), Some(100.0), 10.0)).is_equal_to(0.0);
        // Above a maximum off the step grid: the last step below it.
        assert_that!(snap_value_to_step(150.0, Some(0.0), Some(97.0), 10.0)).is_equal_to(90.0);
        assert_that!(snap_value_to_step(0.123, Some(0.0), Some(1.0), 0.1)).is_equal_to(0.1);
        assert_that!(snap_value_to_step(0.156, Some(0.0), Some(1.0), 0.1)).is_equal_to(0.2);
        assert_that!(snap_value_to_step(0.3, Some(0.0), Some(1.0), 0.1)).is_equal_to(0.3);
        // Without bounds.
        assert_that!(snap_value_to_step(7.0, None, None, 5.0)).is_equal_to(5.0);
        assert_that!(snap_value_to_step(23.0, None, Some(22.0), 5.0)).is_equal_to(20.0);
        // A step that isn't positive only clamps.
        assert_that!(snap_value_to_step(7.3, Some(0.0), Some(5.0), 0.0)).is_equal_to(5.0);
    }

    // number.test.ts: "should return the input unchanged for integer steps", "should round to
    // the correct decimal places for steps with decimals", "should handle rounding for
    // exponential step values".
    #[test]
    fn round_to_step_precision_like_react_aria() {
        assert_that!(round_to_step_precision(7.123, 1.0)).is_equal_to(7.123);
        assert_that!(round_to_step_precision(5.0, 10.0)).is_equal_to(5.0);
        assert_that!(round_to_step_precision(1.24, 0.1)).is_equal_to(1.24);
        assert_that!(round_to_step_precision(1.456, 0.01)).is_equal_to(1.456);
        assert_that!(round_to_step_precision(1.2345, 0.015)).is_equal_to(1.2345);
        assert_that!(round_to_step_precision(1.2345, 0.25)).is_equal_to(1.235);
        assert_that!(round_to_step_precision(2.349, 0.1)).is_equal_to(2.35);
        assert_that!(round_to_step_precision(2.35, 0.1)).is_equal_to(2.35);
        assert_that!(round_to_step_precision(-1.456, 0.01)).is_equal_to(-1.456);
        assert_that!(round_to_step_precision(0.0, 0.01)).is_equal_to(0.0);
        assert_that!(round_to_step_precision(0.123_456_789, 1e-3)).is_equal_to(0.1235);
        assert_that!(round_to_step_precision(0.123_456_789, 1e-7)).is_equal_to(0.123_456_79);
        assert_that!(round_to_step_precision(0.123_456_789, 1.5e-7)).is_equal_to(0.123_456_789);
        assert_that!(round_to_step_precision(0.123_456_789, 2.5e-6)).is_equal_to(0.123_456_79);
        assert_that!(round_to_step_precision(0.123_456_789, 1e-8)).is_equal_to(0.123_456_789);
    }

    // number.test.ts: "should snap value to nearest step based on min and max", "should snap
    // value nearest step when min or max are undefined".
    #[test]
    fn snap_value_to_step_like_react_aria() {
        assert_that!(snap_value_to_step(2.0, Some(-0.5), Some(100.0), 3.0)).is_equal_to(2.5);
        assert_that!(snap_value_to_step(-6.2, Some(-2.5), Some(100.0), 3.0)).is_equal_to(-2.5);
        assert_that!(snap_value_to_step(106.2, Some(-2.5), Some(100.0), 3.0)).is_equal_to(99.5);
        assert_that!(snap_value_to_step(-0.009_999, Some(-0.5), Some(0.5), 0.01))
            .is_equal_to(-0.01);
        assert_that!(snap_value_to_step(-8.0, Some(-100.0), Some(100.0), 5.0)).is_equal_to(-10.0);
        assert_that!(snap_value_to_step(-6.0, Some(-100.0), Some(100.0), 5.0)).is_equal_to(-5.0);
        assert_that!(snap_value_to_step(3.0, Some(-100.0), Some(100.0), 5.0)).is_equal_to(5.0);
        assert_that!(snap_value_to_step(2.0, Some(-100.0), Some(100.0), 5.0)).is_equal_to(0.0);
        assert_that!(snap_value_to_step(2.0, None, None, 3.0)).is_equal_to(3.0);
        assert_that!(snap_value_to_step(6.0, None, Some(5.0), 3.0)).is_equal_to(3.0);
        assert_that!(snap_value_to_step(4.0, None, Some(5.0), 3.0)).is_equal_to(3.0);
        assert_that!(snap_value_to_step(1.0, Some(3.0), None, 3.0)).is_equal_to(3.0);
    }

    #[test]
    fn to_fixed_number_rounds_to_digits() {
        assert_that!(to_fixed_number(33.333_333, 2)).is_equal_to(33.33);
        assert_that!(to_fixed_number(0.123_456, 4)).is_equal_to(0.1235);
    }

    #[test]
    fn test_handle_decimal_operation_add() {
        assert_that!(handle_decimal_operation(DecimalOperation::Add, 0.1, 0.2)).is_equal_to(0.3);
        assert_that!(handle_decimal_operation(DecimalOperation::Add, 0.01, 0.02)).is_equal_to(0.03);
        assert_that!(handle_decimal_operation(DecimalOperation::Add, 1.0, 2.0)).is_equal_to(3.0);
        assert_that!(handle_decimal_operation(DecimalOperation::Add, 1.0, 0.1)).is_equal_to(1.1);
        assert_that!(handle_decimal_operation(
            DecimalOperation::Add,
            1_000_000.001,
            0.002
        ))
        .is_equal_to(1_000_000.003);
    }

    #[test]
    fn test_handle_decimal_operation_subtract() {
        assert_that!(handle_decimal_operation(
            DecimalOperation::Subtract,
            0.3,
            0.1
        ))
        .is_equal_to(0.2);
        assert_that!(handle_decimal_operation(
            DecimalOperation::Subtract,
            0.1,
            0.3
        ))
        .is_equal_to(-0.2);
    }
}
