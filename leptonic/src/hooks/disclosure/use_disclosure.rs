// Upstream: react-aria/src/disclosure/useDisclosure.ts @ 99e6102368
// Upstream: react-aria/test/disclosure/useDisclosure.test.ts @ 99e6102368
use leptos::{ev, prelude::*};

use super::use_disclosure_state::DisclosureState;
use crate::{
    CapturedElement, ElementCaptureAttr, EventHandler, Id, IntoAttrs, OnEvent, Part,
    hooks::{button::UseButtonInput, interactions::PressEvent},
    utils::{
        aria::{AriaExpanded, AriaHidden, AriaRole},
        id::use_id,
        pointer_type::PointerType,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, signal_attr},
        slot_id::referenced_id,
        spread::Spread,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The state goes into the input (C8); the trigger's configuration is returned as a
//   `UseButtonInput` for `use_button` (react-aria: `buttonProps` for `useButton`).
// - The panel element is captured by `panel_props` instead of passing a ref.
// - The collapsed panel is rendered with `hidden="until-found"` from the start, on the server too
//   (react-aria: `hidden` while server rendering, `until-found` once hydrated).
//
// ## DIFFERENT BEHAVIOR
// - The trigger's `aria-controls` refers to the panel only while the panel is rendered (after
//   mount, as `use_slot`'s references): a panel the app renders conditionally leaves no reference
//   to a missing element (react-aria always sets it; its panel is always rendered).
//
// =============================================================================

/// Input of [`use_disclosure`].
#[derive(Debug, Clone, Copy)]
pub struct UseDisclosureInput {
    /// Whether the panel is expanded.
    pub state: DisclosureState,
    /// Whether the trigger can't toggle the panel.
    pub is_disabled: Signal<bool>,
}

/// Return value of [`use_disclosure`].
#[derive(Debug)]
pub struct UseDisclosureReturn {
    /// Configuration for the trigger button: pass it to `use_button` (with struct update syntax
    /// for your own settings).
    pub button: UseButtonInput,
    /// The panel element: the trigger controls it while it is rendered.
    pub panel: Part<UseDisclosurePanelProps>,
    /// The trigger's id (also in `button`).
    pub trigger_id: Id,
    /// The panel element, once rendered.
    pub panel_element: CapturedElement,
}

/// Props for the panel element.
#[derive(Debug)]
pub struct UseDisclosurePanelProps {
    pub id: Id,
    /// `group` by default; `region` for an important section (a landmark).
    pub role: AriaRole,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_hidden: Signal<Option<AriaHidden>>,
    /// Whether the panel renders collapsed (`hidden="until-found"`): the state when the hook
    /// ran. Afterwards the hook manages the attribute; a panel rendered later (again) sets this
    /// to the state at that time.
    pub hidden_until_found: bool,
    pub element_capture: ElementCaptureAttr,
    /// Find in page matched the collapsed panel's content: expands it.
    pub on_beforematch: EventHandler<web_sys::Event>,
}

impl IntoAttrs for UseDisclosurePanelProps {
    type Attrs = UseDisclosurePanelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Spread((
                fixed_attr("id", self.id),
                fixed_attr("role", self.role),
                signal_attr("aria-labelledby", self.aria_labelledby),
                signal_attr("aria-hidden", self.aria_hidden),
                fixed_attr("hidden", self.hidden_until_found.then_some("until-found")),
                self.element_capture,
            )),
            self.on_beforematch
                .into_on(ev::Custom::<web_sys::Event>::new("beforematch")),
        )
    }
}

pub type UseDisclosurePanelAttrs = (
    Spread<(
        FixedAttr,
        FixedAttr,
        SignalAttr<Option<Id>>,
        SignalAttr<Option<AriaHidden>>,
        FixedAttr,
        ElementCaptureAttr,
    )>,
    OnEvent<ev::Custom<web_sys::Event>>,
);

