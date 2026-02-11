use leptos::attr;
use leptos::attr::Attr;

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_table_body` hook.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseTableBodyInput {}

/// The return value of the `use_table_body` hook.
pub struct UseTableBodyReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub body_props: UseTableBodyProps,
}

/// Props from `use_table_body` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseTableBodyProps {
    pub role: &'static str,
}

impl UseTableBodyProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTableBodyAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTableBodyAttrs {
        (Attr(attr::Role, self.role),)
    }
}

/// Attributes for the table body element.
pub type UseTableBodyAttrs = (Attr<attr::Role, &'static str>,);

/// Provides the behavior and accessibility for a table body group.
///
/// # Example
///
/// ```ignore
/// let body = use_table_body(UseTableBodyInput::default());
///
/// view! {
///     <tbody {..body.body_props.into_attrs()}>
///         // Table rows...
///     </tbody>
/// }
/// ```
pub fn use_table_body(_input: UseTableBodyInput) -> UseTableBodyReturn {
    UseTableBodyReturn {
        body_props: UseTableBodyProps { role: "rowgroup" },
    }
}
