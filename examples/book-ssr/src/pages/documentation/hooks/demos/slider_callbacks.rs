use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn SliderCallbacksDemo() -> impl IntoView {
    let step_state = use_slider_state(UseSliderStateInput {
        values: SliderValues::Uncontrolled(vec![25.0]),
        min_value: 0.0,
        max_value: 100.0,
        step: Some(5.0),
        disabled: false.into(),
        orientation: Signal::default(),
        on_change: None,
        on_change_end: None,
    });

    let UseSliderReturn {
        track_props: step_track_props_with_styles,
        track_ref: step_track_ref,
        label_props: step_label_props,
        ..
    } = use_slider(UseSliderInput {
        state: step_state,
        is_rtl: false,
        aria_label: None,
        aria_labelledby: None,
    });
    let (step_track_props, step_track_styles) = step_track_props_with_styles.into_parts();

    let UseSliderThumbReturn {
        thumb_props: step_thumb_props,
        input_props: step_input_props,
        percentage: step_percent,
        value: step_value,
        ..
    } = use_slider_thumb(UseSliderThumbInput {
        state: step_state,
        track: step_track_ref,
        index: 0,
        name: None,
        aria_label: None,
        aria_labelledby: None,
        disabled: false.into(),
        validation_state: ValidationState::Valid,
        is_rtl: false,
        decimal_places: None,
        is_required: false,
        aria_describedby: None,
        aria_details: None,
        aria_errormessage: None,
        aria_valuetext: None,
    });

    view! {
        <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
            <div style="display: flex; align-items: center; gap: 1em;">
                <label
                    id=step_label_props.id
                    style="min-width: 80px; font-weight: 500;"
                >
                    "Brightness"
                </label>

                <div
                    {..step_track_props}
                    style=step_track_styles
                        .add("flex", "1")
                        .add("height", "8px")
                        .add("background", "#ddd")
                        .add("border-radius", "4px")
                        .add("position", "relative")
                        .add("cursor", "pointer")
                >
                    <div style=move || format!(
                        "position: absolute; left: 0; top: 0; height: 100%; background: orange; border-radius: 4px; width: {}%;",
                        step_percent.get()
                    )></div>

                    <div
                        {..step_thumb_props.into_attrs()}
                        style=move || format!(
                            "position: absolute; top: 50%; transform: translate(-50%, -50%); width: 20px; height: 20px; background: orange; border-radius: 50%; border: 2px solid white; box-shadow: 0 2px 4px rgba(0,0,0,0.2); cursor: grab; left: {}%;",
                            step_percent.get()
                        )
                    >
                        <input
                            {..step_input_props.into_attrs()}
                            style="opacity: 0.0001; width: 100%; height: 100%; pointer-events: none; position: absolute; top: 0; left: 0;"
                        />
                    </div>
                </div>

                <output style="min-width: 50px; text-align: right;">
                    { move || format!("{:.0}%", step_value.get()) }
                </output>
            </div>

            // Step markers
            <div style="display: flex; justify-content: space-between; padding: 0 10px; margin-top: 0.5em; margin-left: 100px; margin-right: 60px;">
                {(0..=20).map(|i| view! {
                    <span style="font-size: 0.7em; color: #999;">{ i * 5 }</span>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
