// Upstream: react-aria/src/separator/useSeparator.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use crate::{
    hooks::IntoAttrs,
    utils::{
        aria::{AriaOrientation, AriaRole},
        orientation::Orientation,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `element_type` is an enum (`SeparatorElementType`) instead of a tag name string; it defaults
//   to `Hr` (react-aria: no element type, which behaves like any non-`hr` element).
// - Of the DOM props react-aria passes through (`filterDOMProps` with labelable props), only `id`,
//   `aria-label` and `aria-labelledby` are offered.
//
// =============================================================================

/// The element type for a separator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SeparatorElementType {
    /// An `<hr>` element: a separator by itself (horizontal).
    #[default]
    Hr,
    /// A `<div>` element, which gets `role="separator"` (e.g. vertical separators, separators in
    /// menus).
    Div,
    /// A `<span>` element, which gets `role="separator"`.
    Span,
}

/// Input parameters for the `use_separator` hook.
#[derive(Debug, Clone)]
pub struct UseSeparatorInput {
    /// The orientation of the separator. Default: horizontal.
    pub orientation: Signal<Orientation>,
    /// The element the separator is rendered as.
    pub element_type: SeparatorElementType,
    /// The element's id.
    pub id: Option<String>,
    /// Names the separator.
    pub aria_label: MaybeProp<String>,
    /// The ids of the elements naming the separator.
    pub aria_labelledby: Option<String>,
}

impl Default for UseSeparatorInput {
    /// A horizontal `<hr>` separator.
    fn default() -> Self {
        Self {
            orientation: Signal::stored(Orientation::Horizontal),
            element_type: SeparatorElementType::Hr,
            id: None,
            aria_label: MaybeProp::default(),
            aria_labelledby: None,
        }
    }
}

/// The return value of the `use_separator` hook.
#[derive(Debug)]
pub struct UseSeparatorReturn {
    /// Props for the separator element.
    pub props: UseSeparatorProps,
}

/// Props for the separator element.
#[derive(Debug)]
pub struct UseSeparatorProps {
    pub id: Option<String>,
    pub role: Option<AriaRole>,
    pub aria_orientation: Signal<Option<AriaOrientation>>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
}

impl IntoAttrs for UseSeparatorProps {
    type Attrs = UseSeparatorAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaOrientation, self.aria_orientation),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

/// Attributes for the separator element.
pub type UseSeparatorAttrs = (
    Attr<attr::Id, Option<String>>,
    Attr<attr::Role, Option<AriaRole>>,
    Attr<attr::AriaOrientation, Signal<Option<AriaOrientation>>>,
    Attr<attr::AriaLabel, MaybeProp<String>>,
    Attr<attr::AriaLabelledby, Option<String>>,
);

/// Provides the accessibility of a separator, which divides content visually and semantically.
///
/// An `<hr>` is a horizontal separator by itself; other elements get `role="separator"` and, when
/// vertical, `aria-orientation="vertical"` (horizontal is the default orientation).
///
/// # Example
///
/// ```ignore
/// let separator = use_separator(UseSeparatorInput::default());
///
/// view! { <hr {..separator.props.into_attrs()} /> }
/// ```
pub fn use_separator(input: UseSeparatorInput) -> UseSeparatorReturn {
    let UseSeparatorInput {
        orientation,
        element_type,
        id,
        aria_label,
        aria_labelledby,
    } = input;

    // An `<hr>` implicitly has the separator role and a horizontal orientation. Horizontal is the
    // default `aria-orientation`; only vertical needs to be stated.
    let is_hr = element_type == SeparatorElementType::Hr;
    let role = (!is_hr).then_some(AriaRole::Separator);
    let aria_orientation = Signal::derive(move || {
        (!is_hr && orientation.get() == Orientation::Vertical).then_some(AriaOrientation::Vertical)
    });

    UseSeparatorReturn {
        props: UseSeparatorProps {
            id,
            role,
            aria_orientation,
            aria_label,
            aria_labelledby,
        },
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    fn props(orientation: Orientation, element_type: SeparatorElementType) -> UseSeparatorProps {
        use_separator(UseSeparatorInput {
            orientation: orientation.into(),
            element_type,
            ..UseSeparatorInput::default()
        })
        .props
    }

    #[test]
    fn hr_needs_no_role() {
        Owner::new().with(|| {
            let props = props(Orientation::Horizontal, SeparatorElementType::Hr);
            assert_that!(props.role).is_none();
            assert_that!(props.aria_orientation.get_untracked()).is_none();
        });
    }

    #[test]
    fn other_elements_get_the_role_and_only_a_vertical_orientation() {
        Owner::new().with(|| {
            let horizontal = props(Orientation::Horizontal, SeparatorElementType::Div);
            assert_that!(horizontal.role).is_equal_to(Some(AriaRole::Separator));
            assert_that!(horizontal.aria_orientation.get_untracked()).is_none();

            let vertical = props(Orientation::Vertical, SeparatorElementType::Div);
            assert_that!(vertical.role).is_equal_to(Some(AriaRole::Separator));
            assert_that!(vertical.aria_orientation.get_untracked())
                .is_equal_to(Some(AriaOrientation::Vertical));
        });
    }
}
