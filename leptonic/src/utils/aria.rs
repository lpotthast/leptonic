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

use std::str::FromStr;

use itertools::Itertools;
use leptos::{attr::AttributeKey, prelude::*, tachys::html::attribute::AttributeValue};
use smallvec::SmallVec;

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

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-describedby>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AriaDescribedby {
    ids: SmallVec<[Oco<'static, str>; 1]>,
}

impl AriaDescribedby {
    pub fn none() -> Self {
        Self {
            ids: SmallVec::new(),
        }
    }

    pub fn element_with_id(id: impl Into<Oco<'static, str>>) -> Self {
        Self {
            ids: SmallVec::from_buf([id.into()]),
        }
    }

    pub fn elements_with_ids<ID: Into<Oco<'static, str>>>(ids: impl Iterator<Item = ID>) -> Self {
        let (_, remaining) = ids.size_hint();
        let capacity = remaining.unwrap_or(0);
        Self {
            ids: {
                let mut vec = SmallVec::with_capacity(capacity);
                vec.extend(ids.map(Into::into));
                vec
            },
        }
    }

    // TODO: This could be made more efficient.
    fn into_oco(self) -> Oco<'static, str> {
        match self.ids.len() {
            1 => self.ids.into_iter().next().unwrap(),
            _ => self.ids.into_iter().join(" ").into(),
        }
    }
}

impl AttributeValue for AriaDescribedby {
    type State = (leptos::tachys::renderer::types::Element, Oco<'static, str>);
    type AsyncOutput = Self;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        let id_lengths: usize = self.ids.iter().map(|id| id.len()).sum();
        let num_spaces = if self.ids.is_empty() {
            0
        } else {
            self.ids.len() - 1
        };
        id_lengths + num_spaces
    }

    fn to_html(self, key: &str, buf: &mut String) {
        <Oco<'static, str> as AttributeValue>::to_html(self.into_oco(), key, buf);
    }

    fn to_template(key: &str, buf: &mut String) {
        <Oco<'static, str> as AttributeValue>::to_template(key, buf);
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        key: &str,
        el: &leptos::tachys::renderer::types::Element,
    ) -> Self::State {
        <Oco<'static, str> as AttributeValue>::hydrate::<FROM_SERVER>(self.into_oco(), key, el)
    }

    fn build(self, el: &leptos::tachys::renderer::types::Element, key: &str) -> Self::State {
        <Oco<'static, str> as AttributeValue>::build(self.into_oco(), el, key)
    }

