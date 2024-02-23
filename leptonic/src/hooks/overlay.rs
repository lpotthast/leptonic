use std::rc::Rc;

use educe::Educe;
use leptos::{
    create_memo, create_signal, html::ElementDescriptor, Attribute, IntoAttribute, NodeRef, Oco,
    ReadSignal, SignalGet, WriteSignal,
};
use leptos_reactive::{MaybeSignal, Signal};
use leptos_use::{use_element_bounding, use_window};

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
pub enum Placement {
    Bottom,
    BottomLeft,
    BottomRight,
    BottomStart,
    BottomEnd,
    Top,
    TopLeft,
    TopRight,
    TopStart,
    TopEnd,
    Left,
    LeftTop,
    LeftBottom,
    Start,
    StartTop,
    StartBottom,
    Right,
    RightTop,
    RightBottom,
    End,
    EndTop,
    EndBottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicalPlacement {
    Bottom,
    BottomLeft,
    BottomRight,
    Top,
    TopLeft,
    TopRight,
    Left,
    LeftTop,
    LeftBottom,
    Right,
    RightTop,
    RightBottom,
}

impl PhysicalPlacement {
    fn flipped(self) -> Self {
        match self {
            Self::Bottom => Self::Top,
            Self::BottomLeft => Self::TopLeft,
            Self::BottomRight => Self::TopRight,
            Self::Top => Self::Bottom,
            Self::TopLeft => Self::BottomLeft,
            Self::TopRight => Self::BottomRight,
            Self::Left => Self::Right,
            Self::LeftTop => Self::RightTop,
            Self::LeftBottom => Self::RightBottom,
            Self::Right => Self::Left,
            Self::RightTop => Self::LeftTop,
            Self::RightBottom => Self::LeftBottom,
        }
    }
}

impl Placement {
    fn direction_aware(self, direction: WritingDirection) -> PhysicalPlacement {
        match self {
            Placement::BottomStart => match direction {
                WritingDirection::Ltr => PhysicalPlacement::BottomLeft,
                WritingDirection::Rtl => PhysicalPlacement::BottomRight,
            },
            Placement::BottomEnd => match direction {
                WritingDirection::Ltr => PhysicalPlacement::BottomRight,
                WritingDirection::Rtl => PhysicalPlacement::BottomLeft,
            },
            Placement::TopStart => match direction {
                WritingDirection::Ltr => PhysicalPlacement::TopLeft,
                WritingDirection::Rtl => PhysicalPlacement::TopRight,
            },
            Placement::TopEnd => match direction {
                WritingDirection::Ltr => PhysicalPlacement::TopRight,
                WritingDirection::Rtl => PhysicalPlacement::TopLeft,
            },
            Placement::Start => match direction {
                WritingDirection::Ltr => PhysicalPlacement::Left,
                WritingDirection::Rtl => PhysicalPlacement::Right,
            },
            Placement::StartTop => match direction {
                WritingDirection::Ltr => PhysicalPlacement::LeftTop,
                WritingDirection::Rtl => PhysicalPlacement::RightTop,
            },
            Placement::StartBottom => match direction {
                WritingDirection::Ltr => PhysicalPlacement::LeftBottom,
                WritingDirection::Rtl => PhysicalPlacement::RightBottom,
            },
            Placement::End => match direction {
                WritingDirection::Ltr => PhysicalPlacement::Right,
                WritingDirection::Rtl => PhysicalPlacement::Left,
            },
            Placement::EndTop => match direction {
                WritingDirection::Ltr => PhysicalPlacement::RightTop,
                WritingDirection::Rtl => PhysicalPlacement::LeftTop,
            },
            Placement::EndBottom => match direction {
                WritingDirection::Ltr => PhysicalPlacement::RightBottom,
                WritingDirection::Rtl => PhysicalPlacement::LeftBottom,
            },
            Placement::Bottom => PhysicalPlacement::Bottom,
            Placement::BottomLeft => PhysicalPlacement::BottomLeft,
            Placement::BottomRight => PhysicalPlacement::BottomRight,
            Placement::Top => PhysicalPlacement::Top,
            Placement::TopLeft => PhysicalPlacement::TopLeft,
            Placement::TopRight => PhysicalPlacement::TopRight,
            Placement::Left => PhysicalPlacement::Left,
            Placement::LeftTop => PhysicalPlacement::LeftTop,
            Placement::LeftBottom => PhysicalPlacement::LeftBottom,
            Placement::Right => PhysicalPlacement::Right,
            Placement::RightTop => PhysicalPlacement::RightTop,
            Placement::RightBottom => PhysicalPlacement::RightBottom,
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

    pub placement: Placement,

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
    // let body_bounding = use_element_bounding(use_document().body().unwrap());

    let overlay_bounding = use_element_bounding(input.overlay_ref);
    let target_bounding = use_element_bounding(input.target_ref);

    let container_width = create_memo(move |_| {
        use_window()
            .as_ref()
            .map(|w| {
                w.inner_width()
                    .ok()
                    .map(|js| js.as_f64().unwrap_or(0.0))
                    .unwrap_or(0.0)
            })
            .unwrap_or(0.0)
    });

    let container_height = create_memo(move |_| {
        use_window()
            .as_ref()
            .map(|w| {
                w.inner_height()
                    .ok()
                    .map(|js| js.as_f64().unwrap_or(0.0))
                    .unwrap_or(0.0)
            })
            .unwrap_or(0.0)
    });

    let placement = create_memo(move |_| {
        let direction_aware_placement = input
            .placement
            .direction_aware(input.writing_direction.get());

        let space_aware_placement = match direction_aware_placement {
            original @ PhysicalPlacement::Bottom
            | original @ PhysicalPlacement::BottomLeft
            | original @ PhysicalPlacement::BottomRight => {
                let space_bottom = container_height.get() - target_bounding.bottom.get();
                match overlay_bounding.height.get() > space_bottom {
                    true => original.flipped(),
                    false => original,
                }
            }
            original @ PhysicalPlacement::Top
            | original @ PhysicalPlacement::TopLeft
            | original @ PhysicalPlacement::TopRight => {
                let space_top = target_bounding.top.get();
                match overlay_bounding.height.get() > space_top {
                    true => original.flipped(),
                    false => original,
                }
            }
            original @ PhysicalPlacement::Left
            | original @ PhysicalPlacement::LeftTop
            | original @ PhysicalPlacement::LeftBottom => {
                let space_left = target_bounding.left.get();
                match overlay_bounding.width.get() > space_left {
                    true => original.flipped(),
                    false => original,
                }
            }
            original @ PhysicalPlacement::Right
            | original @ PhysicalPlacement::RightTop
            | original @ PhysicalPlacement::RightBottom => {
                let space_right = container_width.get() - target_bounding.right.get();
                match overlay_bounding.width.get() > space_right {
                    true => original.flipped(),
                    false => original,
                }
            }
        };

        space_aware_placement
    });

    let position_memo = create_memo(move |_| {
        let placement = placement.get();

        let top = match placement {
            PhysicalPlacement::Bottom
            | PhysicalPlacement::BottomLeft
            | PhysicalPlacement::BottomRight => {
                target_bounding.top.get() + target_bounding.height.get()
            }
            PhysicalPlacement::Top | PhysicalPlacement::TopLeft | PhysicalPlacement::TopRight => {
                target_bounding.top.get() - overlay_bounding.height.get()
            }
            PhysicalPlacement::Left | PhysicalPlacement::Right => {
                target_bounding.top.get() - (target_bounding.height.get() / 2.0)
                    + (overlay_bounding.height.get() / 2.0)
            }
            PhysicalPlacement::LeftTop | PhysicalPlacement::RightTop => target_bounding.top.get(),
            PhysicalPlacement::LeftBottom | PhysicalPlacement::RightBottom => {
                target_bounding.bottom.get() - overlay_bounding.height.get()
            }
        };

        let left = match placement {
            PhysicalPlacement::Top | PhysicalPlacement::Bottom => {
                target_bounding.left.get() + (target_bounding.width.get() / 2.0)
                    - (overlay_bounding.width.get() / 2.0)
            }
            PhysicalPlacement::TopLeft | PhysicalPlacement::BottomLeft => {
                target_bounding.left.get()
            }
            PhysicalPlacement::TopRight | PhysicalPlacement::BottomRight => {
                target_bounding.right.get() - overlay_bounding.width.get()
            }
            PhysicalPlacement::LeftTop
            | PhysicalPlacement::Left
            | PhysicalPlacement::LeftBottom => {
                target_bounding.left.get() - overlay_bounding.width.get()
            }
            PhysicalPlacement::RightTop
            | PhysicalPlacement::Right
            | PhysicalPlacement::RightBottom => target_bounding.right.get(),
        };

        (top, left)
    });

    let mut attrs = Attributes::new();
    attrs.insert(
        "style",
        Attribute::Fn(Rc::new(move || {
            let (top, left) = position_memo.get();

            tracing::info!(top, left);

            let style =
                format!("position: absolute; z-index: 100000; top: {top}px; left: {left}px");
            Attribute::String(Oco::Owned(style))
        })),
    );

    UseOverlayPositionReturn {
        props: UseOverlayPositionProps { attrs },
    }
}
