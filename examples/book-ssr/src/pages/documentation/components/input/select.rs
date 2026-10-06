use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    select_basic::SelectBasicDemo, select_multiple::SelectMultipleDemo, select_optional::SelectOptionalDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageSelect() -> impl IntoView {
    view! {
        <DocPage title="Select Components">
            <p>
                "The themed "<Code inline=true>"Select"</Code>", "<Code inline=true>"OptionalSelect"</Code>" and "
                <Code inline=true>"Multiselect"</Code>" let users choose from options of your own type in a popover with "
                "a search input. See the "<Link href=routes::doc::Select.materialize()>"Select overview"</Link>
                " for concept guidance and keyboard interaction."
            </p>

            <Demo description="Fruit select with a label and a disabled toggle" source=include_str!("demos/select_basic.rs")>
                <SelectBasicDemo/>
            </Demo>

            <Section title="Select">
                <p>"Exactly one option is selected at all times."</p>
                <Section title="Props" id="select-props">
                    <ApiTable kind=ApiKind::Props of="components::select::Select">
                        <ApiRow name="options" ty="Signal<Vec<O>>">"The options to choose from. Required."</ApiRow>
                        <ApiRow name="selected" ty="Signal<O>">"The selected option. Required."</ApiRow>
                        <ApiRow name="set_selected" ty="Out<O>">"Receives the option the user selects. Required."</ApiRow>
                        <ApiRow name="search_text_provider" ty="Callback<O, String>">
                            "The text the search input filters by, also shown in the trigger. Required."
                        </ApiRow>
                        <ApiRow name="render_option" ty="ViewCallback<O>">"Renders an option in the popover. Required."</ApiRow>
                        <ApiRow name="label" ty="MaybeProp<String>" default="None">
                            "A visible label above the select, naming its trigger."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the select when there is no visible "<Code inline=true>"label"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the select."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The name of the hidden form element holding the selection; it submits the "
                            <Code inline=true>"Display"</Code>" text of the selected options."
                        </ApiRow>
                        <ApiRow name="search_filter_provider" ty="Option<Callback<(String, Vec<O>), Vec<O>>>" default="None">
                            "Replaces the default filter, which keeps the options whose search text contains the search input, "
                            "ignoring case."
                        </ApiRow>
                        <ApiRow name="autofocus_search" ty="Option<Signal<bool>>" default="None">
                            "Whether the search input takes focus when the popover opens. Without a value, it does so on desktop "
                            "devices only."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="OptionalSelect">
                <p>
                    "Stores its selection in an "<Code inline=true>"Option"</Code>", so it can start without a value. "
                    "With "<Code inline=true>"allow_deselect"</Code>", a \u{201c}Clear selection\u{201d} button next to the "
                    "trigger clears the selection again. The demo uses a struct as option type: its "<Code inline=true>"Display"</Code>" text includes "
                    "the id, which keeps users with the same name apart, while the popover shows and searches only the name."
                </p>

                <Demo description="Optional select with deselection and struct options" source=include_str!("demos/select_optional.rs")>
                    <SelectOptionalDemo/>
                </Demo>

                <Section title="Props" id="optionalselect-props">
                    <ApiTable kind=ApiKind::Props of="OptionalSelect">
                        <ApiRow name="options" ty="Signal<Vec<O>>">"The options to choose from. Required."</ApiRow>
                        <ApiRow name="selected" ty="Signal<Option<O>>">"The selected option, if any. Required."</ApiRow>
                        <ApiRow name="set_selected" ty="Out<Option<O>>">
                            "Receives the selected option, or "<Code inline=true>"None"</Code>" when the user clears it. Required."
                        </ApiRow>
                        <ApiRow name="allow_deselect" ty="Signal<bool>">
                            "Whether a button next to the trigger clears the selection. Required."
                        </ApiRow>
                        <ApiRow name="search_text_provider" ty="Callback<O, String>">
                            "The text the search input filters by, also shown in the trigger. Required."
                        </ApiRow>
                        <ApiRow name="render_option" ty="ViewCallback<O>">"Renders an option in the popover. Required."</ApiRow>
                        <ApiRow name="label" ty="MaybeProp<String>" default="None">
                            "A visible label above the select, naming its trigger."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the select when there is no visible "<Code inline=true>"label"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the select."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The name of the hidden form element holding the selection; it submits the "
                            <Code inline=true>"Display"</Code>" text of the selected options."
                        </ApiRow>
                        <ApiRow name="search_filter_provider" ty="Option<Callback<(String, Vec<O>), Vec<O>>>" default="None">
                            "Replaces the default filter, which keeps the options whose search text contains the search input, "
                            "ignoring case."
                        </ApiRow>
                        <ApiRow name="autofocus_search" ty="Option<Signal<bool>>" default="None">
                            "Whether the search input takes focus when the popover opens. Without a value, it does so on desktop "
                            "devices only."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Multiselect">
                <p>
                    "Selects several options and shows each as a chip next to the trigger. "<Code inline=true>"max"</Code>
                    " limits the selection: the demo accepts at most two toppings."
                </p>

                <Demo description="Multiselect with at most two selected options" source=include_str!("demos/select_multiple.rs")>
                    <SelectMultipleDemo/>
                </Demo>

                <Section title="Props" id="multiselect-props">
                    <ApiTable kind=ApiKind::Props of="Multiselect">
                        <ApiRow name="options" ty="Signal<Vec<O>>">"The options to choose from. Required."</ApiRow>
                        <ApiRow name="selected" ty="Signal<Vec<O>>">"The selected options. Required."</ApiRow>
                        <ApiRow name="set_selected" ty="Out<Vec<O>>">"Receives the selected options, sorted. Required."</ApiRow>
                        <ApiRow name="max" ty="u64" default="u64::MAX">
                            "The maximum number of selected options. The selection is sorted and cut off after "
                            <Code inline=true>"max"</Code>" options."
                        </ApiRow>
                        <ApiRow name="search_text_provider" ty="Callback<O, String>">
                            "The text the search input filters by. Required."
                        </ApiRow>
                        <ApiRow name="render_option" ty="ViewCallback<O>">
                            "Renders an option in the popover and as a chip next to the trigger. Required."
                        </ApiRow>
                        <ApiRow name="label" ty="MaybeProp<String>" default="None">
                            "A visible label above the select, naming its trigger."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the select when there is no visible "<Code inline=true>"label"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the select."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The name of the hidden form element holding the selection; it submits the "
                            <Code inline=true>"Display"</Code>" text of the selected options."
                        </ApiRow>
                        <ApiRow name="search_filter_provider" ty="Option<Callback<(String, Vec<O>), Vec<O>>>" default="None">
                            "Replaces the default filter, which keeps the options whose search text contains the search input, "
                            "ignoring case."
                        </ApiRow>
                        <ApiRow name="autofocus_search" ty="Option<Signal<bool>>" default="None">
                            "Whether the search input takes focus when the popover opens. Without a value, it does so on desktop "
                            "devices only."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Options">
                <p>
                    "Options can be of any type implementing "<Code inline=true>"Clone"</Code>", "
                    <Code inline=true>"PartialEq"</Code>", "<Code inline=true>"Display"</Code>", "
                    <Code inline=true>"Send"</Code>" and "<Code inline=true>"Sync"</Code>" ("
                    <Code inline=true>"Multiselect"</Code>" also requires "<Code inline=true>"Ord"</Code>
                    ", as it keeps its selection sorted). The "<Code inline=true>"Display"</Code>
                    " text identifies an option (it is also the value submitted with a form), so it must be unique among "
                    "the options."
                </p>
                <p>
                    "Two callbacks turn an option into text: "<Code inline=true>"search_text_provider"</Code>
                    " returns the text the search filters by (also shown in the trigger of a single select), and "
                    <Code inline=true>"render_option"</Code>" returns the view shown for the option. Both can differ from "
                    "the "<Code inline=true>"Display"</Code>" text, as the "
                    <AnchorLink href="#optionalselect">"OptionalSelect"</AnchorLink>" demo shows."
                </p>
                <p>
                    "Give every select a name: a visible "<Code inline=true>"label"</Code>", or an "
                    <Code inline=true>"aria_label"</Code>" where the surrounding content makes its purpose clear."
                </p>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The components are built on the "<Link href=routes::doc::select::Atom.materialize()>"Select Atoms"</Link>
                    " and share their keyboard and screen reader support. Buttons can\u{2019}t contain buttons, so a "
                    <Code inline=true>"Multiselect"</Code>"\u{2019}s chips and an "<Code inline=true>"OptionalSelect"</Code>
                    "\u{2019}s clear button sit next to the trigger, each a button of its own: a chip\u{2019}s button is named "
                    "\u{201c}Remove\u{201d} and the option, and the trigger\u{2019}s name includes the current selection."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the selects to your design:"</p>
                <CssVariables prefix="--select-" scss=theme_scss!("select")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Select.materialize()>"Select overview"</Link></li>
                <li><Link href=routes::doc::select::Hook.materialize()>"Select Hooks"</Link></li>
                <li><Link href=routes::doc::select::Atom.materialize()>"Select Atoms"</Link></li>
                <li><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