/// A disclosure: a trigger button showing and hiding a panel. A collapsed panel is
/// `hidden="until-found"`, so the browser's find in page still finds (and expands) its content.
/// While it expands or collapses, the panel's `--disclosure-panel-width`/`--disclosure-panel-height`
/// hold its size in pixels, for CSS transitions; afterwards they are `auto` (expanded) or `0px`.
///
/// ```ignore
/// let state = use_disclosure_state(UseDisclosureStateInput::default());
/// let disclosure = use_disclosure(UseDisclosureInput {
///     state,
///     is_disabled: Signal::stored(false),
/// });
/// let (button_attrs, button_styles) = use_button(disclosure.button).button.attrs();
/// view! {
///     <button {..button_attrs} style=button_styles>"Details"</button>
///     <div {..disclosure.panel.attrs()}>"Content"</div>
/// }
/// ```
pub fn use_disclosure(input: UseDisclosureInput) -> UseDisclosureReturn {
    crate::metrics::subject!();
    let UseDisclosureInput { state, is_disabled } = input;
    let trigger_id = use_id("disclosure-trigger");
    let panel_id = use_id("disclosure-panel");
    let panel_element = CapturedElement::new();
    let is_expanded = state.is_expanded;

    #[cfg(not(feature = "ssr"))]
    let on_beforematch = manage_panel(state, is_disabled, panel_element);
    #[cfg(feature = "ssr")]
    let on_beforematch = EventHandler::empty();

    // Each rendering of the panel: collapsed as the state is then (the panel may render again,
    // e.g. inside a `Show`).
    let panel = {
        let id = panel_id.clone();
        let aria_labelledby = Signal::stored(Some(trigger_id.clone()));
        let aria_hidden = Signal::derive(move || (!is_expanded.get()).then_some(AriaHidden::True));
        let element_capture = panel_element.attr();
        Part::observed(move || UseDisclosurePanelProps {
            id: id.clone(),
            role: AriaRole::Group,
            aria_labelledby,
            aria_hidden,
            hidden_until_found: !is_expanded.get_untracked(),
            element_capture: element_capture.clone(),
            on_beforematch: on_beforematch.clone(),
        })
    };

    let button = UseButtonInput {
        id: Some(trigger_id.clone()),
        is_disabled,
        aria_expanded: Signal::derive(move || Some(AriaExpanded::from(is_expanded.get()))),
        // While the panel is rendered.
        aria_controls: referenced_id(&panel, panel_id),
        // Toggles on press, except for keyboard presses, which toggle on press start.
        on_press: Some(Callback::new(move |e: PressEvent| {
            if !is_disabled.get_untracked() && e.pointer_type != PointerType::Keyboard {
                state.toggle();
            }
        })),
        on_press_start: Some(Callback::new(move |e: PressEvent| {
            if e.pointer_type == PointerType::Keyboard && !is_disabled.get_untracked() {
                state.toggle();
            }
        })),
        element_type: crate::hooks::button::ButtonElementType::Button,
        button_type: crate::hooks::button::ButtonType::Button,
        aria_label: MaybeProp::default(),
        aria_labelledby: Signal::stored(None),
        is_pending: Signal::stored(false),
        allow_focus_when_disabled: Signal::stored(false),
        exclude_from_tab_order: Signal::stored(false),
        auto_focus: false,
        prevent_focus_on_press: Signal::stored(false),
        href: Signal::stored(None),
        target: Signal::stored(crate::hooks::link::LinkTarget::Same),
        rel: Vec::new(),
        form: crate::hooks::button::ButtonFormAttributes::default(),
        aria_haspopup: Signal::stored(None),
        aria_describedby: Signal::stored(None),
        aria_pressed: Signal::stored(None),
        aria_checked: Signal::stored(None),
        role: None,
        aria_current: Signal::stored(None),
        aria_disabled: Signal::stored(None),
        on_press_end: None,
        on_press_up: None,
        on_press_change: None,
        long_press: None,
        on_hover_start: None,
        on_hover_end: None,
        on_hover_change: None,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
        on_key_down: None,
        on_key_up: None,
        shortcuts: None,
        on_context_menu: None,
    };

    UseDisclosureReturn {
        button,
        panel,
        trigger_id,
        panel_element,
    }
}

