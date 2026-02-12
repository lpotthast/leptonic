use leptos::attr;
use leptos::attr::Attr;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/grid/src/useGridRowGroup.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## OMITTED FEATURES
// - `isVirtualized` — no virtualization support.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Returns a props struct with `into_attrs()` instead of React props.
//
// =============================================================================

/// Return value for a grid row group.
pub struct UseGridRowGroupReturn {
    /// Props for the row group element.
    pub props: UseGridRowGroupProps,
}

/// Props from `use_grid_row_group` that can be extracted and merged programmatically.
#[derive(Debug, Clone, Copy)]
pub struct UseGridRowGroupProps {
    pub role: &'static str,
}

impl UseGridRowGroupProps {
    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseGridRowGroupAttrs {
        Attr(attr::Role, self.role)
    }
}

/// Attributes for a grid row group.
pub type UseGridRowGroupAttrs = Attr<attr::Role, &'static str>;

/// Provides the behavior and accessibility for a grid row group.
///
/// A grid row group is a structural container (e.g., `<tbody>`) that groups
/// grid rows together. It sets `role="rowgroup"` for ARIA compliance.
///
/// # Example
///
/// ```ignore
/// let row_group = use_grid_row_group();
///
/// view! {
///     <div {..row_group.props.into_attrs()}>
///         // Grid rows...
///     </div>
/// }
/// ```
#[must_use]
pub fn use_grid_row_group() -> UseGridRowGroupReturn {
    UseGridRowGroupReturn {
        props: UseGridRowGroupProps { role: "rowgroup" },
    }
}
