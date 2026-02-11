use leptos::attr;
use leptos::attr::Attr;

use crate::utils::aria::AriaOrientation;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/separator/src/useSeparator.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// The orientation of a separator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorOrientation {
    /// Horizontal separator.
    #[default]
    Horizontal,
    /// Vertical separator.
    Vertical,
}

impl From<SeparatorOrientation> for AriaOrientation {
    fn from(value: SeparatorOrientation) -> Self {
        match value {
            SeparatorOrientation::Horizontal => Self::Horizontal,
            SeparatorOrientation::Vertical => Self::Vertical,
        }
    }
}

/// The element type for a separator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorElementType {
    /// An <hr> element.
    #[default]
    Hr,
    /// A <div> element.
    Div,
    /// A <span> element.
    Span,
}

/// Input parameters for the `use_separator` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseSeparatorInput {
    /// The orientation of the separator.
    pub orientation: SeparatorOrientation,

    /// The element type.
    pub element_type: SeparatorElementType,
}

impl Default for UseSeparatorInput {
    fn default() -> Self {
        Self {
            orientation: SeparatorOrientation::Horizontal,
            element_type: SeparatorElementType::Hr,
        }
    }
}

/// The return value of the `use_separator` hook.
pub struct UseSeparatorReturn {
    /// Props for the separator element.
    pub separator_props: UseSeparatorProps,
}

/// Props from `use_separator` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseSeparatorProps {
    pub role: Option<&'static str>,
    pub aria_orientation: Option<AriaOrientation>,
}

impl UseSeparatorProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseSeparatorAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseSeparatorAttrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaOrientation, self.aria_orientation),
        )
    }
}

/// Attributes for the separator element.
pub type UseSeparatorAttrs = (
    Attr<attr::Role, Option<&'static str>>,
    Attr<attr::AriaOrientation, Option<AriaOrientation>>,
);

/// Provides the behavior and accessibility for a separator.
///
/// A separator divides content visually and semantically.
///
/// # Example
///
/// ```ignore
/// let separator = use_separator(UseSeparatorInput {
///     orientation: SeparatorOrientation::Horizontal,
///     ..Default::default()
/// });
///
/// view! {
///     <hr {..separator.separator_props} />
/// }
/// ```
pub fn use_separator(input: UseSeparatorInput) -> UseSeparatorReturn {
    let UseSeparatorInput {
        orientation,
        element_type,
    } = input;

    // <hr> elements don't need role or aria-orientation
    // Other elements need role="separator"
    let (role, aria_orientation) = match element_type {
        SeparatorElementType::Hr => (None, None),
        SeparatorElementType::Div | SeparatorElementType::Span => {
            (Some("separator"), Some(AriaOrientation::from(orientation)))
        }
    };

    UseSeparatorReturn {
        separator_props: UseSeparatorProps {
            role,
            aria_orientation,
        },
    }
}
