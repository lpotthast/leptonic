use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, toc::Toc},
    routes,
};

#[component]
pub fn PageMenuOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="menu" class="anchor">
                "Menu"
                <AnchorLink href="#menu" description="Direct link to article header"/>
            </h1>

            <p>
                "Menus present a list of actions or options in an overlay triggered by a button. "
                "They support single-select, multi-select, and purely action-based modes. "
                "Menus are built from multiple cooperating hooks: "
                <Code inline=true>"use_menu_trigger"</Code>", "
                <Code inline=true>"use_menu"</Code>", "
                <Code inline=true>"use_menu_item"</Code>", and "
                <Code inline=true>"use_menu_section"</Code>"."
            </p>

            <p>
                "Menus are currently available as hooks only. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer."
            </p>

            <h2 id="when-to-use" class="anchor">
                "When to Use"
                <AnchorLink href="#when-to-use" description="Direct link to section: When to Use"/>
            </h2>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"If you want to\u{2026}"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Use"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>"Show a list of actions triggered by a button"</TableCell>
                            <TableCell><b>"Menu"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Choose a value from a dropdown"</TableCell>
                            <TableCell>"Select"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Show rich contextual content"</TableCell>
                            <TableCell>"Popover"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="dive-deeper" class="anchor">
                "Dive Deeper"
                <AnchorLink href="#dive-deeper" description="Direct link to section: Dive Deeper"/>
            </h2>

            <ul>
                <li><Link href=routes::doc::menu::Hook.materialize()>"Hook: use_menu"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>
                "Menus are composed from hooks. Here is a brief sketch of the API. "
                "See the "<Link href=routes::doc::menu::Hook.materialize()>"hook deep-dive"</Link>
                " for a full interactive demo."
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let menu_trigger = use_menu_trigger(/* ... */);
                    let menu = use_menu(/* ... */);

                    // Spread trigger attrs on a button
                    <button {..menu_trigger.attrs}>
                        "Actions"
                    </button>

                    // Render the menu overlay with items
                    <ul {..menu.attrs}>
                        // Each item uses use_menu_item
                    </ul>
                "#)}
            </Code>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic menus follow the WAI-ARIA Menu pattern."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"menu\""</Code>" on the container, "
                    <Code inline=true>"role=\"menuitem\""</Code>" (or "<Code inline=true>"\"menuitemradio\""</Code>
                    " / "<Code inline=true>"\"menuitemcheckbox\""</Code>")"</li>
                <li><Code inline=true>"aria-haspopup=\"menu\""</Code>" on the trigger"</li>
                <li><Code inline=true>"aria-expanded"</Code>" on the trigger"</li>
            </ul>

            <h3>"Keyboard interaction"</h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Key"</TableHeaderCell>
                            <TableHeaderCell>"Action"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"Enter / Space"</Code></TableCell>
                            <TableCell>"Activate focused item"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Arrow Down / Up"</Code></TableCell>
                            <TableCell>"Navigate between items"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Escape"</Code></TableCell>
                            <TableCell>"Close the menu"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Menu", link: "#menu" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
