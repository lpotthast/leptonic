use std::rc::Rc;

use educe::Educe;
use leptos::{html::ElementDescriptor, Attribute, NodeRef, Oco};
use leptos_reactive::{create_memo, MaybeSignal, SignalGet};
use leptos_use::{use_document, use_element_bounding};
use typed_builder::TypedBuilder;

use crate::utils::{attributes::Attributes, locale::WritingDirection};

// TODO: Serialize, Deserialize, Display, FormStr ???

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

#[derive(Clone, Copy, Educe, TypedBuilder)]
#[educe(Debug)]
pub struct UseOverlayPositionInput<OverlayRef, TargetRef>
where
    OverlayRef: ElementDescriptor + 'static,
    TargetRef: ElementDescriptor + 'static,
{
    /// Element that resembles the overlay content.
    #[educe(Debug(ignore))]
    pub(crate) overlay_ref: NodeRef<OverlayRef>,

    /// Element to which the overlay should be positioned relative to.
    #[educe(Debug(ignore))]
    pub(crate) target_ref: NodeRef<TargetRef>,

    #[builder(setter(into))]
    pub(crate) placement_x: MaybeSignal<PlacementX>,

    #[builder(setter(into))]
    pub(crate) placement_y: MaybeSignal<PlacementY>,

    #[builder(setter(into))]
    pub(crate) writing_direction: MaybeSignal<WritingDirection>,
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

    let attrs = Attributes::new().insert(
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
