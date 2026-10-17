// No upstream: React props are plain values; Leptos' own `MaybeSignal` is deprecated.
//! Values that are either fixed or reactive, for inputs and props that are usually constants.

use std::panic::Location;

use leptos::{
    prelude::*,
    reactive::effect::RenderEffect,
    tachys::{
        html::attribute::AttributeValue,
        renderer::{Rndr, types::Element},
    },
};

/// A value that is either fixed or a [`Signal`]: the type of leptonic's inputs and props that
/// callers mostly set to constants.
///
/// A fixed value costs nothing beyond itself: no allocation, no arena slot, no reactive node. It
/// is read without tracking and without cloning (`with`, `with_untracked`, [`Self::fixed`]), and
/// renders as a plain attribute, without a `RenderEffect`. A signal behaves exactly as the
/// `Signal` it holds. `Default` is the fixed `T::default()`, so an absent optional prop or a
/// `..Default::default()` input field costs nothing either.
///
/// **Why not `Signal<T>`.** Leptos deprecated its own `MaybeSignal` in 0.7.0-rc3 (commit
/// `396327b6`, leptos-rs/leptos#3098) for ergonomics and maintenance, not for performance: once
/// `Signal<T>` was `Copy` and took plain values through `From<T>`, one type served both, with
/// fewer impls to maintain in Leptos and better `From` inference. But `From<T> for Signal<T>` is
/// `Signal::stored`: an `ArcStoredValue` in an arena slot (2 allocations, held until the owner is
/// disposed), read through the reactive path, and rendered through a `RenderEffect` (~10
/// allocations) that never reruns. Negligible for an app's props; not for leptonic, which creates
/// dozens of mostly-fixed inputs per atom and thousands per collection (a Button's stored defaults
/// alone are ~10% of its mount: `documentation/performance.md`, R2 and R3).
///
/// **Conversions** (`#[prop(into)]` and `.into()`): a value (`T`), a `Signal<T>`, `ReadSignal<T>`,
/// `RwSignal<T>` or `Memo<T>` (as `Signal::from` costs them: 1, 2, 1 allocations); for an
/// optional value (`MaybeSignal<Option<T>>`) also a plain `T` (fixed `Some`) and a `Signal<T>`
/// (a derived `Some`, 2 allocations). A closure is no conversion (coherence forbids `From<F>`
/// next to `From<T>`): pass `Signal::derive(f)`. Converting back into a `Signal<T>` costs a fixed
/// value its stored signal; prefer passing the `MaybeSignal` on.
///
/// ```ignore
/// #[component]
/// fn Field(#[prop(into, optional)] is_disabled: MaybeSignal<bool>) -> impl IntoView { .. }
///
/// view! { <Field is_disabled=true/> }                               // fixed: costs nothing
/// view! { <Field is_disabled=disabled/> }                           // an `RwSignal<bool>`
/// view! { <Field is_disabled=Signal::derive(move || busy.get())/> } // a closure, as a signal
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaybeSignal<T: Send + Sync + 'static>(Repr<T>);

/// Private, so that the representation can change (another variant for a cheaper derived value,
/// say) without breaking callers; leptonic's own code reads [`MaybeSignal::fixed`] as well.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Repr<T: Send + Sync + 'static> {
    Fixed(T),
    Reactive(Signal<T>),
}

impl<T: Send + Sync + 'static> MaybeSignal<T> {
    /// The value, if it is fixed: lets code fold fixed values once (`a || b` of two fixed flags is
    /// a fixed flag) instead of deriving a signal.
    pub fn fixed(&self) -> Option<&T> {
        match &self.0 {
            Repr::Fixed(value) => Some(value),
            Repr::Reactive(_) => None,
        }
    }
}

impl<T: Default + Send + Sync + 'static> Default for MaybeSignal<T> {
    fn default() -> Self {
        Self(Repr::Fixed(T::default()))
    }
}

