// No upstream: React passes a part's props through context on every render, so a part rendered
// again simply reads them again; leptonic's props bind a hook to one element and aren't `Clone`.
//! Props for an element a hook offers: its own element, or one an atom renders elsewhere, possibly
//! again (a label, a description, a section heading, a toggle's button).

use std::{
    fmt,
    panic::Location,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use leptos::{
    either::Either,
    prelude::{ArcTrigger, Notify, Owner, Track, on_cleanup},
};

use crate::{IntoAttrs, PropsWithStyles, utils::styles::Styles};

/// The props of an element a hook offers (every props value a hook returns is a `Part`): an
/// element that may be rendered more than once (inside a `Show`, after its content changed, each
/// time an overlay opens), by the caller or by a child component it hands the part to.
///
/// Every mount takes fresh props with [`props`](Self::props) (or the attributes to spread with
/// [`attrs`](Self::attrs)), built from what the hook shares (ids, signals, event handlers, the
/// element capture), without copying data. One element holds them at a time: props are single-use
/// (`documentation/hooks-implementation.md`), so a second element rendered meanwhile gets none,
/// instead of a duplicate id and a second captured element.
///
/// ```ignore
/// let UseButtonReturn { button, .. } = use_button(input);
/// let (attrs, styles) = button.attrs();
/// view! { <button {..attrs} style=styles>"Save"</button> }
/// ```
///
/// Cloning a `Part` shares it (contexts hand it to the components rendering it).
///
/// # Writing a hook offering a part
///
/// A hook computes its props once and hands them out with [`Part::stored`]: each mount gets a
/// copy ([`Remount`], implemented field-wise for the props type). A hook building on another
/// hook for the same element consumes the other hook's part with [`into_props`](Self::into_props)
/// and builds its own props from them:
///
/// ```ignore
/// pub fn use_my_button(input: UseMyButtonInput) -> UseMyButtonReturn {
///     let UsePressReturn { target: press, is_pressed } = use_press(UsePressInput { .. });
///     let (press, styles) = press.into_props().into_inner();
///     let props = UseMyButtonProps {
///         role: AriaRole::Button,
///         on_pointerdown: press.on_pointerdown,
///         on_keydown: press.on_keydown.chain(handle_keydown),
///         ..
///     };
///     UseMyButtonReturn {
///         button: Part::stored(PropsWithStyles::new(props, styles)),
///         is_pressed,
///     }
/// }
/// ```
pub struct Part<P: 'static>(Arc<dyn Source<P>>);

/// Whether an element holds a part's props, shared by the part and the parts [mapped](Part::map)
/// from it.
struct Lease {
    held: AtomicBool,
    /// Notified when an element takes or gives back the props (see [`Part::observed`]).
    presence: Option<ArcTrigger>,
}

impl Lease {
    fn new(observed: bool) -> Self {
        Self {
            held: AtomicBool::new(false),
            presence: observed.then(ArcTrigger::new),
        }
    }
}

/// The lease of a part's source: not generic, so taking and giving back the props is one function
/// for all parts.
trait Leased: Send + Sync {
    fn lease(&self) -> &Lease;
}

trait Source<P>: Leased {
    fn build(&self) -> P;
    /// The props of a part consumed for good (see [`Part::into_props`]).
    fn into_props(self: Arc<Self>) -> P;
}

/// A part built by a closure.
struct Built<F> {
    lease: Lease,
    build: F,
}

impl<F: Send + Sync> Leased for Built<F> {
    fn lease(&self) -> &Lease {
        &self.lease
    }
}

impl<P, F: Fn() -> P + Send + Sync> Source<P> for Built<F> {
    fn build(&self) -> P {
        (self.build)()
    }

    fn into_props(self: Arc<Self>) -> P {
        (self.build)()
    }
}

/// A part holding the props themselves, giving each mount a copy ([`Remount`]): one
/// implementation per props type, shared by every hook offering such props.
struct Stored<P> {
    lease: Lease,
    props: P,
}

impl<P: Send + Sync> Leased for Stored<P> {
    fn lease(&self) -> &Lease {
        &self.lease
    }
}

impl<P: Remount + Send + Sync> Source<P> for Stored<P> {
    fn build(&self) -> P {
        self.props.remount()
    }

    fn into_props(self: Arc<Self>) -> P {
        // The only holder of the part takes the props themselves.
        match Arc::try_unwrap(self) {
            Ok(stored) => stored.props,
            Err(shared) => shared.props.remount(),
        }
    }
}

/// Props that copy themselves for another mount of their [`Part`] ([`Part::stored`]): the copy
/// shares the original's signals, event handlers and element capture (refcounts and `Copy`
/// values, no data copied). Only a `Part` copies props, under its lease (one element at a time),
/// which is why props types aren't `Clone`.
///
/// Implement it field-wise for the props type of your own hook, naming every field (a field
/// added later then fails to compile until it is copied too):
///
/// ```ignore
/// #[derive(Debug)]
/// pub struct UseMyButtonProps {
///     pub id: Id,                                // refcounted: `clone()`
///     pub disabled: Signal<bool>,                // `Copy`
///     pub element_capture: ElementCaptureAttr,   // refcounted: `clone()`
///     pub on_keydown: EventHandler<KeyboardEvent>, // refcounted (also a chain): `clone()`
/// }
///
/// impl Remount for UseMyButtonProps {
///     fn remount(&self) -> Self {
///         Self {
///             id: self.id.clone(),
///             disabled: self.disabled,
///             element_capture: self.element_capture.clone(),
///             on_keydown: self.on_keydown.clone(),
///         }
///     }
/// }
/// ```
///
/// A field holding data (a `String`, a `Vec`) is copied on every mount: prefer an `Arc` or a
/// signal for it.
pub trait Remount {
    /// The props for another mount.
    #[must_use]
    fn remount(&self) -> Self;
}

impl<P: IntoAttrs + Remount> Remount for PropsWithStyles<P> {
    fn remount(&self) -> Self {
        Self {
            props: self.props.remount(),
            styles: self.styles.clone(),
        }
    }
}

impl<A: Remount, B: Remount> Remount for (A, B) {
    fn remount(&self) -> Self {
        (self.0.remount(), self.1.remount())
    }
}

impl<P: 'static> Part<P> {
    /// A part whose props `build` creates; it runs on every mount, so it should only clone
    /// refcounted or `Copy` values. For props computed once, [`stored`](Self::stored) is
    /// simpler: it copies them with their [`Remount`] implementation.
    pub fn new(build: impl Fn() -> P + Send + Sync + 'static) -> Self {
        Self(Arc::new(Built {
            lease: Lease::new(false),
            build,
        }))
    }

    /// A part whose presence others follow ([`is_held`](Self::is_held)), e.g. a description the
    /// field references only while it is rendered.
    pub fn observed(build: impl Fn() -> P + Send + Sync + 'static) -> Self {
        Self(Arc::new(Built {
            lease: Lease::new(true),
            build,
        }))
    }

    /// Whether an element holds the props: it is rendered (or being rendered), so references to
    /// it resolve. Tracked for an [`observed`](Self::observed) part.
    ///
    /// The lease is taken in the rendering component's body, in the same order on the server and
    /// while hydrating, so a reference following it renders the same on both sides: present if
    /// the part rendered before the reference was rendered, else added once it is.
    #[must_use]
    pub fn is_held(&self) -> bool {
        is_held(self.0.lease())
    }

    /// The props for the element being rendered, held until the current owner is cleaned up
    /// (the element is removed, or rendered again). Call it in the body of the component that
    /// renders the element, so that a re-render gives them back before it takes them again.
    ///
    /// `None`, with a warning in debug builds, while another element holds them.
    ///
    /// To change props before spreading them: take them, change their fields, then call
    /// `into_attrs()`.
    #[must_use]
    #[track_caller]
    pub fn props(&self) -> Option<P> {
        let lease: Arc<dyn Leased> = self.0.clone();
        take(lease, Location::caller()).then(|| self.0.build())
    }

    /// The props for good, consuming the part: for a hook building its own props from another
    /// hook's props for the same element (`use_button` on `use_press`), in its body. The props are
    /// never given back, so the part has no other element afterwards. No lease cleanup (an
    /// allocation) is registered, and a stored part's props are taken as they are.
    ///
    /// # Panics
    ///
    /// If an element holds the props (the part was shared and rendered before it was consumed).
    #[must_use]
    #[track_caller]
    pub fn into_props(self) -> P {
        let lease = self.0.lease();
        assert!(
            !lease.held.swap(true, Ordering::AcqRel),
            "{}: these props are rendered by another element already",
            Location::caller()
        );
        if let Some(presence) = &lease.presence {
            presence.notify();
        }
        self.0.into_props()
    }

    /// A part deriving its props from this part's on every mount: `map` changes them, or builds
    /// other props from them (an atom built on a hook's part, a checkbox's input from a toggle's).
    /// The parts share the lease: one element at a time holds either.
    #[must_use]
    pub fn map<Q: 'static>(self, map: impl Fn(P) -> Q + Send + Sync + 'static) -> Part<Q> {
        Part(Arc::new(Mapped { inner: self.0, map }))
    }
}

