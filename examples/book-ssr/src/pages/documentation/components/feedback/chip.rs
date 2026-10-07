use leptos::prelude::*;

use super::demos::{chip_colors::ChipColorsDemo, chip_dismissible::ChipDismissibleDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageChip() -> impl IntoView {
    view! {
        <DocPage title="Chip Component">
            <p>
                "A chip is a compact, colored label for an attribute, a status or an active filter. Its color conveys "
                "meaning (success, warning, danger, \u{2026}), and a dismissible chip has an icon that lets users remove it. "
                "The themed "<Code inline=true>"Chip"</Code>" component shows one chip on its own."
            </p>

            <p>
                "For a set of tags that users navigate with the keyboard, select or remove, use a "
                <Link href=routes::doc::TagGroup.materialize()>"tag group"</Link>"; to trigger an action, a "
                <Link href=routes::doc::Button.materialize()>"button"</Link>"."
            </p>

            <Demo description="Chips in all six colors" source=include_str!("demos/chip_colors.rs")>
                <ChipColorsDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Chip">
                    <ApiRow name="color" ty="Signal<ChipColor>" default="Primary">
                        <Code inline=true>"Primary"</Code>", "<Code inline=true>"Secondary"</Code>", "
                        <Code inline=true>"Success"</Code>", "<Code inline=true>"Info"</Code>", "
                        <Code inline=true>"Warn"</Code>" or "<Code inline=true>"Danger"</Code>
                        ". Rendered as the "<Code inline=true>"data-color"</Code>" attribute."
                    </ApiRow>
                    <ApiRow name="on_dismiss" ty="Option<Callback<()>>" default="None">
                        "Shows a dismiss button and is called when it is pressed, see "
                        <AnchorLink href="#dismissible-chips">"Dismissible Chips"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="dismiss_label" ty="MaybeProp<String>" default="\"Dismiss\"">
                        "Names the dismiss button, e.g. \u{201c}Remove filter\u{201d}."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the dismiss button."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The chip content."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Dismissible Chips">
                <p>
                    "Chips often show state that users can change, such as an active filter. With "
                    <Code inline=true>"on_dismiss"</Code>" set, the chip shows a dismiss button with an "<Code inline=true>"X"</Code>
                    " icon. Pressing it (with a pointer or the keyboard) calls your handler; the chip doesn\u{2019}t remove "
                    "itself, so stop rendering it there."
                </p>

                <Demo description="Dismissible filter chip with a button restoring it and a status line" source=include_str!("demos/chip_dismissible.rs")>
                    <ChipDismissibleDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A chip is a plain "<Code inline=true>"<div>"</Code>" without a role; its text is read as part of the "
                    "surrounding content. Don\u{2019}t rely on its color alone: say what it means in its text. The dismiss "
                    "button is focusable and named by "<Code inline=true>"dismiss_label"</Code>" (default \u{201c}Dismiss\u{201d}): "
                    "name what it removes, as several chips\u{2019} buttons would otherwise share one name. For a list of "
                    "removable items that users navigate with the arrow keys, use a "
                    <Link href=routes::doc::TagGroup.materialize()>"tag group"</Link>"."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt chips to your design:"</p>
                <CssVariables prefix="--chip-" scss=theme_scss!("chip")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::TagGroup.materialize()>"Tag Group"</Link></li>
                <li><Link href=routes::doc::Button.materialize()>"Button"</Link></li>
                <li><Link href=routes::doc::Layout.materialize()>"Content & Layout"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
