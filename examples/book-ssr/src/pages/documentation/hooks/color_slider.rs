use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_slider::ColorSliderDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseColorSlider() -> impl IntoView {
    view! {
        <DocPage title="Color Slider Hooks">
            <p>
                "The "<Code inline=true>"use_color_slider_state"</Code>" and "<Code inline=true>"use_color_slider"</Code>
                " hooks build a slider that changes one channel of a color along a gradient track. See the "
                <Link href=routes::doc::ColorSlider.materialize()>"Color Slider overview"</Link>" for the concept and its "
                "keyboard interaction."
            </p>

            <ReactAria hook="useColorSlider"/>

            <Section title="Example">
                <p>
                    "Create the state, pass it to "<Code inline=true>"use_color_slider"</Code>", and spread the props of the "
                    <Link href=routes::doc::slider::Hook.materialize()>"Slider Hooks"</Link>" it returns onto a track, a "
                    "thumb and an input inside the thumb. Merge "<Code inline=true>"track_styles"</Code>" (the gradient) "
                    "into the track\u{2019}s styles and give the input "<Code inline=true>"input_styles"</Code>
                    " (visually hidden):"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::{
                            hooks::*,
                            utils::color::{HSV, HsvChannel},
                        };
                        use leptos::prelude::*;

                        let state = use_color_slider_state(UseColorSliderStateInput::new(HSV::new(), HsvChannel::Hue));
                        let UseColorSliderReturn { slider, thumb, track_styles, input_styles } =
                            use_color_slider(UseColorSliderInput::new(state));
                        let (track_attrs, slider_track_styles) = slider.track_props.into_parts();
                        let (thumb_attrs, thumb_styles) = thumb.thumb_props.into_parts();

                        view! {
                            <div {..slider.group_props.into_attrs()}>
                                <div {..track_attrs} style=slider_track_styles.merge(track_styles)>
                                    <div {..thumb_attrs} style=thumb_styles>
                                        <input {..thumb.input_props.into_attrs()} style=input_styles/>
                                    </div>
                                </div>
                            </div>
                        }
                    ")}
                </Code>
                <p>
                    "Give the track a size and the thumb a size, a border and a fill, e.g. "
                    <Code inline=true>"state.display_color()"</Code>"; the demo source shows one way. Without a label, the "
                    "slider is named after its channel (\u{201c}Hue\u{201d})."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Drag the thumb, click the track, or tab to the thumb and use the arrow keys. The hooks report no "
                    "focus state; the demo shows the focus ring with "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>
                    " on the thumb, which contains the focused input."
                </p>
                <Demo description="Hue slider with a label and the formatted hue, and a disabled toggle" source=include_str!("demos/color_slider.rs")>
                    <ColorSliderDemo/>
                </Demo>
            </Section>

            <Section title="use_color_slider_state">
                <p>
                    "Holds the color and a single-thumb "
                    <Link href=format!("{}#use-slider-state", routes::doc::slider::Hook.materialize())>"slider state"</Link>
                    " of the channel, with the channel\u{2019}s range, step and page step. "<Code inline=true>"C"</Code>" is any "
                    <Link href=format!("{}#colorvalue", routes::doc::Color.materialize())>
                        <Code inline=true>"ColorValue"</Code>
                    </Link>". The state owns the color, starting at "<Code inline=true>"default_value"</Code>", unless you bind "
                    "it to app state with "<Code inline=true>"value"</Code>"."
                </p>

                <Section title="Input" id="use-color-slider-state-input">
                    <p>
                        "Start from "<Code inline=true>"UseColorSliderStateInput::new(default_value, channel)"</Code>
                        " and change single fields:"
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseColorSliderStateInput">
                        <ApiRow name="default_value" ty="C">"The initial color. Required (an argument of "<Code inline=true>"new"</Code>")."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<C>>" default="None">
                            "The color as app state, replacing "<Code inline=true>"default_value"</Code>": "
                            <Code inline=true>"ValueBinding::from(rw_signal)"</Code>" or "
                            <Code inline=true>"ValueBinding::new(signal, callback)"</Code>"."
                        </ApiRow>
                        <ApiRow name="channel" ty="C::Channel">"The channel the slider changes. Required (an argument of "<Code inline=true>"new"</Code>")."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the slider is disabled."</ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>" default="Orientation::Horizontal">
                            "The direction of the track: how pointer positions and the gradient map to values."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<C>>" default="None">"Called with the color whenever it changes, also while dragging."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<C>>" default="None">"Called with the color when a drag or a key press ends."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-slider-state-return">
                    <p>"A "<Code inline=true>"Copy"</Code>" "<Code inline=true>"ColorSliderState<C>"</Code>":"</p>
                    <ApiTable kind=ApiKind::Return of="ColorSliderState">
                        <ApiRow name="value" ty="Signal<C>">"The color."</ApiRow>
                        <ApiRow name="channel" ty="C::Channel">"The channel the slider changes."</ApiRow>
                        <ApiRow name="slider" ty="SliderState<f64>">
                            "The slider of the channel\u{2019}s value (one thumb); disabled state and orientation live here."
                        </ApiRow>
                        <ApiRow name="is_dragging" ty="Signal<bool>">"Whether the thumb is being dragged."</ApiRow>
                    </ApiTable>
                    <DocTable headers=&["Method", "Description"]>
                        <TableRow>
                            <TableCell><Code inline=true>"set_value(color)"</Code></TableCell>
                            <TableCell>"Sets the color and calls "<Code inline=true>"on_change"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"display_color()"</Code></TableCell>
                            <TableCell>
                                "The color to draw the thumb with: for a hue, the hue at full saturation and brightness; "
                                "otherwise the color itself."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"formatted_value()"</Code></TableCell>
                            <TableCell>"The channel\u{2019}s value, formatted (e.g. \u{201c}210\u{00b0}\u{201d}, \u{201c}60%\u{201d})."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"default_value()"</Code></TableCell>
                            <TableCell>"The color the slider started with; a form reset restores it."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_color_slider">
                <p>
                    "Adds the interaction and the ARIA semantics of "
                    <Link href=format!("{}#use-slider", routes::doc::slider::Hook.materialize())>"use_slider"</Link>" and "
                    <Link href=format!("{}#use-slider-thumb", routes::doc::slider::Hook.materialize())>"use_slider_thumb"</Link>
                    ", the gradient of the track, and a value text with a name: the hue\u{2019}s for a hue channel (e.g. "
                    "\u{201c}120\u{00b0}, green\u{201d}), the color\u{2019}s for the other channels."
                </p>

                <Section title="Input" id="use-color-slider-input">
                    <p>"Start from "<Code inline=true>"UseColorSliderInput::new(state)"</Code>" and change single fields:"</p>
                    <ApiTable kind=ApiKind::Input of="UseColorSliderInput">
                        <ApiRow name="state" ty="ColorSliderState<C>">
                            "The state from "<Code inline=true>"use_color_slider_state"</Code>". Required (the argument of "
                            <Code inline=true>"new"</Code>")."
                        </ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">
                            "Whether you render a visible label with "<Code inline=true>"slider.label_props"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the slider. Without it, a label or "<Code inline=true>"aria_labelledby"</Code>
                            ", the channel\u{2019}s name does."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The ids of the elements naming the slider."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"The ids of the elements describing the slider."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The form field name of the input."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the input belongs to."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-slider-return">
                    <ApiTable kind=ApiKind::Return of="UseColorSliderReturn">
                        <ApiRow name="slider" ty="UseSliderReturn">
                            "The props of the group, label, output and track, as "<Code inline=true>"use_slider"</Code>" returns them."
                        </ApiRow>
                        <ApiRow name="thumb" ty="UseSliderThumbReturn">
                            "The props of the thumb and its input (with the color\u{2019}s value text), as "
                            <Code inline=true>"use_slider_thumb"</Code>" returns them."
                        </ApiRow>
                        <ApiRow name="track_styles" ty="Styles">
                            "The track\u{2019}s gradient and "<Code inline=true>"forced-color-adjust: none"</Code>
                            ": merge them into the track props\u{2019} styles."
                        </ApiRow>
                        <ApiRow name="input_styles" ty="Styles">"Hide the input visually across the thumb: put them on the input."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link></li>
                <li><Link href=routes::doc::color_slider::Atom.materialize()>"Color Slider Atoms"</Link></li>
                <li><Link href=routes::doc::slider::Hook.materialize()>"Slider Hooks"</Link></li>
                <li><Link href=routes::doc::color_area::Hook.materialize()>"Color Area Hooks"</Link></li>
                <li><Link href=routes::doc::color_wheel::Hook.materialize()>"Color Wheel Hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
