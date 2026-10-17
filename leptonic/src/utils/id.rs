// Upstream: react-aria/src/utils/useId.ts @ 99e6102368
// Upstream: react-aria/test/utils/useId.test.jsx @ 99e6102368
//! Element ids that are identical on the server and on the client.
//!
//! Hooks link elements through ids: `aria-labelledby`, `aria-controls`, `aria-activedescendant`,
//! `for`, ... With random ids, the server renders one set of ids and the hydrating client
//! computes another. Attributes the client updates later then point at ids that don't exist.
//!
//! [`use_id`] draws from the same counter Leptos uses for resources: server and client create
//! components in the same order, so they produce the same ids. On the client, after hydration
//! (and in client-side rendered apps), ids come from a separate counter with a `c` marker, so they
//! never collide with server-generated ones.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `use_id(prefix)` always generates an id, `<prefix>-<n>` (react-aria's `useId(defaultId)`
//   returns a given default id; callers with an id of their own use it instead).
//
// ## DIFFERENT BEHAVIOR
// - Ids come from Leptos' shared-context counter while rendering on the server and hydrating,
//   else from a client counter (react-aria: React's `useId`).
//
// ## OMITTED FEATURES
// - `mergeIds` and the id updater map (re-rendering the users of an id that changed): leptonic's
//   ids never change once created.
//
// =============================================================================

use std::{
    fmt::{self, Write as _},
    ops::Deref,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use leptos::{
    oco::Oco,
    prelude::Owner,
    tachys::{html::attribute::AttributeValue, renderer::types::Element},
};

static CLIENT_ID: AtomicUsize = AtomicUsize::new(0);

/// A document-unique id like `"listbox-17"`, stable between server-side rendering and hydration.
///
/// Call it while a component or hook is being created, unconditionally and in the same order on
/// the server and the client: never inside an effect, an event handler, or a code path that only
/// exists on one side (e.g. behind `#[cfg(feature = "ssr")]`). Otherwise the server and the client
/// disagree on all ids created afterwards.
pub fn use_id(prefix: &'static str) -> Id {
    let shared = Owner::current_shared_context()
        .filter(|context| !context.is_browser() || context.during_hydration());
    match shared {
        Some(context) => Id::derived(format_args!("{prefix}-{}", context.next_id().into_inner())),
        None => Id::derived(format_args!(
            "{prefix}-c{}",
            CLIENT_ID.fetch_add(1, Ordering::Relaxed)
        )),
    }
}

/// An element id, or a space-separated list of ids (an ARIA id reference such as
/// `aria-labelledby`).
///
/// Ids come from [`use_id`] (generated, identical on the server and the client) or from the app
/// (an element's own `id`, references to the app's elements: `Id::from("heading")`). A clone never
/// allocates (a static string or a shared one), and an `Id` renders as an attribute value without
/// being copied, so ids are passed around, stored in signals and read freely.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(
    /// Never `Oco::Owned`: cloning an owned `Oco` allocates on every clone.
    Oco<'static, str>,
);

impl Id {
    /// The id as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// An id derived from other ids or keys (an option's id from its listbox's, a joined id
    /// list). Allocates once (the shared string) for ids of up to 64 bytes, and nothing for a
    /// static format string.
    pub(crate) fn derived(args: fmt::Arguments<'_>) -> Self {
        if let Some(text) = args.as_str() {
            return Self(Oco::Borrowed(text));
        }
        let mut buffer = StackBuffer::default();
        // Writing to a `StackBuffer` never fails.
        let _ = buffer.write_fmt(args);
        Self(Oco::Counted(Arc::from(buffer.as_str())))
    }
}

impl Id {
    /// Whether this is an id list as [`IdListBuilder`] writes it: single spaces between ids, each
    /// id once.
    pub(crate) fn is_joined(&self) -> bool {
        let text = self.as_str();
        !text.is_empty()
            && !text.starts_with(' ')
            && !text.ends_with(' ')
            && !text.contains("  ")
            && !text.contains(['\t', '\n', '\r', '\x0c'])
            && text
                .split(' ')
                .enumerate()
                .all(|(index, id)| !text.split(' ').take(index).any(|before| before == id))
    }
}

/// Builds an id list (`aria-labelledby`, ...) on the stack: every id once, in order; allocates
/// once (the shared string).
///
/// ```ignore
/// let mut ids = IdListBuilder::default();
/// ids.push(&label_id);
/// ids.push("progress-1 hint-2");
/// let labelledby: Option<Id> = ids.finish();
/// ```
#[derive(Default)]
pub struct IdListBuilder(StackBuffer);

