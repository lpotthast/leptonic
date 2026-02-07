use leptos::html;
use leptos::prelude::*;
use leptos_use::{use_element_bounding, use_element_hover};
use std::sync::Arc;
use uuid::Uuid;

use crate::{Size, UseElementBoundingReturnReadOnly};

#[derive(Clone)]
struct PopoverData {
    key: Uuid,
    children: ChildrenFn,
}

#[derive(Clone)]
struct PopoverRootContext {
    popovers: RwSignal<Vec<PopoverData>>,
}

impl PopoverRootContext {
    fn push(&self, data: PopoverData) {
        self.popovers.update(move |p| p.push(data));
    }

    fn remove(&self, key: Uuid) {
        self.popovers.update(move |p| {
            if let Some(idx) = p.iter().position(|it| it.key == key) {
                p.remove(idx);
            }
        });
    }
}

#[component]
pub(crate) fn PopoverRoot(children: Children) -> impl IntoView {
    let popovers = RwSignal::new(Vec::new());
    let ctx = PopoverRootContext { popovers };
    provide_context::<PopoverRootContext>(ctx.clone());

    let children = children();
    view! {
        {children}

        <leptonic-popover-host>
            <For
                each=move || ctx.popovers.get()
                key=|it| it.key
                children=|it| view! { {(it.children)()} }
            />
        </leptonic-popover-host>
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PopoverAlignX {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum PopoverAlignY {
    Top,
    Center,
    Bottom,
}

#[derive(Clone)]
#[slot]
pub struct PopoverContent {
    children: ChildrenFn,
}

#[component]
pub fn Popover(
    #[prop(default = PopoverAlignX::Center)] align_x: PopoverAlignX,
    #[prop(default = PopoverAlignY::Top)] align_y: PopoverAlignY,

    /// Margin.
    #[prop(default = Size::Em(0.25))]
    margin: Size,

    /// Custom X position of the popover.
    #[prop(optional)]
    position_x: Option<Callback<UseElementBoundingReturnReadOnly, String>>,

    /// Custom Y position of the popover.
    #[prop(optional)]
    position_y: Option<Callback<UseElementBoundingReturnReadOnly, String>>,

    #[prop(into, optional)] show: Option<Signal<bool>>,

    popover_content: PopoverContent,

    children: Children,
) -> impl IntoView {
    let (clicked, set_clicked) = signal(false);

    let ctx = expect_context::<PopoverRootContext>();

    let el: NodeRef<html::Div> = NodeRef::new();
    let el_bounds = use_element_bounding(el);

    let pop_el: NodeRef<html::Div> = NodeRef::new();
    let pop_bounds = use_element_bounding(pop_el);

    let show = show.unwrap_or_else(|| {
        let is_hovered = use_element_hover(el);
        Signal::derive(move || is_hovered.get() || clicked.get())
    });

    let pop_bounds_read_only: UseElementBoundingReturnReadOnly = pop_bounds.into();

    let pop_style: Signal<String> = Signal::derive(move || {
        if show.get() {
            {
                let left = if let Some(pos_x) = position_x {
                    pos_x.run(pop_bounds_read_only)
                } else {
                    let x = match align_x {
                        PopoverAlignX::Center => {
                            el_bounds.x.get() + (el_bounds.width.get() / 2.0)
                                - (pop_bounds_read_only.width.get() / 2.0)
                        }
                        PopoverAlignX::Left | PopoverAlignX::Right => el_bounds.x.get(),
                    };

                    match align_x {
                        PopoverAlignX::Left => format!("calc({x}px - {margin})"),
                        PopoverAlignX::Center => format!("{x}px"),
                        PopoverAlignX::Right => format!("calc({x}px + {margin})"),
                    }
                };

                let top = if let Some(pos_y) = position_y {
                    pos_y.run(pop_bounds_read_only)
                } else {
                    let y = match align_y {
                        PopoverAlignY::Top => el_bounds.y.get() - pop_bounds_read_only.height.get(),
                        PopoverAlignY::Center | PopoverAlignY::Bottom => el_bounds.y.get(),
                    };

                    match align_y {
                        PopoverAlignY::Top => format!("calc({y}px - {margin})"),
                        PopoverAlignY::Center => format!("{y}px"),
                        PopoverAlignY::Bottom => format!("calc({y}px + {margin})"),
                    }
                };

                format!("left: {left}; top: {top};")
            }
        } else {
            String::new()
        }
    });

    let key = Uuid::now_v7();

    ctx.push(PopoverData {
        key,
        children: Arc::new(move || {
            let v = view! {
                // id=id class=class style=style
                <div
                    class="leptonic-popover"
                    node_ref=pop_el
                    id=key.to_string()
                    style=pop_style
                    data-active=move || if show.get() { "true" } else { "false" }
                >
                    {(popover_content.children)()}
                </div>
            };
            v.into_any()
        }),
    });

    on_cleanup(move || {
        ctx.remove(key);
    });

    view! {
        <div
            class="leptonic-has-popover"
            node_ref=el
            on:click=move |_| set_clicked.set(!clicked.get_untracked())
        >
            {children()}
        </div>
    }
}
