use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

use super::demos::selection::SelectionDomainDemo;

#[component]
pub fn PageSelection() -> impl IntoView {
    view! {
        <Article>
            <h1 id="selection" class="anchor">
                "Selection"
                <AnchorLink href="#selection" description="Direct link to article header"/>
            </h1>

            <p>
                "Manages item selection in lists, grids, tables, and other collections. "
                "The selection system handles selection modes (none, single, multiple), "
                "keyboard navigation, type-ahead character matching, and ARIA selection attributes."
            </p>

            <p>
                "These hooks power "
                <Link href=routes::doc::listbox::Hook.materialize()>"Listbox"</Link>", "
                <Link href=routes::doc::grid::Hook.materialize()>"Grid"</Link>", "
                <Link href=routes::doc::table::Hook.materialize()>"Table"</Link>", "
                <Link href=routes::doc::tabs::Hook.materialize()>"Tabs"</Link>", and "
                <Link href=routes::doc::menu::Hook.materialize()>"Menu"</Link>
                " \u{2014} any component where users choose items from a set."
            </p>

            <h2 id="overview" class="anchor">
                "Overview"
                <AnchorLink href="#overview" description="Direct link to section: Overview"/>
            </h2>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Name"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                            <TableHeaderCell>"When to Use"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Link href=routes::doc::hooks::Selection.materialize()>"use_selection_state"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Core state: selected keys, modes, toggle/select/clear"</TableCell>
                            <TableCell>"Any component with selectable items"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::hooks::Selection.materialize()>"use_selectable_item"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Click, double-click, and focus handling per item"</TableCell>
                            <TableCell>"Individual items within a selectable collection"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::hooks::Selection.materialize()>"use_selectable_list"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Vertical keyboard navigation (ArrowUp/ArrowDown)"</TableCell>
                            <TableCell>"Single-axis lists (listbox, menu)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::hooks::Selection.materialize()>"use_selectable_collection"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Full arrow-key navigation via KeyboardDelegate"</TableCell>
                            <TableCell>"Multi-axis collections (grids, tables)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::hooks::Selection.materialize()>"use_type_select"</Link></TableCell>
                            <TableCell>"Hook"</TableCell>
                            <TableCell>"Character matching to jump to items by typing"</TableCell>
                            <TableCell>"Type-ahead in lists, menus, selects"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::hooks::Selection.materialize()>"KeyboardDelegate"</Link></TableCell>
                            <TableCell>"Trait"</TableCell>
                            <TableCell>"Abstracts navigation for different collection shapes"</TableCell>
                            <TableCell>"Custom navigation logic (grids, trees)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="relationships" class="anchor">
                "Relationships"
                <AnchorLink href="#relationships" description="Direct link to section: Relationships"/>
            </h2>

            <h3>"Layered Architecture"</h3>

            <p>"The selection hooks form a layered stack, each level building on the one below:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"Layer"</TableHeaderCell>
                            <TableHeaderCell>"Hook"</TableHeaderCell>
                            <TableHeaderCell>"Responsibility"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>"State"</TableCell>
                            <TableCell><Code inline=true>"use_selection_state"</Code></TableCell>
                            <TableCell>"Manages the set of selected keys, selection mode, and mutation operations"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Item"</TableCell>
                            <TableCell><Code inline=true>"use_selectable_item"</Code></TableCell>
                            <TableCell>"Handles click, double-click, and focus for a single item"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Container (1D)"</TableCell>
                            <TableCell><Code inline=true>"use_selectable_list"</Code></TableCell>
                            <TableCell>"Adds vertical keyboard navigation (ArrowUp/ArrowDown, Home/End)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Container (2D+)"</TableCell>
                            <TableCell><Code inline=true>"use_selectable_collection"</Code></TableCell>
                            <TableCell>"Full arrow-key navigation via the KeyboardDelegate trait"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Enhancement"</TableCell>
                            <TableCell><Code inline=true>"use_type_select"</Code></TableCell>
                            <TableCell>"Adds character-matching jump behavior to any collection"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3>"KeyboardDelegate"</h3>

            <p>
                "The "<Code inline=true>"KeyboardDelegate"</Code>" trait abstracts navigation for different collection shapes. "
                "A list implements it with linear up/down logic, while a grid provides row/column navigation. "
                "This lets "<Code inline=true>"use_selectable_collection"</Code>" work with any collection geometry."
            </p>

            <h3>"Used By"</h3>

            <p>"These concepts compose selection hooks internally:"</p>

            <ul>
                <li><Link href=routes::doc::listbox::Hook.materialize()>"Listbox"</Link>" \u{2014} single-axis selection with use_selectable_list"</li>
                <li><Link href=routes::doc::select::Hook.materialize()>"Select"</Link>" \u{2014} dropdown with single selection"</li>
                <li><Link href=routes::doc::combobox::Hook.materialize()>"Combobox"</Link>" \u{2014} filtered list with selection + type-ahead"</li>
                <li><Link href=routes::doc::grid::Hook.materialize()>"Grid"</Link>" \u{2014} 2D selection via use_selectable_collection"</li>
                <li><Link href=routes::doc::table::Hook.materialize()>"Table"</Link>" \u{2014} row selection in tabular data"</li>
                <li><Link href=routes::doc::tabs::Hook.materialize()>"Tabs"</Link>" \u{2014} single selection for tab switching"</li>
                <li><Link href=routes::doc::menu::Hook.materialize()>"Menu"</Link>" \u{2014} action selection with keyboard navigation"</li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"A clickable list with multi-select and toggle behavior:"</p>

            <DemoShell
                source=include_str!("demos/selection.rs")
                description="Selectable list with keyboard navigation"
            >
                <SelectionDomainDemo />
            </DemoShell>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Selection", link: "#selection" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Relationships", link: "#relationships" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
            ]
        }/>
    }
}
