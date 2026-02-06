use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use leptonic::atoms::label::Label as LabelAtom;
use leptonic::atoms::link::AnchorLink;
use leptonic::atoms::slider::{
    Slider as SliderAtom, SliderOutput, SliderThumb, SliderTrack, SliderTrackFill,
};
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::utils::styles::Style::*;
use leptonic::utils::styles::Styles;
use leptos::prelude::*;

#[component]
pub fn SliderContainer(children: Children) -> impl IntoView {
    view! {
        <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
            { children() }
        </div>
    }
}

#[component]
pub fn PageUseSliderHook() -> impl IntoView {
    // =========================================================================
    // Step Slider (step = 5)
    // =========================================================================
    let step_state = use_slider_state(UseSliderStateInput {
        default_values: vec![25.0],
        min_value: 0.0,
        max_value: 100.0,
        step: 5.0,
        disabled: false.into(),
        orientation: Default::default(),
        on_change: None,
        on_change_end: None,
    });

    let UseSliderReturn {
        track_props: step_track_props,
        track_ref: step_track_ref,
        label_props: step_label_props,
        ..
    } = use_slider(UseSliderInput {
        state: step_state,
        is_rtl: false,
        aria_label: None,
        aria_labelledby: None,
    });

    let UseSliderThumbReturn {
        thumb_props: step_thumb_props,
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
    });

    fn track_style(orientation: SliderOrientation) -> Styles {
        Styles::from([
            (Display, "flex"),
            (
                Width,
                match orientation {
                    SliderOrientation::Horizontal => "100%",
                    SliderOrientation::Vertical => "8px",
                },
            ),
            (
                Height,
                match orientation {
                    SliderOrientation::Horizontal => "8px",
                    SliderOrientation::Vertical => "100%",
                },
            ),
            (Background, "#ddd"),
            (BorderRadius, "4px"),
            (Position, "relative"),
            (Cursor, "pointer"),
        ])
    }

    fn thumb_style(color: &'static str) -> Styles {
        Styles::builder()
            .with((Width, "20px"))
            .with((Height, "20px"))
            .with((BackgroundColor, color))
            .with((BorderRadius, "50%"))
            .with((Border, "2px solid white"))
            .with((BoxShadow, "0 2px 4px rgba(0,0,0,0.2)"))
            .with((Cursor, "grab"))
            .build()
    }

    view! {
            <Article>
                <h1 id="use_slider" class="anchor">
                    "use_slider"
                    <AnchorLink href="#use_slider" description="Direct link to article header"/>
                </h1>

                <p>"Create accessible range sliders with keyboard navigation, drag support, and multi-thumb range selection."</p>

                <h2 id="architecture" class="anchor">
                    "Architecture"
                    <AnchorLink href="#architecture" description="Direct link to architecture"/>
                </h2>

                <p>"The slider is split into three hooks following react-aria's pattern:"</p>
                <ul>
                    <li><strong>"use_slider_state"</strong>" - Manages multi-thumb state, values, constraints"</li>
                    <li><strong>"use_slider"</strong>" - Track-level behavior (container, track clicks, labels)"</li>
                    <li><strong>"use_slider_thumb"</strong>" - Per-thumb behavior (drag, keyboard, ARIA)"</li>
                </ul>

                <Code>
                    {indoc!(r#"
                    // Create state for a single-thumb slider
                    let state = use_slider_state(UseSliderStateInput {
                        default_values: vec![50.0],
                        min_value: 0.0,
                        max_value: 100.0,
                        step: 1.0,
                        ..Default::default()
                    });

                    // Create track-level behavior
                    let track_ref = NodeRef::<html::Div>::new();
                    let UseSliderReturn { group_props, track_props, label_props, .. } =
                        use_slider(UseSliderInput {
                            state,
                            track_ref,
                            label: Some("Volume".into()),
                            ..Default::default()
                        });

                    // Create thumb behavior
                    let UseSliderThumbReturn { thumb_props, percentage, value, .. } =
                        use_slider_thumb(UseSliderThumbInput {
                            state,
                            track_ref,
                            index: 0,
                            ..Default::default()
                        });
                "#)}
                </Code>

                <h2 id="basic-slider" class="anchor">
                    "Basic Slider"
                    <AnchorLink href="#basic-slider" description="Direct link to basic slider"/>
                </h2>

                <p>"A simple single-thumb slider with full keyboard and drag support:"</p>

                <SliderContainer>
                    <SliderAtom default_values=vec![50.0] styles=[(Display, "flex"), (AlignItems, "center"), (Gap, "1em")]>
                        <LabelAtom styles=[(MinWidth, "80px"), (FontWeight, "500")]>
                            "Volume"
                        </LabelAtom>
                        <SliderTrack styles=track_style(SliderOrientation::Horizontal)>
                            <SliderTrackFill styles=[(BackgroundColor, "var(--brand-color)"), (BorderRadius, "4px")]/>
                            <SliderThumb styles=thumb_style("var(--brand-color)")/>
                        </SliderTrack>
                        <SliderOutput let:props let:values>
                            <output {..props.into_attrs()} style=Styles::from([(MinWidth, "50px"), (TextAlign, "right")])>
                                { move || {
                                    let val = values.get().get(0).map(|it| *it as i32).unwrap_or(0);
                                    format!("{val}%")
                                } }
                            </output>
                        </SliderOutput>
                    </SliderAtom>
                </SliderContainer>

                <h2 id="range-slider" class="anchor">
                    "Range Slider (Multi-Thumb)"
                    <AnchorLink href="#range-slider" description="Direct link to range slider"/>
                </h2>

                <p>"A slider with two thumbs for selecting a range. Thumbs are constrained and cannot cross each other:"</p>

                <Code>
                    {indoc!(r#"
                    // Create state with two thumbs
                    let state = use_slider_state(UseSliderStateInput {
                        default_values: vec![20.0, 80.0],  // Two thumbs
                        min_value: 0.0,
                        max_value: 100.0,
                        step: 1.0,
                        ..Default::default()
                    });

                    // Create one use_slider_thumb per thumb
                    let thumb1 = use_slider_thumb(UseSliderThumbInput {
                        state, track_ref, index: 0,
                        aria_label: Some("Minimum"),
                        ..Default::default()
                    });
                    let thumb2 = use_slider_thumb(UseSliderThumbInput {
                        state, track_ref, index: 1,
                        aria_label: Some("Maximum"),
                        ..Default::default()
                    });
                "#)}
                </Code>

                <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
                    <SliderAtom default_values=vec![20.0, 80.0] styles=[(Display, "flex"), (AlignItems, "center"), (Gap, "1em")]>
                        <LabelAtom styles=[(MinWidth, "80px"), (FontWeight, "500")]>
                            "Price Range"
                        </LabelAtom>
                        <SliderTrack styles=track_style(SliderOrientation::Horizontal)>
                            <SliderTrackFill styles=[(BackgroundColor, "#4a90d9"), (BorderRadius, "4px")]/>
                            <SliderThumb aria_label="Minimum Price" styles=thumb_style("#4a90d9")/>
                            <SliderThumb aria_label="Maximum Price" styles=thumb_style("#4a90d9")/>
                        </SliderTrack>
                        <SliderOutput let:props let:values>
                            <output {..props.into_attrs()} style=Styles::from([(MinWidth, "80px"), (TextAlign, "right")])>
                                { move || {
                                    let val1 = values.get().get(0).map(|it| *it as i32).unwrap_or(0);
                                    let val2 = values.get().get(1).map(|it| *it as i32).unwrap_or(0);
                                    format!("{val1}% - {val2}%")
                                } }
                            </output>
                        </SliderOutput>
                    </SliderAtom>
                </div>

                <h2 id="step-slider" class="anchor">
                    "Slider with Steps"
                    <AnchorLink href="#step-slider" description="Direct link to step slider"/>
                </h2>

                <p>"Slider with step=5, snapping to multiples of 5. Use Shift+Arrow for larger increments (page size):"</p>

                <div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
                    <div style="display: flex; align-items: center; gap: 1em;">
                        <label
                            id=step_label_props.id
                            style="min-width: 80px; font-weight: 500;"
                        >
                            "Brightness"
                        </label>

                        <div
                            {..step_track_props.into_attrs()}
                            style="flex: 1; height: 8px; background: #ddd; border-radius: 4px; position: relative; cursor: pointer;"
                        >
                            <div style=move || format!(
                                "position: absolute; left: 0; top: 0; height: 100%; background: orange; border-radius: 4px; width: {}%;",
                                step_percent.get()
                            )></div>

                            <div
                                {..step_thumb_props}
                                style=move || format!(
                                    "position: absolute; top: 50%; transform: translate(-50%, -50%); width: 20px; height: 20px; background: orange; border-radius: 50%; border: 2px solid white; box-shadow: 0 2px 4px rgba(0,0,0,0.2); cursor: grab; left: {}%;",
                                    step_percent.get()
                                )
                            ></div>
                        </div>

                        <output style="min-width: 50px; text-align: right;">
                            { move || format!("{}%", step_value.get() as i32) }
                        </output>
                    </div>

                    // Step markers
                    <div style="display: flex; justify-content: space-between; padding: 0 10px; margin-top: 0.5em; margin-left: 100px; margin-right: 60px;">
                        {(0..=20).map(|i| view! {
                            <span style="font-size: 0.7em; color: #999;">{ i * 5 }</span>
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                <h2 id="vertical-slider" class="anchor">
                    "Vertical Slider"
                    <AnchorLink href="#vertical-slider" description="Direct link to vertical slider"/>
                </h2>

                <p>"Set " <code>"orientation: Orientation::Vertical"</code> " for a vertical slider. Note: keyboard Up/Down work regardless of orientation."</p>

                <SliderContainer>
                    <SliderAtom default_values=vec![60.0] orientation=SliderOrientation::Vertical styles=[(Display, "flex"), (AlignItems, "center"), (Gap, "1em"), (Height, "150px")]>
                        <LabelAtom styles=[(MinWidth, "80px"), (FontWeight, "500")]>
                            "Vertical"
                        </LabelAtom>
                        <SliderTrack styles=track_style(SliderOrientation::Vertical)>
                            <SliderTrackFill styles=[(BackgroundColor, "#9b59b6"), (BorderRadius, "4px")]/>
                            <SliderThumb styles=thumb_style("#9b59b6")/>
                        </SliderTrack>
                        <SliderOutput let:props let:values>
                            <output {..props.into_attrs()} style=Styles::from([(MinWidth, "50px"), (TextAlign, "right")])>
                                { move || {
                                    let val = values.get().get(0).map(|it| *it as i32).unwrap_or(0);
                                    format!("{val}%")
                                } }
                            </output>
                        </SliderOutput>
                    </SliderAtom>
                </SliderContainer>

                <p>"Vertical range slider"</p>

                <SliderContainer>
                    <SliderAtom default_values=vec![20.0, 60.0] orientation=SliderOrientation::Vertical styles=[(Display, "flex"), (AlignItems, "center"), (Gap, "1em"), (Height, "150px")]>
                        <LabelAtom styles=[(MinWidth, "80px"), (FontWeight, "500")]>
                            "Vertical"
                        </LabelAtom>
                        <SliderTrack styles=track_style(SliderOrientation::Vertical)>
                            <SliderTrackFill styles=[(BackgroundColor, "#9b59b6"), (BorderRadius, "4px")]/>
                            <SliderThumb styles=thumb_style("#9b59b6")/>
                            <SliderThumb styles=thumb_style("#9b59b6")/>
                        </SliderTrack>
                        <SliderOutput let:props let:values>
                            <output {..props.into_attrs()} style=Styles::from([(MinWidth, "50px"), (TextAlign, "right")])>
                                { move || {
                                    let val1 = values.get().get(0).map(|it| *it as i32).unwrap_or(0);
                                    let val2 = values.get().get(1).map(|it| *it as i32).unwrap_or(0);
                                    format!("{val1}% - {val2}% ")
                                } }
                            </output>
                        </SliderOutput>
                    </SliderAtom>
                </SliderContainer>

                //<div style="padding: 1.5em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
                //    <div {..vertical_group_props.into_attrs()} style="display: flex; align-items: center; gap: 1em; height: 150px;">
                //        <label id=vertical_label_props.id style="min-width: 80px; font-weight: 500;">
                //            "Vertical"
                //        </label>
    //
                //        <div
                //            {..vertical_track_props.into_attrs()}
                //            style="width: 8px; height: 100%; background: #ddd; border-radius: 4px; position: relative; cursor: pointer;"
                //        >
                //            // Filled portion (from bottom up)
                //            <div style=move || format!(
                //                "position: absolute; left: 0; bottom: 0; width: 100%; background: #9b59b6; border-radius: 4px; height: {}%;",
                //                vertical_percent.get()
                //            )></div>
    //
                //            // Thumb (bottom = 0%, top = 100%)
                //            <div
                //                {..vertical_thumb_props}
                //                style=move || format!(
                //                    "position: absolute; left: 50%; transform: translate(-50%, 50%); width: 20px; height: 20px; background: #9b59b6; border-radius: 50%; border: 2px solid white; box-shadow: 0 2px 4px rgba(0,0,0,0.2); cursor: grab; bottom: {}%;",
                //                    vertical_percent.get()
                //                )
                //            ></div>
                //        </div>
    //
                //        <output style="min-width: 50px; text-align: right;">
                //            { move || format!("{}%", vertical_value.get() as i32) }
                //        </output>
                //    </div>
                //</div>

                <h2 id="disabled-slider" class="anchor">
                    "Disabled Slider"
                    <AnchorLink href="#disabled-slider" description="Direct link to disabled slider"/>
                </h2>

                <p>"Set " <code>"is_disabled: true.into()"</code> " on both " <code>"use_slider"</code> " and " <code>"use_slider_thumb"</code> " to disable the slider:"</p>

                <SliderContainer>
                    <SliderAtom default_values=vec![30.0] disabled=true styles=[(Display, "flex"), (AlignItems, "center"), (Gap, "1em")]>
                        <LabelAtom styles=[(MinWidth, "80px"), (FontWeight, "500")]>
                            "Disabled"
                        </LabelAtom>
                        <SliderTrack styles=track_style(SliderOrientation::Horizontal)>
                            <SliderTrackFill styles=[(BackgroundColor, "#999"), (BorderRadius, "4px")]/>
                            <SliderThumb styles=thumb_style("#999")/>
                        </SliderTrack>
                        <SliderOutput let:props let:values>
                            <output {..props.into_attrs()} style=Styles::from([(MinWidth, "50px"), (TextAlign, "right")])>
                                { move || {
                                    let val = values.get().get(0).map(|it| *it as i32).unwrap_or(0);
                                    format!("{val}%")
                                } }
                            </output>
                        </SliderOutput>
                    </SliderAtom>
                </SliderContainer>

                <h2 id="keyboard" class="anchor">
                    "Keyboard Navigation"
                    <AnchorLink href="#keyboard" description="Direct link to keyboard"/>
                </h2>

                <ul>
                    <li><strong>"Arrow Left/Down"</strong> " - Decrease by step"</li>
                    <li><strong>"Arrow Right/Up"</strong> " - Increase by step"</li>
                    <li><strong>"Shift + Arrow Left/Down"</strong> " - Decrease by page size (10% of range)"</li>
                    <li><strong>"Shift + Arrow Right/Up"</strong> " - Increase by page size (10% of range)"</li>
                    <li><strong>"Page Down"</strong> " - Decrease by page size"</li>
                    <li><strong>"Page Up"</strong> " - Increase by page size"</li>
                    <li><strong>"Home"</strong> " - Jump to minimum"</li>
                    <li><strong>"End"</strong> " - Jump to maximum"</li>
                </ul>

                <h2 id="features" class="anchor">
                    "Features"
                    <AnchorLink href="#features" description="Direct link to features"/>
                </h2>

                <ul>
                    <li><strong>"Multi-thumb support"</strong>" - Create range sliders with any number of thumbs"</li>
                    <li><strong>"Thumb constraints"</strong>" - Thumbs cannot cross each other"</li>
                    <li><strong>"Full drag support"</strong>" - Global pointer tracking for reliable dragging"</li>
                    <li><strong>"RTL support"</strong>" - Arrow key directions reverse in RTL mode"</li>
                    <li><strong>"Page size"</strong>" - Shift+Arrow for larger increments, snapped to step"</li>
                    <li><strong>"Closest thumb algorithm"</strong>" - Track clicks move the nearest thumb"</li>
                    <li><strong>"ARIA slider role"</strong>" - Full accessibility support"</li>
                    <li><strong>"on_change_end"</strong>" - Callback fires only when all thumbs stop dragging"</li>
                </ul>

                <h2 id="api" class="anchor">
                    "API Reference"
                    <AnchorLink href="#api" description="Direct link to API"/>
                </h2>

                <h3>"UseSliderStateInput"</h3>
                <ul>
                    <li><code>"default_values: Vec<f64>"</code>" - Initial values for each thumb"</li>
                    <li><code>"min_value: f64"</code>" - Minimum slider value"</li>
                    <li><code>"max_value: f64"</code>" - Maximum slider value"</li>
                    <li><code>"step: f64"</code>" - Step increment"</li>
                    <li><code>"on_change: Option<Callback<Vec<f64>>>"</code>" - Called during interaction"</li>
                    <li><code>"on_change_end: Option<Callback<Vec<f64>>>"</code>" - Called when dragging ends"</li>
                </ul>

                <h3>"UseSliderInput"</h3>
                <ul>
                    <li><code>"state: UseSliderStateReturn"</code>" - State from use_slider_state"</li>
                    <li><code>"label: Option<String>"</code>" - Label text"</li>
                    <li><code>"orientation: Orientation"</code>" - Horizontal or Vertical"</li>
                    <li><code>"is_disabled: Signal<bool>"</code>" - Disabled state"</li>
                    <li><code>"is_rtl: bool"</code>" - Right-to-left layout"</li>
                </ul>

                <h3>"UseSliderReturn"</h3>
                <ul>
                    <li><code>"track_props"</code>" - Props to spread on the track element"</li>
                    <li><code>"track_ref: SliderTrackRef"</code>" - Captured track element (pass to use_slider_thumb)"</li>
                    <li><code>"group_props"</code>" - Props for the group container"</li>
                    <li><code>"label_props"</code>" - Props for the label element"</li>
                    <li><code>"output_props"</code>" - Props for the output element"</li>
                </ul>

                <h3>"UseSliderThumbInput"</h3>
                <ul>
                    <li><code>"state: UseSliderStateReturn"</code>" - State from use_slider_state"</li>
                    <li><code>"track_ref: SliderTrackRef"</code>" - Track ref from use_slider return"</li>
                    <li><code>"index: usize"</code>" - Thumb index (0-based)"</li>
                    <li><code>"aria_label: Option<&'static str>"</code>" - Accessible label"</li>
                    <li><code>"is_disabled: Signal<bool>"</code>" - Disabled state"</li>
                    <li><code>"is_rtl: bool"</code>" - Right-to-left layout"</li>
                </ul>
            </Article>

            <Toc toc=Toc::List {
                inner: vec![
                    Toc::Leaf { title: "use_slider", link: "#use_slider" },
                    Toc::Leaf { title: "Architecture", link: "#architecture" },
                    Toc::Leaf { title: "Basic Slider", link: "#basic-slider" },
                    Toc::Leaf { title: "Range Slider", link: "#range-slider" },
                    Toc::Leaf { title: "Slider with Steps", link: "#step-slider" },
                    Toc::Leaf { title: "Vertical Slider", link: "#vertical-slider" },
                    Toc::Leaf { title: "Disabled Slider", link: "#disabled-slider" },
                    Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard" },
                    Toc::Leaf { title: "Features", link: "#features" },
                    Toc::Leaf { title: "API Reference", link: "#api" },
                ]
            }/>
        }
}
