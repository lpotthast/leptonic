use leptos::attr::AttributeKey;
use leptos::prelude::*;
use std::str::FromStr;

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaRole {
    /// See: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/link_role>
    Link,
}

impl IntoAttributeValue for AriaRole {
    type Output = &'static str;

    fn into_attribute_value(self) -> Self::Output {
        match self {
            Self::Link => "link",
        }
    }
}

// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-controls>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AriaControls {
    // A space-separated list of one or more ID values referencing the elements being controlled by the current element.
    Id(Vec<String>),
    Undefined,
}

impl IntoAttributeValue for AriaControls {
    type Output = Option<String>;

    fn into_attribute_value(self) -> Self::Output {
        match self {
            Self::Id(ids) => Some(ids.join(" ")),
            Self::Undefined => None,
        }
    }
}

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

impl IntoAttributeValue for AriaHasPopup {
    type Output = &'static str;

    fn into_attribute_value(self) -> Self::Output {
        self.into_str()
    }
}

// ----------------------------------------------------------------------------------

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded>
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaExpanded {
    // The element does not own or control a grouping element that is expandable.
    #[default]
    Undefined,

    // The grouping element this element owns or controls is collapsed.
    False,

    /// The grouping element this element owns or controls is expanded.
    True,
}

impl AriaExpanded {
    pub fn into_str(self) -> &'static str {
        match self {
            Self::Undefined => "undefined",
            Self::False => "false",
            Self::True => "true",
        }
    }
}

impl FromStr for AriaExpanded {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "undefined" => Ok(Self::Undefined),
            "false" => Ok(Self::False),
            "true" => Ok(Self::True),
            other => Err(format!(
                "String '{other}' is no a valid AriaExpanded variant."
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

// TODO: Do we want this impl? Is the None => Undefined mapping a valid assumption?
impl From<Option<bool>> for AriaExpanded {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(value) => Self::from(value),
            None => Self::Undefined,
        }
    }
}

impl IntoAttributeValue for AriaExpanded {
    type Output = &'static str;

    fn into_attribute_value(self) -> Self::Output {
        self.into_str()
    }
}

// ----------------------------------------------------------------------------------
