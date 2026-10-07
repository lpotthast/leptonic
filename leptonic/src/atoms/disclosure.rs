// Upstream: react-aria-components/src/Disclosure.tsx @ 99e6102368
use std::collections::HashSet;

use leptos::{context::Provider, ev, prelude::*};

use super::press::{PressResponder, PressResponderProps};
use crate::{
    Out,
    hooks::{
        DisclosureGroupExpansion, DisclosureGroupState, DisclosureState, IntoAttrs, PressEvent,
        PressResponderTrigger, UseDisclosureGroupStateInput, UseDisclosureInput,
        UseDisclosurePanelAttrs, UseDisclosureReturn, UseDisclosureStateInput, UseFocusRingInput,
        collections::Key, use_disclosure, use_disclosure_group_state, use_disclosure_state,
        use_focus_ring,
    },
    utils::{
        CapturedElement, ValueBinding,
        aria::AriaRole,
        classes::Classes,
        data_attributes::flag,
        default_class::with_default_class,
        id::{ensure_element_id, use_id},
        styles::Styles,
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
// - The panel is named by the trigger's rendered id (its own `attr:id`, else the hook's).
// - Expanded state (C4): `default_expanded` + `on_expanded_change`, or `is_expanded` +
//   `set_expanded`; the group's `expanded_keys` likewise.
// - Render props become `data-*` attributes plus plain children.
//
// =============================================================================

/// Context from a [`Disclosure`] to its [`DisclosurePanel`].
#[derive(Clone)]
struct DisclosureContext {
    /// Cloned for each rendering of the panel.
    panel_attrs: StoredValue<UseDisclosurePanelAttrs>,
    /// Set by the panel before it renders.
    panel_role: RwSignal<AriaRole>,
    trigger: TriggerPress,
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
/// Give each disclosure an `id` to address it in `default_expanded_keys` and `expanded_keys`.
///
/// Data attributes: `data-disabled`.
///
/// Default class: `leptonic-DisclosureGroup`.
#[component]
#[allow(clippy::implicit_hasher)]
pub fn DisclosureGroup(
    /// Whether one or several disclosures can be expanded at once.
    #[prop(optional)]
    expansion: DisclosureGroupExpansion,
    /// Whether all disclosures of the group are disabled.
    #[prop(into, optional)]
    is_disabled: Signal<bool>,
    /// The initially expanded disclosures (their `id`s). Ignored with `expanded_keys`.
    #[prop(into, optional)]
    default_expanded_keys: Vec<Key>,
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
    view! {
        <Provider value=state>
            <div class=classes style=styles data-disabled=flag(is_disabled)>
                {children()}
            </div>
        </Provider>
    }
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
/// Data attributes: `data-expanded`, `data-disabled`, `data-focus-visible-within`.
///
/// Default class: `leptonic-Disclosure`.
#[component]
pub fn Disclosure(
    /// The disclosure's key in a surrounding [`DisclosureGroup`].
    #[prop(into, optional)]
    id: Option<Key>,
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
    let classes = with_default_class("leptonic-Disclosure", classes);
    let group = use_context::<DisclosureGroupState>();
    let key = id.unwrap_or_else(|| Key::from(use_id("disclosure")));
    // In a group, the group's expanded keys hold the state.
    let value = group.map(|group| {
        let key = StoredValue::new(key.clone());
        ValueBinding::new(
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
    let is_disabled = Signal::derive(move || {
        is_disabled.get() || group.is_some_and(|group| group.is_disabled.get())
    });
    let UseDisclosureReturn {
        button,
        panel_props,
        trigger_id,
        ..
    } = use_disclosure(UseDisclosureInput {
        is_disabled,
        state: disclosure_state,
    });

    // The panel is named by the trigger: its own id, else the hook's, which it gets once rendered
    // (so the server renders no reference to an id the button doesn't have yet).
    let trigger = CapturedElement::new();
    let labelled_by = RwSignal::new(None);
    Effect::new(move |_| {
        if let Some(id) = ensure_element_id(&trigger, &trigger_id) {
            labelled_by.set(Some(id));
        }
    });

    let focus_ring = use_focus_ring(UseFocusRingInput {
        within: true,
        ..UseFocusRingInput::default()
    });
    let focus_within = (
        focus_ring.props.on_focusin.into_on(ev::focusin),
        focus_ring.props.on_focusout.into_on(ev::focusout),
    );
    let is_expanded = disclosure_state.is_expanded;
    let panel_role = RwSignal::new(AriaRole::Group);
    let context = DisclosureContext {
        panel_attrs: StoredValue::new(
            panel_props
                .labelled_by(labelled_by.into())
                .with_role(panel_role.into())
                .into_attrs(),
        ),
        panel_role,
        trigger: TriggerPress {
            on_press: button.on_press,
            on_press_start: button.on_press_start,
            is_disabled,
            trigger: PressResponderTrigger {
                aria_haspopup: Signal::stored(None),
                aria_expanded: button.aria_expanded,
                aria_controls: button.aria_controls,
                element: trigger,
            },
        },
    };

    view! {
        <Provider value=context>
            <div
                class=classes
                style=styles
                data-expanded=flag(is_expanded)
                data-disabled=flag(is_disabled)
                data-focus-visible-within=flag(focus_ring.is_focus_visible)
                {..focus_within}
            >
                {children()}
            </div>
        </Provider>
    }
}

/// The trigger of the [`Disclosure`] around it: hands the button inside (a `Button` atom) the
/// toggling, `aria-expanded` and `aria-controls`. Renders no element of its own.
#[component]
pub fn DisclosureTrigger(children: Children) -> impl IntoView {
    let Some(TriggerPress {
        on_press,
        on_press_start,
        is_disabled,
        trigger,
    }) = use_context::<DisclosureContext>().map(|ctx| ctx.trigger)
    else {
        crate::utils::dev_warn!("A <DisclosureTrigger> must be inside a <Disclosure>.");
        return children().into_any();
    };
    // Built from its props struct: `view!` can't pass `Option`s.
    PressResponder(PressResponderProps {
        on_press,
        on_press_start,
        on_press_end: None,
        on_press_up: None,
        on_press_change: None,
        on_long_press_start: None,
        on_long_press: None,
        on_long_press_end: None,
        long_press_accessibility_description: MaybeProp::default(),
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
    let classes = with_default_class("leptonic-DisclosurePanel", classes);
    let Some(context) = use_context::<DisclosureContext>() else {
        crate::utils::dev_warn!("A <DisclosurePanel> must be inside a <Disclosure>.");
        return ().into_any();
    };
    context.panel_role.set(role.as_aria_role());
    let attrs = context.panel_attrs.get_value();
    let focus_ring = use_focus_ring(UseFocusRingInput {
        within: true,
        ..UseFocusRingInput::default()
    });
    let focus_within = (
        focus_ring.props.on_focusin.into_on(ev::focusin),
        focus_ring.props.on_focusout.into_on(ev::focusout),
    );
    view! {
        <div
            {..attrs}
            class=classes
            style=styles
            data-focus-visible-within=flag(focus_ring.is_focus_visible)
            {..focus_within}
        >
            {children()}
        </div>
    }
    .into_any()
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
