use std::rc::Rc;

use educe::Educe;
use leptos::{
    create_memo, create_signal, html::ElementDescriptor, Attribute, IntoAttribute, NodeRef, Oco,
    ReadSignal, SignalGet, WriteSignal,
};
use leptos_reactive::{MaybeSignal, Signal};
use leptos_use::{use_document, use_element_bounding};

use crate::{
    prelude::{AriaExpanded, AriaHasPopup},
    utils::{
        aria::{AriaAttribute, AriaControls, GenericAttribute},
        locale::WritingDirection,
        props::Attributes,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/overlays/src/useOverlay.ts

#[derive(Debug, Clone, Copy)]
pub struct UseOverlayInput {
    /// Disables the handling overlay events when true.
    pub disabled: MaybeSignal<bool>,
}

#[derive(Debug)]
pub struct UseOverlayReturn {
    pub props: UseOverlayProps,

    pub id: Oco<'static, str>,

    /// Whether the overlay should be shown.
    pub state: ReadSignal<bool>,
    pub set_state: WriteSignal<bool>,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct UseOverlayProps {
    /// These attributes must be spread onto the target element: `<foo {..attrs} />`
    pub attrs: Attributes,
}

pub fn use_overlay(input: UseOverlayInput) -> UseOverlayReturn {
    let (state, set_state) = create_signal(false);
    let id = uuid::Uuid::new_v4();

    let mut attrs = Attributes::new();
    attrs.insert("id", id.to_string().into_attribute());

    UseOverlayReturn {
        props: UseOverlayProps { attrs },
        id: Oco::Owned(id.to_string()),
        state,
        set_state,
    }
}

// ----------------------------

#[derive(Debug, Clone)]
pub struct UseOverlayTriggerInput {
    /// Whether the overlay is currently shown.
    pub show: Signal<bool>,

    pub overlay_id: Oco<'static, str>,

    /// The type of overlay opened by this trigger.
    /// Using the variants `False` or `True` will result in a runtime warning on debug builds!
    /// Prefer `AriaHasPopup::Menu` if you are unsure what to use otherwise.
    pub overlay_type: AriaHasPopup,
}

#[derive(Debug)]
pub struct UseOverlayTriggerReturn {
    /// Props for the trigger.
    pub props: UseOverlayTriggerProps,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct UseOverlayTriggerProps {
    /// These attributes must be spread onto the target element: `<foo {..attrs} />`
    pub attrs: Attributes,
}

#[derive(Educe)]
#[educe(Debug)]
pub struct UseOverlayTriggerOverlayProps {
    /// These attributes must be spread onto the target element: `<foo {..attrs} />`
    pub attrs: Attributes,
}

pub fn use_overlay_trigger(input: UseOverlayTriggerInput) -> UseOverlayTriggerReturn {
    #[cfg(debug_assertions)]
    fn get_overlay_type(input: &UseOverlayTriggerInput) -> AriaHasPopup {
        match input.overlay_type {
            unexpected @ AriaHasPopup::False | unexpected @ AriaHasPopup::True => {
                tracing::warn!(?unexpected, "use_overlay_trigger received unexpected AriaHasPopup variant. Do not use `False` or `True`.");
                unexpected
            }
            other => other,
        }
    }
    #[cfg(not(debug_assertions))]
    fn get_overlay_type(input: &UseOverlayTriggerInput) -> AriaHasPopup {
        input.overlay_type
    }
    let aria_has_popup = get_overlay_type(&input);

    let overlay_id = input.overlay_id;

    let mut trigger_attrs = Attributes::new();
    trigger_attrs.insert_entry(AriaAttribute::HasPopup(GenericAttribute::Static(
        aria_has_popup,
    )));
    trigger_attrs.insert_entry(AriaAttribute::Expanded(GenericAttribute::Fn(Rc::new(
        move || AriaExpanded::from(input.show.get()),
    ))));
    trigger_attrs.insert_entry(AriaAttribute::Controls(GenericAttribute::Fn(Rc::new(
        move || match input.show.get() {
            true => AriaControls::Id(vec![overlay_id.to_string()]),
            false => AriaControls::Undefined,
        },
    ))));

    UseOverlayTriggerReturn {
        props: UseOverlayTriggerProps {
            attrs: trigger_attrs,
        },
    }
}

// ----------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlacementX {
    OuterLeft,
    OuterStart,
    Start,
    Left,
    Center,
    Right,
    End,
    OuterEnd,
    OuterRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalPlacementX {
    OuterLeft,
    Left,
    Center,
    Right,
    OuterRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlacementY {
    Above,
    Top,
    Center,
    Bottom,
    Below,
}

impl PlacementX {
    fn direction_aware(self, direction: WritingDirection) -> PhysicalPlacementX {
        match self {
            Self::OuterLeft => PhysicalPlacementX::OuterLeft,
            Self::OuterStart => match direction {
                WritingDirection::Ltr => PhysicalPlacementX::OuterLeft,
                WritingDirection::Rtl => PhysicalPlacementX::OuterRight,
            },
            Self::Start => match direction {
                WritingDirection::Ltr => PhysicalPlacementX::Left,
                WritingDirection::Rtl => PhysicalPlacementX::Right,
            },
            Self::Left => PhysicalPlacementX::Left,
            Self::Center => PhysicalPlacementX::Center,
            Self::Right => PhysicalPlacementX::Right,
            Self::End => match direction {
                WritingDirection::Ltr => PhysicalPlacementX::Right,
                WritingDirection::Rtl => PhysicalPlacementX::Left,
            },
            Self::OuterEnd => match direction {
                WritingDirection::Ltr => PhysicalPlacementX::OuterRight,
                WritingDirection::Rtl => PhysicalPlacementX::OuterLeft,
            },
            Self::OuterRight => PhysicalPlacementX::OuterRight,
        }
    }
}

#[derive(Clone, Copy, Educe)]
#[educe(Debug)]
pub struct UseOverlayPositionInput<OverlayRef, TargetRef>
where
    OverlayRef: ElementDescriptor + 'static,
    TargetRef: ElementDescriptor + 'static,
{
    /// Element that resembles the overlay content.
    #[educe(Debug(ignore))]
    pub overlay_ref: NodeRef<OverlayRef>,

    /// Element to which the overlay should be positioned relative to.
    #[educe(Debug(ignore))]
    pub target_ref: NodeRef<TargetRef>,

    pub placement_x: MaybeSignal<PlacementX>,
    pub placement_y: MaybeSignal<PlacementY>,

    pub writing_direction: MaybeSignal<WritingDirection>,
}

#[derive(Debug)]
pub struct UseOverlayPositionReturn {
    pub props: UseOverlayPositionProps,
}

#[derive(Debug)]
pub struct UseOverlayPositionProps {
    pub attrs: Attributes,
}

pub fn use_overlay_position<OverlayRef, TargetRef>(
    input: UseOverlayPositionInput<OverlayRef, TargetRef>,
) -> UseOverlayPositionReturn
where
    OverlayRef: ElementDescriptor + Clone + 'static,
    TargetRef: ElementDescriptor + Clone + 'static,
{
    let overlay_bounding = use_element_bounding(input.overlay_ref);
    let target_bounding = use_element_bounding(input.target_ref);

    let container_width = move || match use_document().as_ref() {
        Some(document) => match document.body() {
            Some(body) => body.client_width() as f64,
            None => 0.0,
        },
        None => 0.0,
    };

    let container_height = move || match use_document().as_ref() {
        Some(document) => match document.body() {
            Some(body) => body.client_height() as f64,
            None => 0.0,
        },
        None => 0.0,
    };

    let placement_x = create_memo(move |_| {
        match input
            .placement_x
            .get()
            .direction_aware(input.writing_direction.get())
        {
            original @ PhysicalPlacementX::OuterLeft => {
                let space_left = target_bounding.left.get();
                match overlay_bounding.width.get() > space_left {
                    true => PhysicalPlacementX::OuterRight,
                    false => original,
                }
            }
            original @ PhysicalPlacementX::OuterRight => {
                let space_right = container_width() - target_bounding.right.get();
                match overlay_bounding.width.get() > space_right {
                    true => PhysicalPlacementX::OuterLeft,
                    false => original,
                }
            }
            other => other,
        }
    });

    let placement_y = create_memo(move |_| match input.placement_y.get() {
        original @ PlacementY::Above => {
            let space_top = target_bounding.top.get();
            match overlay_bounding.height.get() > space_top {
                true => PlacementY::Below,
                false => original,
            }
        }
        original @ PlacementY::Below => {
            let space_bottom = container_height() - target_bounding.bottom.get();
            match overlay_bounding.height.get() > space_bottom {
                true => PlacementY::Above,
                false => original,
            }
        }
        other => other,
    });

    let top = create_memo(move |_| match placement_y.get() {
        PlacementY::Above => target_bounding.top.get() - overlay_bounding.height.get(),
        PlacementY::Top => target_bounding.top.get(),
        PlacementY::Center => {
            target_bounding.top.get() + (target_bounding.height.get() / 2.0)
                - (overlay_bounding.height.get() / 2.0)
        }
        PlacementY::Bottom => target_bounding.bottom.get() - overlay_bounding.height.get(),
        PlacementY::Below => target_bounding.bottom.get(),
    });

    let left = create_memo(move |_| match placement_x.get() {
        PhysicalPlacementX::OuterLeft => target_bounding.left.get() - overlay_bounding.width.get(),
        PhysicalPlacementX::Left => target_bounding.left.get(),
        PhysicalPlacementX::Center => {
            target_bounding.left.get() + (target_bounding.width.get() / 2.0)
                - (overlay_bounding.width.get() / 2.0)
        }
        PhysicalPlacementX::Right => target_bounding.right.get() - overlay_bounding.width.get(),
        PhysicalPlacementX::OuterRight => target_bounding.right.get(),
    });

    let mut attrs = Attributes::new();
    attrs.insert(
        "style",
        Attribute::Fn(Rc::new(move || {
            let top = top.get();
            let left = left.get();
            let style = format!("position: fixed; z-index: 100000; top: {top}px; left: {left}px");
            Attribute::String(Oco::Owned(style))
        })),
    );

    UseOverlayPositionReturn {
        props: UseOverlayPositionProps { attrs },
    }
}
