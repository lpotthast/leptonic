use leptos::*;

#[derive(Default, Debug, Clone, Copy)]
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
            AriaHasPopup::False => "false",
            AriaHasPopup::True => "true",
            AriaHasPopup::Menu => "menu",
            AriaHasPopup::Listbox => "listbox",
            AriaHasPopup::Tree => "tree",
            AriaHasPopup::Grid => "grid",
            AriaHasPopup::Dialog => "dialog",
        }
    }
}

impl IntoAttribute for AriaHasPopup {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.into_str().into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        Attribute::String(self.into_str().into())
    }
}

#[derive(Default, Debug, Clone, Copy)]
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
            AriaExpanded::Undefined => "undefined",
            AriaExpanded::False => "false",
            AriaExpanded::True => "true",
        }
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
