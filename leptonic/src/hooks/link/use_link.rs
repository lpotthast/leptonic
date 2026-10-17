// Upstream: react-aria/src/link/useLink.ts @ 99e6102368
// Upstream: react-aria/test/link/useLink.test.js @ 99e6102368
// Upstream: react-aria-components/test/Link.test.js @ 99e6102368
use leptos::{ev, oco::Oco, prelude::*};
use web_sys::{DragEvent, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use crate::{
    CapturedElement, ElementCaptureAttr, EventHandler, Id, IdRefs, IntoAttrs, OnEvent,
    PropsWithStyles,
    hooks::{
        focus::{
            FocusHandle, FocusRingTarget, FocusableContextAttr, FocusableContextAttrs,
            UseFocusRingInput, UseFocusRingReturn, UseFocusableInput, UseFocusableReturn,
            use_focus_ring, use_focusable,
        },
        interactions::{
            HoverEndEvent, HoverStartEvent, PressEvent, PressPropagation, PressResponderContext,
            UseHoverInput, UseHoverReturn, UsePressInput, UsePressReturn, use_hover,
            use_keyboard::KeyboardEventWrapper, use_press,
        },
        link::{LinkRel, LinkTarget, link_rel},
    },
    utils::{
        aria::{AriaCurrent, AriaDisabled, AriaExpanded, AriaHasPopup, AriaRole, aria_true},
        part::{Part, Remount},
        probe::either,
        reactions::sharing_one_effect,
        signal_attr::{SignalAttr, maybe_attr, signal_attr},
        spread::Spread,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `elementType` is the `LinkElementType` enum; `target` and `rel` are typed (`LinkTarget`,
//   `LinkRel`); `aria_current` is a signal.
// - Client-side routing is not the hook's business: routers (leptos_router) handle clicks on
//   `<a>` elements in a document-level listener, so the link's clicks propagate.
//
// ## DIFFERENT BEHAVIOR
// - A disabled link renders no `href`: it can't be followed by any means (middle click, context
//   menu) and leaves the tab order (react-aria-components renders a `<span>` instead). Such an
//   `<a>` gets `role="link"`, which it loses with its `href`.
// - Like `use_button`, the hook merges a surrounding `PressResponder`'s trigger props
//   (`aria-haspopup`, `aria-expanded`, `aria-controls`, its capture) and shortcuts, which
//   react-aria's `usePress` merges for any pressable.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Composes `use_hover` and `use_focus_ring` as well, so atoms get the state for their data
//   attributes from one hook (as `use_button`).
//
// ## ADDITIONS
// - `rel="noopener"` is added for `LinkTarget::Blank` (unless `rel` has `Opener`), so that the new
//   browsing context gets no access to this one (older browsers don't imply it). React-aria: `rel`
//   as given.
//
// =============================================================================

/// The element a link renders as.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LinkElementType {
    /// An anchor (`<a>`).
    #[default]
    Anchor,
    /// Another element (a `<span>`, ...) with `role="link"`, e.g. the current item of
    /// breadcrumbs, which links nowhere.
    Other,
}

/// Input of [`use_link`].
#[derive(Debug, Clone, Default)]
pub struct UseLinkInput {
    /// Where the link goes. Rendered on anchors only, and not while disabled.
    pub href: Signal<Option<String>>,

    /// Where to open the linked document. Default: the same browsing context.
    pub target: Signal<LinkTarget>,

    /// The relationship of the linked document. `NoOpener` is added for `LinkTarget::Blank`.
    pub rel: Vec<LinkRel>,

    pub is_disabled: Signal<bool>,

    /// The element the props are spread onto. A signal: e.g. the `Link` atom renders a `<span>`
    /// while disabled.
    pub element_type: Signal<LinkElementType>,

    /// Names the link when its content doesn't.
    pub aria_label: MaybeProp<String>,

    /// Marks the link as the current item of a set (e.g. `AriaCurrent::Page` in a navigation).
    pub aria_current: Signal<Option<AriaCurrent>>,

    pub on_press: Option<Callback<PressEvent>>,
    pub on_press_start: Option<Callback<PressEvent>>,
    pub on_press_end: Option<Callback<PressEvent>>,
    pub on_press_change: Option<Callback<bool>>,
    pub on_hover_start: Option<Callback<HoverStartEvent>>,
    pub on_hover_end: Option<Callback<HoverEndEvent>>,
    pub on_hover_change: Option<Callback<bool>>,
    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
    pub on_key_down: Option<Callback<KeyboardEventWrapper>>,
    pub on_key_up: Option<Callback<KeyboardEventWrapper>>,
    /// Focus the link when it mounts.
    pub auto_focus: bool,
    /// Remove the link from the tab order. It stays focusable programmatically and by pointer.
    pub exclude_from_tab_order: Signal<bool>,
}

/// The return value of [`use_link`].
#[derive(Debug)]
pub struct UseLinkReturn {
    /// The link element: `let (attrs, styles) = link.attrs();`, the styles merged with the
    /// element's own.
    pub link: Part<PropsWithStyles<UseLinkProps>>,
    pub is_disabled: Signal<bool>,
    pub is_pressed: Signal<bool>,
    pub is_hovered: Signal<bool>,
    pub is_focused: Signal<bool>,
    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
    /// Focuses the link programmatically.
    pub focus_handle: FocusHandle,
}

/// The link element's props from [`use_link`].
#[derive(Debug)]
pub struct UseLinkProps {
    pub href: Signal<Option<String>>,
    pub target: Signal<Option<Oco<'static, str>>>,
    pub rel: Signal<Option<String>>,
    pub role: Signal<Option<AriaRole>>,
    pub tabindex: Signal<Option<i32>>,
    pub aria_label: MaybeProp<String>,
    pub aria_current: Signal<Option<AriaCurrent>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_haspopup: Signal<Option<AriaHasPopup>>,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_controls: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
    /// The focusable context's further attributes (e.g. a tooltip trigger's).
    pub context_attrs: Option<FocusableContextAttrs>,
}

impl Remount for UseLinkProps {
    fn remount(&self) -> Self {
        Self {
            href: self.href,
            target: self.target,
            rel: self.rel,
            role: self.role,
            tabindex: self.tabindex,
            aria_label: self.aria_label,
            aria_current: self.aria_current,
            aria_disabled: self.aria_disabled,
            aria_haspopup: self.aria_haspopup,
            aria_expanded: self.aria_expanded,
            aria_controls: self.aria_controls,
            aria_describedby: self.aria_describedby,
            data_focus_visible: self.data_focus_visible,
            element_capture: self.element_capture.clone(),
            on_keydown: self.on_keydown.clone(),
            on_keyup: self.on_keyup.clone(),
            on_focus: self.on_focus.clone(),
            on_blur: self.on_blur.clone(),
            on_click: self.on_click.clone(),
            on_dblclick: self.on_dblclick.clone(),
            on_pointerdown: self.on_pointerdown.clone(),
            on_pointerup: self.on_pointerup.clone(),
            on_mousedown: self.on_mousedown.clone(),
            on_dragstart: self.on_dragstart.clone(),
            on_pointerenter: self.on_pointerenter.clone(),
            on_pointerleave: self.on_pointerleave.clone(),
            context_attrs: self.context_attrs.clone(),
        }
    }
}

/// Spread onto the link element: `<a {..attrs}/>`.
pub type UseLinkAttrs = Spread<(
    (
        SignalAttr<Option<String>>,
        SignalAttr<Option<Oco<'static, str>>>,
        SignalAttr<Option<String>>,
        SignalAttr<Option<AriaRole>>,
        SignalAttr<Option<i32>>,
        SignalAttr<Option<String>>,
        SignalAttr<Option<AriaCurrent>>,
        SignalAttr<Option<AriaDisabled>>,
    ),
    (
        SignalAttr<Option<AriaHasPopup>>,
        SignalAttr<Option<AriaExpanded>>,
        SignalAttr<Option<Id>>,
        SignalAttr<Option<Id>>,
        SignalAttr<Option<&'static str>>,
        ElementCaptureAttr,
    ),
    (
        OnEvent<ev::keydown>,
        OnEvent<ev::keyup>,
        OnEvent<ev::focus>,
        OnEvent<ev::blur>,
        OnEvent<ev::click>,
        OnEvent<ev::dblclick>,
        OnEvent<ev::pointerdown>,
        OnEvent<ev::pointerup>,
        OnEvent<ev::mousedown>,
        OnEvent<ev::dragstart>,
        OnEvent<ev::pointerenter>,
        OnEvent<ev::pointerleave>,
    ),
    FocusableContextAttr,
)>;

impl IntoAttrs for UseLinkProps {
    type Attrs = UseLinkAttrs;

    fn into_attrs(self) -> Self::Attrs {
        Spread((
            (
                signal_attr("href", self.href),
                signal_attr("target", self.target),
                signal_attr("rel", self.rel),
                signal_attr("role", self.role),
                signal_attr("tabindex", self.tabindex),
                maybe_attr("aria-label", self.aria_label),
                signal_attr("aria-current", self.aria_current),
                signal_attr("aria-disabled", self.aria_disabled),
            ),
            (
                signal_attr("aria-haspopup", self.aria_haspopup),
                signal_attr("aria-expanded", self.aria_expanded),
                signal_attr("aria-controls", self.aria_controls),
                signal_attr("aria-describedby", self.aria_describedby),
                signal_attr("data-focus-visible", self.data_focus_visible),
                self.element_capture,
            ),
            (
                self.on_keydown.into_on(ev::keydown),
                self.on_keyup.into_on(ev::keyup),
                self.on_focus.into_on(ev::focus),
                self.on_blur.into_on(ev::blur),
                self.on_click.into_on(ev::click),
                self.on_dblclick.into_on(ev::dblclick),
                self.on_pointerdown.into_on(ev::pointerdown),
                self.on_pointerup.into_on(ev::pointerup),
                self.on_mousedown.into_on(ev::mousedown),
                self.on_dragstart.into_on(ev::dragstart),
                self.on_pointerenter.into_on(ev::pointerenter),
                self.on_pointerleave.into_on(ev::pointerleave),
            ),
            FocusableContextAttr(self.context_attrs),
        ))
    }
}

/// Makes an element behave and announce itself as a link: press handling for pointer, keyboard
/// and assistive technology, focus, hover and focus-visible state, and the link attributes.
///
/// ```ignore
/// let link = use_link(UseLinkInput {
///     href: Signal::stored(Some("https://example.com".to_owned())),
///     target: Signal::stored(LinkTarget::Blank),
///     ..UseLinkInput::default()
/// });
/// let (attrs, styles) = link.link.attrs();
/// view! { <a {..attrs} style=styles>"Example"</a> }
/// ```
pub fn use_link(input: UseLinkInput) -> UseLinkReturn {
    crate::metrics::subject!();
    // The disabled resets of `use_press`, `use_hover` and `use_focus_ring` share one effect.
    sharing_one_effect(|| link(input))
}

/// The body of [`use_link`].
fn link(input: UseLinkInput) -> UseLinkReturn {
    let UseLinkInput {
        href,
        target,
        rel,
        is_disabled,
        element_type,
        aria_label,
        aria_current,
        on_press,
        on_press_start,
        on_press_end,
        on_press_change,
        on_hover_start,
        on_hover_end,
        on_hover_change,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
        auto_focus,
        exclude_from_tab_order,
    } = input;

    // A surrounding trigger (`DialogTrigger`, `MenuTrigger`, ...): its ARIA props, capture and
    // shortcuts. Its disabled state disables the link itself.
    let responder = use_context::<PressResponderContext>();
    let trigger = responder.as_ref().and_then(|ctx| ctx.trigger);
    let shortcuts = responder
        .as_ref()
        .and_then(|ctx| ctx.shortcuts)
        .map(|shortcuts| shortcuts.get_value());
    let is_disabled = match responder.as_ref().and_then(|ctx| ctx.is_disabled) {
        Some(responder_disabled) => either(is_disabled, responder_disabled),
        None => is_disabled,
    };

    let is_anchor = move || element_type.get() == LinkElementType::Anchor;

    let UseFocusableReturn {
        target: focusable_props,
        focus_handle,
    } = use_focusable(UseFocusableInput {
        is_disabled,
        auto_focus,
        exclude_from_tab_order,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
        shortcuts,
        allow_shortcut_repeats: false,
    });
    let focusable_props = focusable_props.into_props();

    let UsePressReturn {
        target: press_props,
        is_pressed,
    } = use_press(UsePressInput {
        is_disabled,
        // Routers handle link clicks in a document-level listener.
        propagation: PressPropagation::Continue,
        on_press,
        on_press_start,
        on_press_end,
        on_press_change,
        // Every field named: `..UsePressInput::default()` would create a stored signal for each
        // `Signal` field (`performance.md` R14).
        on_press_up: None,
        on_double_press: None,
        long_press: None,
        prevent_focus_on_press: Signal::stored(false),
        allow_text_selection_on_press: Signal::stored(false),
        should_cancel_on_pointer_exit: Signal::stored(false),
        force_is_pressed: Signal::stored(false),
    });
    let press_props = press_props.into_props();

    let UseHoverReturn {
        target: hover_props,
        is_hovered,
    } = use_hover(UseHoverInput {
        is_disabled,
        on_hover_start,
        on_hover_end,
        on_hover_change,
    });
    let hover_props = hover_props.into_props();

    let UseFocusRingReturn {
        target: focus_ring_props,
        is_focused,
        is_focus_visible,
    } = use_focus_ring(UseFocusRingInput {
        is_disabled,
        target: FocusRingTarget::Element,
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let focus_ring_props = focus_ring_props.into_props();

    let (press_props, press_styles) = press_props.into_inner();
    let context_describedby = focusable_props.context_aria_describedby;
    let press_describedby = press_props.aria_describedby;

    // Other elements are made focusable (react-aria: `tabIndex: 0` unless disabled).
    let focusable_tabindex = focusable_props.tabindex;
    let tabindex = Signal::derive(move || match element_type.get() {
        LinkElementType::Anchor => focusable_tabindex.get(),
        LinkElementType::Other => {
            (!is_disabled.get()).then(|| focusable_tabindex.get().unwrap_or(0))
        }
    });

    let element_capture = match trigger {
        Some(trigger) => {
            // The link renders no id: it gets the trigger's once mounted, unless it has its own.
            let element = CapturedElement::new();
            trigger.sync_id_once_mounted(element);
            focusable_props
                .element_capture
                .chain(trigger.element.attr())
                .chain(element.attr())
        }
        None => focusable_props.element_capture,
    };
    #[cfg(debug_assertions)]
    let element_capture = element_capture.chain(ElementCaptureAttr::new(move |el| {
        super::debug_validate_element_type(element_type.get_untracked(), &el);
    }));

    let props = UseLinkProps {
        href: Signal::derive(move || href.get().filter(|_| is_anchor() && !is_disabled.get())),
        target: Signal::derive(move || {
            target.with(|target| {
                (is_anchor() && *target != LinkTarget::Same).then(|| target.to_oco())
            })
        }),
        rel: Signal::derive(move || {
            target
                .with(|target| link_rel(target, rel.clone()))
                .filter(|_| is_anchor())
        }),
        // An `<a>` without `href` (disabled) has no implicit role: keep it a link.
        role: Signal::derive(move || (!is_anchor() || is_disabled.get()).then_some(AriaRole::Link)),
        tabindex,
        aria_label,
        aria_current,
        aria_disabled: aria_true(is_disabled),
        aria_haspopup: trigger.map_or_else(Signal::default, |t| t.aria_haspopup),
        aria_expanded: trigger.map_or_else(Signal::default, |t| t.aria_expanded),
        aria_controls: trigger.map_or_else(Signal::default, |t| t.aria_controls),
        aria_describedby: IdRefs::derive([press_describedby, context_describedby]),
        data_focus_visible: focus_ring_props.data_focus_visible,
        element_capture,
        on_keydown: focusable_props.on_keydown.chain(press_props.on_keydown),
        on_keyup: focusable_props.on_keyup,
        on_focus: focusable_props.on_focus.chain(focus_ring_props.on_focus),
        on_blur: focusable_props.on_blur.chain(focus_ring_props.on_blur),
        on_click: press_props.on_click,
        on_dblclick: press_props.on_dblclick,
        on_pointerdown: press_props.on_pointerdown,
        on_pointerup: press_props.on_pointerup,
        on_mousedown: press_props.on_mousedown,
        on_dragstart: press_props.on_dragstart,
        on_pointerenter: hover_props.on_pointerenter,
        on_pointerleave: hover_props.on_pointerleave,
        context_attrs: focusable_props.context_attrs,
    };

    UseLinkReturn {
        link: Part::stored(PropsWithStyles::new(props, press_styles)),
        is_disabled,
        is_pressed,
        is_hovered,
        is_focused,
        is_focus_visible,
        focus_handle,
    }
}
