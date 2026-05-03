use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, toc::Toc},
    routes,
};

#[component]
pub fn PageInputCategory() -> impl IntoView {
    view! {
        <Article>
            <h1 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to article header"/>
            </h1>

            <p>
                "Components that accept user input \u{2014} from simple buttons and checkboxes to rich text editors and date pickers."
            </p>

            <h2 id="overview" class="anchor">
                "Overview"
                <AnchorLink href="#overview" description="Direct link to section: Overview"/>
            </h2>

            <h3 id="concepts" class="anchor">
                "Concepts"
                <AnchorLink href="#concepts" description="Direct link to section: Concepts"/>
            </h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Name"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                            <TableHeaderCell>"When to Use"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Button.materialize()>"Button"</Link></TableCell>
                            <TableCell>"Trigger actions with press interactions"</TableCell>
                            <TableCell>"User needs to perform an action (submit, delete, open)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Checkbox.materialize()>"Checkbox"</Link></TableCell>
                            <TableCell>"Toggle a boolean value on or off"</TableCell>
                            <TableCell>"One or more independent on/off choices"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link></TableCell>
                            <TableCell>"Filterable selection from a list of options"</TableCell>
                            <TableCell>"Large option set where typing to filter is valuable"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Listbox.materialize()>"Listbox"</Link></TableCell>
                            <TableCell>"Select one or more items from a scrollable list"</TableCell>
                            <TableCell>"All options should be visible at once"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Radio.materialize()>"Radio"</Link></TableCell>
                            <TableCell>"Choose one option from a mutually exclusive set"</TableCell>
                            <TableCell>"Small set of options (2\u{2013}5) where only one applies"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Select.materialize()>"Select"</Link></TableCell>
                            <TableCell>"Pick a value from a dropdown list"</TableCell>
                            <TableCell>"Space-constrained single choice from a moderate set"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Slider.materialize()>"Slider"</Link></TableCell>
                            <TableCell>"Select a numeric value within a range"</TableCell>
                            <TableCell>"Continuous or stepped numeric input (volume, price)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::TextField.materialize()>"Text Field"</Link></TableCell>
                            <TableCell>"Single-line and multi-line text input"</TableCell>
                            <TableCell>"Free-form text entry"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Toggle.materialize()>"Toggle"</Link></TableCell>
                            <TableCell>"Switch between two states"</TableCell>
                            <TableCell>"Binary setting with immediate effect"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="standalone" class="anchor">
                "Standalone"
                <AnchorLink href="#standalone" description="Direct link to section: Standalone"/>
            </h3>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Name"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Link href=routes::doc::Color.materialize()>"Color"</Link></TableCell>
                            <TableCell>"Visual color selection component"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::components::DateTime.materialize()>"Date & Time"</Link></TableCell>
                            <TableCell>"Date and time picker components"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::components::TiptapEditor.materialize()>"Tiptap Editor"</Link></TableCell>
                            <TableCell>"Rich text editor integration"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Link href=routes::doc::hooks::UseLabel.materialize()>"use_label"</Link></TableCell>
                            <TableCell>"Associate a label with a form element"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            // ── Decision Guide ──────────────────────────────────────

            <h2 id="decision-guide" class="anchor">
                "Decision Guide"
                <AnchorLink href="#decision-guide" description="Direct link to section: Decision Guide"/>
            </h2>

            <p>"Choosing between similar input controls:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell>"Choice"</TableHeaderCell>
                            <TableHeaderCell>"When to pick the first"</TableHeaderCell>
                            <TableHeaderCell>"When to pick the second"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><strong>"Button vs Link"</strong></TableCell>
                            <TableCell>"Performs an action (submit, delete, toggle)"</TableCell>
                            <TableCell>"Navigates to a URL or anchor"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><strong>"Checkbox vs Toggle"</strong></TableCell>
                            <TableCell>"Multi-select from a group, or form-submitted boolean"</TableCell>
                            <TableCell>"Binary switch with immediate effect (settings)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><strong>"Select vs Combobox"</strong></TableCell>
                            <TableCell>"Short list, user picks from fixed options"</TableCell>
                            <TableCell>"Long list, user benefits from typing to filter"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><strong>"Select vs Listbox"</strong></TableCell>
                            <TableCell>"Space-constrained, options hidden until opened"</TableCell>
                            <TableCell>"All options should be visible at once"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><strong>"Text Field vs Search Field"</strong></TableCell>
                            <TableCell>"General-purpose text input"</TableCell>
                            <TableCell>"Input specifically for search queries (has clear button)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Concepts", link: "#concepts" },
                Toc::Leaf { title: "Standalone", link: "#standalone" },
                Toc::Leaf { title: "Decision Guide", link: "#decision-guide" },
            ]
        }/>
    }
}
