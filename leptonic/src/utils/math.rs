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

#[cfg(test)]
mod tests {
    use super::percentage_in_range;
    use super::value_in_range;

    #[test]
    fn test_simple_range() {
        assert_eq!(0.75, percentage_in_range(0.0, 100.0, 75.0));
        assert_eq!(75.0, value_in_range(0.0, 100.0, 0.75));
    }

    #[test]
    fn test_min() {
        assert_eq!(0.0, percentage_in_range(50.0, 100.0, 50.0));
        assert_eq!(50.0, value_in_range(50.0, 100.0, 0.0));
    }

    #[test]
    fn test_max() {
        assert_eq!(1.0, percentage_in_range(50.0, 100.0, 100.0));
        assert_eq!(100.0, value_in_range(50.0, 100.0, 1.0));
    }

    #[test]
    fn test_range_negative_to_positive_skewed() {
        assert_eq!(0.625, percentage_in_range(-20.0, 12.0, 0.0));
        assert_eq!(0.0, value_in_range(-20.0, 12.0, 0.625));
    }
}
