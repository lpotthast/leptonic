use leptos::{attr, attr::Attr};

use crate::{hooks::IntoAttrs, utils::aria::AriaRole};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input for the table header row.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseTableHeaderRowInput {
    /// The row index (usually 0 for header).
    pub row_index: usize,
}

/// Return value for the table header row.
pub struct UseTableHeaderRowReturn {
    /// Props for programmatic merging. Call `.into_attrs()` for view spreading.
    pub row_props: UseTableHeaderRowProps,
}

/// Props from `use_table_header_row` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseTableHeaderRowProps {
    pub role: AriaRole,
    pub aria_rowindex: &'static str,
}

impl IntoAttrs for UseTableHeaderRowProps {
    type Attrs = UseTableHeaderRowAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaRowindex, self.aria_rowindex),
        )
    }
}

/// Attributes for the table header row element.
pub type UseTableHeaderRowAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaRowindex, &'static str>,
);

/// Provides the behavior and accessibility for a table header row.
pub fn use_table_header_row(input: UseTableHeaderRowInput) -> UseTableHeaderRowReturn {
    let UseTableHeaderRowInput { row_index: _ } = input;

    UseTableHeaderRowReturn {
        row_props: UseTableHeaderRowProps {
            role: AriaRole::Row,
            aria_rowindex: "1",
        },
    }
}
