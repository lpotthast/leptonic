use std::{backtrace::Backtrace, borrow::Cow};

use leptos::{
    prelude::{Get, Signal},
    tachys::{
        html::class::IntoClass,
        renderer::{dom::Element, Rndr},
    },
    typed_builder::TypedBuilder,
};
use reactive_graph::{effect::RenderEffect, signal::ReadSignal};
use smallvec::SmallVec;

/// Fast FNV-1a hash for quick string comparison.
/// Used to avoid full string comparison when checking if classes changed.
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

/// Internal wrapper around `SmallVec<[ClassEntry; 3]>` that provides duplicate detection in debug builds.
///
/// Uses `SmallVec` to avoid heap allocation for the common case of ≤3 class entries.
///
/// Note: The TypedBuilder derive requires this to be pub.
#[doc(hidden)]
#[derive(Clone, Debug, Default)]
pub struct ClassList(SmallVec<[ClassEntry; 3]>);

impl ClassList {
    fn push(&mut self, class: ClassEntry) {
        #[cfg(debug_assertions)]
        {
            if self.0.iter().any(|it| it.name() == class.name()) {
                let backtrace = Backtrace::force_capture();
                tracing::warn!(
                    "Duplicate class '{}' added to Classes. This may indicate a bug. At: {backtrace}",
                    class.name()
                );
            }
        }
        self.0.push(class);
    }

    fn iter(&self) -> impl Iterator<Item = &ClassEntry> {
        self.0.iter()
    }
}

/// Leptos component-prop-utility to drill down a list of classes.
///
/// # Duplicate Handling
///
/// Duplicate class names are allowed and will not be deduplicated. However, in debug builds,
/// a warning will be logged when the same class name is added multiple times. This behavior
/// helps identify potential bugs where classes are unintentionally added twice.
///
/// The final class string produced by [`Classes::to_class_string`] may contain repeated
/// class names if duplicates were added.
///
/// # Example
/// ```rust
/// use leptos::prelude::*;
/// use leptonic::utils::classes::Classes;
///
/// /// The lowest-level component renders the class-list into an actual HTML element.
/// #[component]
/// fn NeedingClasses(
///     #[prop(into, optional)] classes: Classes,
/// ) -> impl IntoView {
///     view! {
///         <div class=classes/>
///     }
/// }
///
/// /// Components sitting in the middle can add their own classes.
/// #[component]
/// fn ExtendingClasses(
///     #[prop(into, optional)] classes: Classes,
/// ) -> impl IntoView {
///     view! {
///         <NeedingClasses classes=classes.add("additional-class")/>
///     }
/// }
///
/// /// Root component defines the initial classes using a builder pattern.
/// #[component]
/// fn ProvidingClasses() -> impl IntoView {
///     let (show_second, _) = signal(true);
///     view! {
///         <ExtendingClasses classes="single-class"/>
///         <ExtendingClasses classes=Classes::builder()
///             .with("first")
///             .with(("second", show_second))
///             .build()/>
///     }
/// }
/// ```
#[derive(Clone, Debug, Default, TypedBuilder)]
#[builder(mutators(
    pub fn with(&mut self, class: impl Into<ClassEntry>) {
        self.classes.push(class.into());
    }
    pub fn with_all<C: Into<ClassEntry>>(&mut self, iter: impl Iterator<Item = C>) {
        for class in iter {
            self.classes.push(class.into());
        }
    }
))]
pub struct Classes {
    #[builder(via_mutators)]
    classes: ClassList,
}

/// A class name that can be either a static string slice or an owned `String`.
///
/// Use `Cow<'static, str>` to avoid allocations for static class names while
/// still supporting dynamically constructed class names when needed.
pub type ClassName = Cow<'static, str>;

#[derive(Clone, Debug)]
pub struct ClassEntry {
    name: ClassName,
    when: Signal<bool>,
}