impl<T: Send + Sync + 'static> From<T> for MaybeSignal<T> {
    fn from(value: T) -> Self {
        Self(Repr::Fixed(value))
    }
}

impl<T: Send + Sync + 'static> From<Signal<T>> for MaybeSignal<T> {
    fn from(signal: Signal<T>) -> Self {
        Self(Repr::Reactive(signal))
    }
}

impl<T: Send + Sync + 'static> From<ReadSignal<T>> for MaybeSignal<T> {
    #[track_caller]
    fn from(signal: ReadSignal<T>) -> Self {
        Self(Repr::Reactive(signal.into()))
    }
}

impl<T: Send + Sync + 'static> From<RwSignal<T>> for MaybeSignal<T> {
    #[track_caller]
    fn from(signal: RwSignal<T>) -> Self {
        Self(Repr::Reactive(signal.into()))
    }
}

impl<T: Send + Sync + 'static> From<Memo<T>> for MaybeSignal<T> {
    #[track_caller]
    fn from(memo: Memo<T>) -> Self {
        Self(Repr::Reactive(memo.into()))
    }
}

/// A plain value for an optional one, as `Signal<Option<T>>` takes it.
impl<T: Send + Sync + 'static> From<T> for MaybeSignal<Option<T>> {
    fn from(value: T) -> Self {
        Self(Repr::Fixed(Some(value)))
    }
}

/// A signal of a value for an optional one, as `Signal<Option<T>>` takes it (a derived signal).
impl<T: Clone + Send + Sync + 'static> From<Signal<T>> for MaybeSignal<Option<T>> {
    #[track_caller]
    fn from(signal: Signal<T>) -> Self {
        Self(Repr::Reactive(Signal::derive(move || Some(signal.get()))))
    }
}

/// For inputs that are still `Signal`s: a fixed value becomes a stored signal (2 allocations).
impl<T: Send + Sync + 'static> From<MaybeSignal<T>> for Signal<T> {
    #[track_caller]
    fn from(value: MaybeSignal<T>) -> Self {
        match value.0 {
            Repr::Fixed(value) => Signal::stored(value),
            Repr::Reactive(signal) => signal,
        }
    }
}

impl<T: Send + Sync + 'static> DefinedAt for MaybeSignal<T> {
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        match &self.0 {
            Repr::Fixed(_) => None,
            Repr::Reactive(signal) => signal.defined_at(),
        }
    }
}

impl<T: Send + Sync + 'static> Track for MaybeSignal<T> {
    #[track_caller]
    fn track(&self) {
        if let Repr::Reactive(signal) = &self.0 {
            signal.track();
        }
    }
}

// `With` and `WithUntracked` directly instead of `ReadUntracked` (Leptos' `MaybeSignal`): a read
// guard can't borrow the fixed value, so it would clone it on every read. `Get` and
// `GetUntracked` follow from these.
impl<T: Send + Sync + 'static> WithUntracked for MaybeSignal<T> {
    type Value = T;

    #[track_caller]
    fn try_with_untracked<U>(&self, fun: impl FnOnce(&T) -> U) -> Option<U> {
        match &self.0 {
            Repr::Fixed(value) => Some(fun(value)),
            Repr::Reactive(signal) => signal.try_with_untracked(fun),
        }
    }
}

impl<T: Send + Sync + 'static> With for MaybeSignal<T> {
    type Value = T;

    #[track_caller]
    fn try_with<U>(&self, fun: impl FnOnce(&T) -> U) -> Option<U> {
        match &self.0 {
            Repr::Fixed(value) => Some(fun(value)),
            Repr::Reactive(signal) => signal.try_with(fun),
        }
    }
}

/// The rendered state of a [`MaybeSignal`] attribute value.
pub struct MaybeSignalAttrState<V: AttributeValue<State: 'static>> {
    el: Element,
    inner: AttrState<V>,
}

