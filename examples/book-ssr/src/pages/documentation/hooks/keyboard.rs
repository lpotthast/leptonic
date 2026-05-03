use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::keyboard::KeyboardDemo;

#[component]
pub fn PageUseKeyboard() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-keyboard" class="anchor">
                "use_keyboard"
                <AnchorLink href="#use-keyboard" description="Direct link to section: use_keyboard"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_keyboard"</Code>" hook handles keyboard events with support for disabling and controlling event propagation. "
                "See the "<Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useKeyboard.html" target=LinkTarget::_Blank>
                    "useKeyboard"
                </LinkExt>
                "."
            </p>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to section: Input"/>
            </h2>

            <p><Code inline=true>"UseKeyboardInput"</Code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"disabled"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Disables all keyboard event handling when true."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_key_down"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<KeyboardEventWrapper>>"</Code></TableCell>
                            <TableCell>"Called when a key is pressed down."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_key_up"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<KeyboardEventWrapper>>"</Code></TableCell>
                            <TableCell>"Called when a key is released."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return" class="anchor">
                "Return"
                <AnchorLink href="#return" description="Direct link to section: Return"/>
            </h2>

            <p><Code inline=true>"UseKeyboardReturn"</Code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseKeyboardProps"</Code></TableCell>
                            <TableCell>"Spread onto the target element via "<Code inline=true>"props.into_attrs()"</Code>" to wire up keydown/keyup listeners."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to section: Demo"/>
            </h2>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseKeyboardReturn { props } = use_keyboard(UseKeyboardInput {
                        disabled: disabled.into(),
                        on_key_down: Some(Callback::new(|e: KeyboardEventWrapper| {
                            // e.key(), e.code(), e.shift_key(), e.ctrl_key(), ...
                            e.continue_propagation(); // stops propagation by default
                        })),
                        on_key_up: Some(Callback::new(|e: KeyboardEventWrapper| {
                            e.continue_propagation();
                        })),
                    });

                    view! {
                        <div tabindex="0" {..props.into_attrs()}>
                            "Focus me and press keys"
                        </div>
                    }
                "#)}
            </Code>

            <DemoShell source=include_str!("demos/keyboard.rs")>
                <KeyboardDemo />
            </DemoShell>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to section: Features"/>
            </h2>

            <ul>
                <li>"Handles keydown and keyup events"</li>
                <li>"Supports disabling via signal"</li>
                <li>"Controls event propagation (stops by default, call "<Code inline=true>"continue_propagation()"</Code>" to allow)"</li>
                <li>"Wraps "<Code inline=true>"KeyboardEvent"</Code>" with convenient accessors via "<Code inline=true>"KeyboardEventWrapper"</Code></li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UseHover.materialize()>"use_hover"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_keyboard", link: "#use-keyboard" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return", link: "#return" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
