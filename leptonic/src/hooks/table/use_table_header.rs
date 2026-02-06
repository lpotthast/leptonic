use leptos::attr;
use leptos::attr::Attr;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/table/src/useTableRowGroup.ts

/// Input parameters for the `use_table_header` hook.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseTableHeaderInput {
    /// Whether the header is for a sticky header.
    pub is_sticky: bool,
}

/// The return value of the `use_table_header` hook.
pub struct UseTableHeaderReturn {
    /// Props for the thead element.
    pub header_props: UseTableHeaderAttrs,
}

/// Attributes for the table header element.
pub type UseTableHeaderAttrs = (Attr<attr::Role, &'static str>,);

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
///     <thead {..header.header_props}>
///         <tr>
///             // Column headers...
///         </tr>
///     </thead>
/// }
/// ```
pub fn use_table_header(_input: UseTableHeaderInput) -> UseTableHeaderReturn {
    // The role is "rowgroup" for table headers
    UseTableHeaderReturn {
        header_props: (Attr(attr::Role, "rowgroup"),),
    }
}

/// Input parameters for the `use_table_body` hook.
#[derive(Debug, Clone, Copy, Default)]
pub struct UseTableBodyInput {}

/// The return value of the `use_table_body` hook.
pub struct UseTableBodyReturn {
    /// Props for the tbody element.
    pub body_props: UseTableBodyAttrs,
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
///     <tbody {..body.body_props}>
///         // Table rows...
///     </tbody>
/// }
/// ```
pub fn use_table_body(_input: UseTableBodyInput) -> UseTableBodyReturn {
    UseTableBodyReturn {
        body_props: (Attr(attr::Role, "rowgroup"),),
    }
}
