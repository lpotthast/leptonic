//! Rust types for ARIA attributes, as defined by <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference>
//!
//! We distinguish between two groups of attributes:
//!
//! - Group A (ARIA spec: absence equivalent to "false")
//! - Group B (ARIA spec: absence means "property doesn't apply")
//!
//! Design principles:
//!
//! 1. **Enums contain ONLY real ARIA values** — no `Undefined` variants.
//! 2. **Each type implements `AttributeValue`** (via `impl_attribute_value_via_str!`), delegating
//!    rendering to `&'static str`. This also provides `IntoAttributeValue` via the blanket impl.
//! 3. **Optionality is always expressed via `Option<AriaType>` at call sites** — the hook decides
//!    whether to render.
//! 4. **Typed attrs** — store ARIA enum types in attr tuples (e.g., `Signal<Option<AriaDisabled>>`),
//!    not pre-converted strings. Tachys has blanket `AttributeValue` impls for `Option<V>` and
//!    `Signal<T>` (via `ReactiveFunction`), so this works automatically once the inner type
//!    implements `AttributeValue`.

use leptos::attr::AttributeKey;
use leptos::prelude::*;
use std::str::FromStr;

use leptos::tachys::html::attribute::AttributeValue;

// ----------------------------------------------------------------------------------
// Macros
// ----------------------------------------------------------------------------------

/// Implements `AttributeValue` for a `Copy + Send + 'static` type that has an `into_str(self) -> &'static str` method.
/// This delegates all rendering to the `&'static str` `AttributeValue` impl.
/// The blanket `impl<T: AttributeValue> IntoAttributeValue for T` then provides `IntoAttributeValue` automatically.
macro_rules! impl_attribute_value_via_str {
    ($name:ty) => {
        impl AttributeValue for $name {
            type State = (leptos::tachys::renderer::types::Element, &'static str);
            type AsyncOutput = Self;
            type Cloneable = Self;
            type CloneableOwned = Self;

            fn html_len(&self) -> usize {
                self.into_str().len()
            }

            fn to_html(self, key: &str, buf: &mut String) {
                <&str as AttributeValue>::to_html(self.into_str(), key, buf);
            }

            fn to_template(key: &str, buf: &mut String) {
                <&str as AttributeValue>::to_template(key, buf);
            }

            fn hydrate<const FROM_SERVER: bool>(
                self,
                key: &str,
                el: &leptos::tachys::renderer::types::Element,
            ) -> Self::State {
                <&str as AttributeValue>::hydrate::<FROM_SERVER>(self.into_str(), key, el)
            }

            fn build(
                self,
                el: &leptos::tachys::renderer::types::Element,
                key: &str,
            ) -> Self::State {
                <&str as AttributeValue>::build(self.into_str(), el, key)
            }

            fn rebuild(self, key: &str, state: &mut Self::State) {
                <&str as AttributeValue>::rebuild(self.into_str(), key, state);
            }

            fn into_cloneable(self) -> Self::Cloneable {
                self
            }

            fn into_cloneable_owned(self) -> Self::CloneableOwned {
                self
            }

            fn dry_resolve(&mut self) {}

            async fn resolve(self) -> Self::AsyncOutput {
                self
            }
        }
    };
}

/// Generates a boolean ARIA type with `True` and `False` variants.
/// Always renders as `"true"` or `"false"` (never absent).
macro_rules! define_aria_bool {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            True,
            False,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::False
            }
        }

        impl $name {
            pub fn into_str(self) -> &'static str {
                match self {
                    Self::True => "true",
                    Self::False => "false",
                }
            }
        }

        impl From<bool> for $name {
            fn from(value: bool) -> Self {
                if value { Self::True } else { Self::False }
            }
        }

        impl_attribute_value_via_str!($name);
    };
}

// ----------------------------------------------------------------------------------
// AriaRole
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaRole {
    /// See: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/link_role>
    Link,
}

impl AriaRole {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::Link => "link",
        }
    }
}

impl_attribute_value_via_str!(AriaRole);

// ----------------------------------------------------------------------------------
// AriaControls
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-controls>
///
/// A space-separated list of one or more ID values referencing the elements being controlled
/// by the current element. Used as `Option<AriaControls>` at call sites.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AriaControls(pub Vec<String>);

impl IntoAttributeValue for AriaControls {
    type Output = String;

    fn into_attribute_value(self) -> Self::Output {
        self.0.join(" ")
    }
}

// ----------------------------------------------------------------------------------
// AriaHasPopup
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-haspopup>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaHasPopup {
    // The element does not have a popup.
    #[default]
    False,

    /// The popup is a menu.
    True,

    /// The popup is a menu.
    Menu,

    /// The popup is a listbox.
    Listbox,

    /// The popup is a tree.
    Tree,

    /// The popup is a grid.
    Grid,

    /// The popup is a dialog.
    Dialog,
}

