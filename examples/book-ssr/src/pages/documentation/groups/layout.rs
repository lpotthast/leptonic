use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageLayout() -> impl IntoView {
    view! {
        <DocPage title="Content & Layout">
            <p>
                "Content & Layout covers what structures a page and presents its content: the app bar at the top, stacks "
                "and grids arranging elements, cards grouping them, separators and toolbars dividing and bundling them, "
                "and the elements that display content \u{2014} text, code, HTML from other sources, icons, key caps, chips "
                "and placeholders for content that is still loading."
            </p>
            <p>
                "They belong together because they arrange or display content instead of collecting input or navigating. "
                "Most of them are styled components without behavior of their own; the toolbar and the separator carry "
                "ARIA semantics and also have a hook and an atom."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::Layout.materialize()/>
            </Section>

            <Section title="Relationships">
                <ul>
                    <li>
                        <Link href=routes::doc::Stack.materialize()>"Stack"</Link>" and "
                        <Link href=routes::doc::GridLayout.materialize()>"Grid Layout"</Link>" arrange elements: a stack along "
                        "one axis with even spacing, a grid layout in responsive columns that change with the screen width."
                    </li>
                    <li>
                        <Link href=routes::doc::CardAndTile.materialize()>"Card & Tile"</Link>" put related content on a "
                        "surface of its own. Cards often sit in a stack or a grid layout, which then also spaces them."
                    </li>
                    <li>
                        <Link href=routes::doc::Toolbar.materialize()>"Toolbar"</Link>" groups controls into one tab stop "
                        "whose controls the arrow keys move between. A vertical "
                        <Link href=routes::doc::Separator.materialize()>"Separator"</Link>" divides groups of controls "
                        "inside it. Groups of "<Link href=routes::doc::ToggleButton.materialize()>"toggle buttons"</Link>
                        " are toolbars too."
                    </li>
                    <li>
                        <Link href=routes::doc::AppBar.materialize()>"App Bar"</Link>" holds the app name, navigation and "
                        "global actions, often as icon buttons: an "<Link href=routes::doc::Icon.materialize()>"Icon"</Link>
                        " inside a button labelled with "<Code inline=true>"aria-label"</Code>"."
                    </li>
                    <li>
                        <Link href=routes::doc::Typography.materialize()>"Typography"</Link>" styles headings, paragraphs, "
                        "lists and code; "<Link href=routes::doc::Kbd.materialize()>"Kbd"</Link>" adds key caps and keyboard "
                        "shortcuts to such text. "<Link href=routes::doc::SanitizedHtml.materialize()>"Sanitized HTML"</Link>
                        " renders text that arrives as HTML, such as the output of a Markdown renderer or a rich text editor."
                    </li>
                    <li>
                        <Link href=routes::doc::Chip.materialize()>"Chip"</Link>" is a static label. A set of tags that "
                        "users navigate, select and remove is a "<Link href=routes::doc::TagGroup.materialize()>"Tag Group"</Link>
                        "."
                    </li>
                    <li>
                        <Link href=routes::doc::Skeleton.materialize()>"Skeleton"</Link>" holds the place of any of these "
                        "while its content loads."
                    </li>
                </ul>
            </Section>

            <Section title="Decision Guide">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Arrange elements in a row or a column with even spacing"</TableCell>
                        <TableCell><Link href=routes::doc::Stack.materialize()>"Stack"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Arrange content in columns that adapt to the screen width"</TableCell>
                        <TableCell><Link href=routes::doc::GridLayout.materialize()>"Grid Layout"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Navigate rows and cells of data with the keyboard"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::Grid.materialize()>"Grid"</Link>" or "
                            <Link href=routes::doc::Table.materialize()>"Table"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show related content, such as a summary with its actions, on a surface of its own"</TableCell>
                        <TableCell><Link href=routes::doc::CardAndTile.materialize()>"Card & Tile"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Put the app name, navigation and global actions at the top"</TableCell>
                        <TableCell><Link href=routes::doc::AppBar.materialize()>"App Bar"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Group related controls, such as the formatting buttons of an editor"</TableCell>
                        <TableCell><Link href=routes::doc::Toolbar.materialize()>"Toolbar"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Mark a boundary between sections or groups of controls"</TableCell>
                        <TableCell><Link href=routes::doc::Separator.materialize()>"Separator"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Write headings, paragraphs, lists and code"</TableCell>
                        <TableCell><Link href=routes::doc::Typography.materialize()>"Typography"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Render HTML you didn\u{2019}t write, e.g. from users or a CMS"</TableCell>
                        <TableCell><Link href=routes::doc::SanitizedHtml.materialize()>"Sanitized HTML"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show a key or a keyboard shortcut"</TableCell>
                        <TableCell><Link href=routes::doc::Kbd.materialize()>"Kbd"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Show an icon"</TableCell>
                        <TableCell><Link href=routes::doc::Icon.materialize()>"Icon"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Label an item with a tag or a status"</TableCell>
                        <TableCell><Link href=routes::doc::Chip.materialize()>"Chip"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Hold the place of content that is still loading"</TableCell>
                        <TableCell><Link href=routes::doc::Skeleton.materialize()>"Skeleton"</Link></TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "A separator is announced to screen readers as a boundary. For spacing without that meaning, use CSS "
                    "margins or the spacing of a stack."
                </p>
            </Section>
        </DocPage>
    }
}
