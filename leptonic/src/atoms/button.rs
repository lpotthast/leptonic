// Upstream: react-aria-components/src/Button.tsx @ 99e6102368
// Upstream: react-aria-components/test/Button.test.js @ 99e6102368
use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;
use web_sys::FocusEvent;

use super::progress_bar::ProgressBarIdContext;
use crate::{
    Id,
    hooks::{
        button::{
            ButtonElementType, ButtonFormAttributes, ButtonType, UseButtonInput, UseButtonReturn,
            use_button,
        },
        interactions::{HoverEndEvent, HoverStartEvent, KeyboardEventWrapper, PressEvent},
        link::LinkTarget,
    },
    utils::{
        aria::{AriaCurrent, AriaExpanded, AriaHasPopup, AriaPressed},
        data_attributes::data_flags,
        default_class::with_default_class,
        id::{IdListBuilder, id_ref_prop, use_id},
        live_announcer::{Announcement, Assertiveness, announce},
        probe::is_constant,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Render props become `data-*` attributes plus plain children.
// - The button always has an id, known while rendering (its `id` prop, else a generated one):
//   a pending button with `aria_label` is named by its own id plus its progress bar's, and an
//   overlay trigger's references (a disclosure panel's label, an untitled dialog's) follow it, in
//   the server's HTML too (react-aria-components: `id` if given; ids merged with `mergeIds`).
// - The ARIA props (`aria_haspopup`, `aria_current`, ...) are typed (`utils::aria`).
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Pending, the button ignores presses, hover, keyboard handlers and context menu requests in
//   `use_button` (react-aria-components removes the event handlers from the props, keeping
//   focus, blur and hover ones for tooltips).
//
// ## OMITTED FEATURES
// - Slots (`ButtonContext`) and the `render` prop: no use without react-aria-components' context
//   system; wrap the button instead.
// - `onClick`: deprecated in react-aria; use `on_press`.
//
// =============================================================================

/// A button: presses from mouse, touch, keyboard and screen readers through `use_button`.
///
/// While `is_pending` (an action it started is in progress) it stays focusable but ignores
/// presses, is `aria-disabled` and doesn't submit forms. Render a
/// [`ProgressBar`](super::progress_bar::ProgressBar) inside it (shown while pending): it names the
/// button then, and focus changes of the pending state are announced.
///
/// Data attributes: `data-pressed`, `data-hovered`, `data-focused`, `data-focus-visible`,
/// `data-disabled`, `data-pending`.
///
/// Default class: `leptonic-Button`.
#[component]
pub fn Button(
    #[prop(into, optional)] on_press: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_start: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_end: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_up: Option<Callback<PressEvent>>,
    #[prop(into, optional)] on_press_change: Option<Callback<bool>>,
    #[prop(into, optional)] on_hover_start: Option<Callback<HoverStartEvent>>,
    #[prop(into, optional)] on_hover_end: Option<Callback<HoverEndEvent>>,
    #[prop(into, optional)] on_hover_change: Option<Callback<bool>>,
    #[prop(into, optional)] on_focus: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<FocusEvent>>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(into, optional)] on_key_down: Option<Callback<KeyboardEventWrapper>>,
    #[prop(into, optional)] on_key_up: Option<Callback<KeyboardEventWrapper>>,
    /// The button's id. Default: a generated one (inside an overlay trigger, the trigger's
    /// references follow it).
    #[prop(into, optional)]
    id: Option<Id>,
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Whether an action the button started is in progress: the button stays focusable but
    /// can't be pressed. Default: `false`.
    #[prop(into, optional)]
    is_pending: Signal<bool>,
    /// Focus the button when it mounts.
    #[prop(optional)]
    auto_focus: bool,
    /// Don't move focus to the button when it is pressed.
    #[prop(into, optional)]
    prevent_focus_on_press: Signal<bool>,
    /// The `type` of the button. Defaults to `button`, so that buttons in forms don't submit them
    /// unless asked to.
    #[prop(optional)]
    button_type: ButtonType,
    #[prop(into, optional)] exclude_from_tab_order: Signal<bool>,
    #[prop(into, optional)] aria_haspopup: Signal<Option<AriaHasPopup>>,
    #[prop(into, optional)] aria_expanded: Signal<Option<AriaExpanded>>,
    #[prop(into, optional)] aria_pressed: Signal<Option<AriaPressed>>,
    /// Labels the button when its content doesn't (icon-only buttons).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: MaybeProp<String>,
    #[prop(into, optional)] aria_describedby: MaybeProp<String>,
    #[prop(into, optional)] aria_controls: MaybeProp<String>,
    /// Marks the button as the current item of a set (e.g. the current page of a pagination).
    #[prop(into, optional)]
    aria_current: Signal<Option<AriaCurrent>>,
    /// The form attributes of a submit or reset button (`form`, `formaction`, `name`, ...).
    #[prop(optional)]
    form: ButtonFormAttributes,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    crate::metrics::subject!();
    let classes = with_default_class("leptonic-Button", classes);
    let progress_id = use_id("button-progress");
    // Always an id (known while rendering): pending, it names the button; in an overlay trigger,
    // the trigger adopts it.
    let id = id.unwrap_or_else(|| use_id("button"));
    let has_aria_label = move || aria_label.read().is_some();
    // Only a pending state that can change needs the pending name and its announcement.
    let pending_changes = !is_constant(is_pending);
    let labelledby = if pending_changes || is_pending.get_untracked() {
        // Pending, the progress bar joins the name: after the labelling elements, or after the
        // button itself when it has an `aria-label` (`aria-labelledby` wins over `aria-label`).
        let own_id = id.clone();
        let label_progress = progress_id.clone();
        let aria_labelledby = id_ref_prop(aria_labelledby);
        Signal::derive(move || {
            if !is_pending.get() {
                return aria_labelledby.get();
            }
            aria_labelledby.with(|labelled_by| {
                let named_by = match labelled_by {
                    Some(ids) => ids,
                    None if has_aria_label() => &own_id,
                    None => return None,
                };
                let mut ids = IdListBuilder::default();
                ids.push(named_by);
                ids.push(&label_progress);
                ids.finish()
            })
        })
    } else {
        id_ref_prop(aria_labelledby)
    };
    let announced = pending_changes.then(|| (id.clone(), progress_id.clone()));

    let UseButtonReturn {
        button,
        is_disabled,
        is_pressed,
        is_hovered,
        is_focused,
        ..
    } = use_button(UseButtonInput {
        // Every field named: `..UseButtonInput::default()` would create a stored signal for each
        // `Signal` field, also the ones set here (`performance.md` R3).
        element_type: ButtonElementType::Button,
        id: Some(id),
        button_type,
        is_disabled,
        is_pending,
        allow_focus_when_disabled: Signal::stored(false),
        auto_focus,
        prevent_focus_on_press,
        exclude_from_tab_order,
        href: Signal::stored(None),
        target: Signal::stored(LinkTarget::Same),
        rel: Vec::new(),
        aria_haspopup,
        aria_expanded,
        aria_pressed,
        aria_checked: Signal::stored(None),
        role: None,
        aria_label,
        aria_labelledby: labelledby,
        aria_describedby: id_ref_prop(aria_describedby),
        aria_controls: id_ref_prop(aria_controls),
        aria_current,
        aria_disabled: Signal::stored(None),
        form,
        on_press,
        on_press_start,
        on_press_end,
        on_press_up,
        on_press_change,
        on_hover_start,
        on_hover_end,
        on_hover_change,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
        long_press: None,
        shortcuts: None,
        on_context_menu: None,
    });

    // Announce the pending state to a user focusing the button when it starts or ends: the
    // button's name (with the progress bar's while pending). Focus changes alone announce nothing,
    // so the effect reads the focus untracked (react-aria-components compares the previous pending
    // state on every run).
    if let Some((announced_id, progress)) = announced {
        Effect::new(move |was_pending: Option<bool>| {
            let pending = is_pending.get();
            let changed = was_pending.is_some_and(|was_pending| was_pending != pending);
            if changed && is_focused.get_untracked() {
                let mut ids = IdListBuilder::default();
                let labelledby = aria_labelledby.get_untracked();
                match &labelledby {
                    Some(labelledby) => ids.push(labelledby),
                    None => ids.push(announced_id.as_str()),
                }
                if pending && (labelledby.is_some() || untrack(has_aria_label)) {
                    ids.push(&progress);
                }
                let ids = ids.finish();
                announce(
                    Announcement::LabelledBy(
                        ids.iter()
                            .flat_map(|ids| ids.split(' '))
                            .map(str::to_owned)
                            .collect(),
                    ),
                    Assertiveness::Assertive,
                );
            }
            pending
        });
    }

    let (button_attrs, button_styles) = button.attrs();
    let styles = button_styles.merge(styles);

    view! {
        <button
            {..button_attrs}
            // Two groups, so that no effect tracks more than 3 signals (a 4th grows its source
            // set on every rerun, `performance.md` R4): the interaction state, and the props
            // (usually constants: no effect then).
            {..data_flags([
                ("data-pressed", is_pressed),
                ("data-hovered", is_hovered),
                ("data-focused", is_focused),
            ])}
            {..data_flags([("data-disabled", is_disabled), ("data-pending", is_pending)])}
            class=classes
            style=styles
        >
            <Provider value=ProgressBarIdContext(progress_id)>{children()}</Provider>
        </button>
    }
}
