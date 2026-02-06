use leptos::attr::IntoAttributeValue;
use leptos::prelude::{Get, Signal};
use leptos::tachys::html::style::IntoStyle;
use leptos::tachys::renderer::dom::Element;
use leptos::tachys::renderer::Rndr;
use leptos::typed_builder::TypedBuilder;
use reactive_graph::effect::RenderEffect;
use smallvec::SmallVec;
use std::borrow::Cow;
use std::sync::Arc;

pub use crate::utils::style::Style;
pub use crate::utils::style::Style::*;

/// Fast FNV-1a hash for quick string comparison.
/// Used to avoid full string comparison when checking if styles changed.
#[inline]
fn fxhash(s: &str) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0100_0000_01b3;
    let mut hash = FNV_OFFSET;
    for byte in s.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// A CSS property name that can be either a static string slice or an owned `String`.
pub type StyleProperty = Cow<'static, str>;

/// A CSS value that can be either a static string slice or an owned `String`.
pub type StyleValue = Cow<'static, str>;

/// Internal wrapper around `SmallVec<[StyleEntry; 4]>` that provides duplicate detection in debug builds.
///
/// Uses `SmallVec` to avoid heap allocation for the common case of ≤4 style entries.
///
/// Note: The TypedBuilder derive requires this to be pub.
#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct StyleList(SmallVec<[StyleEntry; 4]>);

impl StyleList {
    fn push(&mut self, style: StyleEntry) {
        #[cfg(debug_assertions)]
        {
            if self.0.iter().any(|it| it.property() == style.property()) {
                tracing::warn!(
                    "Duplicate style property '{}' added to Styles. \
                     This may indicate conflicting styles.",
                    style.property()
                );
            }
        }
        self.0.push(style);
    }

    fn iter(&self) -> impl Iterator<Item = &StyleEntry> {
        self.0.iter()
    }
}

/// Leptos component-prop-utility to drill down a list of inline styles.
///
/// # Duplicate Handling
///
/// Duplicate style properties are allowed and will not be deduplicated. However, in debug builds,
/// a warning will be logged when the same property name is added multiple times. This behavior
/// helps identify potential bugs where styles are unintentionally added twice.
///
/// The final style string produced by [`Styles::to_style_string`] may contain repeated
/// properties if duplicates were added.
///
/// # Example
/// ```rust
/// use leptos::prelude::*;
/// use leptonic::utils::styles::Styles;
///
/// /// The lowest-level component renders the style-list into an actual HTML element.
/// #[component]
/// fn NeedingStyles(
///     #[prop(into, optional)] styles: Styles,
/// ) -> impl IntoView {
///     view! {
///         <div style=styles/>
///     }
/// }
///
/// /// Components sitting in the middle can add their own styles.
/// #[component]
/// fn ExtendingStyles(
///     #[prop(into, optional)] styles: Styles,
/// ) -> impl IntoView {
///     view! {
///         <NeedingStyles styles=styles.add(("margin", "10px"))/>
///     }
/// }
///
/// /// Root component defines the initial styles using a builder pattern.
/// #[component]
/// fn ProvidingStyles() -> impl IntoView {
///     let color = RwSignal::new(Some("blue".to_string()));
///     view! {
///         <ExtendingStyles styles=("color", "red")/>
///         <ExtendingStyles styles=Styles::builder()
///             .with(("padding", "10px"))
///             .with(("color", color.into()))
///             .build()
///         />
///     }
/// }
/// ```
#[derive(Clone, Debug, Default, TypedBuilder)]
#[builder(mutators(
    pub fn with(&mut self, style: impl Into<StyleEntry>) {
        self.styles.push(style.into());
    }

    pub fn with_all<S: Into<StyleEntry>>(&mut self, iter: impl Iterator<Item = S>) {
        for style in iter {
            self.styles.push(style.into());
        }
    }
))]
pub struct Styles {
    #[builder(via_mutators)]
    styles: StyleList,
}

/// A single style entry consisting of a CSS property and its reactive optional value.
///
/// When the value signal returns `None`, the style entry is excluded from the output.
#[derive(Clone, Debug)]
pub struct StyleEntry {
    property: StyleProperty,
    value: Signal<Option<StyleValue>>,
}

impl StyleEntry {
    /// Creates a style entry with a reactive optional value.
    ///
    /// When the signal returns `None`, this style will be excluded from the output.
    pub fn reactive(
        property: impl Into<StyleProperty>,
        value: impl Into<Signal<Option<StyleValue>>>,
    ) -> Self {
        Self {
            property: property.into(),
            value: value.into(),
        }
    }

