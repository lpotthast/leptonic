use leptos::prelude::*;

use crate::{
    Out, Width,
    atoms::{
        dialog::Dialog,
        modal::{ModalBackdrop, ModalBackdropProps, ModalContent},
    },
    utils::{classes::Classes, css::css_custom_property, styles::Styles},
};

css_custom_property!(DRAWER_WIDTH: Width = "--drawer-width");

/// The side of the screen a [`Drawer`] slides in from.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerSide {
    #[default]
    Left,
    Right,
}

impl DrawerSide {
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    /// The panel's class (`leptonic-drawer-left`/`-right`).
    const fn class(self) -> &'static str {
        match self {
            Self::Left => "leptonic-drawer-left",
            Self::Right => "leptonic-drawer-right",
        }
    }
}

/// A panel sliding in from a side of the screen over the page (e.g. a menu on small screens): a
/// modal dialog on the modal atoms. Escape and (when `is_dismissable`) a press outside close it;
/// the focus stays inside while it is open and returns afterwards; the page behind it is inert and
/// doesn't scroll. It slides in and out (`data-entering`/`data-exiting`; not with reduced motion).
///
/// Its open state is `is_open` + `set_open`, or `default_open`; `on_open_change` observes. Inside
/// a `DialogTrigger`, the trigger opens it. Name it with `aria_label` or a `DialogTitle` inside.
#[component]
#[allow(clippy::too_many_arguments)]
pub fn Drawer(
    #[prop(optional)] side: DrawerSide,
    /// The panel's width. Default: the theme's (`17em`, at most 85% of the viewport).
    #[prop(optional)]
    width: Option<Width>,
    /// Whether it is open (controlled): a value or any signal. Default: the surrounding
    /// `DialogTrigger`'s state.
    #[prop(into, optional)]
    is_open: Option<Signal<bool>>,
    /// Receives the open state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_open: Option<Out<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    /// Whether a press outside closes it. Default: `true`.
    #[prop(into, default = Signal::stored(true))]
    is_dismissable: Signal<bool>,
    #[prop(into, optional)] is_keyboard_dismiss_disabled: Signal<bool>,
    /// Names the drawer when it has no `DialogTitle`.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    let panel = StoredValue::new((classes, styles, children));
    let content = move || {
        let (classes, styles, children) = panel.get_value();
        view! {
            <ModalContent classes=Classes::from("leptonic-drawer").add(side.class())>
                <Dialog aria_label=aria_label classes=classes.add("leptonic-drawer-dialog") styles=styles>
                    {children()}
                </Dialog>
            </ModalContent>
        }
    };
    let mut props = ModalBackdropProps::builder()
        .is_dismissable(is_dismissable)
        .is_keyboard_dismiss_disabled(is_keyboard_dismiss_disabled)
        .classes(Classes::from("leptonic-modal-backdrop").add("leptonic-drawer-backdrop"))
        // Inherited by the panel.
        .styles(Styles::new().add_optional(width.map(|width| DRAWER_WIDTH.declare(width))))
        .children(ToChildren::to_children(content))
        .build();
    props.is_open = is_open;
    props.set_open = set_open;
    props.default_open = default_open;
    props.on_open_change = on_open_change;
    ModalBackdrop(props)
}