impl AriaHasPopup {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::False => "false",
            Self::True => "true",
            Self::Menu => "menu",
            Self::Listbox => "listbox",
            Self::Tree => "tree",
            Self::Grid => "grid",
            Self::Dialog => "dialog",
        }
    }
}

impl AsRef<str> for AriaHasPopup {
    fn as_ref(&self) -> &'static str {
        leptos::attr::AriaHaspopup::KEY
    }
}

impl_attribute_value_via_str!(AriaHasPopup);

// ----------------------------------------------------------------------------------
// AriaExpanded
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaExpanded {
    /// The grouping element this element owns or controls is expanded.
    True,

    /// The grouping element this element owns or controls is collapsed.
    #[default]
    False,
}

impl AriaExpanded {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
        }
    }
}

impl FromStr for AriaExpanded {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "false" => Ok(Self::False),
            "true" => Ok(Self::True),
            other => Err(format!(
                "String '{other}' is not a valid AriaExpanded variant."
            )),
        }
    }
}

impl From<bool> for AriaExpanded {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl_attribute_value_via_str!(AriaExpanded);

// ----------------------------------------------------------------------------------
// Boolean ARIA types (always-present: renders "true" or "false")
// ----------------------------------------------------------------------------------

define_aria_bool! {
    /// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-disabled>
    AriaDisabled
}

define_aria_bool! {
    /// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-readonly>
    AriaReadonly
}

define_aria_bool! {
    /// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-grabbed>
    AriaGrabbed
}

define_aria_bool! {
    /// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden>
    AriaHidden
}

define_aria_bool! {
    /// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-modal>
    AriaModal
}

define_aria_bool! {
    /// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-required>
    AriaRequired
}

define_aria_bool! {
    /// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-multiselectable>
    AriaMultiselectable
}

// ----------------------------------------------------------------------------------
// AriaOrientation
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-orientation>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl AriaOrientation {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

impl_attribute_value_via_str!(AriaOrientation);

// ----------------------------------------------------------------------------------
// AriaSort
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-sort>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaSort {
    #[default]
    None,
    Ascending,
    Descending,
    Other,
}

impl AriaSort {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Ascending => "ascending",
            Self::Descending => "descending",
            Self::Other => "other",
        }
    }
}

impl_attribute_value_via_str!(AriaSort);

// ----------------------------------------------------------------------------------
// AriaCurrent
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-current>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaCurrent {
    #[default]
    False,
    True,
    Page,
    Step,
    Location,
    Date,
    Time,
}

impl AriaCurrent {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::False => "false",
            Self::True => "true",
            Self::Page => "page",
            Self::Step => "step",
            Self::Location => "location",
            Self::Date => "date",
            Self::Time => "time",
        }
    }
}

impl From<bool> for AriaCurrent {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl_attribute_value_via_str!(AriaCurrent);

// ----------------------------------------------------------------------------------
// AriaLive
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-live>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaLive {
    #[default]
    Off,
    Polite,
    Assertive,
}

impl AriaLive {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Polite => "polite",
            Self::Assertive => "assertive",
        }
    }
}

impl_attribute_value_via_str!(AriaLive);

// ----------------------------------------------------------------------------------
// AriaPressed
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-pressed>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaPressed {
    True,
    #[default]
    False,
    Mixed,
}

impl AriaPressed {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
            Self::Mixed => "mixed",
        }
    }
}

impl From<bool> for AriaPressed {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl_attribute_value_via_str!(AriaPressed);

// ----------------------------------------------------------------------------------
// AriaChecked
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-checked>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaChecked {
    True,
    #[default]
    False,
    Mixed,
}

impl AriaChecked {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
            Self::Mixed => "mixed",
        }
    }
}

impl From<bool> for AriaChecked {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl_attribute_value_via_str!(AriaChecked);

// ----------------------------------------------------------------------------------
// AriaInvalid
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-invalid>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaInvalid {
    #[default]
    False,
    True,
    Grammar,
    Spelling,
}

impl AriaInvalid {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::False => "false",
            Self::True => "true",
            Self::Grammar => "grammar",
            Self::Spelling => "spelling",
        }
    }
}

impl From<bool> for AriaInvalid {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl_attribute_value_via_str!(AriaInvalid);

// ----------------------------------------------------------------------------------
// AriaAutocomplete
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-autocomplete>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaAutocomplete {
    #[default]
    None,
    Inline,
    List,
    Both,
}

impl AriaAutocomplete {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Inline => "inline",
            Self::List => "list",
            Self::Both => "both",
        }
    }
}

impl_attribute_value_via_str!(AriaAutocomplete);

// ----------------------------------------------------------------------------------
// AriaSelected
// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-selected>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaSelected {
    True,
    #[default]
    False,
}

impl AriaSelected {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
        }
    }
}

impl From<bool> for AriaSelected {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl_attribute_value_via_str!(AriaSelected);
