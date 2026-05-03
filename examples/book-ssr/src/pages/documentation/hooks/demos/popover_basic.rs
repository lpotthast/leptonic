use leptonic::hooks::{PlacementX, PlacementY, *};
use leptonic::utils::{classes::Classes, locale::WritingDirection};
use leptos::{portal::Portal, prelude::*};

/// Basic popover demo
#[component]
pub fn BasicPopoverDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    let UsePopoverReturn {
        props,
        trigger_props,
        underlay_props,
        id: _,
        resolved_placement_x: _,
        resolved_placement_y: _,
    } = use_popover(UsePopoverInput {
        is_open: is_open.into(),
        on_close: Callback::new(move |()| set_is_open.set(false)),
        placement_x: Signal::derive(|| PlacementX::Center),
        placement_y: Signal::derive(|| PlacementY::Below),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        offset: 0.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        is_non_modal: false,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });

    let trigger_attrs = StoredValue::new(trigger_props.into_attrs());
    let (popover_props, popover_styles) = props.into_parts();
    let popover_props = StoredValue::new(popover_props);
    let popover_styles = StoredValue::new(popover_styles);
    let underlay_props = StoredValue::new(underlay_props.into_attrs());

    view! {
        <div class=Classes::from("demo-flex-center")>
            <button
                {..trigger_attrs.get_value()}
                on:click=move |_| set_is_open.set(!is_open.get())
                class=Classes::from("demo-btn-primary")
            >
                {move || if is_open.get() { "Close Popover" } else { "Open Popover" }}
            </button>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                // Optional underlay - captures pointer events to close
                <div
                    {..underlay_props.get_value()}
                    style="position: fixed; inset: 0; z-index: 999;"
                />
                // Popover content
                <div
                    {..popover_props.get_value()}
                    style=popover_styles.get_value()
                    class=Classes::from("demo-popover-panel")
                >
                    <h4 style="margin: 0 0 0.5em 0; color: #333;">"Popover Title"</h4>
                    <p style="margin: 0; color: #666;">
                        "This is popover content. Press Escape or click outside to close."
                    </p>
                </div>
            </Show>
        </Portal>
    }
}
