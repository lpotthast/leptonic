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
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub header_props: UseTableHeaderProps,
}

/// Props from `use_table_header` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseTableHeaderProps {
    pub role: &'static str,
}

impl UseTableHeaderProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseTableHeaderAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseTableHeaderAttrs {
        (Attr(attr::Role, self.role),)
    }
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
        header_props: UseTableHeaderProps { role: "rowgroup" },
    }
}

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
