use std::marker::PhantomData;

use educe::Educe;
use leptos::prelude::*;
use leptos::tachys::html::style::Style;
use leptos_use::core::IntoElementMaybeSignal;
use leptos_use::{use_document, use_element_bounding};

use crate::utils::locale::WritingDirection;

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

#[derive(Clone, Copy, Educe)]
#[educe(Debug)]
pub struct UseOverlayPositionInput<Overlay, Target, M>
where
    Overlay: IntoElementMaybeSignal<web_sys::Element, M>,
    Target: IntoElementMaybeSignal<web_sys::Element, M>,
{
    /// Element that resembles the overlay content.
    #[educe(Debug(ignore))]
    pub overlay: Overlay,

    /// Element to which the overlay should be positioned relative to.
    #[educe(Debug(ignore))]
    pub target: Target,

    pub placement_x: Signal<PlacementX>,
    pub placement_y: Signal<PlacementY>,

    pub writing_direction: Signal<WritingDirection>,

    pub phantom_data: PhantomData<M>,
}

#[derive(Debug)]
pub struct UseOverlayPositionReturn {
    pub attrs: UseOverlayPositionAttrs,
}

pub type UseOverlayPositionAttrs = (
    Style<Signal<(&'static str, String)>>,
);

pub fn use_overlay_position<Overlay, Target, M>(
    input: UseOverlayPositionInput<Overlay, Target, M>,
) -> UseOverlayPositionReturn
where
    Overlay: IntoElementMaybeSignal<web_sys::Element, M>,
    Target: IntoElementMaybeSignal<web_sys::Element, M>,
{
    let overlay_bounding = use_element_bounding(input.overlay);
    let target_bounding = use_element_bounding(input.target);

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

    let placement_x = Memo::new(move |_| {
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

    let placement_y = Memo::new(move |_| match input.placement_y.get() {
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

    let top = Memo::new(move |_| match placement_y.get() {
        PlacementY::Above => target_bounding.top.get() - overlay_bounding.height.get(),
        PlacementY::Top => target_bounding.top.get(),
        PlacementY::Center => {
            target_bounding.top.get() + (target_bounding.height.get() / 2.0)
                - (overlay_bounding.height.get() / 2.0)
        }
        PlacementY::Bottom => target_bounding.bottom.get() - overlay_bounding.height.get(),
        PlacementY::Below => target_bounding.bottom.get(),
    });

    let left = Memo::new(move |_| match placement_x.get() {
        PhysicalPlacementX::OuterLeft => target_bounding.left.get() - overlay_bounding.width.get(),
        PhysicalPlacementX::Left => target_bounding.left.get(),
        PhysicalPlacementX::Center => {
            target_bounding.left.get() + (target_bounding.width.get() / 2.0)
                - (overlay_bounding.width.get() / 2.0)
        }
        PhysicalPlacementX::Right => target_bounding.right.get() - overlay_bounding.width.get(),
        PhysicalPlacementX::OuterRight => target_bounding.right.get(),
    });

    UseOverlayPositionReturn {
        attrs: (
            leptos::tachys::html::style::style(Signal::derive(move || {
                let top = top.get();
                let left = left.get();
                let position = format!("fixed; z-index: 100000; top: {top}px; left: {left}px");
                ("position", position)
            })),
        ),
    }
}