enum AttrState<V: AttributeValue<State: 'static>> {
    /// Rendered once, as the plain value.
    Fixed(V::State),
    /// The signal's render effect.
    Reactive(RenderEffect<V::State>),
}

/// A fixed value renders as the plain value does (no `RenderEffect`), a signal as a `Signal`
/// does. Server and client render the same: the HTML is the value's either way, and hydration
/// finds it.
impl<V> AttributeValue for MaybeSignal<V>
where
    V: AttributeValue + Clone + Send + Sync + 'static,
    V::State: 'static,
{
    type State = MaybeSignalAttrState<V>;
    type AsyncOutput = Self;
    type Cloneable = Self;
    type CloneableOwned = Self;

    fn html_len(&self) -> usize {
        match &self.0 {
            Repr::Fixed(value) => V::html_len(value),
            Repr::Reactive(signal) => <Signal<V> as AttributeValue>::html_len(signal),
        }
    }

    fn to_html(self, key: &str, buf: &mut String) {
        match self.0 {
            Repr::Fixed(value) => V::to_html(value, key, buf),
            Repr::Reactive(signal) => <Signal<V> as AttributeValue>::to_html(signal, key, buf),
        }
    }

    // Like `Signal`'s: the type doesn't tell whether the value is fixed.
    fn to_template(_key: &str, _buf: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(self, key: &str, el: &Element) -> Self::State {
        let inner = match self.0 {
            Repr::Fixed(value) => AttrState::Fixed(V::hydrate::<FROM_SERVER>(value, key, el)),
            Repr::Reactive(signal) => AttrState::Reactive(
                <Signal<V> as AttributeValue>::hydrate::<FROM_SERVER>(signal, key, el),
            ),
        };
        MaybeSignalAttrState {
            el: el.clone(),
            inner,
        }
    }

    fn build(self, el: &Element, key: &str) -> Self::State {
        let inner = match self.0 {
            Repr::Fixed(value) => AttrState::Fixed(V::build(value, el, key)),
            Repr::Reactive(signal) => {
                AttrState::Reactive(<Signal<V> as AttributeValue>::build(signal, el, key))
            }
        };
        MaybeSignalAttrState {
            el: el.clone(),
            inner,
        }
    }

    fn rebuild(self, key: &str, state: &mut Self::State) {
        let repr = match (self.0, &mut state.inner) {
            (Repr::Fixed(value), AttrState::Fixed(rendered)) => {
                return V::rebuild(value, key, rendered);
            }
            (Repr::Reactive(signal), AttrState::Reactive(rendered)) => {
                return <Signal<V> as AttributeValue>::rebuild(signal, key, rendered);
            }
            (repr, _) => repr,
        };
        // The other kind replaces the rendered value from scratch (dropping a render effect ends
        // it). Building doesn't remove an attribute an absent value leaves behind.
        Rndr::remove_attribute(&state.el, key);
        state.inner = AttributeValue::build(Self(repr), &state.el, key).inner;
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {
        if let Repr::Reactive(signal) = &mut self.0 {
            <Signal<V> as AttributeValue>::dry_resolve(signal);
        }
    }

    fn resolve(self) -> impl Future<Output = Self::AsyncOutput> + Send {
        std::future::ready(self)
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::{
        testing::{count_allocations, flush_effects, with_owner},
        utils::probe::probe_sources,
    };

    #[test]
    fn fixed_values_allocate_nothing_and_track_nothing() {
        with_owner(|| {
            let text = String::from("fixed");
            // The thread's probe observer is created on its first use.
            let _ = probe_sources(|| ());
            let ((value, length, tracked), allocations) = count_allocations(move || {
                let value: MaybeSignal<String> = text.into();
                let length = value.with(String::len) + value.with_untracked(String::len);
                let tracked = probe_sources(|| value.with(String::len)).1;
                (value, length, tracked)
            });
            assert_that!(allocations).is_equal_to(0);
            assert_that!(length).is_equal_to(10);
            assert_that!(tracked).is_false();
            assert_that!(value.fixed().map(String::as_str)).is_equal_to(Some("fixed"));
        });
    }

    #[test]
    fn default_is_fixed_and_allocates_nothing() {
        with_owner(|| {
            let (value, allocations) = count_allocations(MaybeSignal::<Option<String>>::default);
            assert_that!(allocations).is_equal_to(0);
            assert_that!(value.fixed()).is_equal_to(Some(&None));
        });
    }

    /// A stored `Signal` costs what a fixed `MaybeSignal` saves: its value's `Arc` and arena box
    /// (`performance.md`), plus the growth of the arena and the owner's node list.
    #[test]
    fn a_stored_signal_allocates() {
        with_owner(|| {
            let (_, allocations) = count_allocations(|| Signal::stored(false));
            assert_that!(allocations).is_greater_or_equal_to(2);
        });
    }

    #[test]
    fn signals_stay_reactive() {
        with_owner(|| {
            let source = RwSignal::new(1);
            let value: MaybeSignal<i32> = Signal::derive(move || source.get() * 2).into();
            let seen = StoredValue::new(Vec::new());
            Effect::new(move || {
                let doubled = value.get();
                seen.update_value(|seen| seen.push(doubled));
            });
            flush_effects();
            source.set(2);
            flush_effects();
            assert_that!(value.fixed()).is_none();
            assert_that!(value.get_untracked()).is_equal_to(4);
            assert_that!(probe_sources(|| value.track()).1).is_true();
            assert_that!(seen.get_value()).contains_exactly([2, 4]);
        });
    }

    #[test]
    fn conversions() {
        with_owner(|| {
            let rw = RwSignal::new(3);
            let memo = Memo::new(move |_| rw.get() + 1);
            assert_that!(MaybeSignal::<i32>::from(5).get()).is_equal_to(5);
            assert_that!(MaybeSignal::<i32>::from(rw).get()).is_equal_to(3);
            assert_that!(MaybeSignal::<i32>::from(rw.read_only()).get()).is_equal_to(3);
            assert_that!(MaybeSignal::<i32>::from(memo).get()).is_equal_to(4);

            let some: MaybeSignal<Option<i32>> = 5.into();
            assert_that!(some.fixed()).is_equal_to(Some(&Some(5)));
            let none: MaybeSignal<Option<i32>> = None.into();
            assert_that!(none.fixed()).is_equal_to(Some(&None));
            let derived: MaybeSignal<Option<i32>> = Signal::<i32>::from(rw).into();
            rw.set(7);
            assert_that!(derived.get()).is_equal_to(Some(7));

            let stored: Signal<i32> = MaybeSignal::<i32>::from(9).into();
            assert_that!(stored.get()).is_equal_to(9);
            // A signal passes back unchanged, no new signal.
            let signal = Signal::<i32>::from(rw);
            let passed_on: Signal<i32> = MaybeSignal::from(signal).into();
            assert_that!(passed_on).is_equal_to(signal);
        });
    }

    /// The server renders both kinds as the value (the client's render and hydration rely on it).
    #[test]
    fn both_kinds_render_the_same_html() {
        with_owner(|| {
            let fixed: MaybeSignal<Option<&'static str>> = Some("menu").into();
            let reactive: MaybeSignal<Option<&'static str>> =
                Signal::derive(|| Some("menu")).into();
            let absent = MaybeSignal::<Option<&'static str>>::default();
            let mut html = [String::new(), String::new(), String::new()];
            fixed.to_html("aria-haspopup", &mut html[0]);
            reactive.to_html("aria-haspopup", &mut html[1]);
            absent.to_html("aria-haspopup", &mut html[2]);
            assert_that!(&html[0]).is_equal_to(&html[1]);
            assert_that!(html[0].as_str()).is_equal_to(" aria-haspopup=\"menu\"");
            assert_that!(html[2].as_str()).is_equal_to("");
        });
    }
}
