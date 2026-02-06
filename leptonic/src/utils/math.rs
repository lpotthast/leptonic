pub(crate) fn percentage_in_range(min: f64, max: f64, value: f64) -> f64 {
    (value - min) / (max - min)
}

pub(crate) fn value_in_range(min: f64, max: f64, percentage: f64) -> f64 {
    (max - min).mul_add(percentage, min)
}

/// Rounds to the nearest possible step value if a step is provided.
pub(crate) fn project_into_range(value: f64, range: f64, min: f64, step: Option<f64>) -> f64 {
    let projected = value.mul_add(range, min);
    match step {
        Some(step) => (projected / step).round() * step,
        None => projected,
    }
}

/// Returns the number of decimal places in a floating-point value.
///
/// Used to precompute precision once, avoiding repeated string allocations
/// in hot paths like slider dragging.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn decimal_precision(value: f64) -> u32 {
    let s = value.to_string();
    s.find('.').map_or(0, |i| (s.len() - i - 1) as u32)
}

/// Rounds a value to the specified decimal precision.
///
/// Floating-point arithmetic (IEEE 754) cannot represent some decimals exactly,
/// causing errors like `0.1 + 0.1 + 0.1 = 0.30000000000000004`. When calculating
/// `min + steps * step`, these errors accumulate and produce ugly display values.
/// Use `decimal_precision()` to compute precision once, then pass it here.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn round_to_precision(value: f64, precision: u32) -> f64 {
    if precision > 0 {
        let pow = 10_f64.powi(precision as i32);
        (value * pow).round() / pow
    } else {
        value
    }
}

/// Snaps a value to the nearest step within a range.
///
/// Pass `precision` from `decimal_precision(step)` to avoid repeated string allocations.
#[must_use]
pub fn snap_value_to_step(value: f64, min: f64, max: f64, step: f64, precision: u32) -> f64 {
    let steps = ((value - min) / step).round();
    let snapped = round_to_precision(min + steps * step, precision);

    if snapped < min {
        min
    } else if snapped > max {
        // Snap to highest valid step within range
        let max_steps = ((max - min) / step).floor();
        round_to_precision(min + max_steps * step, precision)
    } else {
        snapped
    }
}

/// Calculate page size: (max-min)/10, snapped to step, minimum is step.
/// This follows react-aria's behavior for slider keyboard navigation.
#[must_use]
pub fn calculate_page_size(min: f64, max: f64, step: f64) -> f64 {
    let raw = (max - min) / 10.0;
    let precision = decimal_precision(step);
    // Snap to step (using 0 as min, raw+step as max to allow full range snapping)
    let snapped = snap_value_to_step(raw, 0.0, raw + step, step, precision);
    snapped.max(step)
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::{
        calculate_page_size, decimal_precision, percentage_in_range, round_to_precision,
        snap_value_to_step, value_in_range,
    };

    #[test]
    fn test_simple_range() {
        assert_that(percentage_in_range(0.0, 100.0, 75.0)).is_equal_to(0.75);
        assert_that(value_in_range(0.0, 100.0, 0.75)).is_equal_to(75.0);
    }

    #[test]
    fn test_min() {
        assert_that(percentage_in_range(50.0, 100.0, 50.0)).is_equal_to(0.0);
        assert_that(value_in_range(50.0, 100.0, 0.0)).is_equal_to(50.0);
    }

    #[test]
    fn test_max() {
        assert_that(percentage_in_range(50.0, 100.0, 100.0)).is_equal_to(1.0);
        assert_that(value_in_range(50.0, 100.0, 1.0)).is_equal_to(100.0);
    }

    #[test]
    fn test_range_negative_to_positive_skewed() {
        assert_that(percentage_in_range(-20.0, 12.0, 0.0)).is_equal_to(0.625);
        assert_that(value_in_range(-20.0, 12.0, 0.625)).is_equal_to(0.0);
    }

    #[test]
    fn test_decimal_precision() {
        // Integer values (no decimal point in string representation)
        assert_that(decimal_precision(1.0)).is_equal_to(0);
        assert_that(decimal_precision(10.0)).is_equal_to(0);

        // Decimal values
        assert_that(decimal_precision(0.1)).is_equal_to(1);
        assert_that(decimal_precision(0.01)).is_equal_to(2);
        assert_that(decimal_precision(0.001)).is_equal_to(3);
        assert_that(decimal_precision(1.5)).is_equal_to(1);
    }

    #[test]
    fn test_round_to_precision() {
        // Fixes floating-point errors
        assert_that(round_to_precision(0.30000000000000004, 1)).is_equal_to(0.3);
        assert_that(round_to_precision(0.123456, 2)).is_equal_to(0.12);
        assert_that(round_to_precision(0.125, 2)).is_equal_to(0.13);

        // Precision 0 returns value unchanged (no rounding)
        assert_that(round_to_precision(1.5, 0)).is_equal_to(1.5);
        assert_that(round_to_precision(47.3, 0)).is_equal_to(47.3);
    }

    #[test]
    fn test_snap_value_to_step() {
        // Basic snapping
        assert_that(snap_value_to_step(47.0, 0.0, 100.0, 10.0, 0)).is_equal_to(50.0);
        assert_that(snap_value_to_step(44.0, 0.0, 100.0, 10.0, 0)).is_equal_to(40.0);

        // Snapping with min offset
        assert_that(snap_value_to_step(52.0, 5.0, 95.0, 10.0, 0)).is_equal_to(55.0);

        // Clamping to range when max aligns with step
        assert_that(snap_value_to_step(150.0, 0.0, 100.0, 10.0, 0)).is_equal_to(100.0);
        assert_that(snap_value_to_step(-50.0, 0.0, 100.0, 10.0, 0)).is_equal_to(0.0);

        // Clamping to highest valid step when max doesn't align
        assert_that(snap_value_to_step(150.0, 0.0, 97.0, 10.0, 0)).is_equal_to(90.0);

        // Fine steps with precision
        let precision = decimal_precision(0.1);
        assert_that(snap_value_to_step(0.123, 0.0, 1.0, 0.1, precision)).is_equal_to(0.1);
        assert_that(snap_value_to_step(0.156, 0.0, 1.0, 0.1, precision)).is_equal_to(0.2);

        // Floating-point precision fix: 0.3 should equal exactly 0.3
        assert_that(snap_value_to_step(0.3, 0.0, 1.0, 0.1, precision)).is_equal_to(0.3);
    }

    #[test]
    fn test_calculate_page_size() {
        // Standard 0-100 range with step 1: (100-0)/10 = 10, snapped to 1 = 10
        assert_that(calculate_page_size(0.0, 100.0, 1.0)).is_equal_to(10.0);

        // 0-100 range with step 5: (100-0)/10 = 10, snapped to 5 = 10
        assert_that(calculate_page_size(0.0, 100.0, 5.0)).is_equal_to(10.0);

        // 0-100 range with step 7: (100-0)/10 = 10, snapped to 7 = 7
        assert_that(calculate_page_size(0.0, 100.0, 7.0)).is_equal_to(7.0);

        // Small range: (10-0)/10 = 1, with step 0.5, snapped = 1.0
        assert_that(calculate_page_size(0.0, 10.0, 0.5)).is_equal_to(1.0);

        // Very large step: page size should be at least step
        assert_that(calculate_page_size(0.0, 100.0, 50.0)).is_equal_to(50.0);
    }
}
