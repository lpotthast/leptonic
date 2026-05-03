use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{button_basic::ButtonBasicConceptDemo, button_styled::ButtonStyledConceptDemo};
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageButtonOverview() -> impl IntoView {
    view! {
        <Article>
            <h1 id="button" class="anchor">
                "Button"
                <AnchorLink href="#button" description="Direct link to article header"/>
            </h1>

            <p>
                "Buttons are interactive elements that trigger an action when activated. "
                "They are the primary way users initiate operations like submitting forms, "
                "opening dialogs, or triggering side effects."
            </p>

            <p>
                "Leptonic provides buttons at three abstraction levels. "
                "See "<Link href=routes::doc::Architecture.materialize()>"Hooks, Atoms & Components"</Link>
                " for a detailed explanation of each layer. "
                "This page covers the button concept; choose a layer below to dive into specifics."
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
                            <TableCell>"Trigger an action (submit, delete, open)"</TableCell>
                            <TableCell><b>"Button"</b></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Navigate to another page or URL"</TableCell>
                            <TableCell>"Link"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Toggle a binary state on/off"</TableCell>
                            <TableCell>"Toggle / Switch"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Select from a set of options"</TableCell>
                            <TableCell>"Checkbox / Radio"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <p>
                "If an element looks like a button but navigates, use "
                <Code inline=true>"LinkButton"</Code>
                " (atom) or style a "
                <Code inline=true>"Link"</Code>
                " as a button. If an element looks like a link but triggers an action, use a button."
            </p>

            <h2 id="dive-deeper" class="anchor">
                "Dive Deeper"
                <AnchorLink href="#dive-deeper" description="Direct link to section: Dive Deeper"/>
            </h2>

            <p>
                "Not sure which layer to pick? Read the "
                <Link href=routes::doc::Architecture.materialize()>"architecture guide"</Link>
                ". Otherwise, pick a layer:"
            </p>

            <ul>
                <li><Link href=routes::doc::button::Hook.materialize()>"Hook: use_button"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Atom: Button"</Link></li>
                <li><Link href=routes::doc::button::Component.materialize()>"Component: Button"</Link></li>
            </ul>

            <h2 id="quick-start" class="anchor">
                "Quick Start"
                <AnchorLink href="#quick-start" description="Direct link to section: Quick Start"/>
            </h2>

            <p>"The simplest way to use a button (component layer):"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <Button on_press=move |_| {}>"My Button"</Button>
                "#)}
            </Code>

            <DemoShell description="Basic button rendering" source=include_str!("demos/button_basic.rs")>
                <ButtonBasicConceptDemo />
            </DemoShell>

            <p>"A button with a color and variant:"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <Button on_press=move |_| {} color=ButtonColor::Primary variant=ButtonVariant::Filled>
                        "Save"
                    </Button>
                "#)}
            </Code>

            <DemoShell description="Button with variant styling" source=include_str!("demos/button_styled.rs")>
                <ButtonStyledConceptDemo />
            </DemoShell>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to section: Accessibility"/>
            </h2>

            <p>
                "Leptonic buttons follow the WAI-ARIA Button pattern. "
                "All three layers (hook, atom, component) produce the same accessibility behavior."
            </p>

            <h3>"ARIA attributes"</h3>

            <ul>
                <li><Code inline=true>"role=\"button\""</Code>" \u{2014} set automatically when using non-button elements with the hook"</li>
                <li><Code inline=true>"aria-disabled"</Code>" \u{2014} mirrors the disabled prop as \"true\"/\"false\" (not HTML boolean)"</li>
                <li><Code inline=true>"aria-haspopup"</Code>" \u{2014} indicates if the button opens a popup (menu, dialog, etc.)"</li>
                <li><Code inline=true>"aria-expanded"</Code>" \u{2014} indicates if the controlled popup is currently open"</li>
                <li><Code inline=true>"tabindex=\"0\""</Code>" \u{2014} ensures focusability; set to \"-1\" when disabled"</li>
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
                            <TableCell><Code inline=true>"Enter"</Code></TableCell>
                            <TableCell>"Activates the button"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Space"</Code></TableCell>
                            <TableCell>"Activates the button"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"Tab"</Code></TableCell>
                            <TableCell>"Moves focus to the button"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Button", link: "#button" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Dive Deeper", link: "#dive-deeper" },
                Toc::Leaf { title: "Quick Start", link: "#quick-start" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
