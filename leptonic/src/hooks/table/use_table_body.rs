use leptos::{attr, attr::Attr};

use crate::{hooks::IntoAttrs, utils::aria::AriaRole};

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
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub body_props: UseTableBodyProps,
}

/// Props from `use_table_body` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTableBodyProps {
    pub role: AriaRole,
}

impl IntoAttrs for UseTableBodyProps {
    type Attrs = UseTableBodyAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Role, self.role),)
    }
}

/// Attributes for the table body element.
pub type UseTableBodyAttrs = (Attr<attr::Role, AriaRole>,);

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
        body_props: UseTableBodyProps {
            role: AriaRole::Rowgroup,
        },
    }
}
