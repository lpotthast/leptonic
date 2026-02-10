use std::marker::PhantomData;

use educe::Educe;
use leptos::prelude::*;
use leptos::tachys::html::style::style;
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
    /// Props for the overlay element. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub props: UseOverlayPositionProps,
}

/// Props from `use_overlay_position` that can be converted to spreadable attributes.
#[derive(Debug, Clone)]
pub struct UseOverlayPositionProps {
    pub position: Signal<(&'static str, String)>,
    pub z_index: Signal<(&'static str, String)>,
    pub top: Signal<(&'static str, String)>,
    pub left: Signal<(&'static str, String)>,
}

impl UseOverlayPositionProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseOverlayPositionAttrs {
        (
            style(self.position),
            style(self.z_index),
            style(self.top),
            style(self.left),
        )
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseOverlayPositionAttrs {
        (
            style(self.position),
            style(self.z_index),
            style(self.top),
            style(self.left),
        )
    }
}

pub type UseOverlayPositionAttrs = (
    Style<Signal<(&'static str, String)>>, // position: fixed
    Style<Signal<(&'static str, String)>>, // z-index: 100000
    Style<Signal<(&'static str, String)>>, // top: Xpx
    Style<Signal<(&'static str, String)>>, // left: Xpx
);

pub fn use_overlay_position<Overlay, Target, M>(
    input: UseOverlayPositionInput<Overlay, Target, M>,
) -> UseOverlayPositionReturn
where
    Overlay: IntoElementMaybeSignal<web_sys::Element, M>,
    Target: IntoElementMaybeSignal<web_sys::Element, M>,
{
    let UseOverlayPositionInput {
        overlay,
        target,
        placement_x,
        placement_y,
        writing_direction,
        phantom_data: _,
    } = input;

    let overlay_bounding = use_element_bounding(overlay);
    let target_bounding = use_element_bounding(target);

    let container_width = move || match use_document().as_ref() {
        Some(document) => match document.body() {
            Some(body) => f64::from(body.client_width()),
            None => 0.0,
        },
        None => 0.0,
    };

    let container_height = move || match use_document().as_ref() {
        Some(document) => match document.body() {
            Some(body) => f64::from(body.client_height()),
            None => 0.0,
        },
        None => 0.0,
    };

    let placement_x =
        Memo::new(
            move |_| match placement_x.get().direction_aware(writing_direction.get()) {
                original @ PhysicalPlacementX::OuterLeft => {
                    let space_left = target_bounding.left.get();
                    if overlay_bounding.width.get() > space_left {
                        PhysicalPlacementX::OuterRight
                    } else {
                        original
                    }
                }
                original @ PhysicalPlacementX::OuterRight => {
                    let space_right = container_width() - target_bounding.right.get();
                    if overlay_bounding.width.get() > space_right {
                        PhysicalPlacementX::OuterLeft
                    } else {
                        original
                    }
                }
                other => other,
            },
        );

    let placement_y = Memo::new(move |_| match placement_y.get() {
        original @ PlacementY::Above => {
            let space_top = target_bounding.top.get();
            if overlay_bounding.height.get() > space_top {
                PlacementY::Below
            } else {
                original
            }
        }
        original @ PlacementY::Below => {
            let space_bottom = container_height() - target_bounding.bottom.get();
            if overlay_bounding.height.get() > space_bottom {
                PlacementY::Above
            } else {
                original
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
        props: UseOverlayPositionProps {
            position: Signal::derive(|| ("position", String::from("fixed"))),
            z_index: Signal::derive(|| ("z-index", String::from("100000"))),
            top: Signal::derive(move || ("top", format!("{}px", top.get()))),
            left: Signal::derive(move || ("left", format!("{}px", left.get()))),
        },
    }
}
