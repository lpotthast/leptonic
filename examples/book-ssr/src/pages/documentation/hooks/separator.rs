use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    separator_decorative::SeparatorDecorativeDemo, separator_horizontal::SeparatorHorizontalDemo,
    separator_vertical::SeparatorVerticalDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseSeparatorHook() -> impl IntoView {
    view! {
        <DocPage title="use_separator">
            <p>
                "The "<Code inline=true>"use_separator"</Code>" hook gives an element the semantics of a separator, so "
                "screen readers announce it as a boundary between two groups of content. See the "
                <Link href=routes::doc::Separator.materialize()>"Separator overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useSeparator"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseSeparatorInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    ", which describes a horizontal "<Code inline=true>"<hr>"</Code>"."
                </p>

                <ApiTable kind=ApiKind::Input of="UseSeparatorInput">
                    <ApiRow name="orientation" ty="Orientation" default="Horizontal">
                        "Whether the separator divides content stacked on top of each other ("<Code inline=true>"Horizontal"</Code>
                        ") or placed side by side ("<Code inline=true>"Vertical"</Code>")."
                    </ApiRow>
                    <ApiRow name="element_type" ty="SeparatorElementType" default="Hr">
                        "The element you render: "<Code inline=true>"Hr"</Code>", "<Code inline=true>"Div"</Code>" or "
                        <Code inline=true>"Span"</Code>". Decides which attributes are set, see "
                        <AnchorLink href="#element-types">"Element Types"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="id" ty="Option<String>" default="None">"The element\u{2019}s id."</ApiRow>
                    <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>, Option<String>" default="None">
                        "Names the separator, e.g. when it separates groups of controls in a toolbar."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseSeparatorReturn">
                    <ApiRow name="props" ty="UseSeparatorProps">
                        "The "<Code inline=true>"role"</Code>", "<Code inline=true>"aria-orientation"</Code>", id and label "
                        "for the separator element. Spread them with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::*;

                        // Using an <hr> element (no role needed)
                        let hr_sep = use_separator(UseSeparatorInput::default());

                        // Using a <div> element (role="separator" is added)
                        let div_sep = use_separator(UseSeparatorInput {
                            orientation: Orientation::Vertical,
                            element_type: SeparatorElementType::Div,
                            ..UseSeparatorInput::default()
                        });

                        view! {
                            <hr {..hr_sep.props.into_attrs()} />
                            <div {..div_sep.props.into_attrs()}></div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>"A horizontal separator rendered as a native "<Code inline=true>"<hr>"</Code>":"</p>
                <Demo description="Horizontal separator on an hr element" source=include_str!("demos/separator_horizontal.rs")>
                    <SeparatorHorizontalDemo/>
                </Demo>

                <p>"A vertical separator between two pieces of inline content, rendered as a "<Code inline=true>"<div>"</Code>":"</p>
                <Demo description="Vertical separator on a div element" source=include_str!("demos/separator_vertical.rs")>
                    <SeparatorVerticalDemo/>
                </Demo>

                <p>"Any element can be styled freely, as long as it carries the separator attributes:"</p>
                <Demo description="Horizontal separator on a styled div element" source=include_str!("demos/separator_decorative.rs")>
                    <SeparatorDecorativeDemo/>
                </Demo>
            </Section>

            <Section title="Element Types">
                <p>
                    "A native "<Code inline=true>"<hr>"</Code>" already is a horizontal separator, so the hook adds nothing to it. "
                    "Other elements need ARIA to say what they are:"
                </p>

                <DocTable headers=&["SeparatorElementType", "Attributes set"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Hr"</Code></TableCell>
                        <TableCell>"None. The element\u{2019}s implicit "<Code inline=true>"separator"</Code>" role is enough."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Div"</Code>", "<Code inline=true>"Span"</Code></TableCell>
                        <TableCell>
                            <Code inline=true>"role=\"separator\""</Code>", and "<Code inline=true>"aria-orientation=\"vertical\""</Code>
                            " when vertical (horizontal is the default orientation)."
                        </TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "Because an "<Code inline=true>"<hr>"</Code>" gets no "<Code inline=true>"aria-orientation"</Code>
                    ", render vertical separators as a "<Code inline=true>"<div>"</Code>" or "<Code inline=true>"<span>"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Separator.materialize()>"Separator overview"</Link></li>
                <li><Link href=routes::doc::separator::Atom.materialize()>"Separator Atom"</Link></li>
                <li><Link href=routes::doc::Layout.materialize()>"Content & Layout"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