impl ClassEntry {
    pub fn reactive(name: impl Into<ClassName>, when: impl Into<Signal<bool>>) -> Self {
        Self {
            name: name.into(),
            when: when.into(),
        }
    }

    pub fn always(name: impl Into<ClassName>) -> Self {
        Self {
            name: name.into(),
            when: true.into(),
        }
    }

    /// Returns a reference to the class name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Classes {
    pub fn new() -> Self {
        Self {
            classes: ClassList::default(),
        }
    }

    /// Add one additional class to this class-list.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, entry: impl Into<ClassEntry>) -> Self {
        self.classes.push(entry.into());
        self
    }

    /// Add multiple classes to this class-list.
    pub fn add_all<C: Into<ClassEntry>>(&mut self, iter: impl Iterator<Item = C>) {
        for class in iter {
            self.classes.push(class.into());
        }
    }

    /// Appends all active classes to the given string buffer.
    ///
    /// This method is zero-allocation when the buffer has sufficient capacity.
    pub(crate) fn write_class_string(&self, buf: &mut String) {
        let mut first = true;
        for entry in self.classes.iter() {
            if entry.when.get() {
                if !first {
                    buf.push(' ');
                }
                first = false;
                buf.push_str(entry.name());
            }
        }
    }

    /// Reactively combines all defined classes into one space-separated String.
    ///
    /// Note: Prefer using `class=classes` directly (via `IntoClass`) for better performance,
    /// as it reuses the string buffer across reactive updates.
    #[allow(dead_code)]
    pub(crate) fn to_class_string(&self) -> String {
        let mut s = String::new();
        self.write_class_string(&mut s);
        s
    }
}

/// State for the `IntoClass` implementation of `Classes`.
///
/// Uses `RenderEffect` to automatically track reactive dependencies within the classes
/// and update the DOM when any tracked signal changes.
pub struct ClassesState {
    effect: RenderEffect<(Element, String)>,
}

