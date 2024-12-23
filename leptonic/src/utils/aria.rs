use std::{rc::Rc, str::FromStr};

use educe::Educe;
use leptos::*;

use super::attributes::{IntoAttributeName, StaticAttributeName};

#[derive(Debug, Clone)]
pub enum AriaAttribute {
    /// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles>
    Role(GenericAttribute<AriaRole>),
    /// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-controls>
    Controls(GenericAttribute<AriaControls>),
    /// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-haspopup>
    HasPopup(GenericAttribute<AriaHasPopup>),
    /// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded>
    Expanded(GenericAttribute<AriaExpanded>),
    AccessibleName(GenericAttribute<AriaAccessibleName>),
}

impl From<AriaAttribute> for (&'static str, Attribute) {
    fn from(value: AriaAttribute) -> Self {
        let attr_name = value.to_attribute_name();
        match value {
            AriaAttribute::Role(val) => (attr_name, val.into_attribute()),
            AriaAttribute::Controls(val) => (attr_name, val.into_attribute()),
            AriaAttribute::HasPopup(val) => (attr_name, val.into_attribute()),
            AriaAttribute::Expanded(val) => (attr_name, val.into_attribute()),
            AriaAttribute::AccessibleName(val) => (attr_name, val.into_attribute()),
        }
    }
}

impl IntoAttributeName for AriaAttribute {
    fn to_attribute_name(&self) -> &'static str {
        match self {
            Self::Role(a) => a.to_attribute_name(),
            Self::Controls(a) => a.to_attribute_name(),
            Self::HasPopup(a) => a.to_attribute_name(),
            Self::Expanded(a) => a.to_attribute_name(),
            Self::AccessibleName(a) => a.to_attribute_name(),
        }
    }
}

impl std::fmt::Display for AriaAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_attribute_name())
    }
}

// ----------------------------------------------------------------------------------

#[derive(Clone, Educe)]
#[educe(Debug)]
pub enum GenericAttribute<T: IntoAttributeName + IntoAttribute + 'static> {
    /// A plain value.
    Static(T),
    /// A (presumably reactive) function, which will be run inside an effect to do targeted updates to the attribute.
    Fn(#[educe(Debug(ignore))] Rc<dyn Fn() -> T>),
    // An optional value, which sets the attribute to the value if `Some` and removes the attribute if `None`.
    // Option(Option<T>),
    // A boolean attribute, which sets the attribute if `true` and removes the attribute if `false`.
    // Bool(bool),
}

impl<T: IntoAttributeName + IntoAttribute + Clone + 'static, F: Fn() -> T + 'static> From<F> for GenericAttribute<T> {
    fn from(f: F) -> Self {
        Self::Fn(Rc::new(f))
    }
}

#[cfg(not(feature = "nightly"))]
impl<T: IntoAttributeName + IntoAttribute + Clone + 'static> From<Signal<T>> for GenericAttribute<T> {
    fn from(signal: Signal<T>) -> Self {
        Self::Fn(Rc::new(move || signal.get()))
    }
}

impl<T: IntoAttributeName + IntoAttribute + 'static> IntoAttributeName for GenericAttribute<T> {
    fn to_attribute_name(&self) -> &'static str {
        match self {
            GenericAttribute::Static(v) => v.to_attribute_name(),
            GenericAttribute::Fn(v) => v().to_attribute_name(),
            //GenericAttribute::Option(v) => match v {
            //    Some(t) => t.to_attribute_name(),
            //    None => None,
            //},
            // GenericAttribute::Bool(v) => Attribute::Bool(v),
        }
    }
}

impl<T: IntoAttributeName + IntoAttribute + 'static> IntoAttribute for GenericAttribute<T> {
    fn into_attribute(self) -> Attribute {
        match self {
            GenericAttribute::Static(v) => v.into_attribute(),
            GenericAttribute::Fn(v) => Attribute::Fn(Rc::new(move || v().into_attribute())),
            //GenericAttribute::Option(v) => match v {
            //    Some(t) => t.into_attribute(),
            //    None => Attribute::Option(None),
            //},
            //GenericAttribute::Bool(v) => Attribute::Bool(v),
        }
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

// ----------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AriaAccessibleName {
    AriaLabel(AriaLabel),
    AriaLabelledby(AriaLabelledby),
}

impl AriaAccessibleName {
    pub fn label(l: impl Into<Oco<'static, str>>) -> Self {
        Self::AriaLabel(AriaLabel(l.into()))
    }

    pub fn labelledby(l: impl Into<Oco<'static, str>>) -> Self {
        Self::AriaLabelledby(AriaLabelledby(l.into()))
    }
}

impl IntoAttributeName for AriaAccessibleName {
    fn to_attribute_name(&self) -> &'static str {
        match self {
            AriaAccessibleName::AriaLabel(_) => AriaLabel::static_attribute_name(),
            AriaAccessibleName::AriaLabelledby(_) => AriaLabelledby::static_attribute_name(),
        }
    }
}

impl IntoAttribute for AriaAccessibleName {
    fn into_attribute(self) -> Attribute {
        match self {
            AriaAccessibleName::AriaLabel(label) => label.into_attribute(),
            AriaAccessibleName::AriaLabelledby(labelledby) => labelledby.into_attribute(),
        }
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

// ----------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AriaLabel(Oco<'static, str>);

impl StaticAttributeName for AriaLabel {
    fn static_attribute_name() -> &'static str {
        "aria-label"
    }
}

impl IntoAttribute for AriaLabel {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.0)
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

// ----------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// See: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-labelledby>
pub struct AriaLabelledby(Oco<'static, str>);

impl StaticAttributeName for AriaLabelledby {
    fn static_attribute_name() -> &'static str {
        "aria-labelledby"
    }
}

impl IntoAttribute for AriaLabelledby {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.0)
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

// ----------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaRole {
    /// See: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/link_role>
    Link,
    /// See: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/menuitem_role>
    MenuItem,
}

impl StaticAttributeName for AriaRole {
    fn static_attribute_name() -> &'static str {
        "role"
    }
}

impl IntoAttribute for AriaRole {
    fn into_attribute(self) -> Attribute {
        match self {
            Self::Link => Attribute::String(Oco::Borrowed("link")),
            Self::MenuItem => Attribute::String(Oco::Borrowed("menuitem")),
        }
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

// ----------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AriaControls {
    // A space-separated list of one or more ID values referencing the elements being controlled by the current element.
    Id(Vec<String>),
    Undefined,
}

impl StaticAttributeName for AriaControls {
    fn static_attribute_name() -> &'static str {
        "aria-controls"
    }
}

impl IntoAttribute for AriaControls {
    fn into_attribute(self) -> Attribute {
        match self {
            Self::Id(ids) => Attribute::String(Oco::Owned(ids.join(" "))),
            Self::Undefined => Attribute::Option(None),
        }
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

// ----------------------------------------------------------------------------------

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

impl StaticAttributeName for AriaHasPopup {
    fn static_attribute_name() -> &'static str {
        "aria-haspopup"
    }
}

impl IntoAttribute for AriaHasPopup {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.into_str().into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

// ----------------------------------------------------------------------------------

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
        match value {
            true => Self::True,
            false => Self::False,
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

impl StaticAttributeName for AriaExpanded {
    fn static_attribute_name() -> &'static str {
        "aria-expanded"
    }
}

impl IntoAttribute for AriaExpanded {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.into_str().into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        Attribute::String(self.into_str().into())
    }
}

// ----------------------------------------------------------------------------------
