use leptos::prelude::*;

use super::demos::tag_group::TagGroupConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageTagGroupOverview() -> impl IntoView {
    view! {
        <DocPage title="Tag Group">
            <p>
                "A tag group is a set of tags, such as keywords, active filters or the recipients of a message, that users "
                "navigate with the arrow keys and can select and remove. It is a single tab stop: inside it, the arrow "
                "keys move between the tags, and "<Keys keys="Delete"/>" or "<Keys keys="Backspace"/>" removes the "
                "focused one."
            </p>
            <p>
                "A tag group is a horizontal "<Link href=routes::doc::GridList.materialize()>"grid list"</Link>
                ". Removing a tag is up to you: the group tells you which keys to remove, and you drop them from your data."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Show a set of removable or selectable labels, such as filters or recipients"</TableCell>
                        <TableCell><b>"Tag Group"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a single static label, such as a status"</TableCell>
                        <TableCell>"A styled "<Code inline=true>"<span>"</Code>" (no behavior needed)"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Let users pick options from a list with richer items"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Listbox.materialize()>"ListBox"</Link>" or "
                            <Link href=routes::doc::GridList.materialize()>"Grid List"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Toggle a few options with buttons"</TableCell>
                        <TableCell><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button"</Link>" group"</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::tag_group::Hook.materialize()>"Tag Group Hooks"</Link></TableCell>
                        <TableCell>
                            <Code inline=true>"use_tag_group"</Code>" and "<Code inline=true>"use_tag"</Code>": the grid "
                            "semantics, keyboard navigation, selection and removal, for elements you render from a list state."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::tag_group::Atom.materialize()>"Tag Group Atoms"</Link></TableCell>
                        <TableCell>
                            "Unstyled "<Code inline=true>"TagGroup"</Code>", "<Code inline=true>"TagList"</Code>", "
                            <Code inline=true>"Tag"</Code>" and "<Code inline=true>"TagRemoveButton"</Code>" for a collection, "
                            "with a "<Code inline=true>"Label"</Code>" and an optional "<Code inline=true>"Description"</Code>
                            ", styled through data attributes."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Active filters as removable tags, built from the atoms. The classes are the book\u{2019}s own; the "
                    <Link href=format!("{}#styling", routes::doc::tag_group::Atom.materialize())>"styling section"</Link>
                    " of the atoms shows their CSS."
                </p>
                <Demo description="Removable filter tags" source=include_str!("demos/tag_group.rs") source_open=true>
                    <TagGroupConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A tag group follows the WAI-ARIA "
                    <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/grid/" target=LinkTarget::Blank>"grid pattern"</Link>
                    ":"
                </p>
                <ul>
                    <li>
                        "The list of tags is a "<Code inline=true>"grid"</Code>" (a "<Code inline=true>"group"</Code>
                        " while empty), named by its label; each tag is a "<Code inline=true>"row"</Code>" with one "
                        <Code inline=true>"gridcell"</Code>"."
                    </li>
                    <li>
                        "Selectable tags carry "<Code inline=true>"aria-selected"</Code>". Removable tags are described as "
                        "\u{201c}Press Delete or Backspace to remove.\u{201d}, and their remove buttons are named "
                        "\u{201c}Remove\u{201d} and the tag\u{2019}s name."
                    </li>
                    <li>
                        "When a tag is removed, the focus moves to a neighbor, or stays in the empty group. Tags added "
                        "while the focus is in the group are announced."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves the focus into the group, to the last focused tag, and out again."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">"Focuses the previous or next tag, wrapping around."</KeyRow>
                    <KeyRow keys="Home / End">"Focuses the first or last tag."</KeyRow>
                    <KeyRow keys="Space">"Selects or deselects the focused tag (when selection is enabled)."</KeyRow>
                    <KeyRow keys="Control + A">
                        "Selects all tags (multiple selection; "<Keys keys="Meta + A"/>" on macOS)."
                    </KeyRow>
                    <KeyRow keys="Escape">"Clears the selection."</KeyRow>
                    <KeyRow keys="Delete / Backspace">
                        "Removes the focused tag, or all selected tags if it is selected."
                    </KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::tag_group::Hook.materialize()>"Tag Group Hooks"</Link></li>
                <li><Link href=routes::doc::tag_group::Atom.materialize()>"Tag Group Atoms"</Link></li>
                <li><Link href=routes::doc::GridList.materialize()>"Grid List overview"</Link></li>
                <li><Link href=routes::doc::Collections.materialize()>"Collections"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
