use leptos::prelude::*;

use crate::{
    atoms::separator::Separator as SeparatorAtom,
    utils::{classes::Classes, orientation::Orientation, styles::Styles},
};

/// A themed separator: a line between content, horizontal by default. A vertical separator stretches to the height
/// of its row (put it in a flex row).
///
/// Styled with the `--separator-*` theme variables.
#[component]
pub fn Separator(
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    view! {
        <SeparatorAtom
            orientation=orientation
            aria_label=aria_label
            classes=classes.add("leptonic-separator")
            styles=styles
        />
    }
}
