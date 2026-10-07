// Upstream: react-aria/src/disclosure/useDisclosure.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
    tachys::html::attribute::custom::{CustomAttr, custom_attribute},
};

use super::use_disclosure_state::DisclosureState;
use crate::{
    hooks::{IntoAttrs, PressEvent, UseButtonInput},
    utils::{
        CapturedElement, ElementCaptureAttr,
        aria::{AriaExpanded, AriaHidden, AriaRole},
        id::use_id,
        pointer_type::PointerType,
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
    /// Props for the panel element.
    pub panel_props: UseDisclosurePanelProps,
    /// The trigger's id (also in `button`).
    pub trigger_id: String,
    /// The panel element, once rendered.
    pub panel_element: CapturedElement,
}

/// Props for the panel element.
#[derive(Debug)]
pub struct UseDisclosurePanelProps {
    pub id: String,
    /// `group` by default; `region` for an important section (a landmark).
    pub role: Signal<AriaRole>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_hidden: Signal<Option<AriaHidden>>,
    /// `until-found` when the panel starts collapsed; afterwards the hook manages the attribute.
    pub hidden: Option<&'static str>,
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseDisclosurePanelProps {
    type Attrs = UseDisclosurePanelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaHidden, self.aria_hidden),
            custom_attribute("hidden", self.hidden),
            self.element_capture,
        )
    }
}

pub type UseDisclosurePanelAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, Signal<AriaRole>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
    Attr<attr::AriaHidden, Signal<Option<AriaHidden>>>,
    CustomAttr<&'static str, Option<&'static str>>,
    ElementCaptureAttr,
);

impl UseDisclosurePanelProps {
    /// Names the panel by another element than the hook's trigger (e.g. a trigger that keeps its
    /// own id).
    #[must_use]
    pub fn labelled_by(self, aria_labelledby: Signal<Option<String>>) -> Self {
        Self {
            aria_labelledby,
            ..self
        }
    }

    /// Gives the panel another role than `group` (react-aria: `role` in the panel's props).
    #[must_use]
    pub fn with_role(self, role: Signal<AriaRole>) -> Self {
        Self { role, ..self }
    }
}

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
/// let (button_attrs, button_styles) = use_button(disclosure.button).props.into_parts();
/// view! {
///     <button {..button_attrs} style=button_styles>"Details"</button>
///     <div {..disclosure.panel_props.into_attrs()}>"Content"</div>
/// }
/// ```
pub fn use_disclosure(input: UseDisclosureInput) -> UseDisclosureReturn {
    let UseDisclosureInput { state, is_disabled } = input;
    let trigger_id = use_id("disclosure-trigger");
    let panel_id = use_id("disclosure-panel");
    let panel_element = CapturedElement::new();
    let is_expanded = state.is_expanded;

    #[cfg(not(feature = "ssr"))]
    manage_panel(state, is_disabled, panel_element);

    let button = UseButtonInput {
        id: Some(trigger_id.clone()),
        is_disabled,
        aria_expanded: Signal::derive(move || Some(AriaExpanded::from(is_expanded.get()))),
        aria_controls: Signal::stored(Some(panel_id.clone())),
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
        ..UseButtonInput::default()
    };

    UseDisclosureReturn {
        button,
        panel_props: UseDisclosurePanelProps {
            id: panel_id,
            role: Signal::stored(AriaRole::Group),
            aria_labelledby: Signal::stored(Some(trigger_id.clone())),
            aria_hidden: Signal::derive(move || (!is_expanded.get()).then_some(AriaHidden::True)),
            hidden: (!is_expanded.get_untracked()).then_some("until-found"),
            element_capture: panel_element.attr(),
        },
        trigger_id,
        panel_element,
    }
}

/// Shows and hides the panel (`hidden="until-found"`), with its size in CSS variables for
/// transitions, and expands it when find in page matches its content.
#[cfg(not(feature = "ssr"))]
fn manage_panel(state: DisclosureState, is_disabled: Signal<bool>, panel: CapturedElement) {
    use leptos::ev;
    use leptos_use::use_event_listener;
    use wasm_bindgen::JsCast;

    let pending_frame = StoredValue::new(None::<AnimationFrameRequestHandle>);
    let cancel_frame = move || {
        if let Some(Some(handle)) = pending_frame.try_update_value(Option::take) {
            handle.cancel();
        }
    };

    // Find in page removes `hidden`: expand, and restore `until-found` should the state refuse.
    let _ = use_event_listener(
        Signal::derive(move || panel.get()),
        ev::Custom::<web_sys::Event>::new("beforematch"),
        move |_| {
            cancel_frame();
            let handle = request_animation_frame_with_handle(move || {
                if let Some(panel) = panel.get_untracked() {
                    let _ = panel.set_attribute("hidden", "until-found");
                }
            })
            .ok();
            pending_frame.set_value(handle);
            state.toggle();
        },
    );

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

    let previous = StoredValue::new(None::<bool>);
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
        match previous.get_value() {
            // First render: no animation.
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
        previous.set_value(Some(expanded));
    });

    on_cleanup(cancel_frame);
}
