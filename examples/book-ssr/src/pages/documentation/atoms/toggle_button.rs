use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    toggle_button::ToggleButtonAtomDemo, toggle_button_group::ToggleButtonGroupAtomDemo,
};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomToggleButton() -> impl IntoView {
    let hooks = routes::doc::toggle_button::Hook.materialize();

    view! {
        <DocPage title="Toggle Button Atoms">
            <p>
                <AnchorLink href="#togglebutton"><Code inline=true>"ToggleButton"</Code></AnchorLink>" renders an unstyled "
                <Code inline=true>"<button>"</Code>" that stays pressed until it is pressed again; "
                <AnchorLink href="#togglebuttongroup"><Code inline=true>"ToggleButtonGroup"</Code></AnchorLink>
                " groups toggle buttons with a shared selection. See the "
                <Link href=routes::doc::ToggleButton.materialize()>"Toggle Button overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <ul>
                    <li>
                        <Code inline=true>"ToggleButton"</Code>": "
                        <Link href=format!("{hooks}#use-toggle-button")><Code inline=true>"use_toggle_button"</Code></Link>
                        " with "<Link href=format!("{}#use-toggle-state", routes::doc::checkbox::Hook.materialize())>
                        <Code inline=true>"use_toggle_state"</Code></Link>" (inside a group: "
                        <Link href=format!("{hooks}#use-toggle-button-group-item")><Code inline=true>"use_toggle_button_group_item"</Code></Link>
                        "), and "<Link href=routes::doc::button::Hook.materialize()><Code inline=true>"use_button"</Code></Link>"."
                    </li>
                    <li>
                        <Code inline=true>"ToggleButtonGroup"</Code>": "
                        <Link href=format!("{hooks}#use-toggle-group-state")><Code inline=true>"use_toggle_group_state"</Code></Link>
                        " and "<Link href=format!("{hooks}#use-toggle-button-group")><Code inline=true>"use_toggle_button_group"</Code></Link>"."
                    </li>
                </ul>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use std::collections::HashSet;

                        use leptonic::{
                            atoms::toggle_button::{ToggleButton, ToggleButtonGroup},
                            hooks::Key,
                        };
                        use leptos::prelude::*;

                        let muted = RwSignal::new(false);
                        let alignment = RwSignal::new(HashSet::from([Key::from("left")]));

                        view! {
                            <ToggleButton is_selected=muted set_selected=muted classes="my-toggle">"Mute"</ToggleButton>

                            <ToggleButtonGroup aria_label="Text alignment" selected_keys=alignment set_selected_keys=alignment>
                                <ToggleButton value="left" classes="my-toggle">"Left"</ToggleButton>
                                <ToggleButton value="center" classes="my-toggle">"Center"</ToggleButton>
                                <ToggleButton value="right" classes="my-toggle">"Right"</ToggleButton>
                            </ToggleButtonGroup>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo
                    description="Mute toggle button with a disabled toggle"
                    source=include_str!("demos/toggle_button.rs")
                >
                    <ToggleButtonAtomDemo/>
                </Demo>

                <p>"A single-selection group that can\u{2019}t be emptied, bound to app state:"</p>
                <Demo
                    description="Single-selection view switcher group with a disabled toggle"
                    source=include_str!("demos/toggle_button_group.rs")
                >
                    <ToggleButtonGroupAtomDemo/>
                </Demo>
            </Section>

            <Section title="ToggleButton">
                <p>
                    "A "<Code inline=true>"<button>"</Code>" with "<Code inline=true>"aria-pressed"</Code>
                    ". Alone, it holds its own selection; inside a "<Code inline=true>"ToggleButtonGroup"</Code>
                    ", the group does, and the button needs a "<Code inline=true>"value"</Code>"."
                </p>

                <Section title="Props" id="toggle-button-props">
                    <ApiTable kind=ApiKind::Props of="ToggleButton">
                        <ApiRow name="value" ty="Option<Key>" default="None">
                            "The button\u{2019}s key in its "<Code inline=true>"ToggleButtonGroup"</Code>"; required there. "
                            "The group then holds the selection: "<Code inline=true>"default_selected"</Code>", "
                            <Code inline=true>"on_change"</Code>", "<Code inline=true>"is_selected"</Code>" and "
                            <Code inline=true>"set_selected"</Code>" don\u{2019}t apply."
                        </ApiRow>
                        <ApiRow name="default_selected" ty="bool" default="false">
                            "Whether the button starts pressed. Ignored with "<Code inline=true>"is_selected"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_selected" ty="Option<Signal<bool>>" default="None">
                            "Binds the selection to app state: a value or any signal. Without "<Code inline=true>"set_selected"</Code>
                            ", the button can\u{2019}t change it."
                        </ApiRow>
                        <ApiRow name="set_selected" ty="Option<Out<bool>>" default="None">
                            "Receives the new selection: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure or "<Code inline=true>"Callback"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the button is selected or deselected."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the button."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "The accessible name, for a button without text, such as an icon button. Keep it the same in both "
                            "states: the pressed state is announced on its own."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<button>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"The button\u{2019}s content. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ToggleButtonGroup">
                <p>
                    "A "<Code inline=true>"<div>"</Code>" around toggle buttons with a "<Code inline=true>"value"</Code>
                    " each. It is a toolbar: one tab stop, with the arrow keys moving focus between the buttons. With single "
                    "selection, it is a "<Code inline=true>"radiogroup"</Code>" of buttons with "<Code inline=true>"role=\"radio\""</Code>
                    " and "<Code inline=true>"aria-checked"</Code>". Give it a name with "<Code inline=true>"aria_label"</Code>" or "
                    <Code inline=true>"aria_labelledby"</Code>"."
                </p>

                <Section title="Props" id="toggle-button-group-props">
                    <ApiTable kind=ApiKind::Props of="ToggleButtonGroup">
                        <ApiRow name="selection_mode" ty="ToggleGroupSelectionMode" default="Single">
                            <Code inline=true>"Single"</Code>": at most one button is selected. "<Code inline=true>"Multiple"</Code>": any number."
                        </ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="bool" default="false">
                            "Keeps at least one button selected: the last selected button can\u{2019}t be deselected."
                        </ApiRow>
                        <ApiRow name="default_selected_keys" ty="HashSet<Key>" default="HashSet::new()">
                            "The initially selected buttons. Ignored with "<Code inline=true>"selected_keys"</Code>"."
                        </ApiRow>
                        <ApiRow name="selected_keys" ty="Option<Signal<HashSet<Key>>>" default="None">
                            "Binds the selection to app state: a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_selected_keys" ty="Option<Out<HashSet<Key>>>" default="None">
                            "Receives the new selection: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure or "<Code inline=true>"Callback"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<HashSet<Key>>>" default="None">
                            "Called with the selected buttons when they change."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables all buttons."</ApiRow>
                        <ApiRow name="orientation" ty="Orientation" default="Horizontal">
                            "Which arrow keys move focus: "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>", or "
                            <Keys keys="ArrowUp"/>" and "<Keys keys="ArrowDown"/>"."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The group\u{2019}s accessible name."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The ids of the elements naming the group."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The toggle buttons. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <Section title="ToggleButton" id="toggle-button-data-attributes">
                    <p>"Set to "<Code inline=true>"true"</Code>" on the "<Code inline=true>"<button>"</Code>" while the state applies, absent otherwise:"</p>
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-selected" ty="true">"The button is selected (pressed)."</ApiRow>
                        <ApiRow name="data-pressed" ty="true">"The button is being pressed right now."</ApiRow>
                        <ApiRow name="data-hovered" ty="true">"A mouse or pen is over the button."</ApiRow>
                        <ApiRow name="data-focused" ty="true">"The button has focus."</ApiRow>
                        <ApiRow name="data-focus-visible" ty="true">"The button has keyboard focus and should show a focus ring."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The button, or its group, is disabled."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="ToggleButtonGroup" id="toggle-button-group-data-attributes">
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-orientation" ty="horizontal | vertical">"The group\u{2019}s orientation."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The group is disabled (absent otherwise)."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. Pass "<Code inline=true>"classes"</Code>" and target the state with the data "
                    "attributes:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-toggle { border: 1px solid var(--border); background: var(--surface); }
                        .my-toggle[data-hovered] { border-color: var(--accent); }
                        .my-toggle[data-selected] { background: var(--accent); }
                        .my-toggle[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                        .my-toggle[data-disabled] { opacity: 0.5; }
                    ")}
                </Code>
            </Section>

            <Section title="Composition">
                <p>
                    <Code inline=true>"ToggleButtonGroup"</Code>" hands its state to the buttons inside it through the "
                    <Code inline=true>"ToggleButtonGroupCtx"</Code>" context; a "<Code inline=true>"ToggleButton"</Code>
                    " anywhere below the group joins it. A multiple-selection group inside a "
                    <Link href=routes::doc::toolbar::Atom.materialize()>"Toolbar"</Link>" becomes a group of that toolbar: the "
                    "toolbar\u{2019}s arrow keys move through its buttons too."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button overview"</Link></li>
                <li><Link href=routes::doc::toggle_button::Hook.materialize()>"Toggle Button Hooks"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button Atom"</Link></li>
                <li><Link href=routes::doc::toolbar::Atom.materialize()>"Toolbar Atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
