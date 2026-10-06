// Upstream: react-aria/intl/table/en-US.json @ 99e6102368
//! The texts tables announce (English; localization comes with the localized strings
//! infrastructure, see PLAN.md).

/// The value of a column resizer.
pub(crate) fn column_size(pixels: f64) -> String {
    format!("{pixels} pixels")
}

pub(crate) const RESIZER_DESCRIPTION: &str = "Press Enter to start resizing";

/// The default label of a column resizer (react-aria-components' `tableResizer`).
pub(crate) const RESIZER: &str = "Resizer";
