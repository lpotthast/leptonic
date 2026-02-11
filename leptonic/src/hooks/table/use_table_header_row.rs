use leptos::attr;
use leptos::attr::Attr;

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
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub row_props: UseTableHeaderRowProps,
}

/// Props from `use_table_header_row` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseTableHeaderRowProps {
    pub role: &'static str,
    pub aria_rowindex: &'static str,
}

impl UseTableHeaderRowProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTableHeaderRowAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTableHeaderRowAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaRowindex, self.aria_rowindex),
        )
    }
}

/// Attributes for the table header row element.
pub type UseTableHeaderRowAttrs = (
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaRowindex, &'static str>,
);

/// Provides the behavior and accessibility for a table header row.
pub fn use_table_header_row(input: UseTableHeaderRowInput) -> UseTableHeaderRowReturn {
    let UseTableHeaderRowInput { row_index: _ } = input;

    UseTableHeaderRowReturn {
        row_props: UseTableHeaderRowProps {
            role: "row",
            aria_rowindex: "1",
        },
    }
}