    /// Creates a style entry with a static always-present value.
    pub fn always(property: impl Into<StyleProperty>, value: impl Into<StyleValue>) -> Self {
        let value: Option<StyleValue> = Some(value.into());
        Self {
            property: property.into(),
            value: Signal::derive(move || value.clone()),
        }
    }

    /// Returns a reference to the CSS property name.
    pub fn property(&self) -> &str {
        &self.property
    }
}

impl Styles {
    pub fn new() -> Self {
        Self {
            styles: StyleList::default(),
        }
    }

    /// Add one additional style to this style-list.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, entry: impl Into<StyleEntry>) -> Self {
        self.styles.push(entry.into());
        self
    }

    /// Add multiple styles to this style-list.
    pub fn add_all<S: Into<StyleEntry>>(&mut self, iter: impl Iterator<Item = S>) {
        for style in iter {
            self.styles.push(style.into());
        }
    }

    /// Appends all defined styles to the given string buffer.
    ///
    /// Entries with `None` values are excluded from the output.
    /// This method is zero-allocation when the buffer has sufficient capacity.
    pub fn write_style_string(&self, buf: &mut String) {
        let mut first = true;
        for entry in self.styles.iter() {
            if let Some(value) = entry.value.get() {
                if !first {
                    buf.push_str("; ");
                }
                first = false;
                buf.push_str(&entry.property);
                buf.push_str(": ");
                buf.push_str(&value);
            }
        }
    }

    /// Reactively combines all defined styles into one semicolon-separated String.
    ///
    /// Entries with `None` values are excluded from the output.
    ///
    /// Note: Prefer using `style=styles` directly (via `IntoStyle`) for better performance,
    /// as it reuses the string buffer across reactive updates.
    pub fn to_style_string(&self) -> String {
        let mut s = String::new();
        self.write_style_string(&mut s);
        s
    }
}

impl IntoAttributeValue for Styles {
    type Output = Arc<dyn Fn() -> String + Send + Sync>;

    fn into_attribute_value(self) -> Self::Output {
        Arc::new(move || {
            let mut s = String::new();
            self.write_style_string(&mut s);
            s
        })
    }
}

/// State for the `IntoStyle` implementation of `Styles`.
///
/// Uses `RenderEffect` to automatically track reactive dependencies within the styles
/// and update the DOM when any tracked signal changes.
pub struct StylesState {
    effect: RenderEffect<(Element, String)>,
}

impl IntoStyle for Styles {
    type AsyncOutput = Self;
    type State = StylesState;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn to_html(self, style: &mut String) {
        // SSR path: build style string directly, avoiding intermediate allocations.
        for entry in self.styles.iter() {
            if let Some(value) = entry.value.get() {
                style.push_str(&entry.property);
                style.push(':');
                style.push_str(&value);
                style.push(';');
            }
        }
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        let el = el.clone();
        StylesState {
            effect: RenderEffect::new(move |prev: Option<(Element, String)>| {
                let (el, mut style_str) = prev.unwrap_or_else(|| (el.clone(), String::new()));
                let prev_len = style_str.len();
                let prev_hash = fxhash(&style_str);
                style_str.clear();
                self.write_style_string(&mut style_str);

                // Subsequent runs: update DOM if style changed
                // Use length + hash for fast inequality check before string comparison
                if prev_len > 0 && (style_str.len() != prev_len || fxhash(&style_str) != prev_hash)
                {
                    if style_str.is_empty() {
                        Rndr::remove_attribute(&el, "style");
                    } else {
                        Rndr::set_attribute(&el, "style", &style_str);
                    }
                }
                (el, style_str)
            }),
        }
    }

    fn build(self, el: &Element) -> Self::State {
        let el = el.clone();
        StylesState {
            effect: RenderEffect::new(move |prev: Option<(Element, String)>| {
                if let Some((el, mut style_str)) = prev {
                    // Subsequent runs: update DOM if style changed
                    let prev_len = style_str.len();
                    let prev_hash = fxhash(&style_str);
                    style_str.clear();
                    self.write_style_string(&mut style_str);

                    if style_str.len() != prev_len || fxhash(&style_str) != prev_hash {
                        if style_str.is_empty() {
                            Rndr::remove_attribute(&el, "style");
                        } else {
                            Rndr::set_attribute(&el, "style", &style_str);
                        }
                    }
                    (el, style_str)
                } else {
                    // First run: set initial attribute
                    let mut style_str = String::new();
                    self.write_style_string(&mut style_str);
                    if !style_str.is_empty() {
                        Rndr::set_attribute(&el, "style", &style_str);
                    }
                    (el.clone(), style_str)
                }
            }),
        }
    }