impl<P: 'static> Part<P> {
    /// A part giving each mount a copy of `props` (see [`Remount`]): how a hook hands out the
    /// props it computed.
    pub fn stored(props: P) -> Self
    where
        P: Remount + Send + Sync,
    {
        Self(Arc::new(Stored {
            lease: Lease::new(false),
            props,
        }))
    }
}

/// Takes the lease for the current owner, giving it back on the owner's cleanup; `false` (and a
/// warning in debug builds) while another element holds it.
fn take(source: Arc<dyn Leased>, caller: &'static Location<'static>) -> bool {
    let lease = source.lease();
    if lease.held.swap(true, Ordering::AcqRel) {
        crate::utils::dev_warn!(
            "{caller}: these props are rendered by another element already (one element at a time)"
        );
        return false;
    }
    if cfg!(debug_assertions) && Owner::current().is_none() {
        crate::utils::dev_warn!(
            "{caller}: part props taken outside a reactive owner are never given back"
        );
    }
    if let Some(presence) = &lease.presence {
        presence.notify();
    }
    on_cleanup(move || {
        let lease = source.lease();
        lease.held.store(false, Ordering::Release);
        if let Some(presence) = &lease.presence {
            presence.notify();
        }
    });
    true
}

fn is_held(lease: &Lease) -> bool {
    if let Some(presence) = &lease.presence {
        presence.track();
    }
    lease.held.load(Ordering::Acquire)
}

