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
    view! {
        <DocPage title="Toggle Button Atoms">
            <p>
                "The toggle button atoms render unstyled toggle buttons: "<Code inline=true>"ToggleButton"</Code>" is a "
                <Code inline=true>"<button>"</Code>" that is pressed or not, "<Code inline=true>"ToggleButtonGroup"</Code>
                " groups them with a shared selection. You style them through the data attributes they render. See the "
                <Link href=routes::doc::ToggleButton.materialize()>"Toggle Button overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"ToggleButton"</Code>" calls "<Link href=routes::doc::toggle_button::Hook.materialize()>
                    <Code inline=true>"use_toggle_button"</Code></Link>" (inside a group: "<Code inline=true>"use_toggle_button_group_item"</Code>
                    ") and "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>". "
                    <Code inline=true>"ToggleButtonGroup"</Code>" calls "<Code inline=true>"use_toggle_group_state"</Code>" and "
                    <Code inline=true>"use_toggle_button_group"</Code>"."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::toggle_button::{ToggleButton, ToggleButtonGroup};

                        let muted = RwSignal::new(false);

                        view! {
                            <ToggleButton state=muted classes="my-toggle">"Mute"</ToggleButton>

                            <ToggleButtonGroup aria_label="Text alignment" default_selected_keys=HashSet::from([Key::from("left")])>
                                <ToggleButton value="left" classes="my-toggle">"Left"</ToggleButton>
                                <ToggleButton value="center" classes="my-toggle">"Center"</ToggleButton>
                                <ToggleButton value="right" classes="my-toggle">"Right"</ToggleButton>
                            </ToggleButtonGroup>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="ToggleButton">
                <Demo
                    description="Mute toggle button with a disabled toggle"
                    source=include_str!("demos/toggle_button.rs")
                >
                    <ToggleButtonAtomDemo/>
                </Demo>

                <Section title="Props" id="toggle-button-props">
                    <ApiTable kind=ApiKind::Props of="ToggleButton">
                        <ApiRow name="value" ty="Option<Key>" default="None">
                            "The button\u{2019}s key in its "<Code inline=true>"ToggleButtonGroup"</Code>" (required there). "
                            "The group then holds the selection: "<Code inline=true>"default_selected"</Code>", "
                            <Code inline=true>"on_change"</Code>" and "<Code inline=true>"state"</Code>" don\u{2019}t apply."
                        </ApiRow>
                        <ApiRow name="default_selected" ty="bool" default="false">"Whether the button starts pressed."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the button is pressed or released. Also called when "<Code inline=true>"state"</Code>
                            " is given."
                        </ApiRow>
                        <ApiRow name="state" ty="Option<ToggleState>" default="None">
                            "Binds the button to your state, e.g. "<Code inline=true>"state=rw_signal"</Code>". Replaces "
                            <Code inline=true>"default_selected"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the button."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a button without text (e.g. an icon)."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<button>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"The button\u{2019}s content."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Data Attributes" id="toggle-button-data-attributes">
                    <p>"Set to "<Code inline=true>"true"</Code>" on the "<Code inline=true>"<button>"</Code>" while the state applies:"</p>
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-selected" ty="true">"The button is pressed (selected)."</ApiRow>
                        <ApiRow name="data-pressed" ty="true">"The button is being pressed right now."</ApiRow>
                        <ApiRow name="data-hovered" ty="true">"A mouse or pen is over the button."</ApiRow>
                        <ApiRow name="data-focused" ty="true">"The button has focus."</ApiRow>
                        <ApiRow name="data-focus-visible" ty="true">"The button has keyboard focus: show a focus ring."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The button (or its group) is disabled."</ApiRow>
                    </ApiTable>
                    <p>
                        "The button also carries "<Code inline=true>"aria-pressed"</Code>" (in a single-selection group: "
                        <Code inline=true>"role=\"radio\""</Code>" and "<Code inline=true>"aria-checked"</Code>")."
                    </p>
                </Section>
            </Section>

            <Section title="ToggleButtonGroup">
                <p>
                    "A "<Code inline=true>"<div>"</Code>" around toggle buttons with a "<Code inline=true>"value"</Code>
                    " each. It is a toolbar: one tab stop, with the arrow keys moving focus between the buttons. With single "
                    "selection, it is a "<Code inline=true>"radiogroup"</Code>"."
                </p>
                <Demo
                    description="Single-selection view switcher group with a disabled toggle"
                    source=include_str!("demos/toggle_button_group.rs")
                >
                    <ToggleButtonGroupAtomDemo/>
                </Demo>

                <Section title="Props" id="toggle-button-group-props">
                    <ApiTable kind=ApiKind::Props of="ToggleButtonGroup">
                        <ApiRow name="selection_mode" ty="ToggleGroupSelectionMode" default="Single">"One button at a time, or any number."</ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="bool" default="false">"Keeps at least one button selected."</ApiRow>
                        <ApiRow name="default_selected_keys" ty="HashSet<Key>" default="empty">"The initially selected buttons."</ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<HashSet<Key>>>" default="None">
                            "Called with the selected buttons when they change."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables all buttons."</ApiRow>
                        <ApiRow name="orientation" ty="Orientation" default="Horizontal">"The axis of the arrow keys."</ApiRow>
                        <ApiRow name="aria_label, aria_labelledby" ty="MaybeProp<String>, Option<String>" default="None">"The group\u{2019}s accessible name."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The toggle buttons."</ApiRow>
                    </ApiTable>
                    <p>
                        "See "<Link href=format!("{}#use-toggle-group-state", routes::doc::toggle_button::Hook.materialize())>
                        "use_toggle_group_state"</Link>" for how these settings behave."
                    </p>
                </Section>

                <Section title="Data Attributes" id="toggle-button-group-data-attributes">
                    <ApiTable kind=ApiKind::DataAttributes>
                        <ApiRow name="data-orientation" ty="horizontal | vertical">"The group\u{2019}s orientation."</ApiRow>
                        <ApiRow name="data-disabled" ty="true">"The group is disabled (absent otherwise)."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-toggle { padding: 0.5em 1em; border: 1px solid gray; border-radius: 8px; background: none; }
                        .my-toggle[data-selected] { background: royalblue; color: white; }
                        .my-toggle[data-focus-visible] { outline: 2px solid royalblue; outline-offset: 2px; }
                        .my-toggle[data-disabled] { opacity: 0.5; }
                    ")}
                </Code>
                <p>"The demos show their complete styles under \u{201c}View styles\u{201d}."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ToggleButton.materialize()>"Toggle Button overview"</Link></li>
                <li><Link href=routes::doc::toggle_button::Hook.materialize()>"Toggle button hooks"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button atom"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
