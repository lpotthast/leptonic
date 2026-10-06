use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{select_basic::SelectBasicDemo, select_optional::SelectOptionalDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageSelect() -> impl IntoView {
    view! {
        <DocPage title="Select components">
            <p>
                "The themed "<Code inline=true>"Select"</Code>", "<Code inline=true>"OptionalSelect"</Code>" and "
                <Code inline=true>"Multiselect"</Code>" let you choose from predefined options in a searchable dropdown. "
                "See the "<Link href=routes::doc::Select.materialize()>"Select overview"</Link>" for concept guidance."
            </p>

            <Demo description="Select and Multiselect with fruit options" source=include_str!("demos/select_basic.rs")>
                <SelectBasicDemo/>
            </Demo>

            <Section title="Options">
                <p>
                    "Options can be of any type implementing "<Code inline=true>"Clone"</Code>", "
                    <Code inline=true>"PartialEq"</Code>", "<Code inline=true>"Display"</Code>", "
                    <Code inline=true>"Send"</Code>" and "<Code inline=true>"Sync"</Code>" ("
                    <Code inline=true>"Multiselect"</Code>" additionally requires "<Code inline=true>"Ord"</Code>
                    ", as it keeps its selection sorted). The "<Code inline=true>"Display"</Code>
                    " text identifies an option (it is also the value submitted with a form), "
                    "so it must be unique among the options."
                </p>
                <p>
                    "Two callbacks turn an option into text: "<Code inline=true>"search_text_provider"</Code>
                    " returns the text the search filters by (also shown in the trigger of a single select), and "
                    <Code inline=true>"render_option"</Code>
                    " returns the view shown for the option. Both can differ from the "<Code inline=true>"Display"</Code>
                    " text, as the "<a href="#optional-selection">"optional selection"</a>" demo shows."
                </p>
            </Section>

            <Section title="Props">
                <Section title="Select">
                    <ApiTable kind=ApiKind::Props of="components::select::Select">
                        <ApiRow name="options" ty="Signal<Vec<O>>">"The options to choose from. Required."</ApiRow>
                        <ApiRow name="selected" ty="Signal<O>">"The selected option. Required."</ApiRow>
                        <ApiRow name="set_selected" ty="Out<O>">"Receives the option the user selects. Required."</ApiRow>
                        <ApiRow name="search_text_provider" ty="Callback<O, String>">
                            "The text the search input filters by. "<Code inline=true>"Select"</Code>" and "
                            <Code inline=true>"OptionalSelect"</Code>" show it in the trigger. Required."
                        </ApiRow>
                        <ApiRow name="render_option" ty="ViewCallback<O>">
                            "Renders an option in the dropdown ("<Code inline=true>"Multiselect"</Code>": also its chips). Required."
                        </ApiRow>
                        <ApiRow name="search_filter_provider" ty="Option<Callback<(String, Vec<O>), Vec<O>>>" default="None">
                            "Replaces the default filter, which keeps the options whose search text contains the "
                            "search input, ignoring case."
                        </ApiRow>
                        <ApiRow name="autofocus_search" ty="Option<Signal<bool>>" default="None">
                            "Whether the search input takes focus when the dropdown opens. "
                            "Without a value, it does so on desktop devices only."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="OptionalSelect">
                    <p>"Like "<Code inline=true>"Select"</Code>", but the selection is optional:"</p>
                    <ApiTable kind=ApiKind::Props of="OptionalSelect">
                        <ApiRow name="options" ty="Signal<Vec<O>>">"The options to choose from. Required."</ApiRow>
                        <ApiRow name="selected" ty="Signal<Option<O>>">"The selected option, if any. Required."</ApiRow>
                        <ApiRow name="set_selected" ty="Out<Option<O>>">
                            "Receives the selected option, or "<Code inline=true>"None"</Code>" when the user deselects it. Required."
                        </ApiRow>
                        <ApiRow name="allow_deselect" ty="Signal<bool>">
                            "Whether the trigger shows a button to clear the selection. Required."
                        </ApiRow>
                        <ApiRow name="search_text_provider" ty="Callback<O, String>">
                            "The text the search input filters by. Required."
                        </ApiRow>
                        <ApiRow name="render_option" ty="ViewCallback<O>">"Renders an option. Required."</ApiRow>
                        <ApiRow name="search_filter_provider" ty="Option<Callback<(String, Vec<O>), Vec<O>>>" default="None">
                            "Replaces the default filter, as for "<Code inline=true>"Select"</Code>"."
                        </ApiRow>
                        <ApiRow name="autofocus_search" ty="Option<Signal<bool>>" default="None">
                            "Whether the search input takes focus when the dropdown opens, as for "
                            <Code inline=true>"Select"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Multiselect">
                    <p>"Like "<Code inline=true>"Select"</Code>", but selects several options:"</p>
                    <ApiTable kind=ApiKind::Props of="Multiselect">
                        <ApiRow name="options" ty="Signal<Vec<O>>">"The options to choose from. Required."</ApiRow>
                        <ApiRow name="selected" ty="Signal<Vec<O>>">"The selected options. Required."</ApiRow>
                        <ApiRow name="set_selected" ty="Out<Vec<O>>">
                            "Receives the selected options, sorted. Required."
                        </ApiRow>
                        <ApiRow name="max" ty="u64" default="u64::MAX">
                            "The maximum number of selected options. The selection is sorted and cut off after "
                            <Code inline=true>"max"</Code>" options."
                        </ApiRow>
                        <ApiRow name="search_text_provider" ty="Callback<O, String>">
                            "The text the search input filters by. Required."
                        </ApiRow>
                        <ApiRow name="render_option" ty="ViewCallback<O>">"Renders an option in the dropdown and as a chip in the trigger. Required."</ApiRow>
                        <ApiRow name="search_filter_provider" ty="Option<Callback<(String, Vec<O>), Vec<O>>>" default="None">
                            "Replaces the default filter, as for "<Code inline=true>"Select"</Code>"."
                        </ApiRow>
                        <ApiRow name="autofocus_search" ty="Option<Signal<bool>>" default="None">
                            "Whether the search input takes focus when the dropdown opens, as for "
                            <Code inline=true>"Select"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Multiple selection">
                <p>
                    "A "<Code inline=true>"Multiselect"</Code>" shows each selected option as a chip in the trigger. "
                    "Dismiss a chip to deselect its option. Limit the selection with the "<Code inline=true>"max"</Code>
                    " prop: the second select in the demo above accepts at most two fruits."
                </p>
            </Section>

            <Section title="Optional selection">
                <p>
                    "An "<Code inline=true>"OptionalSelect"</Code>" stores its selection in an "<Code inline=true>"Option"</Code>
                    ", so it can start without a value. With "<Code inline=true>"allow_deselect"</Code>
                    ", the user can clear the selection again."
                </p>
                <p>
                    "The demo uses a struct as option type. Its "<Code inline=true>"Display"</Code>" text includes the id, "
                    "which keeps users with the same name apart, while the dropdown shows and searches only the name."
                </p>

                <Demo
                    description="Optional select with deselection and struct options"
                    source=include_str!("demos/select_optional.rs")
                >
                    <SelectOptionalDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Tab / Shift + Tab">"Move focus to the next or previous select."</KeyRow>
                    <KeyRow keys="Enter / Space">"Open the dropdown, or select the focused option."</KeyRow>
                    <KeyRow keys="ArrowDown / ArrowUp">
                        "Open the dropdown, focusing the first or last option. In the open dropdown, move between options."
                    </KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowRight">
                        "Select the previous or next option without opening the dropdown ("<Code inline=true>"Select"</Code>
                        " and "<Code inline=true>"OptionalSelect"</Code>" only)."
                    </KeyRow>
                    <KeyRow keys="Escape">"Close the dropdown."</KeyRow>
                </KeyboardTable>
                <p>
                    "When the dropdown opens on a desktop device, the search input receives focus, so you can type right "
                    "away. When the dropdown closes, focus returns to the select."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the selects to your design:"</p>
                <CssVariables prefix="--select-" scss=theme_scss!("select")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Select.materialize()>"Select overview"</Link></li>
                <li><Link href=routes::doc::select::Hook.materialize()>"use_select"</Link></li>
                <li><Link href=routes::doc::Listbox.materialize()>"Listbox"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