    fn rebuild(self, state: &mut Self::State) {
        let prev_value = state.effect.take_value();
        if let Some((el, prev_str)) = prev_value {
            state.effect = RenderEffect::new_with_value(
                move |prev: Option<(Element, String)>| {
                    if let Some((el, mut style_str)) = prev {
                        let prev_len = style_str.len();
                        let prev_hash = fxhash(&style_str);
                        style_str.clear();
                        self.write_style_string(&mut style_str);

                        if style_str.len() != prev_len || fxhash(&style_str) != prev_hash {
                            if style_str.is_empty() {
                                Rndr::remove_attribute(&el, "style");
                            } else {
                                Rndr::set_attribute(&el, "style", &style_str);
                            }
                        }
                        (el, style_str)
                    } else {
                        unreachable!("rebuild should always have previous value")
                    }
                },
                Some((el, prev_str)),
            );
        }
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {
        // Touch all reactive values to register dependencies
        for entry in self.styles.iter() {
            let _ = entry.value.get();
        }
    }

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn reset(state: &mut Self::State) {
        if let Some((el, _)) = state.effect.take_value() {
            Rndr::remove_attribute(&el, "style");
        }
    }
}

/// Parses a style string like "color: red" into property and value parts.
fn parse_style_string(s: &str) -> (Cow<'static, str>, Cow<'static, str>) {
    if let Some((property, value)) = s.split_once(':') {
        (
            Cow::Owned(property.trim().to_string()),
            Cow::Owned(value.trim().trim_end_matches(';').to_string()),
        )
    } else {
        tracing::warn!(
            "Style string '{}' missing ':'. Expected 'property: value'",
            s
        );
        (Cow::Owned(s.trim().to_string()), Cow::Owned(String::new()))
    }
}

// ============================================================================
// From implementations for StyleEntry
// ============================================================================

// Static tuple: (&str, &str) -> always-on style
impl From<(&'static str, &'static str)> for StyleEntry {
    fn from((property, value): (&'static str, &'static str)) -> Self {
        StyleEntry::always(property, value)
    }
}

// Static tuple: (String, String) -> always-on style
impl From<(String, String)> for StyleEntry {
    fn from((property, value): (String, String)) -> Self {
        StyleEntry::always(property, value)
    }
}

// Mixed ownership: (&str, String)
impl From<(&'static str, String)> for StyleEntry {
    fn from((property, value): (&'static str, String)) -> Self {
        StyleEntry::always(property, value)
    }
}

// Mixed ownership: (String, &str)
impl From<(String, &'static str)> for StyleEntry {
    fn from((property, value): (String, &'static str)) -> Self {
        StyleEntry::always(property, value)
    }
}

// String parsing: "color: red"
impl From<&'static str> for StyleEntry {
    fn from(s: &'static str) -> Self {
        let (property, value) = parse_style_string(s);
        StyleEntry::always(property, value)
    }
}

// String parsing: String
impl From<String> for StyleEntry {
    fn from(s: String) -> Self {
        let (property, value) = parse_style_string(&s);
        StyleEntry::always(property, value)
    }
}

// Reactive: (&str, Signal<Option<StyleValue>>)
impl From<(&'static str, Signal<Option<StyleValue>>)> for StyleEntry {
    fn from((property, value): (&'static str, Signal<Option<StyleValue>>)) -> Self {
        StyleEntry::reactive(property, value)
    }
}

// Reactive: (String, Signal<Option<StyleValue>>)
impl From<(String, Signal<Option<StyleValue>>)> for StyleEntry {
    fn from((property, value): (String, Signal<Option<StyleValue>>)) -> Self {
        StyleEntry::reactive(property, value)
    }
}

// Reactive with String value: (&str, Signal<Option<String>>)
impl From<(&'static str, Signal<Option<String>>)> for StyleEntry {
    fn from((property, value): (&'static str, Signal<Option<String>>)) -> Self {
        Self {
            property: property.into(),
            value: Signal::derive(move || value.get().map(Cow::Owned)),
        }
    }
}

// Reactive with String value: (String, Signal<Option<String>>)
impl From<(String, Signal<Option<String>>)> for StyleEntry {
    fn from((property, value): (String, Signal<Option<String>>)) -> Self {
        Self {
            property: property.into(),
            value: Signal::derive(move || value.get().map(Cow::Owned)),
        }
    }
}

// Static optional: (&str, Option<&str>)
impl From<(&'static str, Option<&'static str>)> for StyleEntry {
    fn from((property, value): (&'static str, Option<&'static str>)) -> Self {
        let value: Option<StyleValue> = value.map(Cow::Borrowed);
        Self {
            property: property.into(),
            value: Signal::derive(move || value.clone()),
        }
    }
}

// Static optional: (&str, Option<String>)
impl From<(&'static str, Option<String>)> for StyleEntry {
    fn from((property, value): (&'static str, Option<String>)) -> Self {
        let value: Option<StyleValue> = value.map(Cow::Owned);
        Self {
            property: property.into(),
            value: Signal::derive(move || value.clone()),
        }
    }
}

// Static optional: (String, Option<&str>)
impl From<(String, Option<&'static str>)> for StyleEntry {
    fn from((property, value): (String, Option<&'static str>)) -> Self {
        let value: Option<StyleValue> = value.map(Cow::Borrowed);
        Self {
            property: property.into(),
            value: Signal::derive(move || value.clone()),
        }
    }
}

// Static optional: (String, Option<String>)
impl From<(String, Option<String>)> for StyleEntry {
    fn from((property, value): (String, Option<String>)) -> Self {
        let value: Option<StyleValue> = value.map(Cow::Owned);
        Self {
            property: property.into(),
            value: Signal::derive(move || value.clone()),
        }
    }
}

// Style enum: (Style, &str) -> always-on style
impl From<(Style, &'static str)> for StyleEntry {
    fn from((property, value): (Style, &'static str)) -> Self {
        StyleEntry::always(property.as_str(), value)
    }
}

// Style enum: (Style, String) -> always-on style
impl From<(Style, String)> for StyleEntry {
    fn from((property, value): (Style, String)) -> Self {
        StyleEntry::always(property.as_str(), value)
    }
}

// Style enum reactive: (Style, Signal<Option<StyleValue>>)
impl From<(Style, Signal<Option<StyleValue>>)> for StyleEntry {
    fn from((property, value): (Style, Signal<Option<StyleValue>>)) -> Self {
        StyleEntry::reactive(property.as_str(), value)
    }
}

// Style enum reactive with String value: (Style, Signal<Option<String>>)
impl From<(Style, Signal<Option<String>>)> for StyleEntry {
    fn from((property, value): (Style, Signal<Option<String>>)) -> Self {
        Self {
            property: Cow::Borrowed(property.as_str()),
            value: Signal::derive(move || value.get().map(Cow::Owned)),
        }
    }
}

// Style enum optional: (Style, Option<&str>)
impl From<(Style, Option<&'static str>)> for StyleEntry {
    fn from((property, value): (Style, Option<&'static str>)) -> Self {
        let value: Option<StyleValue> = value.map(Cow::Borrowed);
        Self {
            property: Cow::Borrowed(property.as_str()),
            value: Signal::derive(move || value.clone()),
        }
    }
}

// Style enum optional: (Style, Option<String>)
impl From<(Style, Option<String>)> for StyleEntry {
    fn from((property, value): (Style, Option<String>)) -> Self {
        let value: Option<StyleValue> = value.map(Cow::Owned);
        Self {
            property: Cow::Borrowed(property.as_str()),
            value: Signal::derive(move || value.clone()),
        }
    }
}

// Style enum with closure: (Style, impl Fn() -> Option<String>)
impl<F> From<(Style, F)> for StyleEntry
where
    F: Fn() -> Option<String> + Send + Sync + 'static,
{
    fn from((property, value_fn): (Style, F)) -> Self {
        Self {
            property: Cow::Borrowed(property.as_str()),
            value: Signal::derive(move || value_fn().map(Cow::Owned)),
        }
    }
}

// ============================================================================
// From implementations for Styles
// ============================================================================

// Single static tuple
impl From<(&'static str, &'static str)> for Styles {
    fn from(entry: (&'static str, &'static str)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(String, String)> for Styles {
    fn from(entry: (String, String)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(&'static str, String)> for Styles {
    fn from(entry: (&'static str, String)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(String, &'static str)> for Styles {
    fn from(entry: (String, &'static str)) -> Self {
        Styles::builder().with(entry).build()
    }
}

// String parsing
impl From<&'static str> for Styles {
    fn from(s: &'static str) -> Self {
        Styles::builder().with(s).build()
    }
}

impl From<String> for Styles {
    fn from(s: String) -> Self {
        Styles::builder().with(s).build()
    }
}

// Reactive tuples
impl From<(&'static str, Signal<Option<StyleValue>>)> for Styles {
    fn from(entry: (&'static str, Signal<Option<StyleValue>>)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(String, Signal<Option<StyleValue>>)> for Styles {
    fn from(entry: (String, Signal<Option<StyleValue>>)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(&'static str, Signal<Option<String>>)> for Styles {
    fn from(entry: (&'static str, Signal<Option<String>>)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(String, Signal<Option<String>>)> for Styles {
    fn from(entry: (String, Signal<Option<String>>)) -> Self {
        Styles::builder().with(entry).build()
    }
}

// Static optional tuples
impl From<(&'static str, Option<&'static str>)> for Styles {
    fn from(entry: (&'static str, Option<&'static str>)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(&'static str, Option<String>)> for Styles {
    fn from(entry: (&'static str, Option<String>)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(String, Option<&'static str>)> for Styles {
    fn from(entry: (String, Option<&'static str>)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(String, Option<String>)> for Styles {
    fn from(entry: (String, Option<String>)) -> Self {
        Styles::builder().with(entry).build()
    }
}

// Style enum tuples
impl From<(Style, &'static str)> for Styles {
    fn from(entry: (Style, &'static str)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(Style, String)> for Styles {
    fn from(entry: (Style, String)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(Style, Signal<Option<StyleValue>>)> for Styles {
    fn from(entry: (Style, Signal<Option<StyleValue>>)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(Style, Signal<Option<String>>)> for Styles {
    fn from(entry: (Style, Signal<Option<String>>)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(Style, Option<&'static str>)> for Styles {
    fn from(entry: (Style, Option<&'static str>)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl From<(Style, Option<String>)> for Styles {
    fn from(entry: (Style, Option<String>)) -> Self {
        Styles::builder().with(entry).build()
    }
}

impl<F> From<(Style, F)> for Styles
where
    F: Fn() -> Option<String> + Send + Sync + 'static,
{
    fn from(entry: (Style, F)) -> Self {
        Styles::builder().with(entry).build()
    }
}

// Arrays of static tuples
impl<const N: usize> From<[(&'static str, &'static str); N]> for Styles {
    fn from(entries: [(&'static str, &'static str); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

impl<const N: usize> From<[(String, String); N]> for Styles {
    fn from(entries: [(String, String); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

impl<const N: usize> From<[(&'static str, String); N]> for Styles {
    fn from(entries: [(&'static str, String); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

impl<const N: usize> From<[(String, &'static str); N]> for Styles {
    fn from(entries: [(String, &'static str); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

// Arrays of reactive tuples
impl<const N: usize> From<[(&'static str, Signal<Option<StyleValue>>); N]> for Styles {
    fn from(entries: [(&'static str, Signal<Option<StyleValue>>); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

impl<const N: usize> From<[(String, Signal<Option<StyleValue>>); N]> for Styles {
    fn from(entries: [(String, Signal<Option<StyleValue>>); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

impl<const N: usize> From<[(&'static str, Signal<Option<String>>); N]> for Styles {
    fn from(entries: [(&'static str, Signal<Option<String>>); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

impl<const N: usize> From<[(String, Signal<Option<String>>); N]> for Styles {
    fn from(entries: [(String, Signal<Option<String>>); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

// Arrays of Style enum tuples
impl<const N: usize> From<[(Style, &'static str); N]> for Styles {
    fn from(entries: [(Style, &'static str); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

impl<const N: usize> From<[(Style, String); N]> for Styles {
    fn from(entries: [(Style, String); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

impl<const N: usize> From<[(Style, Signal<Option<StyleValue>>); N]> for Styles {
    fn from(entries: [(Style, Signal<Option<StyleValue>>); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

impl<const N: usize> From<[(Style, Signal<Option<String>>); N]> for Styles {
    fn from(entries: [(Style, Signal<Option<String>>); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

impl<const N: usize> From<[(Style, Option<&'static str>); N]> for Styles {
    fn from(entries: [(Style, Option<&'static str>); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

impl<const N: usize> From<[(Style, Option<String>); N]> for Styles {
    fn from(entries: [(Style, Option<String>); N]) -> Self {
        Styles::builder().with_all(entries.into_iter()).build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertr::prelude::*;

    #[test]
    fn test_static_tuple() {
        let styles: Styles = ("color", "red").into();
        assert_that(styles.to_style_string()).is_equal_to("color: red".to_string());
    }

    #[test]
    fn test_static_string_parsing() {
        let styles: Styles = "padding: 10px".into();
        assert_that(styles.to_style_string()).is_equal_to("padding: 10px".to_string());
    }

    #[test]
    fn test_static_string_parsing_with_semicolon() {
        let styles: Styles = "margin: 5px;".into();
        assert_that(styles.to_style_string()).is_equal_to("margin: 5px".to_string());
    }

    #[test]
    fn test_array_of_tuples() {
        let styles: Styles = [("color", "blue"), ("padding", "20px")].into();
        assert_that(styles.to_style_string()).is_equal_to("color: blue; padding: 20px".to_string());
    }

    #[test]
    fn test_builder_pattern() {
        let styles = Styles::builder()
            .with(("margin", "10px"))
            .with(("display", "flex"))
            .build();
        assert_that(styles.to_style_string())
            .is_equal_to("margin: 10px; display: flex".to_string());
    }

    #[test]
    fn test_add_method() {
        let styles = Styles::new()
            .add(("color", "green"))
            .add(("font-size", "14px"));
        assert_that(styles.to_style_string())
            .is_equal_to("color: green; font-size: 14px".to_string());
    }

    #[test]
    fn test_static_optional_some() {
        let styles: Styles = ("color", Some("red")).into();
        assert_that(styles.to_style_string()).is_equal_to("color: red".to_string());
    }

    #[test]
    fn test_static_optional_none() {
        let styles: Styles = ("color", None::<&str>).into();
        assert_that(styles.to_style_string()).is_equal_to(String::new());
    }

    #[test]
    fn test_mixed_static_and_optional() {
        let styles = Styles::builder()
            .with(("color", "red"))
            .with(("padding", None::<&str>))
            .with(("margin", "10px"))
            .build();
        assert_that(styles.to_style_string()).is_equal_to("color: red; margin: 10px".to_string());
    }

    #[test]
    fn test_string_ownership_variants() {
        // (String, String)
        let styles1: Styles = ("color".to_string(), "red".to_string()).into();
        assert_that(styles1.to_style_string()).is_equal_to("color: red".to_string());

        // (&str, String)
        let styles2: Styles = ("color", "red".to_string()).into();
        assert_that(styles2.to_style_string()).is_equal_to("color: red".to_string());

        // (String, &str)
        let styles3: Styles = ("color".to_string(), "red").into();
        assert_that(styles3.to_style_string()).is_equal_to("color: red".to_string());
    }

    #[test]
    fn test_empty_styles() {
        let styles = Styles::new();
        assert_that(styles.to_style_string()).is_equal_to(String::new());
    }

    #[test]
    fn test_drilling_down() {
        // Simulates component hierarchy drilling
        let initial: Styles = ("color", "red").into();
        let extended = initial.add(("margin", "5px"));
        let final_styles = extended.add(("padding", "10px"));

        assert_that(final_styles.to_style_string())
            .is_equal_to("color: red; margin: 5px; padding: 10px".to_string());
    }

    #[test]
    fn test_style_enum() {
        let styles: Styles = (Style::BackgroundColor, "red").into();
        assert_that(styles.to_style_string()).is_equal_to("background-color: red".to_string());
    }

    #[test]
    fn test_into_style_to_html() {
        // Test the IntoStyle::to_html method for SSR
        let styles: Styles = [("color", "blue"), ("padding", "20px")].into();
        let mut html = String::new();
        styles.to_html(&mut html);
        // Note: to_html uses ':' without space and ends with ';' (CSS standard format)
        assert_that(html).is_equal_to("color:blue;padding:20px;".to_string());
    }

    #[test]
    fn test_into_style_to_html_empty() {
        let styles = Styles::new();
        let mut html = String::new();
        styles.to_html(&mut html);
        assert_that(html).is_equal_to(String::new());
    }

    #[test]
    fn test_into_style_to_html_with_none_values() {
        // Test that None values are excluded from SSR output
        let styles = Styles::builder()
            .with(("color", "red"))
            .with(("padding", None::<&str>))
            .with(("margin", "10px"))
            .build();
        let mut html = String::new();
        styles.to_html(&mut html);
        assert_that(html).is_equal_to("color:red;margin:10px;".to_string());
    }
}
