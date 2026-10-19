// Upstream: react-aria-components/src/Disclosure.tsx @ 99e6102368
// Upstream: react-aria-components/test/Disclosure.test.js @ 99e6102368
// Upstream: react-aria-components/test/Disclosure.ssr.test.js @ 99e6102368
use std::collections::HashSet;

use leptos::{ev, prelude::*};
use leptos_classes::Classes;

use super::press::{PressResponder, PressResponderProps};
use crate::{
    CapturedElement, IntoAttrs, Out, Part, ValueBinding,
    hooks::{
        collections::Key,
        disclosure::{
            DisclosureGroupExpansion, DisclosureGroupState, DisclosureState,
            UseDisclosureGroupStateInput, UseDisclosureInput, UseDisclosurePanelProps,
            UseDisclosureReturn, UseDisclosureStateInput, use_disclosure,
            use_disclosure_group_state, use_disclosure_state,
        },
        focus::{FocusRingTarget, UseFocusRingInput, use_focus_ring},
        interactions::{PressEvent, PressResponderTrigger},
    },
    utils::{
        aria::AriaRole, data_attributes::data_flags, default_class::with_default_class, id::use_id,
        probe::either, scoped_context::scoped_view, styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The trigger is the pressable atom (a `Button`) inside a `DisclosureTrigger`, which hands it
//   the props through a `PressResponder` (react-aria-components: a `Button` with `slot="trigger"`
//   through `ButtonContext`).
// - The panel is named by the trigger's id (the trigger `Button`'s, known while rendering).
// - Expanded state (C4): `default_expanded` + `on_expanded_change`, or `is_expanded` +
//   `set_expanded`; the group's `expanded_keys` likewise.
// - Render props become `data-*` attributes plus plain children.
//
// =============================================================================

/// Context from a [`Disclosure`] to its [`DisclosurePanel`].
#[derive(Clone)]
struct DisclosureContext {
    /// The panel (its role and labelling are the panel's and the trigger's).
    panel: Part<UseDisclosurePanelProps>,
    /// The trigger's id while it is rendered.
    labelled_by: Signal<Option<crate::Id>>,
    /// The trigger's press handlers and ARIA props: one trigger at a time.
    trigger: Part<TriggerPress>,
}

/// What the [`DisclosureTrigger`] hands its button.
#[derive(Debug, Clone, Copy)]
struct TriggerPress {
    on_press: Option<Callback<PressEvent>>,
    on_press_start: Option<Callback<PressEvent>>,
    is_disabled: Signal<bool>,
    trigger: PressResponderTrigger,
}

/// A group of [`Disclosure`]s (an accordion): by default, expanding one collapses the others.
/// Give each disclosure a `key` to address it in `default_expanded_keys` and `expanded_keys`.
///
/// Data attributes: `data-disabled`.
///
/// Default class: `leptonic-DisclosureGroup`.
#[component]
#[allow(clippy::implicit_hasher)]
pub fn DisclosureGroup(
    /// Whether one or several disclosures can be expanded at once.
    #[prop(into, optional)]
    expansion: Signal<DisclosureGroupExpansion>,
    /// Whether all disclosures of the group are disabled.
    #[prop(into, optional)]
    is_disabled: Signal<bool>,
    /// The initially expanded disclosures (their `key`s). Ignored with `expanded_keys`.
    #[prop(into, optional)]
    default_expanded_keys: HashSet<Key>,
    /// The expanded disclosures (controlled): a value or any signal.
    #[prop(into, optional)]
    expanded_keys: Option<Signal<HashSet<Key>>>,
    /// Receives the expanded disclosures: an `RwSignal`, `WriteSignal`, closure, ...
    #[prop(into, optional)]
    set_expanded_keys: Option<Out<HashSet<Key>>>,
    #[prop(into, optional)] on_expanded_change: Option<Callback<HashSet<Key>>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-DisclosureGroup", classes);
    let (value, on_expanded_change) =
        ValueBinding::from_state_props(expanded_keys, set_expanded_keys, on_expanded_change);
    let state = use_disclosure_group_state(UseDisclosureGroupStateInput {
        expansion,
        is_disabled,
        default_expanded_keys,
        value,
        on_expanded_change,
    });
    // The state for the children only (an owner without `<Provider>`'s component and children
    // boxes: `performance.md` R11).
    scoped_view(
        move || provide_context(state),
        move || {
            view! {
                <div class=classes style={styles} {..data_flags([("data-disabled", is_disabled)])}>
                    {children()}
                </div>
            }
        },
    )
}

/// A disclosure: a trigger button (the `Button` inside its [`DisclosureTrigger`]) showing and
/// hiding its [`DisclosurePanel`]. Put the trigger in a heading when the disclosure is a section of
/// the page.
///
/// ```ignore
/// <Disclosure>
///     <h3><DisclosureTrigger><Button>"Details"</Button></DisclosureTrigger></h3>
///     <DisclosurePanel>"Content"</DisclosurePanel>
/// </Disclosure>
/// ```
///
/// Its [`DisclosureState`] is in the context of its children (react-aria-components'
/// `DisclosureStateContext`), e.g. for a custom part: `use_context::<DisclosureState>()`.
///
/// Data attributes: `data-expanded`, `data-disabled`, `data-focus-visible-within`.
///
/// Default class: `leptonic-Disclosure`.
#[component]
pub fn Disclosure(
    /// The disclosure's key in a surrounding [`DisclosureGroup`].
    #[prop(into, optional)]
    key: Option<Key>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Whether the panel starts expanded. Ignored with `is_expanded` or in a group.
    #[prop(optional)]
    default_expanded: bool,
    /// Whether the panel is expanded (controlled): a value or any signal.
    #[prop(into, optional)]
    is_expanded: Option<Signal<bool>>,
    /// Receives the expanded state: an `RwSignal`, `WriteSignal`, closure, ...
    #[prop(into, optional)]
    set_expanded: Option<Out<bool>>,
    #[prop(into, optional)] on_expanded_change: Option<Callback<bool>>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-Disclosure", classes);
    let group = use_context::<DisclosureGroupState>();
    let key = key.unwrap_or_else(|| Key::from(use_id("disclosure")));
    // In a group, the group's expanded keys hold the state.
    let value = group.map(|group| {
        let key = StoredValue::new(key.clone());
        ValueBinding::new(
            // Only this disclosure's own changes reach it (`performance.md` R1).
            Signal::derive(move || key.with_value(|key| group.is_expanded(key))),
            Callback::new(move |expanded: bool| {
                key.with_value(|key| {
                    if group
                        .expanded_keys
                        .with_untracked(|keys| keys.contains(key))
                        != expanded
                    {
                        group.toggle_key(key);
                    }
                });
            }),
        )
    });
    let (own, on_expanded_change) =
        ValueBinding::from_state_props(is_expanded, set_expanded, on_expanded_change);
    let value = value.or(own);
    let disclosure_state: DisclosureState = use_disclosure_state(UseDisclosureStateInput {
        default_expanded,
        value,
        on_expanded_change,
    });
    // Folded: a constant side costs no derive (`performance.md` R15).
    let is_disabled = match group {
        Some(group) => either(is_disabled, group.is_disabled),
        None => is_disabled,
    };
    let UseDisclosureReturn {
        button,
        panel,
        trigger_id,
        ..
    } = use_disclosure(UseDisclosureInput {
        is_disabled,
        state: disclosure_state,
    });

    // The panel is named by the trigger while it is rendered: the hook's id, or the trigger
    // button's own, which it writes into `trigger_id` while rendering (react-aria-components
    // merges both ids).
    let trigger = CapturedElement::new();
    let trigger_id = RwSignal::new(trigger_id);
    // The trigger: one at a time, and its presence names the panel (no effect watching the
    // trigger's element).
    let trigger_press = TriggerPress {
        on_press: button.on_press,
        on_press_start: button.on_press_start,
        is_disabled,
        trigger: PressResponderTrigger {
            aria_haspopup: Signal::stored(None),
            aria_expanded: button.aria_expanded,
            aria_controls: button.aria_controls,
            element: trigger,
            id: trigger_id,
        },
    };
    let trigger_part = Part::observed(move || trigger_press);
    let labelled_by = {
        let trigger_part = trigger_part.clone();
        Signal::derive(move || trigger_part.is_held().then(|| trigger_id.get()))
    };

    let focus_ring = use_focus_ring(UseFocusRingInput {
        target: FocusRingTarget::Within,
        ..UseFocusRingInput::default()
    });
    let focus_ring_props = focus_ring.target.into_props();
    let focus_within = (
        focus_ring_props.on_focusin.into_on(ev::focusin),
        focus_ring_props.on_focusout.into_on(ev::focusout),
    );
    let is_expanded = disclosure_state.is_expanded;
    let context = DisclosureContext {
        panel,
        labelled_by,
        trigger: trigger_part,
    };

    // Both contexts in one scope (one owner, no `<Provider>` per context: `performance.md` R11).
    scoped_view(
        move || {
            provide_context(context);
            provide_context(disclosure_state);
        },
        move || {
            view! {
                <div
                    class=classes
                    style={styles}
                    {..data_flags([("data-focus-visible-within", focus_ring.is_focus_visible)])}
                    {..data_flags([
                        ("data-expanded", is_expanded),
                        ("data-disabled", is_disabled),
                    ])}
                    {..focus_within}
                >
                    {children()}
                </div>
            }
        },
    )
}

/// The trigger of the [`Disclosure`] around it: hands the button inside (a `Button` atom) the
/// toggling, `aria-expanded` and `aria-controls`. Renders no element of its own.
#[component]
pub fn DisclosureTrigger(children: Children) -> impl IntoView {
    crate::metrics::subject!();
    let Some(TriggerPress {
        on_press,
        on_press_start,
        is_disabled,
        trigger,
    }) = use_context::<DisclosureContext>().and_then(|ctx| ctx.trigger.props())
    else {
        crate::utils::dev_warn!(
            "A <DisclosureTrigger> must be inside a <Disclosure>, one per disclosure."
        );
        return children().into_any();
    };
    // Built from its props struct: `view!` can't pass `Option`s.
    PressResponder(PressResponderProps {
        on_press,
        on_press_start,
        on_press_end: None,
        on_press_up: None,
        on_press_change: None,
        long_press: None,
        is_disabled: Some(is_disabled),
        force_is_pressed: None,
        prevent_focus_on_press: None,
        should_cancel_on_pointer_exit: None,
        allow_text_selection_on_press: None,
        trigger: Some(trigger),
        shortcuts: None,
        on_context_menu: None,
        children,
    })
    .into_any()
}

/// The panel of the [`Disclosure`] around it: shown while expanded, named by the trigger. When
/// collapsed it is `hidden="until-found"`, so find in page still finds (and expands) its content.
/// For transitions, `--disclosure-panel-width`/`--disclosure-panel-height` hold its size while it
/// expands or collapses.
///
/// Data attributes: `data-focus-visible-within`.
///
/// Default class: `leptonic-DisclosurePanel`.
#[component]
pub fn DisclosurePanel(
    /// `Group` (default), or `Region` for an important section that should be a landmark.
    #[prop(optional)]
    role: DisclosurePanelRole,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-DisclosurePanel", classes);
    let Some(context) = use_context::<DisclosureContext>() else {
        crate::utils::dev_warn!("A <DisclosurePanel> must be inside a <Disclosure>.");
        return None;
    };
    // One panel per disclosure (`Part`: a second one renders nothing).
    let mut props = context.panel.props()?;
    props.role = role.as_aria_role();
    props.aria_labelledby = context.labelled_by;
    let attrs = props.into_attrs();
    let focus_ring = use_focus_ring(UseFocusRingInput {
        target: FocusRingTarget::Within,
        ..UseFocusRingInput::default()
    });
    let focus_ring_props = focus_ring.target.into_props();
    let focus_within = (
        focus_ring_props.on_focusin.into_on(ev::focusin),
        focus_ring_props.on_focusout.into_on(ev::focusout),
    );
    Some(view! {
        <div
            {..attrs}
            class=classes
            style={styles}
            {..data_flags([("data-focus-visible-within", focus_ring.is_focus_visible)])}
            {..focus_within}
        >
            {children()}
        </div>
    })
}

/// The role of a [`DisclosurePanel`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisclosurePanelRole {
    #[default]
    Group,
    /// A landmark; use sparingly.
    Region,
}

impl DisclosurePanelRole {
    fn as_aria_role(self) -> AriaRole {
        match self {
            Self::Group => AriaRole::Group,
            Self::Region => AriaRole::Region,
        }
    }
}
