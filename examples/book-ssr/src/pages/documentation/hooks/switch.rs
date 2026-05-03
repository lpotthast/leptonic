use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::switch::SwitchDemo;

#[component]
pub fn PageUseSwitchHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-switch" class="anchor">
                "use_switch & use_toggle"
                <AnchorLink href="#use-switch" description="Direct link to article header"/>
            </h1>

            <p>
                "Hooks for creating toggle switches with proper accessibility and hidden form inputs. "
                "See the "<Link href=crate::routes::doc::Toggle.materialize()>"Toggle overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useSwitch.html" target=LinkTarget::_Blank>
                    "useSwitch"
                </LinkExt>
                "."
            </p>

            // ── use_switch Input ────────────────────────────────────

            <h2 id="switch-input" class="anchor">
                "use_switch Input"
                <AnchorLink href="#switch-input" description="Direct link to section: use_switch Input"/>
            </h2>

            <p><Code inline=true>"UseSwitchInput"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"is_selected"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the switch is selected (controlled)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_change"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<bool>>"</Code></TableCell>
                            <TableCell>"Callback when the selection changes"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_disabled"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the switch is disabled"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"name"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<&'static str>"</Code></TableCell>
                            <TableCell>"Name attribute for form submission"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            // ── use_switch Return ───────────────────────────────────

            <h2 id="switch-return" class="anchor">
                "use_switch Return"
                <AnchorLink href="#switch-return" description="Direct link to section: use_switch Return"/>
            </h2>

            <p><Code inline=true>"UseSwitchReturn"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"switch_props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseSwitchProps"</Code></TableCell>
                            <TableCell>"ARIA attributes and event handlers for the switch element"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"input_props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseSwitchInputProps"</Code></TableCell>
                            <TableCell>"Props for a hidden input for form submission"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_selected"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the switch is currently selected"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_pressed"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the switch is pressed (during interaction)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            // ── Demo ─────────────────────────────────────

            <h2 id="switch-demo" class="anchor">
                "Demo"
                <AnchorLink href="#switch-demo" description="Direct link to section: Demo"/>
            </h2>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseSwitchStateReturn { is_selected, set_selected, toggle } = use_switch_state(false);

                    let UseSwitchReturn { switch_props, input_props, .. } = use_switch(UseSwitchInput {
                        is_selected: is_selected.into(),
                        is_disabled: false.into(),
                        is_read_only: false.into(),
                        name: Some("notifications"),
                        value: Some("enabled"),
                        on_change: Some(Callback::new(move |v| set_selected.run(v))),
                        ..Default::default()
                    });
                "#)}
            </Code>

            <DemoShell source=include_str!("demos/switch.rs")>
                <SwitchDemo />
            </DemoShell>

            // ── Switch vs Toggle comparison ─────────────────────────

            <h2 id="comparison" class="anchor">
                "Switch vs Toggle"
                <AnchorLink href="#comparison" description="Direct link to comparison"/>
            </h2>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Feature"</TableHeaderCell>
                            <TableHeaderCell>"use_switch"</TableHeaderCell>
                            <TableHeaderCell>"use_toggle"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell>"ARIA role"</TableCell>
                            <TableCell><Code inline=true>"switch"</Code></TableCell>
                            <TableCell><Code inline=true>"button"</Code></TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Hidden input"</TableCell>
                            <TableCell>"Yes (for forms)"</TableCell>
                            <TableCell>"No"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>"Use case"</TableCell>
                            <TableCell>"Settings toggles"</TableCell>
                            <TableCell>"Toggle buttons"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            // ── Features ────────────────────────────────────────────

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Proper role=\"switch\" for accessibility"</li>
                <li>"Hidden input for form submission"</li>
                <li>"Toggle via click and keyboard (Space/Enter)"</li>
                <li>"Disabled and read-only states"</li>
                <li>"Change callbacks with new state value"</li>
            </ul>

            // ── See Also ────────────────────────────────────────────

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Toggle.materialize()>"Toggle overview"</Link></li>
                <li><Link href=crate::routes::doc::toggle::Component.materialize()>"Toggle component"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_switch & use_toggle", link: "#use-switch" },
                Toc::Leaf { title: "use_switch Input", link: "#switch-input" },
                Toc::Leaf { title: "use_switch Return", link: "#switch-return" },
                Toc::Leaf { title: "Demo", link: "#switch-demo" },
                Toc::Leaf { title: "Comparison", link: "#comparison" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
