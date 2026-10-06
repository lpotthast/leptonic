use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{chip_colors::ChipColorsDemo, chip_dismissible::ChipDismissibleDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageChip() -> impl IntoView {
    view! {
        <DocPage title="Chip component">
            <p>
                "The themed "<Code inline=true>"Chip"</Code>" is a small, colored label, for example for a tag, a status "
                "or a filter. See the "<Link href=routes::doc::Chip.materialize()>"Chip overview"</Link>
                " for concept guidance."
            </p>

            <Demo description="Chips in all six colors" source=include_str!("demos/chip_colors.rs")>
                <ChipColorsDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Chip">
                    <ApiRow name="color" ty="Option<Signal<ChipColor>>" default="None">
                        <Code inline=true>"Primary"</Code>" (used when omitted), "<Code inline=true>"Secondary"</Code>", "
                        <Code inline=true>"Success"</Code>", "<Code inline=true>"Info"</Code>", "
                        <Code inline=true>"Warn"</Code>" or "<Code inline=true>"Danger"</Code>
                        ". Rendered as the "<Code inline=true>"data-color"</Code>" attribute."
                    </ApiRow>
                    <ApiRow name="dismissible" ty="Option<Out<MouseEvent, LocalStorage>>" default="None">
                        "Shows a dismiss icon and receives its click events, see "
                        <a href="#dismissible-chips">"Dismissible Chips"</a>"."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The chip content."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Dismissible Chips">
                <p>
                    "Chips often show state that users can change, such as an active filter. With "
                    <Code inline=true>"dismissible"</Code>" set, the chip shows an "<Code inline=true>"X"</Code>
                    " icon. Clicking it calls your handler; the chip doesn\u{2019}t remove itself, so stop rendering it there."
                </p>

                <Demo description="Chip that hides itself when dismissed" source=include_str!("demos/chip_dismissible.rs")>
                    <ChipDismissibleDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt chips to your design:"</p>
                <CssVariables prefix="--chip-" scss=theme_scss!("chip")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Chip.materialize()>"Chip overview"</Link></li>
                <li><Link href=routes::doc::chip::Hook.materialize()>"use_tag"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
