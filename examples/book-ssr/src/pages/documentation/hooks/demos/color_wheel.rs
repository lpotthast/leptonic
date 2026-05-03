use leptonic::{
    hooks::*,
    utils::color::{ColorValue, HSL, HSV, HslChannel, HsvChannel},
};
use leptos::prelude::*;

#[component]
pub fn ColorWheelDemo() -> impl IntoView {
    let (mode, set_mode) = signal("hsv");

    view! {
        <div>
            <div style="display: flex; gap: 0.5em; margin-bottom: 1em;">
                <button
                    on:click=move |_| set_mode.set("hsv")
                    style=move || format!(
                        "padding: 0.25em 0.75em; border-radius: 4px; cursor: pointer; \
                         border: 1px solid #888; {}",
                        if mode.get() == "hsv" { "background: #555; color: white;" } else { "background: transparent;" }
                    )
                >
                    "HSV"
                </button>
                <button
                    on:click=move |_| set_mode.set("hsl")
                    style=move || format!(
                        "padding: 0.25em 0.75em; border-radius: 4px; cursor: pointer; \
                         border: 1px solid #888; {}",
                        if mode.get() == "hsl" { "background: #555; color: white;" } else { "background: transparent;" }
                    )
                >
                    "HSL"
                </button>
            </div>

            // Both wheels stay mounted; CSS display toggling preserves state.
            <div style=move || if mode.get() == "hsv" { "" } else { "display: none;" }>
                <HsvWheel />
            </div>
            <div style=move || if mode.get() == "hsl" { "" } else { "display: none;" }>
                <HslWheel />
            </div>
        </div>
    }
}

#[component]
fn HsvWheel() -> impl IntoView {
    let state = use_color_wheel_state(UseColorWheelStateInput {
        default_value: HSV::new(),
        channel: HsvChannel::Hue,
        disabled: false.into(),
        on_change: None,
        on_change_end: None,
    });
    let value = state.value;
    let info = Signal::derive(move || {
        let c = value.get();
        format!(
            "HSV({:.0}\u{00b0}, {:.0}%, {:.0}%)",
            c.get_channel_value(HsvChannel::Hue),
            c.get_channel_value(HsvChannel::Saturation) * 100.0,
            c.get_channel_value(HsvChannel::Brightness) * 100.0,
        )
    });
    wheel_view(state, info)
}

#[component]
fn HslWheel() -> impl IntoView {
    let state = use_color_wheel_state(UseColorWheelStateInput {
        default_value: HSL::new(),
        channel: HslChannel::Hue,
        disabled: false.into(),
        on_change: None,
        on_change_end: None,
    });
    let value = state.value;
    let info = Signal::derive(move || {
        let c = value.get();
        format!(
            "HSL({:.0}\u{00b0}, {:.0}%, {:.0}%)",
            c.get_channel_value(HslChannel::Hue),
            c.get_channel_value(HslChannel::Saturation) * 100.0,
            c.get_channel_value(HslChannel::Lightness) * 100.0,
        )
    });
    wheel_view(state, info)
}

fn wheel_view<C: ColorValue>(
    state: UseColorWheelStateReturn<C>,
    info: Signal<String>,
) -> impl IntoView {
    let hue = state.hue;
    let display_color = state.display_color;

    let wheel = use_color_wheel(UseColorWheelInput {
        state,
        outer_radius: 100.0,
        inner_radius: 70.0,
        disabled: false.into(),
        aria_label: Some("Hue wheel"),
        name: None,
        form: None,
    });

    let track_size = wheel.track_size;
    let clip_path = wheel.clip_path.clone();
    let thumb_x = wheel.thumb_x;
    let thumb_y = wheel.thumb_y;

    view! {
        <div style="position: relative;">
            <div
                {..wheel.track_props.into_attrs()}
                style=move || format!(
                    "position: relative; width: {track_size}px; height: {track_size}px; \
                     border-radius: 50%; touch-action: none; \
                     background: {}; clip-path: {clip_path};",
                    wheel.background.get(),
                )
            >
                <div
                    {..wheel.thumb_props.into_attrs()}
                    style=move || format!(
                        "position: absolute; width: 20px; height: 20px; border-radius: 50%; \
                         border: 2px solid white; box-shadow: 0 0 3px rgba(0,0,0,0.5); \
                         transform: translate(-50%, -50%); touch-action: none; cursor: grab; \
                         left: {}px; top: {}px; background: {};",
                        thumb_x.get(),
                        thumb_y.get(),
                        display_color.get().to_css_string(),
                    )
                >
                    <input
                        {..wheel.input_props.into_attrs()}
                        style="opacity: 0.0001; width: 100%; height: 100%; pointer-events: none; position: absolute; top: 0; left: 0;"
                    />
                </div>
            </div>

            <p style="margin: 0; position: absolute; top: 40%; left: 22%; text-align: center;">
                <span style=move || format!(
                    "display: inline-block; width: 1em; height: 1em; vertical-align: middle; \
                     border-radius: 2px; background: {};",
                    display_color.get().to_css_string()
                )></span>
                " "
                <code>{ move || format!("{:.0}\u{00b0}", hue.get()) }</code>
                <br />
                <small><code>{ info }</code></small>
            </p>
        </div>
    }
}