/// A part building another part's props further (see [`Part::map`]): shares its lease.
struct Mapped<P: 'static, F> {
    inner: Arc<dyn Source<P>>,
    map: F,
}

impl<P: 'static, F: Send + Sync> Leased for Mapped<P, F> {
    fn lease(&self) -> &Lease {
        self.inner.lease()
    }
}

impl<P: 'static, Q, F: Fn(P) -> Q + Send + Sync> Source<Q> for Mapped<P, F> {
    fn build(&self) -> Q {
        (self.map)(self.inner.build())
    }

    fn into_props(self: Arc<Self>) -> Q {
        match Arc::try_unwrap(self) {
            Ok(mapped) => (mapped.map)(mapped.inner.into_props()),
            Err(shared) => (shared.map)(shared.inner.build()),
        }
    }
}

impl<P: IntoAttrs + 'static> Part<P> {
    /// The attributes of [`props`](Self::props) to spread onto the element, or none while
    /// another element holds them.
    #[must_use]
    #[track_caller]
    pub fn attrs(&self) -> Either<P::Attrs, ()> {
        match self.props() {
            Some(props) => Either::Left(props.into_attrs()),
            None => Either::Right(()),
        }
    }
}

impl<P: IntoAttrs + 'static> Part<PropsWithStyles<P>> {
    /// The attributes of [`props`](Self::props) to spread onto the element and the hook's styles
    /// to merge with the caller's (one writer of `style`): `style=styles.merge(own)`. None and no
    /// styles while another element holds them.
    #[must_use]
    #[track_caller]
    pub fn attrs(&self) -> (Either<P::Attrs, ()>, Styles) {
        match self.props() {
            Some(props) => {
                let (attrs, styles) = props.into_parts();
                (Either::Left(attrs), styles)
            }
            None => (Either::Right(()), Styles::default()),
        }
    }
}

/// Whether an element is rendered, for references to an element that may be missing (the listbox
/// of a select or combo box, which the trigger's `aria-controls` names only while it is
/// rendered). The element's component [holds](Self::hold) it in its body, as it takes a part's
/// props: the same on the server and while hydrating.
#[derive(Clone, Debug)]
pub struct Presence(Part<()>);

impl Presence {
    #[must_use]
    pub fn new() -> Self {
        Self(Part::observed(|| ()))
    }

    /// Whether the element is rendered (tracked).
    #[must_use]
    pub fn is_present(&self) -> bool {
        self.0.is_held()
    }

    /// Marks the element of the current owner (the component rendering it) present until the
    /// owner is cleaned up.
    #[track_caller]
    pub fn hold(&self) {
        let _ = self.0.props();
    }
}

impl Default for Presence {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: 'static> Clone for Part<P> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<P: 'static> fmt::Debug for Part<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Part")
            .field("held", &self.0.lease().held.load(Ordering::Acquire))
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::with_owner;

    #[test]
    fn one_element_at_a_time() {
        with_owner(|| {
            let part = Part::new(|| "props");
            let first = Owner::current().expect("owner").child();
            assert_that!(first.with(|| part.props())).is_equal_to(Some("props"));
            // While the first element holds them, a second one gets none.
            let second = Owner::current().expect("owner").child();
            assert_that!(second.with(|| part.clone().props())).is_none();
            first.cleanup();
            let third = Owner::current().expect("owner").child();
            assert_that!(third.with(|| part.props())).is_equal_to(Some("props"));
        });
    }

