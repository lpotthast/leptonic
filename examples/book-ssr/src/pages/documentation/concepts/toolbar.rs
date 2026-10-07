use leptos::prelude::*;

use super::demos::toolbar::ToolbarConceptDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageToolbarOverview() -> impl IntoView {
    view! {
        <DocPage title="Toolbar">
            <p>
                "A toolbar groups related controls, such as the formatting buttons of a text editor. The whole toolbar is one "
                "tab stop: "<Keys keys="Tab"/>" enters and leaves it, the arrow keys move between its controls. Keyboard "
                "users pass a toolbar of twenty buttons with a single key press, instead of tabbing through all of them, and "
                "find the control they used last when they come back."
            </p>
            <p>
                "A toolbar holds any focusable controls: "<Link href=routes::doc::Button.materialize()>"buttons"</Link>", "
                <Link href=routes::doc::ToggleButton.materialize()>"toggle buttons"</Link>", "
                <Link href=routes::doc::Menu.materialize()>"menu"</Link>" triggers, "
                <Link href=routes::doc::Select.materialize()>"selects"</Link>". "
                <Link href=routes::doc::Separator.materialize()>"Separators"</Link>" divide them into groups."
            </p>

            <Section title="When to Use">
                <DocTable headers=&["If you want to\u{2026}", "Use"]>
                    <TableRow>
                        <TableCell>"Group several related controls into one tab stop"</TableCell>
                        <TableCell><b>"Toolbar"</b></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Toggle options on and off, or choose one of them (bold, alignment)"</TableCell>
                        <TableCell>
                            <Link href=routes::doc::ToggleButton.materialize()>"Toggle Button"</Link>
                            " group, a toolbar of toggle buttons"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Offer many actions behind one button"</TableCell>
                        <TableCell><Link href=routes::doc::Menu.materialize()>"Menu"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Switch between views"</TableCell>
                        <TableCell><Link href=routes::doc::Tabs.materialize()>"Tabs"</Link></TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Lay out buttons without changing their keyboard behavior"</TableCell>
                        <TableCell>"A "<Link href=format!("{}#stack", routes::doc::Layout.materialize())>"stack"</Link>" (CSS flexbox)"</TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "Use a toolbar for three or more controls that act on the same thing. Two buttons of a dialog footer stay "
                    "separate tab stops."
                </p>
            </Section>

            <Section title="Choose Your Layer">
                <p>
                    "See "<Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    " for how the layers relate."
                </p>
                <DocTable headers=&["Layer", "What you get"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::toolbar::Hook.materialize()>"use_toolbar"</Link></TableCell>
                        <TableCell>
                            "The role, orientation, label and arrow-key navigation for "
                            "a container you render yourself."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::toolbar::Atom.materialize()>"Toolbar Atom"</Link></TableCell>
                        <TableCell>"An unstyled "<Code inline=true>"Toolbar"</Code>", styled through data attributes."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Quick Start">
                <p>
                    "Put the controls into a "<Code inline=true>"Toolbar"</Code>" atom and name it with "
                    <Code inline=true>"aria_label"</Code>". The controls need no tabindex of their own: the toolbar moves "
                    "the focus. Tab into the demo, move with the arrow keys, tab out and back in:"
                </p>
                <Demo
                    description="Clipboard toolbar with three buttons and a separator, showing the last action"
                    source=include_str!("demos/toolbar.rs")
                    source_open=true
                >
                    <ToolbarConceptDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "A toolbar is a "<Code inline=true>"role=\"toolbar\""</Code>" with "
                        <Code inline=true>"aria-orientation"</Code>", following the WAI-ARIA "
                        <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/toolbar/" target=LinkTarget::Blank>"Toolbar pattern"</Link>"."
                    </li>
                    <li>
                        "Name it with "<Code inline=true>"aria_label"</Code>" or "<Code inline=true>"aria_labelledby"</Code>
                        ", especially when a page has more than one."
                    </li>
                    <li>
                        "A toolbar inside another toolbar becomes a "<Code inline=true>"group"</Code>" and leaves the keyboard "
                        "handling to the outer one, so the arrow keys move through the controls of both."
                    </li>
                </ul>
                <KeyboardTable>
                    <KeyRow keys="Tab / Shift + Tab">"Moves focus into the toolbar (to the control focused last) or out of it."</KeyRow>
                    <KeyRow keys="ArrowRight / ArrowLeft">"Horizontal toolbars: focuses the next or previous control (swapped in right-to-left locales)."</KeyRow>
                    <KeyRow keys="ArrowDown / ArrowUp">"Vertical toolbars: focuses the next or previous control."</KeyRow>
                </KeyboardTable>
                <p>"Focus doesn\u{2019}t wrap around: at the first or last control, the arrow keys do nothing."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::toolbar::Hook.materialize()>"use_toolbar"</Link></li>
                <li><Link href=routes::doc::toolbar::Atom.materialize()>"Toolbar Atom"</Link></li>
                <li><Link href=routes::doc::Separator.materialize()>"Separator"</Link></li>
                <li><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
