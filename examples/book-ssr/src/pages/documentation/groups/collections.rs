use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageCollections() -> impl IntoView {
    view! {
        <DocPage title="Collections">
            <p>
                "Collections show a set of items that users navigate with the keyboard and often select: lists of "
                "options, menus of actions, grids, tables, trees and tags. Each collection is a single tab stop; the "
                "arrow keys move between its items."
            </p>
            <p>
                "They belong together because they share one state model: leptonic builds the items from your data, "
                "identifies them by keys and keeps their selection, focus and disabled state in one place, so every "
                "collection handles focus, selection and disabled items the same way. The shared building blocks are "
                "documented in "<Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link>"; the "
                "states of grids, tables and trees, which build on them, on their concepts\u{2019} Hooks tabs."

            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Collections.materialize()/>
            </Section>

            <Section title="Relationships">
                <ul>
                    <li>
                        <Link href=routes::doc::Listbox.materialize()>"Listbox"</Link>" and "
                        <Link href=routes::doc::Menu.materialize()>"Menu"</Link>" are one-dimensional lists. A listbox "
                        "selects values; a menu runs actions and opens from a trigger. The "
                        <Link href=routes::doc::Pickers.materialize()>"pickers"</Link>" (select and combobox) show a "
                        "listbox in a popover."
                    </li>
                    <li>
                        <Link href=routes::doc::Grid.materialize()>"Grid"</Link>" navigates rows and cells in two "
                        "dimensions. "<Link href=routes::doc::GridList.materialize()>"Grid List"</Link>" is a "
                        "one-dimensional list like a listbox, but its rows can contain buttons, checkboxes or links."
                    </li>
                    <li>
                        <Link href=routes::doc::Table.materialize()>"Table"</Link>" is built on the grid and adds column "
                        "headers, sortable columns, row headers and selection checkboxes."
                    </li>
                    <li>
                        <Link href=routes::doc::TagGroup.materialize()>"Tag Group"</Link>" and "
                        <Link href=routes::doc::Tree.materialize()>"Tree"</Link>" are grid lists: a tag group lays its "
                        "items out horizontally and lets users remove them, a tree gives its rows levels that expand and "
                        "collapse."
                    </li>
                </ul>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Let users select one or more options from a visible list"</TableCell>
                        <TableCell><Link href=routes::doc::Listbox.materialize()>"Listbox"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Choose a value from a list that opens on demand"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Select.materialize()>"Select"</Link>" or "
                            <Link href=routes::doc::Combobox.materialize()>"Combobox"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Offer a list of actions behind a button"</TableCell>
                        <TableCell><Link href=routes::doc::Menu.materialize()>"Menu"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show records with the same fields, to sort, compare and select them"</TableCell>
                        <TableCell><Link href=routes::doc::Table.materialize()>"Table"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Navigate a two-dimensional arrangement without column headers, such as a palette"</TableCell>
                        <TableCell><Link href=routes::doc::Grid.materialize()>"Grid"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a list whose items contain buttons or links"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::GridList.materialize()>"Grid List"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show keywords, filters or recipients that users can select and remove"</TableCell>
                        <TableCell><Link href=routes::doc::TagGroup.materialize()>"Tag Group"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show hierarchical items, such as files and folders"</TableCell>
                        <TableCell><Link href=routes::doc::Tree.materialize()>"Tree"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a few static labels, not a navigable set"</TableCell>
                        <TableCell>"Styled "<Code inline=true>"<span>"</Code>"s (no behavior needed)"</TableCell>
                    </TableRow>
                </DocTable>
            </Section>
        </DocPage>
    }
}