impl IdListBuilder {
    /// Adds the ids of `list` (one id or a space-separated list) that aren't in the list yet.
    pub fn push(&mut self, list: &str) {
        for id in list.split_whitespace() {
            if self.0.as_str().split(' ').any(|known| known == id) {
                continue;
            }
            if self.0.len() > 0 {
                let _ = self.0.write_str(" ");
            }
            let _ = self.0.write_str(id);
        }
    }

    /// The list; `None` without ids.
    #[must_use]
    pub fn finish(self) -> Option<Id> {
        (self.0.len() > 0).then(|| Id(Oco::Counted(Arc::from(self.0.as_str()))))
    }
}

/// The present ids (or id lists) joined into one list: the only present one as it is (no
/// allocation, if it is a joined list already), else one shared string. For a hook's id
/// reference combining several sources (its own `aria_describedby`, a description's id, ...).
pub fn join_ids(ids: impl IntoIterator<Item = Option<Id>>) -> Option<Id> {
    let mut builder = IdListBuilder::default();
    let mut first = None;
    let mut present = 0_usize;
    for id in ids.into_iter().flatten() {
        present += 1;
        builder.push(&id);
        if first.is_none() {
            first = Some(id);
        }
    }
    match first {
        Some(first) if present == 1 && first.is_joined() => Some(first),
        _ => builder.finish(),
    }
}

impl From<&'static str> for Id {
    /// An app-given id (no allocation).
    fn from(id: &'static str) -> Self {
        Self(Oco::Borrowed(id))
    }
}

impl From<String> for Id {
    /// An app-given id (one allocation: the shared string).
    fn from(id: String) -> Self {
        Self(Oco::Counted(Arc::from(id)))
    }
}

impl From<Arc<str>> for Id {
    fn from(id: Arc<str>) -> Self {
        Self(Oco::Counted(id))
    }
}

impl Id {
    /// The id as a shared string (no allocation for a generated id: it shares the string).
    pub(crate) fn into_arc(self) -> Arc<str> {
        match self.0 {
            Oco::Counted(id) => id,
            id => Arc::from(id.as_str()),
        }
    }
}

impl From<Id> for Oco<'static, str> {
    /// The id as text, e.g. a generated form field `name` (no allocation).
    fn from(id: Id) -> Self {
        id.0
    }
}

impl Deref for Id {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for Id {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Id {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

/// Renders exactly as an `Oco<'static, str>`: the shared string is the attribute value, neither
/// copied when the attribute is built nor when it is cloned for a reactive attribute.
impl AttributeValue for Id {
    type State = <Oco<'static, str> as AttributeValue>::State;
    type AsyncOutput = Self;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        self.0.len()
    }

    fn to_html(self, key: &str, buf: &mut String) {
        self.0.to_html(key, buf);
    }

    fn to_template(key: &str, buf: &mut String) {
        <Oco<'static, str> as AttributeValue>::to_template(key, buf);
    }

    fn hydrate<const FROM_SERVER: bool>(self, key: &str, el: &Element) -> Self::State {
        self.0.hydrate::<FROM_SERVER>(key, el)
    }

    fn build(self, el: &Element, key: &str) -> Self::State {
        self.0.build(el, key)
    }

    fn rebuild(self, key: &str, state: &mut Self::State) {
        self.0.rebuild(key, state);
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {}

    fn resolve(self) -> impl Future<Output = Self::AsyncOutput> + Send {
        std::future::ready(self)
    }
}

impl crate::utils::signal_attr::ToAttrText for Id {
    fn to_attr_text(&self) -> crate::utils::signal_attr::AttrText {
        crate::utils::signal_attr::AttrText::Text(self.0.clone())
    }

