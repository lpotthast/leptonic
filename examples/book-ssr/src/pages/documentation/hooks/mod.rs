use leptos::prelude::*;

pub mod demos;

pub mod anchor_link;
pub mod breadcrumbs;
pub mod button;
pub mod checkbox;
pub mod color;
pub mod color_area;
pub mod color_channel_field;
pub mod color_field;
pub mod color_slider;
pub mod color_swatch;
pub mod color_wheel;
pub mod combobox;
pub mod disclosure;
pub mod dnd;
pub mod focus;
pub mod focus_manager;
pub mod focus_ring;
pub mod focus_visible;
pub mod focus_within;
pub mod focusable;
pub mod grid;
pub mod has_tabbable_child;
pub mod hover;
pub mod interact_outside;
pub mod keyboard;
pub mod label;
pub mod link;
pub mod listbox;
pub mod menu;
pub mod meter;
pub mod modal;
pub mod r#move;
pub mod number_field;
pub mod overlay;
pub mod popover;
pub mod press;
pub mod prevent_scroll;
pub mod progress;
pub mod radio;
pub mod scroll_wheel;
pub mod select;
pub mod selection;
pub mod separator;
pub mod slider;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod tag;
pub mod text_field;
pub mod toolbar;
pub mod tooltip;
pub mod tree;

#[component]
pub fn PageHooks() -> impl IntoView {
    view! {
        <div>
            <h1>Hooks</h1>
            <p>
                Hooks are a way to extend the functionality of Leptos components.
                They are a way to add functionality to components without having to
                modify the component itself.
            </p>
        </div>
    }
}
