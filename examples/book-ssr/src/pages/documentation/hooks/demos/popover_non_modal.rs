use leptonic::hooks::{PlacementX, PlacementY, *};
use leptonic::utils::{classes::Classes, locale::WritingDirection};
use leptos::{portal::Portal, prelude::*};

/// Non-modal popover demo
#[component]
pub fn NonModalPopoverDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (counter, set_counter) = signal(0);

    let UsePopoverReturn {
        props,
        trigger_props,
        underlay_props: _,
        id: _,
        resolved_placement_x: _,
        resolved_placement_y: _,
    } = use_popover(UsePopoverInput {
        is_open: is_open.into(),
        on_close: Callback::new(move |()| set_is_open.set(false)),
        placement_x: Signal::derive(|| PlacementX::OuterRight),
        placement_y: Signal::derive(|| PlacementY::Top),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        offset: 0.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        is_non_modal: true, // Allow interaction outside
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });

    let trigger_attrs = StoredValue::new(trigger_props.into_attrs());
    let (popover_props, popover_styles) = props.into_parts();
    let popover_props = StoredValue::new(popover_props);
    let popover_styles = StoredValue::new(popover_styles);

    view! {
        <div style="display: flex; gap: 1em; align-items: center; justify-content: center; padding: 2em;">
            <button
                {..trigger_attrs.get_value()}
                on:click=move |_| set_is_open.set(!is_open.get())
                class=Classes::from("demo-btn-primary")
            >
                {move || if is_open.get() { "Close" } else { "Open Non-Modal" }}
            </button>

            <button
                on:click=move |_| set_counter.update(|c| *c += 1)
                style="padding: 0.75em 1.5em; border-radius: 8px; cursor: pointer; border: 1px solid #ccc; font-size: 1em;"
            >
                {move || format!("Counter: {}", counter.get())}
            </button>
        </div>

        <p style="text-align: center; color: #666;">
            "Notice: You can still click the counter button while the popover is open!"
        </p>

        <Portal>
            <Show when=move || is_open.get()>
                // No underlay for non-modal popover
                <div
                    {..popover_props.get_value()}
                    style=popover_styles.get_value()
                        .add("background", "white")
                        .add("border", "1px solid #ccc")
                        .add("border-radius", "8px")
                        .add("padding", "1em")
                        .add("box-shadow", "0 4px 12px rgba(0,0,0,0.15)")
                        .add("z-index", "1000")
                        .add("max-width", "200px")
                >
                    <p style="margin: 0; color: #666;">
                        "This is a non-modal popover. You can interact with elements outside!"
                    </p>
                </div>
            </Show>
        </Portal>
    }
}