    fn into_attr_text(self) -> crate::utils::signal_attr::AttrText {
        crate::utils::signal_attr::AttrText::Text(self.0)
    }
}

/// An atom's id reference prop (`aria_labelledby`, `aria_describedby`, ...) as a hook input: an
/// absent or fixed prop is converted once (no per-read work), a reactive one per read.
///
/// ```ignore
/// use_button(UseButtonInput { aria_describedby: id_ref_prop(aria_describedby), .. })
/// ```
#[cfg(feature = "atoms")]
pub fn id_ref_prop(
    prop: leptos::prelude::MaybeProp<String>,
) -> leptos::prelude::Signal<Option<Id>> {
    use leptos::prelude::{Get, Signal, With};

    use crate::utils::probe::probe_sources;

    let Some(ids) = Option::<Signal<Option<String>>>::from(prop) else {
        return Signal::stored(None);
    };
    let (value, is_reactive) = probe_sources(|| ids.get());
    if !is_reactive {
        return Signal::stored(value.map(Id::from));
    }
    Signal::derive(move || ids.with(|ids| ids.as_deref().map(|ids| Id::from(ids.to_owned()))))
}

/// Formats short strings without a heap allocation; longer ones spill into a `String`.
struct StackBuffer {
    bytes: [u8; 64],
    len: usize,
    spilled: Option<String>,
}

impl Default for StackBuffer {
    fn default() -> Self {
        Self {
            bytes: [0; 64],
            len: 0,
            spilled: None,
        }
    }
}

impl StackBuffer {
    fn len(&self) -> usize {
        self.spilled.as_ref().map_or(self.len, String::len)
    }

    fn as_str(&self) -> &str {
        match &self.spilled {
            Some(spilled) => spilled,
            // Only whole `&str`s are copied in, so the bytes are valid UTF-8.
            None => std::str::from_utf8(&self.bytes[..self.len]).unwrap_or_default(),
        }
    }
}

impl fmt::Write for StackBuffer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        if let Some(spilled) = &mut self.spilled {
            spilled.push_str(text);
            return Ok(());
        }
        let end = self.len + text.len();
        if let Some(target) = self.bytes.get_mut(self.len..end) {
            target.copy_from_slice(text.as_bytes());
            self.len = end;
        } else {
            let mut spilled = String::with_capacity(end);
            spilled.push_str(self.as_str());
            spilled.push_str(text);
            self.spilled = Some(spilled);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    #[test]
    fn ids_are_unique_and_shared() {
        with_owner(|| {
            let a = use_id("listbox");
            let b = use_id("listbox");
            assert_that!(a.as_str()).starts_with("listbox-c");
            assert_that!(a.clone()).is_not_equal_to(b);
            assert_that!(a.0.is_counted()).is_true();
            // A clone shares the string.
            assert_that!(a.clone().as_ptr()).is_equal_to(a.as_ptr());
        });
    }

    #[test]
    fn app_ids_are_never_owned() {
        let literal = Id::from("heading");
        assert_that!(literal.0.is_borrowed()).is_true();
        let dynamic = Id::from(String::from("row-7"));
        assert_that!(dynamic.0.is_counted()).is_true();
        assert_that!(dynamic.clone().as_ptr()).is_equal_to(dynamic.as_ptr());
        assert_that!(dynamic == "row-7").is_true();
    }

    #[test]
    fn derived_ids_format_short_and_long_ids() {
        let short = Id::derived(format_args!("{}-option-{}", "listbox-3", 12));
        assert_that!(short.as_str()).is_equal_to("listbox-3-option-12");
        let long_part = "x".repeat(70);
        let long = Id::derived(format_args!("{}-{long_part}", "a"));
        assert_that!(long.as_str()).is_equal_to(format!("a-{long_part}").as_str());
        assert_that!(long.0.is_counted()).is_true();
        let fixed = Id::derived(format_args!("static"));
        assert_that!(fixed.0.is_borrowed()).is_true();
    }

    #[test]
    fn joins_id_lists() {
        let join = |lists: &[&str]| {
            let mut builder = IdListBuilder::default();
            for list in lists {
                builder.push(list);
            }
            builder.finish()
        };
        let joined = join(&["a  b", " c ", "b a d"]).expect("ids");
        assert_that!(joined.as_str()).is_equal_to("a b c d");
        assert_that!(joined.is_joined()).is_true();
        assert_that!(join(&["", "  "])).is_none();
        assert_that!(Id::from(" a").is_joined()).is_false();
        assert_that!(Id::from("a  b").is_joined()).is_false();
        assert_that!(Id::from("a b a").is_joined()).is_false();
        assert_that!(Id::from("a\tb").is_joined()).is_false();
        assert_that!(Id::from("a").is_joined()).is_true();
    }

    #[test]
    fn renders_as_its_text() {
        let mut html = String::new();
        Id::from("a\"b").to_html("id", &mut html);
        assert_that!(html.as_str()).is_equal_to(" id=\"a&quot;b\"");
    }
}
