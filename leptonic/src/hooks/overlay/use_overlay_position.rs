use std::rc::Rc;

use educe::Educe;
use leptos::{html::ElementDescriptor, Attribute, NodeRef, Oco};
use leptos_reactive::{create_memo, MaybeSignal, SignalGet};
use leptos_use::{use_element_bounding, use_window, UseElementBoundingReturn};
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
pub struct UseOverlayPositionInput<OverlayRef, TargetRef, ContainerRef = leptos::html::Custom>
where
    OverlayRef: ElementDescriptor + 'static,
    TargetRef: ElementDescriptor + 'static,
    ContainerRef: ElementDescriptor + 'static,
{
    /// Element that resembles the overlay content.
    #[educe(Debug(ignore))]
    #[builder(setter(into))]
    pub(crate) overlay_ref: NodeRef<OverlayRef>,

    /// Element to which the overlay should be positioned relative to.
    #[educe(Debug(ignore))]
    #[builder(setter(into))]
    pub(crate) target_ref: NodeRef<TargetRef>,

    /// Element in which the overlay should be contained.
    /// Dimensions of this container are used to calculate positional flips of the overlay.
    /// If omitted, the **window** acts as the container,
    /// resulting in overlays switching places if they would otherwise go out ot the visible viewport.
    #[educe(Debug(ignore))]
    #[builder(default = None, setter(into))]
    pub(crate) container_ref: Option<NodeRef<ContainerRef>>,

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

pub fn use_overlay_position<OverlayRef, TargetRef, ContainerRef>(
    input: UseOverlayPositionInput<OverlayRef, TargetRef, ContainerRef>,
) -> UseOverlayPositionReturn
where
    OverlayRef: ElementDescriptor + Clone + 'static,
    TargetRef: ElementDescriptor + Clone + 'static,
    ContainerRef: ElementDescriptor + Clone + 'static,
{
    let UseElementBoundingReturn {
        height: overlay_height,
        width: overlay_width,
        left: _overlay_left,
        right: _overlay_right,
        top: _overlay_top,
        bottom: _overlay_bottom,
        x: _overlay_x,
        y: _overlay_y,
        update: _overlay_update,
    } = use_element_bounding(input.overlay_ref);

    let UseElementBoundingReturn {
        height: target_height,
        width: target_width,
        left: target_left,
        right: target_right,
        top: target_top,
        bottom: target_bottom,
        x: _target_x,
        y: _target_y,
        update: _target_update,
    } = use_element_bounding(input.target_ref);

    let container_bounding = input.container_ref.map(use_element_bounding);
    let container_width = container_bounding.as_ref().map(|it| it.width);
    let container_height = container_bounding.as_ref().map(|it| it.height);

    let window_width = move || match use_window().as_ref() {
        Some(window) => match window.inner_width() {
            Ok(val) => val.as_f64().unwrap_or(0.0),
            Err(_val) => 0.0,
        },
        None => 0.0,
    };

    let window_height = move || match use_window().as_ref() {
        Some(window) => match window.inner_height() {
            Ok(val) => val.as_f64().unwrap_or(0.0),
            Err(_val) => 0.0,
        },
        None => 0.0,
    };

    let container_width = move || match container_width {
        Some(container_width) => container_width.get(),
        None => window_width(),
    };

    let container_height = move || match container_height {
        Some(container_height) => container_height.get(),
        None => window_height(),
    };

    let placement_x = create_memo(move |_| {
        match input
            .placement_x
            .get()
            .direction_aware(input.writing_direction.get())
        {
            original @ PhysicalPlacementX::OuterLeft => {
                let space_left = target_left.get();
                match overlay_width.get() > space_left {
                    true => PhysicalPlacementX::OuterRight,
                    false => original,
                }
            }
            original @ PhysicalPlacementX::OuterRight => {
                let space_right = container_width() - target_right.get();
                match overlay_width.get() > space_right {
                    true => PhysicalPlacementX::OuterLeft,
                    false => original,
                }
            }
            other => other,
        }
    });

    let placement_y = create_memo(move |_| match input.placement_y.get() {
        original @ PlacementY::Above => {
            let space_top = target_top.get();
            match overlay_height.get() > space_top {
                true => PlacementY::Below,
                false => original,
            }
        }
        original @ PlacementY::Below => {
            let space_bottom = container_height() - target_bottom.get();
            match overlay_height.get() > space_bottom {
                true => PlacementY::Above,
                false => original,
            }
        }
        other => other,
    });

    let top = create_memo(move |_| match placement_y.get() {
        PlacementY::Above => target_top.get() - overlay_height.get(),
        PlacementY::Top => target_top.get(),
        PlacementY::Center => {
            target_top.get() + (target_height.get() / 2.0) - (overlay_height.get() / 2.0)
        }
        PlacementY::Bottom => target_bottom.get() - overlay_height.get(),
        PlacementY::Below => target_bottom.get(),
    });

    let left = create_memo(move |_| match placement_x.get() {
        PhysicalPlacementX::OuterLeft => target_left.get() - overlay_width.get(),
        PhysicalPlacementX::Left => target_left.get(),
        PhysicalPlacementX::Center => {
            target_left.get() + (target_width.get() / 2.0) - (overlay_width.get() / 2.0)
        }
        PhysicalPlacementX::Right => target_right.get() - overlay_width.get(),
        PhysicalPlacementX::OuterRight => target_right.get(),
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