    fn rebuild(self, key: &str, state: &mut Self::State) {
        <Oco<'static, str> as AttributeValue>::rebuild(self.into_oco(), key, state);
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

/// see: <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaRole {
    // -- Widget roles --
    Button,
    Checkbox,
    Combobox,
    Grid,
    Gridcell,
    Link,
    Listbox,
    Menu,
    Menubar,
    Menuitem,
    Menuitemcheckbox,
    Menuitemradio,
    Option,
    Progressbar,
    Radio,
    Radiogroup,
    Scrollbar,
    Searchbox,
    Slider,
    Spinbutton,
    Switch,
    Tab,
    Tablist,
    Tabpanel,
    Textbox,
    Tree,
    Treegrid,
    Treeitem,

    // -- Document structure roles --
    Application,
    Article,
    Blockquote,
    Caption,
    Cell,
    Code,
    Columnheader,
    Definition,
    Deletion,
    Directory,
    Document,
    Emphasis,
    Feed,
    Figure,
    Group,
    Heading,
    Img,
    Insertion,
    List,
    Listitem,
    Math,
    Meter,
    None,
    Note,
    Paragraph,
    Presentation,
    Row,
    Rowgroup,
    Rowheader,
    Separator,
    Strong,
    Subscript,
    Superscript,
    Table,
    Term,
    Time,
    Toolbar,
    Tooltip,

    // -- Landmark roles --
    Banner,
    Complementary,
    Contentinfo,
    Form,
    Main,
    Navigation,
    Region,
    Search,

    // -- Live region roles --
    Alert,
    Log,
    Marquee,
    Status,
    Timer,

    // -- Window roles --
    Alertdialog,
    Dialog,
}

impl AriaRole {
    pub fn into_str(self) -> &'static str {
        match self {
            // Widget roles.
            Self::Button => "button",
            Self::Checkbox => "checkbox",
            Self::Combobox => "combobox",
            Self::Grid => "grid",
            Self::Gridcell => "gridcell",
            Self::Link => "link",
            Self::Listbox => "listbox",
            Self::Menu => "menu",
            Self::Menubar => "menubar",
            Self::Menuitem => "menuitem",
            Self::Menuitemcheckbox => "menuitemcheckbox",
            Self::Menuitemradio => "menuitemradio",
            Self::Option => "option",
            Self::Progressbar => "progressbar",
            Self::Radio => "radio",
            Self::Radiogroup => "radiogroup",
            Self::Scrollbar => "scrollbar",
            Self::Searchbox => "searchbox",
            Self::Slider => "slider",
            Self::Spinbutton => "spinbutton",
            Self::Switch => "switch",
            Self::Tab => "tab",
            Self::Tablist => "tablist",
            Self::Tabpanel => "tabpanel",
            Self::Textbox => "textbox",
            Self::Tree => "tree",
            Self::Treegrid => "treegrid",
            Self::Treeitem => "treeitem",
            // Document structure roles.
            Self::Application => "application",
            Self::Article => "article",
            Self::Blockquote => "blockquote",
            Self::Caption => "caption",
            Self::Cell => "cell",
            Self::Code => "code",
            Self::Columnheader => "columnheader",
            Self::Definition => "definition",
            Self::Deletion => "deletion",
            Self::Directory => "directory",
            Self::Document => "document",
            Self::Emphasis => "emphasis",
            Self::Feed => "feed",
            Self::Figure => "figure",
            Self::Group => "group",
            Self::Heading => "heading",
            Self::Img => "img",
            Self::Insertion => "insertion",
            Self::List => "list",
            Self::Listitem => "listitem",
            Self::Math => "math",
            Self::Meter => "meter",
            Self::None => "none",
            Self::Note => "note",
            Self::Paragraph => "paragraph",
            Self::Presentation => "presentation",
            Self::Row => "row",
            Self::Rowgroup => "rowgroup",
            Self::Rowheader => "rowheader",
            Self::Separator => "separator",
            Self::Strong => "strong",
            Self::Subscript => "subscript",
            Self::Superscript => "superscript",
            Self::Table => "table",
            Self::Term => "term",
            Self::Time => "time",
            Self::Toolbar => "toolbar",
            Self::Tooltip => "tooltip",
            // Landmark roles.
            Self::Banner => "banner",
            Self::Complementary => "complementary",
            Self::Contentinfo => "contentinfo",
            Self::Form => "form",
            Self::Main => "main",
            Self::Navigation => "navigation",
            Self::Region => "region",
            Self::Search => "search",
            // Live region roles.
            Self::Alert => "alert",
            Self::Log => "log",
            Self::Marquee => "marquee",
            Self::Status => "status",
            Self::Timer => "timer",
            // Window roles.
            Self::Alertdialog => "alertdialog",
            Self::Dialog => "dialog",
        }
    }
}

impl_attribute_value_via_str!(AriaRole);

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
        if value { Self::True } else { Self::False }
    }
}

impl_attribute_value_via_str!(AriaExpanded);

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
        if value { Self::True } else { Self::False }
    }
}

impl_attribute_value_via_str!(AriaCurrent);

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
        if value { Self::True } else { Self::False }
    }
}

impl_attribute_value_via_str!(AriaPressed);

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
        if value { Self::True } else { Self::False }
    }
}

impl_attribute_value_via_str!(AriaChecked);

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
        if value { Self::True } else { Self::False }
    }
}

impl_attribute_value_via_str!(AriaInvalid);

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
        if value { Self::True } else { Self::False }
    }
}

impl_attribute_value_via_str!(AriaSelected);