impl IntoClass for Classes {
    type AsyncOutput = Self;
    type State = ClassesState;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        // Estimate: sum of class names + spaces
        self.classes
            .iter()
            .map(|class| class.name().len() + 1)
            .sum()
    }

    fn to_html(self, class: &mut String) {
        // SSR path: build class string directly, avoiding intermediate allocations.
        let mut first = class.is_empty();
        for entry in self.classes.iter() {
            if entry.when.get() {
                if !first {
                    class.push(' ');
                }
                first = false;
                class.push_str(entry.name());
            }
        }
    }

    fn should_overwrite(&self) -> bool {
        true // This represents a full class attribute value
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        let el = el.clone();
        ClassesState {
            effect: RenderEffect::new(move |prev: Option<(Element, String)>| {
                let (el, mut class_str) = prev.unwrap_or_else(|| (el.clone(), String::new()));
                let prev_len = class_str.len();
                let prev_hash = fxhash(&class_str);
                class_str.clear();
                self.write_class_string(&mut class_str);

                // Subsequent runs: update DOM if class changed
                // Use length + hash for fast inequality check before string comparison
                if prev_len > 0 && (class_str.len() != prev_len || fxhash(&class_str) != prev_hash)
                {
                    if class_str.is_empty() {
                        Rndr::remove_attribute(&el, "class");
                    } else {
                        Rndr::set_attribute(&el, "class", &class_str);
                    }
                }
                (el, class_str)
            }),
        }
    }

    fn build(self, el: &Element) -> Self::State {
        let el = el.clone();
        ClassesState {
            effect: RenderEffect::new(move |prev: Option<(Element, String)>| {
                if let Some((el, mut class_str)) = prev {
                    // Subsequent runs: update DOM if class changed
                    let prev_len = class_str.len();
                    let prev_hash = fxhash(&class_str);
                    class_str.clear();
                    self.write_class_string(&mut class_str);

                    if class_str.len() != prev_len || fxhash(&class_str) != prev_hash {
                        if class_str.is_empty() {
                            Rndr::remove_attribute(&el, "class");
                        } else {
                            Rndr::set_attribute(&el, "class", &class_str);
                        }
                    }
                    (el, class_str)
                } else {
                    // First run: set initial attribute
                    let mut class_str = String::new();
                    self.write_class_string(&mut class_str);
                    if !class_str.is_empty() {
                        Rndr::set_attribute(&el, "class", &class_str);
                    }
                    (el.clone(), class_str)
                }
            }),
        }
    }

    fn rebuild(self, state: &mut Self::State) {
        let prev_value = state.effect.take_value();
        if let Some((el, prev_str)) = prev_value {
            state.effect = RenderEffect::new_with_value(
                move |prev: Option<(Element, String)>| {
                    if let Some((el, mut class_str)) = prev {
                        let prev_len = class_str.len();
                        let prev_hash = fxhash(&class_str);
                        class_str.clear();
                        self.write_class_string(&mut class_str);

                        if class_str.len() != prev_len || fxhash(&class_str) != prev_hash {
                            if class_str.is_empty() {
                                Rndr::remove_attribute(&el, "class");
                            } else {
                                Rndr::set_attribute(&el, "class", &class_str);
                            }
                        }
                        (el, class_str)
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
        for entry in self.classes.iter() {
            let _ = entry.when.get();
        }
    }

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn reset(state: &mut Self::State) {
        if let Some((el, _)) = state.effect.take_value() {
            Rndr::remove_attribute(&el, "class");
        }
    }
}

impl From<&'static str> for ClassEntry {
    fn from(s: &'static str) -> Self {
        ClassEntry::always(s)
    }
}

impl From<String> for ClassEntry {
    fn from(s: String) -> Self {
        ClassEntry::always(s)
    }
}

impl From<(&'static str, bool)> for ClassEntry {
    fn from((name, when): (&'static str, bool)) -> Self {
        ClassEntry::reactive(name, when)
    }
}

impl From<(&'static str, ReadSignal<bool>)> for ClassEntry {
    fn from((name, when): (&'static str, ReadSignal<bool>)) -> Self {
        let when: Signal<bool> = when.into();
        ClassEntry::reactive(name, when)
    }
}

impl From<(&'static str, Signal<bool>)> for ClassEntry {
    fn from((name, when): (&'static str, Signal<bool>)) -> Self {
        ClassEntry::reactive(name, when)
    }
}

impl From<(String, bool)> for ClassEntry {
    fn from((name, when): (String, bool)) -> Self {
        ClassEntry::reactive(name, when)
    }
}

impl From<(String, Signal<bool>)> for ClassEntry {
    fn from((name, when): (String, Signal<bool>)) -> Self {
        ClassEntry::reactive(name, when)
    }
}

impl From<&'static str> for Classes {
    fn from(name: &'static str) -> Self {
        Classes::builder().with(name).build()
    }
}

impl From<String> for Classes {
    fn from(name: String) -> Self {
        Classes::builder().with(name).build()
    }
}

impl From<&[&'static str]> for Classes {
    fn from(names: &[&'static str]) -> Self {
        Classes::builder().with_all(names.iter().copied()).build()
    }
}

impl From<(&'static str, bool)> for Classes {
    fn from((name, when): (&'static str, bool)) -> Self {
        Classes::builder().with((name, when)).build()
    }
}

impl From<(&'static str, Signal<bool>)> for Classes {
    fn from((name, when): (&'static str, Signal<bool>)) -> Self {
        Classes::builder().with((name, when)).build()
    }
}

impl From<(String, bool)> for Classes {
    fn from((name, when): (String, bool)) -> Self {
        Classes::builder().with((name, when)).build()
    }
}

impl From<(String, Signal<bool>)> for Classes {
    fn from((name, when): (String, Signal<bool>)) -> Self {
        Classes::builder().with((name, when)).build()
    }
}

impl<const N: usize> From<[(&'static str, bool); N]> for Classes {
    fn from(names: [(&'static str, bool); N]) -> Self {
        Classes::builder().with_all(names.into_iter()).build()
    }
}

impl<const N: usize> From<[(&'static str, Signal<bool>); N]> for Classes {
    fn from(names: [(&'static str, Signal<bool>); N]) -> Self {
        Classes::builder().with_all(names.into_iter()).build()
    }
}

impl<const N: usize> From<[(String, bool); N]> for Classes {
    fn from(names: [(String, bool); N]) -> Self {
        Classes::builder().with_all(names.into_iter()).build()
    }
}

impl<const N: usize> From<[(String, Signal<bool>); N]> for Classes {
    fn from(names: [(String, Signal<bool>); N]) -> Self {
        Classes::builder().with_all(names.into_iter()).build()
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn test_single_class() {
        let classes: Classes = "foo".into();
        assert_that(classes.to_class_string()).is_equal_to("foo");
    }

    #[test]
    fn test_multiple_classes() {
        let classes = Classes::builder().with("foo").with("bar").build();
        assert_that(classes.to_class_string()).is_equal_to("foo bar");
    }

    #[test]
    fn test_conditional_class_true() {
        let classes: Classes = ("foo", true).into();
        assert_that(classes.to_class_string()).is_equal_to("foo".to_string());
    }

    #[test]
    fn test_conditional_class_false() {
        let classes: Classes = ("foo", false).into();
        assert_that(classes.to_class_string()).is_equal_to(String::new());
    }

    #[test]
    fn test_mixed_conditional_classes() {
        let classes = Classes::builder()
            .with(("always", true))
            .with(("never", false))
            .with(("also-always", true))
            .build();
        assert_that(classes.to_class_string()).is_equal_to("always also-always".to_string());
    }

    #[test]
    fn test_add_method() {
        let classes = Classes::new().add("foo").add("bar");
        assert_that(classes.to_class_string()).is_equal_to("foo bar".to_string());
    }

    #[test]
    fn test_empty_classes() {
        let classes = Classes::new();
        assert_that(classes.to_class_string()).is_equal_to(String::new());
    }

    #[test]
    fn test_drilling_down() {
        let initial: Classes = "base".into();
        let extended = initial.add("extended");
        let final_classes = extended.add("final");
        assert_that(final_classes.to_class_string()).is_equal_to("base extended final".to_string());
    }

    #[test]
    fn test_into_class_to_html() {
        // Test the IntoClass::to_html method for SSR
        let classes = Classes::builder().with("foo").with("bar").build();
        let mut html = String::new();
        classes.to_html(&mut html);
        assert_that(html).is_equal_to("foo bar".to_string());
    }

    #[test]
    fn test_into_class_to_html_empty() {
        let classes = Classes::new();
        let mut html = String::new();
        classes.to_html(&mut html);
        assert_that(html).is_equal_to(String::new());
    }

    #[test]
    fn test_into_class_to_html_with_false_conditions() {
        let classes = Classes::builder()
            .with(("active", true))
            .with(("disabled", false))
            .with(("visible", true))
            .build();
        let mut html = String::new();
        classes.to_html(&mut html);
        assert_that(html).is_equal_to("active visible".to_string());
    }

    #[test]
    fn test_into_class_to_html_appends_to_existing() {
        let classes = Classes::builder().with("new-class").build();
        let mut html = String::from("existing");
        classes.to_html(&mut html);
        assert_that(html).is_equal_to("existing new-class".to_string());
    }

    #[test]
    fn test_should_overwrite() {
        let classes = Classes::new();
        assert_that(classes.should_overwrite()).is_true();
    }

    #[test]
    fn test_html_len_estimate() {
        let classes = Classes::builder().with("foo").with("bar").build();
        // "foo" (3) + 1 space estimate + "bar" (3) + 1 space estimate = 8
        assert_that(classes.html_len()).is_equal_to(8);
    }
}
