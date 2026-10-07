use indoc::indoc;
use leptos::prelude::*;

use super::demos::color_swatch_picker::ColorSwatchPickerDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomColorSwatchPicker() -> impl IntoView {
    view! {
        <DocPage title="Color Swatch Picker Atoms">
            <p>
                "A color swatch picker shows a few predefined colors as "
                <Link href=routes::doc::ColorSwatch.materialize()>"swatches"</Link>" and lets users pick one of them, "
                "like the colors of a theme or the labels of a calendar. Each swatch is an option of a "
                <Link href=routes::doc::Listbox.materialize()>"listbox"</Link>" named after its color, so the colors are "
                "reachable with the keyboard and announced by name. For any color, use a "
                <Link href=routes::doc::ColorPicker.materialize()>"color picker"</Link>" instead."
            </p>
            <ReactAria hook="ColorSwatchPicker"/>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"ColorSwatchPicker"</Code>" is a single-selection "
                    <Link href=routes::doc::listbox::Atom.materialize()>"ListBox"</Link>" atom (built on "
                    <Link href=routes::doc::listbox::Hook.materialize()>"use_listbox"</Link>") over a collection of its colors, "
                    "keyed by their hex codes; each "<Code inline=true>"ColorSwatchPickerItem"</Code>" is a "
                    <Code inline=true>"ListBoxItem"</Code>". The swatches inside are "
                    <Link href=routes::doc::color_swatch::Atom.materialize()>"ColorSwatch"</Link>" atoms."
                </p>
            </Section>

            <Section title="Example">
                <p>
                    "Give the picker its colors and the picked color; render a "<Code inline=true>"ColorSwatch"</Code>
                    " for each color with "<Code inline=true>"ColorSwatchPickerItems"</Code>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::prelude::*, utils::color::RGB8};
                        use leptos::prelude::*;

                        let colors = vec![
                            RGB8 { r: 170, g: 0, b: 0 },
                            RGB8 { r: 0, g: 136, b: 0 },
                            RGB8 { r: 0, g: 0, b: 136 },
                        ];
                        let color = RwSignal::new(colors[0]);

                        view! {
                            <ColorSwatchPicker colors=colors value=color set_value=color aria_label="Label color">
                                <ColorSwatchPickerItems>
                                    <ColorSwatch classes="my-swatch"/>
                                </ColorSwatchPickerItems>
                            </ColorSwatchPicker>
                        }
                    "#)}
                </Code>
                <p>
                    "The colors must differ as hex codes: the same color in two color spaces is the same swatch. A swatch "
                    "inside an item shows the item\u{2019}s color, so it needs no "<Code inline=true>"color"</Code>"."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Click a swatch, or tab into the picker, move between the swatches with the arrow keys and pick one with "
                    <Keys keys="Enter"/>" or "<Keys keys="Space"/>"."
                </p>
                <Demo
                    description="Six color swatches in a grid, one of them picked"
                    source=include_str!("demos/color_swatch_picker.rs")
                >
                    <ColorSwatchPickerDemo/>
                </Demo>
            </Section>

            <Section title="ColorSwatchPicker">
                <p>
                    "Renders the listbox "<Code inline=true>"<div>"</Code>" and provides its colors to the items. It is "
                    "generic over the color type "<Code inline=true>"C"</Code>" ("<Code inline=true>"HSV"</Code>", "
                    <Code inline=true>"HSL"</Code>" or "<Code inline=true>"RGB8"</Code>", see "
                    <Link href=format!("{}#colorvalue", routes::doc::Color.materialize())><Code inline=true>"ColorValue"</Code></Link>
                    "). A picked color stays picked: users can change it but not clear it. Inside a "
                    <Link href=routes::doc::color_picker::Atom.materialize()>"ColorPicker"</Link>" atom without its own "
                    <Code inline=true>"value"</Code>", it picks the picker\u{2019}s color."
                </p>
                <Section title="Props" id="color-swatch-picker-props">
                    <ApiTable kind=ApiKind::Props of="ColorSwatchPicker">
                        <ApiRow name="colors" ty="Signal<Vec<C>>">"Required. The colors to pick from, distinct as hex codes."</ApiRow>
                        <ApiRow name="default_value" ty="Option<C>" default="None">
                            "The initially picked color, unless "<Code inline=true>"value"</Code>" is set; "
                            <Code inline=true>"None"</Code>": none."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<Signal<C>>" default="None">
                            "The picked color (controlled): a value or any signal. "<Code inline=true>"None"</Code>": the color "
                            "of the "<Code inline=true>"ColorPicker"</Code>" around it, if any."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<C>>" default="None">
                            "Receives the picked color: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<C>>" default="None">"Called with the picked color."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="\u{201c}Color swatches\u{201d}">
                            "Names the picker. The default applies without "<Code inline=true>"aria_labelledby"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The ids of the elements naming the picker."</ApiRow>
                        <ApiRow name="layout" ty="Option<ListLayout>" default="ListLayout::Grid">
                            "How the swatches are laid out, for the arrow keys: "<Code inline=true>"Grid"</Code>" (rows that wrap: "
                            "the up and down keys keep the column) or "<Code inline=true>"Stack"</Code>" (one row)."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the listbox."</ApiRow>
                        <ApiRow name="children" ty="Children">"Required. The items."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ColorSwatchPickerItem">
                <p>
                    "An option for one of the picker\u{2019}s colors, named after the color (e.g. \u{201c}vibrant red\u{201d}). "
                    "Put a "<Code inline=true>"ColorSwatch"</Code>" in it, which shows the item\u{2019}s color. Use it to render "
                    "the items yourself, e.g. with a caption next to each swatch or to disable some of them."
                </p>
                <Section title="Props" id="color-swatch-picker-item-props">
                    <ApiTable kind=ApiKind::Props of="ColorSwatchPickerItem">
                        <ApiRow name="color" ty="Color">
                            "Required. The color, one of the picker\u{2019}s "<Code inline=true>"colors"</Code>"; any color "
                            "value converts into it."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                            "Whether the color can\u{2019}t be picked. Focus skips it."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the option."</ApiRow>
                        <ApiRow name="children" ty="Children">"Required. The swatch, and anything else in the option."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ColorSwatchPickerItems">
                <p>
                    "A "<Code inline=true>"ColorSwatchPickerItem"</Code>" for each of the picker\u{2019}s colors, rendering "
                    <Code inline=true>"children"</Code>" in each, or a plain "<Code inline=true>"<ColorSwatch/>"</Code>
                    " without them. It follows changes of "<Code inline=true>"colors"</Code>"."
                </p>
                <Section title="Props" id="color-swatch-picker-items-props">
                    <ApiTable kind=ApiKind::Props of="ColorSwatchPickerItems">
                        <ApiRow name="classes" ty="Classes" default="empty">"Classes of every item."</ApiRow>
                        <ApiRow name="children" ty="Option<ChildrenFn>" default="a ColorSwatch">
                            "The content of every item; a "<Code inline=true>"ColorSwatch"</Code>" in it shows the item\u{2019}s color."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>"On the items, set to "<Code inline=true>"true"</Code>" while the state applies:"</p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-selected" ty="true">"The item\u{2019}s color is picked."</ApiRow>
                    <ApiRow name="data-focused" ty="true">"The item has focus."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">"The item has keyboard focus: show a focus ring."</ApiRow>
                    <ApiRow name="data-pressed" ty="true">"The item is being pressed."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">"The item is disabled."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. "<Code inline=true>"ColorSwatchPicker"</Code>" renders a listbox "<Code inline=true>"<div>"</Code>" with the class "
                    <Code inline=true>"leptonic-ColorSwatchPicker"</Code>", each item an option "<Code inline=true>"<div>"</Code>" with "<Code inline=true>"leptonic-ColorSwatchPickerItem"</Code>", "
                    "followed by the "<Code inline=true>"classes"</Code>" you pass; the "<Code inline=true>"ColorSwatch"</Code>" inside an item is yours to size. Lay the items "
                    "out as a wrapping row, and mark the picked one and the focused one. The demo above uses this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .demo-color-swatch-picker { display: flex; flex-wrap: wrap; gap: 0.25rem; max-width: 16em; }
                        .demo-color-swatch-picker-item { display: flex; padding: 3px; border-radius: 50%; cursor: pointer; }
                        .demo-color-swatch-picker-item[data-selected] { box-shadow: 0 0 0 2px var(--accent); }
                        .demo-color-swatch-picker-item[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                        .demo-color-swatch-picker-swatch { width: 2em; height: 2em; border-radius: 50%; box-shadow: inset 0 0 0 1px var(--border); }
                    ")}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "The picker follows the WAI-ARIA "
                        <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/listbox/" target=LinkTarget::Blank>"Listbox pattern"</Link>
                        " with single selection: a "<Code inline=true>"listbox"</Code>" named \u{201c}Color swatches\u{201d} unless "
                        "you name it, with an "<Code inline=true>"option"</Code>" per color whose "<Code inline=true>"aria-selected"</Code>
                        " tells the picked one."
                    </li>
                    <li>
                        "Each option is named after its color by the swatch inside it (\u{201c}dark vibrant blue\u{201d}). Give "
                        "colors with a meaning a name of their own with the swatch\u{2019}s "<Code inline=true>"color_name"</Code>
                        " (\u{201c}Ocean\u{201d}). Typing a name\u{2019}s first letters moves the focus to the color."
                    </li>
                    <li>"The picked color is marked by more than its color: draw a ring or a check mark on it."</li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus into the picker, to the picked swatch (else the first), and out of it."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">"Move focus to the previous or next swatch (reversed right to left)."</KeyRow>
                    <KeyRow keys="ArrowUp / ArrowDown">"Move focus to the swatch above or below, in the same column."</KeyRow>
                    <KeyRow keys="Home / End">"Move focus to the first or last swatch."</KeyRow>
                    <KeyRow keys="Enter / Space">"Pick the focused swatch."</KeyRow>
                    <KeyRow keys="Any character">"Focus the next swatch whose color name starts with the typed text."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorSwatch.materialize()>"Color Swatch"</Link></li>
                <li><Link href=routes::doc::color_swatch::Atom.materialize()>"Color Swatch Atom"</Link></li>
                <li><Link href=routes::doc::ColorPicker.materialize()>"Color Picker"</Link></li>
                <li><Link href=routes::doc::listbox::Atom.materialize()>"Listbox Atoms"</Link></li>
                <li><Link href=routes::doc::Color.materialize()>"Color"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
