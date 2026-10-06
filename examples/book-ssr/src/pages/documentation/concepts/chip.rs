use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::chip::ChipConceptDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageChipOverview() -> impl IntoView {
    view! {
        <DocPage title="Chip">
            <p>
                "Chips are compact, interactive elements that represent attributes, tags, or actions. "
                "They can be dismissible (removable by the user) and come in color variants "
                "to convey meaning \u{2014} success, warning, danger, etc."
            </p>

            <p>
                "Groups of chips that can be navigated, selected and removed are built with the tag group hooks ("
                <Code inline=true>"use_tag_group"</Code>", "<Code inline=true>"use_tag"</Code>"), following react-aria\u{2019}s "
                "TagGroup. The themed "<Code inline=true>"Chip"</Code>" component is not built on them yet."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow><TableCell>"Display a removable tag or attribute"</TableCell><TableCell><b>"Chip"</b>" (dismissible)"</TableCell></TableRow>
                    <TableRow><TableCell>"Display a status label"</TableCell><TableCell><b>"Chip"</b>" (with color)"</TableCell></TableRow>
                    <TableRow><TableCell>"Trigger an action"</TableCell><TableCell><Link href=routes::doc::Button.materialize()>"Button"</Link></TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "Chips exist as a hook and as a component. See "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                    " for how the layers relate."
                </p>

                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::chip::Hook.materialize()>"Tag group hooks"</Link></TableCell>
                        <TableCell>"Keyboard navigation, selection, removal and ARIA attributes for tags you render yourself."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::chip::Component.materialize()>"Chip component"</Link></TableCell>
                        <TableCell>"A themed, standalone chip with colors and an optional dismiss button (mouse only for now)."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>"The component is the quickest way to chips:"</p>

                <Demo description="Chips in the default, success and danger colors" source=include_str!("demos/chip.rs") source_open=true>
                    <ChipConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "Tag groups built with the hooks follow the WAI-ARIA Grid pattern: the group is a "
                    <Code inline=true>"grid"</Code>", each tag a "<Code inline=true>"row"</Code>" with a "
                    <Code inline=true>"gridcell"</Code>", selectable tags carry "<Code inline=true>"aria-selected"</Code>
                    ", and removable tags are described as removable with Delete or Backspace."
                </p>

                <KeyboardTable>
                    <KeyRow keys="ArrowLeft / ArrowRight">"Focus the previous or next tag, wrapping around."</KeyRow>
                    <KeyRow keys="Space">"Toggle the selection of the focused tag (if selectable)."</KeyRow>
                    <KeyRow keys="Delete / Backspace">"Remove the focused tag, or all selected tags (if removable)."</KeyRow>
                </KeyboardTable>

                <p>
                    "The "<Code inline=true>"Chip"</Code>" component does not provide this yet: its dismiss icon can only be "
                    "used with a pointer. Build keyboard-accessible tag groups with the hooks."
                </p>
            </Section>
        </DocPage>
    }
}
