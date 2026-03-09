use leptos::{attr, attr::Attr};

use crate::{hooks::IntoAttrs, utils::aria::AriaRole};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/table/src/useTableRowGroup.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_table_header` hook.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseTableHeaderInput {
    /// Whether the header is for a sticky header.
    pub is_sticky: bool,
}

/// The return value of the `use_table_header` hook.
pub struct UseTableHeaderReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub header_props: UseTableHeaderProps,
}

/// Props from `use_table_header` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTableHeaderProps {
    pub role: AriaRole,
}

impl IntoAttrs for UseTableHeaderProps {
    type Attrs = UseTableHeaderAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Role, self.role),)
    }
}

/// Attributes for the table header element.
pub type UseTableHeaderAttrs = (Attr<attr::Role, AriaRole>,);

/// Provides the behavior and accessibility for a table header group.
///
/// The table header contains column headers and optionally supports sticky positioning.
///
/// # Example
///
/// ```ignore
/// let header = use_table_header(UseTableHeaderInput::default());
///
/// view! {
///     <thead {..header.header_props.into_attrs()}>
///         <tr>
///             // Column headers...
///         </tr>
///     </thead>
/// }
/// ```
pub fn use_table_header(_input: UseTableHeaderInput) -> UseTableHeaderReturn {
    // The role is "rowgroup" for table headers
    UseTableHeaderReturn {
        header_props: UseTableHeaderProps {
            role: AriaRole::Rowgroup,
        },
    }
}
