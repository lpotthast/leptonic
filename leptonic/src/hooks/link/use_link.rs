// Upstream: react-aria/src/link/useLink.ts @ 99e6102368
use leptos::{
    attr,
    attr::{
        Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    ev::{On, SharedEventCallback},
    oco::Oco,
    prelude::*,
};
use web_sys::{DragEvent, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use crate::{
    hooks::{
        FocusHandle, FocusableContextAttr, FocusableContextAttrs, HoverEndEvent, HoverStartEvent,
        IntoAttrs, LinkRel, LinkTarget, PressEvent, PressResponderContext, PropsWithStyles,
        UseFocusRingInput, UseFocusRingReturn, UseFocusableInput, UseFocusableReturn,
        UseHoverInput, UseHoverReturn, UsePressInput, UsePressReturn, link_rel_to_string,
        use_focus_ring, use_focusable, use_hover, use_press,
    },
    utils::{
        ElementCaptureAttr, EventHandler,
        aria::{AriaCurrent, AriaDisabled, AriaExpanded, AriaHasPopup, AriaRole},
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
// - `rel="noopener"` is added for `LinkTarget::Blank`, so that the new browsing context gets no
//   access to this one (older browsers don't imply it). React-aria: `rel` as given.
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
    pub target: LinkTarget,

    /// The relationship of the linked document. `NoOpener` is added for `LinkTarget::Blank`.
    pub rel: Vec<LinkRel>,

    pub is_disabled: Signal<bool>,

    pub element_type: LinkElementType,

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
}

/// Return value of [`use_link`].
#[derive(Debug)]
pub struct UseLinkReturn {
    /// For the link element. Call `.into_parts()` for spreading and styles.
    pub props: PropsWithStyles<UseLinkProps>,
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
    pub target: Option<Oco<'static, str>>,
    pub rel: Option<String>,
    pub role: Signal<Option<AriaRole>>,
    pub tabindex: Signal<Option<i32>>,
    pub aria_label: MaybeProp<String>,
    pub aria_current: Signal<Option<AriaCurrent>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_haspopup: Signal<Option<AriaHasPopup>>,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_controls: Signal<Option<String>>,
    pub aria_describedby: Signal<Option<String>>,
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

/// Spread onto the link element: `<a {..attrs}/>`.
pub type UseLinkAttrs = (
    (
        Attr<attr::Href, Signal<Option<String>>>,
        Attr<attr::Target, Option<Oco<'static, str>>>,
        Attr<attr::Rel, Option<String>>,
        Attr<attr::Role, Signal<Option<AriaRole>>>,
        Attr<attr::Tabindex, Signal<Option<i32>>>,
        Attr<attr::AriaLabel, MaybeProp<String>>,
        Attr<attr::AriaCurrent, Signal<Option<AriaCurrent>>>,
        Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    ),
    (
        Attr<attr::AriaHaspopup, Signal<Option<AriaHasPopup>>>,
        Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
        Attr<attr::AriaControls, Signal<Option<String>>>,
        Attr<attr::AriaDescribedby, Signal<Option<String>>>,
        CustomAttr<&'static str, Signal<Option<&'static str>>>,
        ElementCaptureAttr,
    ),
    (
        On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
        On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
        On<ev::focus, SharedEventCallback<FocusEvent>>,
        On<ev::blur, SharedEventCallback<FocusEvent>>,
        On<ev::click, SharedEventCallback<MouseEvent>>,
        On<ev::dblclick, SharedEventCallback<MouseEvent>>,
        On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
        On<ev::pointerup, SharedEventCallback<PointerEvent>>,
        On<ev::mousedown, SharedEventCallback<MouseEvent>>,
        On<ev::dragstart, SharedEventCallback<DragEvent>>,
        On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
        On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
    ),
    FocusableContextAttr,
);

impl IntoAttrs for UseLinkProps {
    type Attrs = UseLinkAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                Attr(attr::Href, self.href),
                Attr(attr::Target, self.target),
                Attr(attr::Rel, self.rel),
                Attr(attr::Role, self.role),
                Attr(attr::Tabindex, self.tabindex),
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaCurrent, self.aria_current),
                Attr(attr::AriaDisabled, self.aria_disabled),
            ),
            (
                Attr(attr::AriaHaspopup, self.aria_haspopup),
                Attr(attr::AriaExpanded, self.aria_expanded),
                Attr(attr::AriaControls, self.aria_controls),
                Attr(attr::AriaDescribedby, self.aria_describedby),
                custom_attribute("data-focus-visible", self.data_focus_visible),
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
        )
    }
}

/// Makes an element behave and announce itself as a link: press handling for pointer, keyboard
/// and assistive technology, focus, hover and focus-visible state, and the link attributes.
///
/// ```ignore
/// let link = use_link(UseLinkInput {
///     href: Signal::stored(Some("https://example.com".to_owned())),
///     target: LinkTarget::Blank,
///     ..UseLinkInput::default()
/// });
/// let (attrs, styles) = link.props.into_parts();
/// view! { <a {..attrs} style=styles>"Example"</a> }
/// ```
pub fn use_link(input: UseLinkInput) -> UseLinkReturn {
    let UseLinkInput {
        href,
        target,
        mut rel,
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
        Some(responder_disabled) => {
            Signal::derive(move || is_disabled.get() || responder_disabled.get())
        }
        None => is_disabled,
    };

    let is_anchor = element_type == LinkElementType::Anchor;
    // A new browsing context must not get access to this one.
    // See: <https://developer.chrome.com/docs/lighthouse/best-practices/external-anchors-use-rel-noopener/>
    if target == LinkTarget::Blank && !rel.contains(&LinkRel::NoOpener) {
        rel.push(LinkRel::NoOpener);
    }

    let UseFocusableReturn {
        props: focusable_props,
        focus_handle,
    } = use_focusable(UseFocusableInput {
        is_disabled,
        shortcuts,
        ..UseFocusableInput::default()
    });

    let UsePressReturn {
        props: press_props,
        is_pressed,
    } = use_press(UsePressInput {
        is_disabled,
        // Routers handle link clicks in a document-level listener.
        propagation: crate::hooks::PressPropagation::Continue,
        on_press,
        on_press_start,
        on_press_end,
        on_press_change,
        ..UsePressInput::default()
    });

    let UseHoverReturn {
        props: hover_props,
        is_hovered,
    } = use_hover(UseHoverInput {
        is_disabled,
        on_hover_start,
        on_hover_end,
        ..UseHoverInput::default()
    });

    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focused,
        is_focus_visible,
    } = use_focus_ring(UseFocusRingInput {
        is_disabled,
        ..UseFocusRingInput::default()
    });

    let (press_props, press_styles) = press_props.into_inner();
    let context_describedby = focusable_props.context_aria_describedby;
    let press_describedby = press_props.aria_describedby;

    // Other elements are made focusable (react-aria: `tabIndex: 0` unless disabled).
    let focusable_tabindex = focusable_props.tabindex;
    let tabindex = Signal::derive(move || match element_type {
        LinkElementType::Anchor => focusable_tabindex.get(),
        LinkElementType::Other => {
            (!is_disabled.get()).then(|| focusable_tabindex.get().unwrap_or(0))
        }
    });

    let element_capture = match trigger {
        Some(trigger) => focusable_props
            .element_capture
            .chain(trigger.element.attr()),
        None => focusable_props.element_capture,
    };
    #[cfg(debug_assertions)]
    let element_capture = element_capture.chain(ElementCaptureAttr::new(move |el| {
        super::debug_validate_element_type(element_type, &el);
    }));

    let props = UseLinkProps {
        href: Signal::derive(move || href.get().filter(|_| is_anchor && !is_disabled.get())),
        target: (is_anchor && target != LinkTarget::Same).then(|| target.to_oco()),
        rel: link_rel_to_string(&rel).filter(|_| is_anchor),
        // An `<a>` without `href` (disabled) has no implicit role: keep it a link.
        role: Signal::derive(move || (!is_anchor || is_disabled.get()).then_some(AriaRole::Link)),
        tabindex,
        aria_label,
        aria_current,
        aria_disabled: Signal::derive(move || is_disabled.get().then_some(AriaDisabled::True)),
        aria_haspopup: trigger.map_or_else(Signal::default, |t| t.aria_haspopup),
        aria_expanded: trigger.map_or_else(Signal::default, |t| t.aria_expanded),
        aria_controls: trigger.map_or_else(Signal::default, |t| t.aria_controls),
        aria_describedby: Signal::derive(move || {
            let ids: Vec<String> = press_describedby
                .with(|d| {
                    d.as_ref()
                        .map(|d| d.ids().map(str::to_owned).collect::<Vec<_>>())
                })
                .unwrap_or_default()
                .into_iter()
                .chain(context_describedby.get())
                .collect();
            (!ids.is_empty()).then(|| ids.join(" "))
        }),
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
        props: PropsWithStyles {
            props,
            styles: press_styles,
        },
        is_disabled,
        is_pressed,
        is_hovered,
        is_focused,
        is_focus_visible,
        focus_handle,
    }
}
