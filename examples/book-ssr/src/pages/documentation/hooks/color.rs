use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{pages::documentation::article::Article, routes};

#[component]
pub fn PageUseColorHooks() -> impl IntoView {
    view! {
        <Article>
            <h1 id="color-hooks" class="anchor">
                "Color Hooks"
                <AnchorLink href="#color-hooks" description="Direct link to article header"/>
            </h1>

            <p>
                "Leptonic provides 6 hook families for building custom color picking UIs. "
                "Each family consists of a state hook and a behavior/ARIA hook, following the "
                "same pattern as other leptonic hooks."
            </p>

            <p>
                "All color hooks are based on "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useColorArea.html" target=leptonic::hooks::LinkTarget::_Blank>
                    "react-aria\u{2019}s color hooks"
                </LinkExt>
                " from the @react-aria/color and @react-stately/color packages."
            </p>

            <h2 id="hook-families" class="anchor">
                "Hook Families"
                <AnchorLink href="#hook-families" description="Direct link to section"/>
            </h2>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"Hook"</TableHeaderCell>
                            <TableHeaderCell>"Purpose"</TableHeaderCell>
                            <TableHeaderCell>"Deep Dive"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"use_color_area"</Code></TableCell>
                            <TableCell>"2D gradient area for selecting two channels (e.g. saturation + brightness). Composes use_move with MoveConstraint."</TableCell>
                            <TableCell><Link href=routes::doc::hooks::UseColorArea.materialize()>"Details"</Link></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_color_slider"</Code></TableCell>
                            <TableCell>"Linear slider for a single channel. Wraps use_slider with color-specific gradient and ARIA."</TableCell>
                            <TableCell><Link href=routes::doc::hooks::UseColorSlider.materialize()>"Details"</Link></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_color_wheel"</Code></TableCell>
                            <TableCell>"Circular hue wheel (0\u{00b0}\u{2013}360\u{00b0}). Handles pointer-in-annulus validation, conic gradient, and polar coordinates."</TableCell>
                            <TableCell><Link href=routes::doc::hooks::UseColorWheel.materialize()>"Details"</Link></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_color_field"</Code></TableCell>
                            <TableCell>"Text input for hex color values (#RRGGBB). Validates during typing, commits on blur."</TableCell>
                            <TableCell><Link href=routes::doc::hooks::UseColorField.materialize()>"Details"</Link></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_color_channel_field"</Code></TableCell>
                            <TableCell>"Numeric input for a single channel value. Bridges ColorValue and use_number_field."</TableCell>
                            <TableCell><Link href=routes::doc::hooks::UseColorChannelField.materialize()>"Details"</Link></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"use_color_swatch"</Code></TableCell>
                            <TableCell>"Display-only color preview with role=\"img\" and aria-roledescription."</TableCell>
                            <TableCell><Link href=routes::doc::hooks::UseColorSwatch.materialize()>"Details"</Link></TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="color-value-trait" class="anchor">
                "The ColorValue Trait"
                <AnchorLink href="#color-value-trait" description="Direct link to section"/>
            </h2>

            <p>
                "All color hooks are generic over the "<Code inline=true>"ColorValue"</Code>" trait. "
                "Each color type brings its own "<Code inline=true>"Channel"</Code>" associated type, "
                "so you can\u{2019}t accidentally ask an RGB color for its Hue channel."
            </p>

            <Code language=Language::Rust>
                {indoc!(r"
                    pub trait ColorValue: Clone + Copy + PartialEq + Debug + Send + Sync {
                        type Channel: Debug + Clone + Copy + PartialEq + Eq + Hash;

                        fn get_channel_value(&self, channel: Self::Channel) -> f64;
                        fn with_channel_value(&self, channel: Self::Channel, value: f64) -> Self;
                        fn get_channel_range(channel: Self::Channel) -> ColorChannelRange;
                        fn channels() -> &'static [Self::Channel];
                        fn to_css_string(&self) -> String;
                        // ...
                    }
                ")}
            </Code>

            <p>
                "Two implementations are provided: "<Code inline=true>"HSV"</Code>
                " (with "<Code inline=true>"HsvChannel"</Code>": Hue, Saturation, Brightness) and "
                <Code inline=true>"RGB8"</Code>
                " (with "<Code inline=true>"RgbChannel"</Code>": Red, Green, Blue)."
            </p>

            <h2 id="pattern" class="anchor">
                "Usage Pattern"
                <AnchorLink href="#pattern" description="Direct link to section"/>
            </h2>

            <p>
                "Each hook family follows the state + behavior pattern:"
            </p>

            <Code language=Language::Rust>
                {indoc!(r"
                    // 1. Create state
                    let state = use_color_area_state(UseColorAreaStateInput {
                        default_value: HSV::new(),
                        x_channel: HsvChannel::Saturation,
                        y_channel: HsvChannel::Brightness,
                        ..
                    });

                    // 2. Create behavior + ARIA props
                    let area = use_color_area(UseColorAreaInput {
                        state: state.clone(),
                        disabled: Signal::derive(|| false),
                        ..
                    });

                    // 3. Spread attrs on elements
                    view! {
                        <div {..area.area_props.into_attrs()} style:background=area.background>
                            <div {..area.thumb_props.into_attrs()} />
                        </div>
                    }
                ")}
            </Code>
        </Article>
    }
}
