use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
    tachys::html::style::{style, Style},
};
use web_sys::{FocusEvent, KeyboardEvent};

use crate::{
    hooks::{
        overlay::{use_overlay::UseOverlayProps, use_overlay_position::UseOverlayPositionProps},
        IntoAttrs,
    },
    utils::{ElementCaptureAttr, EventHandler, MergeWith},
};

/// Combined props from `use_overlay` and `use_overlay_position` hooks.
///
/// This type includes:
/// - Overlay props: `id`, `element_capture`, `on_keydown`, `on_focusin`, `on_focusout`
/// - Position props: `element_capture` (chained with overlay's), `position`, `z_index`, `top`, `left`, `max_height`
///
/// The two `element_capture` fields are chained into a single `ElementCaptureAttr`
/// so that both hooks capture the same overlay element.
///
/// # Example
///
/// ```ignore
/// use leptonic::utils::MergeWith;
///
/// let overlay = use_overlay(overlay_input);
/// let position = use_overlay_position(position_input);
/// let combined = overlay.props.merge_with(position.props);
///
/// view! {
///     <div {..combined.into_attrs()}>
///         "Positioned overlay"
///     </div>
/// }
/// ```
#[derive(Debug)]
pub struct MergedOverlayOverlayPositionProps {
    // From use_overlay (element_capture is chained from both hooks).
    pub id: String,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_focusin: EventHandler<FocusEvent>,
    pub on_focusout: EventHandler<FocusEvent>,
    // From use_overlay_position.
    pub position: Signal<(&'static str, String)>,
    pub z_index: Signal<(&'static str, String)>,
    pub top: Signal<(&'static str, String)>,
    pub left: Signal<(&'static str, String)>,
    pub max_height: Signal<(&'static str, String)>,
}

/// Attribute tuple type for [`MergedOverlayOverlayPositionProps`].
///
/// Spread this onto elements: `<div {..attrs}/>`
pub type MergedOverlayOverlayPositionAttrs = (
    // From use_overlay.
    Attr<attr::Id, String>,
    ElementCaptureAttr,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::focusin, SharedEventCallback<FocusEvent>>,
    On<ev::focusout, SharedEventCallback<FocusEvent>>,
    // From use_overlay_position.
    Style<Signal<(&'static str, String)>>,
    Style<Signal<(&'static str, String)>>,
    Style<Signal<(&'static str, String)>>,
    Style<Signal<(&'static str, String)>>,
    Style<Signal<(&'static str, String)>>,
);

impl IntoAttrs for MergedOverlayOverlayPositionProps {
    type Attrs = MergedOverlayOverlayPositionAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            // Overlay attrs.
            Attr(attr::Id, self.id),
            self.element_capture,
            self.on_keydown.into_on(ev::keydown),
            self.on_focusin.into_on(ev::focusin),
            self.on_focusout.into_on(ev::focusout),
            // Position attrs.
            style(self.position),
            style(self.z_index),
            style(self.top),
            style(self.left),
            style(self.max_height),
        )
    }
}

impl MergeWith<UseOverlayPositionProps> for UseOverlayProps {
    type Output = MergedOverlayOverlayPositionProps;

    fn merge_with(self, position: UseOverlayPositionProps) -> Self::Output {
        let UseOverlayProps {
            id: overlay_id,
            element_capture: overlay_element_capture,
            on_keydown: overlay_on_keydown,
            on_focusin: overlay_on_focusin,
            on_focusout: overlay_on_focusout,
        } = self;

        let UseOverlayPositionProps {
            element_capture: position_element_capture,
            position: position_position,
            z_index: position_z_index,
            top: position_top,
            left: position_left,
            max_height: position_max_height,
        } = position;

        MergedOverlayOverlayPositionProps {
            // From overlay (element_capture chained from both hooks).
            id: overlay_id,
            element_capture: overlay_element_capture.chain(position_element_capture),
            on_keydown: overlay_on_keydown,
            on_focusin: overlay_on_focusin,
            on_focusout: overlay_on_focusout,
            // From position (distinct).
            position: position_position,
            z_index: position_z_index,
            top: position_top,
            left: position_left,
            max_height: position_max_height,
        }
    }
}

impl MergeWith<UseOverlayProps> for UseOverlayPositionProps {
    type Output = MergedOverlayOverlayPositionProps;

    fn merge_with(self, other: UseOverlayProps) -> Self::Output {
        // Merge order for these types is irrelevant.
        other.merge_with(self)
    }
}
