use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::radio::RadioDemo;

#[component]
pub fn PageUseRadioHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-radio-group" class="anchor">
                "use_radio & use_radio_group"
                <AnchorLink href="#use-radio-group" description="Direct link to article header"/>
            </h1>

            <p>
                "Hooks for creating accessible radio buttons with group management and mutual exclusion. "
                "See the "<Link href=crate::routes::doc::Radio.materialize()>"Radio overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useRadioGroup.html" target=LinkTarget::_Blank>
                    "useRadioGroup"
                </LinkExt>
                "."
            </p>

            // ── use_radio_group Input ───────────────────────────────

            <h2 id="group-input" class="anchor">
                "use_radio_group Input"
                <AnchorLink href="#group-input" description="Direct link to section: use_radio_group Input"/>
            </h2>

            <p><Code inline=true>"UseRadioGroupInput<T>"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"value"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<Option<T>>"</Code></TableCell>
                            <TableCell>"The current selected value (controlled)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_change"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<T>>"</Code></TableCell>
                            <TableCell>"Callback when the selection changes"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_disabled"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the group is disabled"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_read_only"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the group is read-only"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"orientation"</Code></TableCell>
                            <TableCell><Code inline=true>"Orientation"</Code></TableCell>
                            <TableCell>"The orientation of the group (Vertical or Horizontal)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            // ── use_radio_group Return ──────────────────────────────

            <h2 id="group-return" class="anchor">
                "use_radio_group Return"
                <AnchorLink href="#group-return" description="Direct link to section: use_radio_group Return"/>
            </h2>

            <p><Code inline=true>"UseRadioGroupReturn<T>"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"group_props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseRadioGroupProps"</Code></TableCell>
                            <TableCell>"ARIA attributes for the group container element"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"label_props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseRadioGroupLabelProps"</Code></TableCell>
                            <TableCell>"Props for the label element (contains generated id)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"state"</Code></TableCell>
                            <TableCell><Code inline=true>"UseRadioGroupState<T>"</Code></TableCell>
                            <TableCell>"Group state for individual radio buttons (selected_value, set_selected_value, name)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            // ── Radio Group Demo ────────────────────────────────────

            <h2 id="group-demo" class="anchor">
                "Radio Group Demo"
                <AnchorLink href="#group-demo" description="Direct link to section: Radio Group Demo"/>
            </h2>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseRadioGroupStateReturn { selected_value, set_selected } =
                        use_radio_group_state(Some("option1".to_string()));

                    let UseRadioGroupReturn { group_props, label_props, state, .. } = use_radio_group(
                        UseRadioGroupInput {
                            label: Some("Select an option".into()),
                            value: selected_value.into(),
                            orientation: Orientation::Vertical,
                            on_change: Some(Callback::new(move |value| set_selected.run(value))),
                            ..Default::default()
                        }
                    );
                "#)}
            </Code>

            <DemoShell source=include_str!("demos/radio.rs")>
                <RadioDemo />
            </DemoShell>

            // ── use_radio ───────────────────────────────────────────

            <h2 id="use-radio" class="anchor">
                "use_radio"
                <AnchorLink href="#use-radio" description="Direct link to use_radio"/>
            </h2>

            <p>"For individual radio buttons within a group:"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseRadioReturn { input_props, is_selected, .. } = use_radio(UseRadioInput {
                        value: "option1".to_string(),
                        state: group.state,
                        is_disabled: Signal::derive(|| false),
                        validation_state: ValidationState::Valid,
                        aria_label: Some("Option 1"),
                        name: Some("my-radio"),
                    });
                "#)}
            </Code>

            // ── Keyboard Navigation ─────────────────────────────────

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard"/>
            </h2>

            <p>"Radio groups support standard keyboard navigation:"</p>
            <ul>
                <li><strong>"Arrow keys"</strong> " - Move between options"</li>
                <li><strong>"Space"</strong> " - Select the focused option"</li>
                <li><strong>"Tab"</strong> " - Move focus to/from the group"</li>
            </ul>

            // ── Features ────────────────────────────────────────────

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Mutual exclusion (only one selection at a time)"</li>
                <li>"Keyboard navigation with arrow keys"</li>
                <li>"Support for horizontal and vertical orientations"</li>
                <li>"Disabled and read-only modes"</li>
                <li>"Required field validation"</li>
                <li>"ARIA role and attributes for accessibility"</li>
            </ul>

            // ── See Also ────────────────────────────────────────────

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Radio.materialize()>"Radio overview"</Link></li>
                <li><Link href=crate::routes::doc::radio::Component.materialize()>"Radio component"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_radio & use_radio_group", link: "#use-radio-group" },
                Toc::Leaf { title: "use_radio_group Input", link: "#group-input" },
                Toc::Leaf { title: "use_radio_group Return", link: "#group-return" },
                Toc::Leaf { title: "Radio Group Demo", link: "#group-demo" },
                Toc::Leaf { title: "use_radio", link: "#use-radio" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