    #[test]
    fn a_re_render_gives_them_back_first() {
        with_owner(|| {
            let part = Part::new(|| 7);
            let scope = Owner::current().expect("owner").child();
            assert_that!(scope.with(|| part.props())).is_equal_to(Some(7));
            // A reactive closure re-renders as `with_cleanup`: the old run's cleanups run before
            // the new run takes the props again.
            assert_that!(scope.with_cleanup(|| part.props())).is_equal_to(Some(7));
        });
    }

    #[test]
    fn an_observed_part_reports_its_presence() {
        with_owner(|| {
            let part = Part::observed(|| ());
            let presence = {
                let part = part.clone();
                leptos::prelude::Memo::new(move |_| part.is_held())
            };
            assert_that!(leptos::prelude::GetUntracked::get_untracked(&presence)).is_false();
            let scope = Owner::current().expect("owner").child();
            let _ = scope.with(|| part.props());
            assert_that!(leptos::prelude::GetUntracked::get_untracked(&presence)).is_true();
            scope.cleanup();
            assert_that!(leptos::prelude::GetUntracked::get_untracked(&presence)).is_false();
        });
    }

    #[derive(Debug, PartialEq)]
    struct Props(Arc<str>);

    impl Remount for Props {
        fn remount(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }

    #[test]
    fn a_stored_part_gives_every_mount_a_copy() {
        with_owner(|| {
            let part = Part::stored(Props(Arc::from("id")));
            let scope = Owner::current().expect("owner").child();
            let first = scope.with(|| part.props()).expect("props");
            assert_that!(scope.with(|| part.props())).is_none();
            scope.cleanup();
            let second = scope.with(|| part.props()).expect("props");
            // Copies share the ingredients.
            assert_that!(Arc::ptr_eq(&first.0, &second.0)).is_true();
        });
    }

    #[test]
    fn a_consumed_part_has_no_other_element() {
        with_owner(|| {
            let part = Part::stored(Props(Arc::from("id")));
            let shared = part.clone();
            let props = part.into_props();
            assert_that!(&*props.0).is_equal_to("id");
            let scope = Owner::current().expect("owner").child();
            assert_that!(scope.with(|| shared.props())).is_none();
            scope.cleanup();
            assert_that!(scope.with(|| shared.props())).is_none();
        });
    }

    #[test]
    fn a_consumed_stored_part_hands_over_its_props() {
        with_owner(|| {
            let id: Arc<str> = Arc::from("id");
            let props = Part::stored(Props(Arc::clone(&id))).into_props();
            // No copy: the part's own props (two refs: `id` and the props).
            assert_that!(Arc::strong_count(&id)).is_equal_to(2);
            drop(props);
        });
    }

    #[test]
    fn a_mapped_part_shares_the_lease() {
        with_owner(|| {
            let part = Part::new(|| 1);
            let doubled = part.clone().map(|n| n * 2);
            let scope = Owner::current().expect("owner").child();
            assert_that!(scope.with(|| doubled.props())).is_equal_to(Some(2));
            // While the mapped part is rendered, the original is held too.
            assert_that!(part.is_held()).is_true();
            assert_that!(scope.with(|| part.props())).is_none();
            scope.cleanup();
            assert_that!(scope.with(|| part.props())).is_equal_to(Some(1));
        });
    }

    #[test]
    fn presence_follows_the_rendering_component() {
        with_owner(|| {
            let presence = Presence::new();
            let present = {
                let presence = presence.clone();
                leptos::prelude::Memo::new(move |_| presence.is_present())
            };
            assert_that!(leptos::prelude::GetUntracked::get_untracked(&present)).is_false();
            let scope = Owner::current().expect("owner").child();
            scope.with(|| presence.hold());
            assert_that!(leptos::prelude::GetUntracked::get_untracked(&present)).is_true();
            scope.cleanup();
            assert_that!(leptos::prelude::GetUntracked::get_untracked(&present)).is_false();
        });
    }

    #[test]
    fn every_mount_builds_fresh_props() {
        with_owner(|| {
            let builds = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            let counter = Arc::clone(&builds);
            let part = Part::new(move || counter.fetch_add(1, Ordering::Relaxed));
            let scope = Owner::current().expect("owner").child();
            let _ = scope.with(|| part.props());
            let _ = scope.with_cleanup(|| part.props());
            assert_that!(builds.load(Ordering::Relaxed)).is_equal_to(2);
        });
    }
}
