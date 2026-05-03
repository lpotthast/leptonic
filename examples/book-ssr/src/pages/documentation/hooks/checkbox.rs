use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::{checkbox_basic::CheckboxBasicDemo, checkbox_group::CheckboxGroupDemo};

#[component]
pub fn PageUseCheckboxHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use-checkbox" class="anchor">
                "use_checkbox & use_checkbox_group"
                <AnchorLink href="#use-checkbox" description="Direct link to article header"/>
            </h1>

            <p>
                "Hooks for creating accessible checkboxes with support for indeterminate state and checkbox groups. "
                "See the "<Link href=crate::routes::doc::Checkbox.materialize()>"Checkbox overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useCheckbox.html" target=LinkTarget::_Blank>
                    "useCheckbox"
                </LinkExt>
                "."
            </p>

            // ── use_checkbox Input ──────────────────────────────────

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to section: Input"/>
            </h2>

            <p><Code inline=true>"UseCheckboxInput"</Code>" fields:"</p>

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
                            <TableCell>"Whether the checkbox is selected (controlled)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_indeterminate"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the checkbox is in an indeterminate state"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_change"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<bool>>"</Code></TableCell>
                            <TableCell>"Callback when the selection changes"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_disabled"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the checkbox is disabled"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_read_only"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the checkbox is read-only"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_required"</Code></TableCell>
                            <TableCell><Code inline=true>"bool"</Code></TableCell>
                            <TableCell>"Whether the checkbox is required"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_invalid"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Signal<bool>>"</Code></TableCell>
                            <TableCell>"Controlled invalid state; overrides all other validation when set"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"validate"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<ValidateFn<bool>>"</Code></TableCell>
                            <TableCell>"Custom client-side validation function"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"validation_behavior"</Code></TableCell>
                            <TableCell><Code inline=true>"ValidationBehavior"</Code></TableCell>
                            <TableCell>"Validation behavior mode"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"default_value"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<bool>"</Code></TableCell>
                            <TableCell>"Default value to restore on form reset"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_label"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<&'static str>"</Code></TableCell>
                            <TableCell>"Accessibility label for the checkbox"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"name"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<&'static str>"</Code></TableCell>
                            <TableCell>"Name attribute for form submission"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"value"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<&'static str>"</Code></TableCell>
                            <TableCell>"Value attribute for form submission"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            // ── use_checkbox Return ─────────────────────────────────

            <h2 id="return" class="anchor">
                "Return"
                <AnchorLink href="#return" description="Direct link to section: Return"/>
            </h2>

            <p><Code inline=true>"UseCheckboxReturn"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"input_props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseCheckboxInputProps"</Code></TableCell>
                            <TableCell>"ARIA attributes and event handlers to spread onto the input element"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_selected"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the checkbox is currently selected"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_indeterminate"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the checkbox is currently indeterminate"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_pressed"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the checkbox is pressed (during interaction)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_focus_visible"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the focus ring should be visible"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_invalid"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the displayed validation is invalid"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"validation_errors"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<Vec<String>>"</Code></TableCell>
                            <TableCell>"The displayed validation error messages"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"validation_details"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<ValidityStateSnapshot>"</Code></TableCell>
                            <TableCell>"Detailed validity state (mirrors native ValidityState)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            // ── use_checkbox Demo ───────────────────────────────────

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to section: Demo"/>
            </h2>

            <DemoShell
                source=include_str!("demos/checkbox_basic.rs")
                description="Checkbox with controlled state"
            >
                <CheckboxBasicDemo />
            </DemoShell>

            // ── Checkbox Group ──────────────────────────────────────

            <h2 id="checkbox-group" class="anchor">
                "use_checkbox_group"
                <AnchorLink href="#checkbox-group" description="Direct link to checkbox group"/>
            </h2>

            <p>"Manages a group of checkboxes with shared state."</p>

            <h3 id="group-input" class="anchor">
                "Checkbox Group Input"
                <AnchorLink href="#group-input" description="Direct link to section: Checkbox Group Input"/>
            </h3>

            <p><Code inline=true>"UseCheckboxGroupInput<T>"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"Signal<HashSet<T>>"</Code></TableCell>
                            <TableCell>"The current selected values (controlled)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_change"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<HashSet<T>>>"</Code></TableCell>
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
                            <TableCell><Code inline=true>"is_required"</Code></TableCell>
                            <TableCell><Code inline=true>"bool"</Code></TableCell>
                            <TableCell>"Whether the group is required"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_invalid"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Signal<bool>>"</Code></TableCell>
                            <TableCell>"Controlled invalid state; overrides other validation when set"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"validate"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<ValidateFn<HashSet<T>>>"</Code></TableCell>
                            <TableCell>"Custom client-side validation function"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"validation_behavior"</Code></TableCell>
                            <TableCell><Code inline=true>"ValidationBehavior"</Code></TableCell>
                            <TableCell>"Validation behavior mode"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"name"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<&'static str>"</Code></TableCell>
                            <TableCell>"Name attribute for form submission"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"label"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<String>"</Code></TableCell>
                            <TableCell>"The label for the group"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"description"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<String>"</Code></TableCell>
                            <TableCell>"A description for the group"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"orientation"</Code></TableCell>
                            <TableCell><Code inline=true>"Orientation"</Code></TableCell>
                            <TableCell>"The orientation of the group (Horizontal or Vertical)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="group-return" class="anchor">
                "Checkbox Group Return"
                <AnchorLink href="#group-return" description="Direct link to section: Checkbox Group Return"/>
            </h3>

            <p><Code inline=true>"UseCheckboxGroupReturn<T>"</Code>" fields:"</p>

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
                            <TableCell><Code inline=true>"UseCheckboxGroupProps"</Code></TableCell>
                            <TableCell>"ARIA attributes for the group container element"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"label_props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseCheckboxGroupLabelProps"</Code></TableCell>
                            <TableCell>"Props for the label element (contains generated id)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"state"</Code></TableCell>
                            <TableCell><Code inline=true>"UseCheckboxGroupState<T>"</Code></TableCell>
                            <TableCell>"Group state with is_selected, add_value, remove_value, toggle_value"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"is_invalid"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell>"Whether the displayed validation is invalid"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"validation_errors"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<Vec<String>>"</Code></TableCell>
                            <TableCell>"The displayed validation error messages"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"validation_details"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<ValidityStateSnapshot>"</Code></TableCell>
                            <TableCell>"Detailed validity state (mirrors native ValidityState)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            // ── Checkbox Group Demo ─────────────────────────────────

            <h3 id="group-demo" class="anchor">
                "Checkbox Group Demo"
                <AnchorLink href="#group-demo" description="Direct link to section: Checkbox Group Demo"/>
            </h3>

            <DemoShell
                source=include_str!("demos/checkbox_group.rs")
                description="Checkbox group with multiple options"
            >
                <CheckboxGroupDemo />
            </DemoShell>

            // ── Indeterminate State ─────────────────────────────────

            <h2 id="indeterminate" class="anchor">
                "Indeterminate State"
                <AnchorLink href="#indeterminate" description="Direct link to indeterminate"/>
            </h2>

            <p>"The indeterminate state is useful for \"select all\" checkboxes:"</p>
            <ul>
                <li>"Unchecked when no children are selected"</li>
                <li>"Indeterminate when some children are selected"</li>
                <li>"Checked when all children are selected"</li>
            </ul>

            // ── Features ────────────────────────────────────────────

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Native checkbox with controlled state"</li>
                <li>"Indeterminate state support"</li>
                <li>"Disabled and read-only modes"</li>
                <li>"Required field support"</li>
                <li>"Validation state (valid/invalid)"</li>
                <li>"Group management with add/remove/toggle operations"</li>
                <li>"Horizontal and vertical orientations for groups"</li>
            </ul>

            // ── See Also ────────────────────────────────────────────

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Checkbox.materialize()>"Checkbox overview"</Link></li>
                <li><Link href=crate::routes::doc::checkbox::Component.materialize()>"Checkbox component"</Link></li>
                <li><Link href=crate::routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_checkbox", link: "#use-checkbox" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return", link: "#return" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "use_checkbox_group", link: "#checkbox-group" },
                Toc::Leaf { title: "Group Input", link: "#group-input" },
                Toc::Leaf { title: "Group Return", link: "#group-return" },
                Toc::Leaf { title: "Group Demo", link: "#group-demo" },
                Toc::Leaf { title: "Indeterminate State", link: "#indeterminate" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