/// Shows and hides the panel (`hidden="until-found"`), with its size in CSS variables for
/// transitions. Returns the panel's `beforematch` handler, which expands it when find in page
/// matches its content (a listener of the panel's attributes: no effect watching the element).
#[cfg(not(feature = "ssr"))]
fn manage_panel(
    state: DisclosureState,
    is_disabled: Signal<bool>,
    panel: CapturedElement,
) -> EventHandler<web_sys::Event> {
    use wasm_bindgen::JsCast;

    let pending_frame = StoredValue::new(None::<AnimationFrameRequestHandle>);
    let cancel_frame = move || {
        if let Some(Some(handle)) = pending_frame.try_update_value(Option::take) {
            handle.cancel();
        }
    };

    // Find in page removes `hidden`: expand, and restore `until-found` should the state refuse.
    let on_beforematch = EventHandler::new(move |_: web_sys::Event| {
        cancel_frame();
        let handle = request_animation_frame_with_handle(move || {
            if let Some(panel) = panel.get_untracked() {
                let _ = panel.set_attribute("hidden", "until-found");
            }
        })
        .ok();
        pending_frame.set_value(handle);
        state.toggle();
    });

    let set_size = |panel: &web_sys::HtmlElement, width: &str, height: &str| {
        let style = panel.style();
        let _ = style.set_property("--disclosure-panel-width", width);
        let _ = style.set_property("--disclosure-panel-height", height);
    };
    // After the panel's running animations finish (not when one is cancelled, e.g. by toggling
    // again).
    let after_animations =
        move |panel: web_sys::HtmlElement, then: Box<dyn FnOnce(&web_sys::HtmlElement)>| {
            let promises: js_sys::Array = panel
                .get_animations()
                .iter()
                .filter_map(|animation| animation.dyn_into::<web_sys::Animation>().ok())
                .filter_map(|animation| animation.finished().ok())
                .collect();
            wasm_bindgen_futures::spawn_local(async move {
                if wasm_bindgen_futures::JsFuture::from(js_sys::Promise::all(&promises))
                    .await
                    .is_ok()
                {
                    then(&panel);
                }
            });
        };

    // The panel element and its state when last shown: a new element (the panel rendered again,
    // e.g. inside a `Show`) starts without animation, as on the first render.
    let previous =
        StoredValue::new(None::<(send_wrapper::SendWrapper<web_sys::HtmlElement>, bool)>);
    Effect::new(move |_| {
        let expanded = state.is_expanded.get();
        is_disabled.track();
        cancel_frame();
        let Some(panel) = panel.get() else {
            return;
        };
        let Some(panel) = panel.dyn_ref::<web_sys::HtmlElement>().cloned() else {
            return;
        };
        let current = send_wrapper::SendWrapper::new(panel.clone());
        let was_expanded = previous.with_value(|previous| {
            previous
                .as_ref()
                .filter(|(element, _)| **element == panel)
                .map(|(_, was_expanded)| *was_expanded)
        });
        match was_expanded {
            // First render (of this element): no animation.
            None => {
                if expanded {
                    let _ = panel.remove_attribute("hidden");
                    set_size(&panel, "auto", "auto");
                } else {
                    let _ = panel.set_attribute("hidden", "until-found");
                    set_size(&panel, "0px", "0px");
                }
            }
            Some(was_expanded) if was_expanded != expanded => {
                let width = format!("{}px", panel.scroll_width());
                if expanded {
                    let _ = panel.remove_attribute("hidden");
                    // Pixel sizes, so they can be animated.
                    let height = format!("{}px", panel.scroll_height());
                    set_size(&panel, &width, &height);
                    after_animations(
                        panel,
                        // Back to `auto`, so the content can resize.
                        Box::new(move |panel| set_size(panel, "auto", "auto")),
                    );
                } else {
                    let height = format!("{}px", panel.scroll_height());
                    set_size(&panel, &width, &height);
                    // Force a style recalculation, so the change to zero animates.
                    if let Some(window) = leptos_use::use_window().as_ref() {
                        let _ = window
                            .get_computed_style(&panel)
                            .map(|style| style.map(|style| style.get_property_value("height")));
                    }
                    set_size(&panel, "0px", "0px");
                    after_animations(
                        panel,
                        Box::new(move |panel| {
                            let _ = panel.set_attribute("hidden", "until-found");
                        }),
                    );
                }
            }
            Some(_) => {}
        }
        previous.set_value(Some((current, expanded)));
    });

    on_cleanup(cancel_frame);
    on_beforematch
}
