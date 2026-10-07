// Upstream: react-aria-components/src/Button.tsx @ 99e6102368
use leptos::{context::Provider, prelude::*};
use web_sys::FocusEvent;

use super::progress_bar::ProgressBarIdContext;
use crate::{
    hooks::*,
    utils::{
        CapturedElement,
        aria::{AriaCurrent, AriaExpanded, AriaHasPopup, AriaPressed},
        classes::Classes,
        data_attributes::flag,
        default_class::with_default_class,
        id::{ensure_element_id, use_id},
        live_announcer::{Announcement, Assertiveness, announce},
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Render props become `data-*` attributes plus plain children.
// - The id is set with `attr:id` (react-aria-components: `id`, else generated). A pending button
//   with `aria_label` is named by its own id plus its progress bar's, so it then gets a generated
//   id unless it has one.
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
#[allow(clippy::too_many_lines)]
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
    #[prop(into, optional)] is_disabled: Signal<bool>,
    /// Whether an action the button started is in progress: the button stays focusable but
    /// can't be pressed. Default: `false`.
    #[prop(into, optional)]
    is_pending: Signal<bool>,
    /// Focus the button when it mounts.
    #[prop(optional)]
    auto_focus: bool,
    /// Don't move focus to the button when it is pressed.
    #[prop(optional)]
    prevent_focus_on_press: bool,
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
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] aria_describedby: Signal<Option<String>>,
    #[prop(into, optional)] aria_controls: Signal<Option<String>>,
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
    let classes = with_default_class("leptonic-Button", classes);
    let progress_id = use_id("button-progress");
    let generated_id = use_id("button");
    let element = CapturedElement::new();
    // The button's rendered id, once needed (pending, it names the button).
    let own_id = RwSignal::new(None::<String>);
    let has_aria_label = Signal::derive(move || aria_label.read().is_some());
    let labelled_by = aria_labelledby.clone();
    let label_progress = progress_id.clone();

    let UseButtonReturn {
        props,
        is_disabled,
        is_pressed,
        is_hovered,
        is_focused,
        ..
    } = use_button(UseButtonInput {
        button_type,
        is_disabled,
        is_pending,
        auto_focus,
        prevent_focus_on_press,
        exclude_from_tab_order,
        aria_haspopup,
        aria_expanded,
        aria_pressed,
        aria_label,
        // Pending, the progress bar joins the name: after the labelling elements, or after the
        // button itself when it has an `aria-label` (`aria-labelledby` wins over `aria-label`).
        aria_labelledby: Signal::derive(move || {
            if !is_pending.get() {
                return labelled_by.clone();
            }
            match &labelled_by {
                Some(ids) => Some(format!("{ids} {label_progress}")),
                None if has_aria_label.get() => {
                    own_id.get().map(|id| format!("{id} {label_progress}"))
                }
                None => None,
            }
        }),
        aria_describedby,
        aria_controls,
        aria_current,
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
        ..UseButtonInput::default()
    });

    // Announce the pending state to a user focusing the button when it starts or ends: the
    // button's name (with the progress bar's while pending).
    let progress = progress_id.clone();
    Effect::new(move |was_pending: Option<bool>| {
        let pending = is_pending.get();
        let focused = is_focused.get();
        let changed = was_pending.is_some_and(|was_pending| was_pending != pending);
        // Named by its own id: pending with an `aria-label`, or announced. Only then does a
        // button without an id get one.
        if aria_labelledby.is_none()
            && (pending || (changed && focused))
            && own_id.get_untracked().is_none()
        {
            own_id.set(ensure_element_id(&element, &generated_id));
        }
        if changed && focused {
            let ids: Vec<String> = match (&aria_labelledby, own_id.get_untracked()) {
                (Some(ids), _) => ids
                    .split_whitespace()
                    .map(str::to_owned)
                    .chain(pending.then(|| progress.clone()))
                    .collect(),
                (None, Some(id)) if pending && has_aria_label.get_untracked() => {
                    vec![id, progress.clone()]
                }
                (None, Some(id)) => vec![id],
                (None, None) => Vec::new(),
            };
            if !ids.is_empty() {
                announce(Announcement::LabelledBy(ids), Assertiveness::Assertive);
            }
        }
        pending
    });

    let (button_attrs, button_styles) = props.into_parts();
    let styles = button_styles.merge(styles);

    view! {
        <button
            {..button_attrs}
            {..element.attr()}
            class=classes
            style=styles
            data-pressed=flag(is_pressed)
            data-hovered=flag(is_hovered)
            data-focused=flag(is_focused)
            data-disabled=flag(is_disabled)
            data-pending=flag(is_pending)
        >
            <Provider value=ProgressBarIdContext(progress_id)>{children()}</Provider>
        </button>
    }
}
