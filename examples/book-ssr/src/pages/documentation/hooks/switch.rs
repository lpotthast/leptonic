use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::switch::SwitchDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseSwitchHook() -> impl IntoView {
    let toggle_options = format!(
        "{}#toggleoptions",
        routes::doc::checkbox::Hook.materialize()
    );
    let toggle_return = format!(
        "{}#use-checkbox-return",
        routes::doc::checkbox::Hook.materialize()
    );
    let toggle_state = format!(
        "{}#use-toggle-state",
        routes::doc::checkbox::Hook.materialize()
    );

    view! {
        <DocPage title="Switch Hooks">
            <p>
                "The "<Code inline=true>"use_switch"</Code>" hook makes an "<Code inline=true>"<input type=\"checkbox\" role=\"switch\">"</Code>
                " inside a "<Code inline=true>"<label>"</Code>" an accessible on/off switch; you draw the track next to the "
                "visually hidden input. "<Code inline=true>"use_toggle"</Code>" is the toggle behavior it is built on. See the "
                <Link href=routes::doc::Switch.materialize()>"Switch overview"</Link>" for concept guidance."
            </p>
            <ReactAria hook="useSwitch"/>

            <Section title="use_switch">
                <p>
                    "A switch submits, resets and validates like a "<Link href=routes::doc::checkbox::Hook.materialize()>"checkbox"</Link>
                    ", and shares its settings and return value. Its state comes from "
                    <Link href=toggle_state.clone()>"use_toggle_state"</Link>"."
                </p>

                <Section title="Input" id="use-switch-input">
                    <p>
                        <Code inline=true>"UseSwitchInput"</Code>" is an alias of "<Code inline=true>"UseToggleInput"</Code>
                        ". Create it with "<Code inline=true>"UseSwitchInput::new(state)"</Code>"."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseToggleInput">
                        <ApiRow name="state" ty="ToggleState">"Whether the switch is on, from "<Code inline=true>"use_toggle_state"</Code>"."</ApiRow>
                        <ApiRow name="options" ty="ToggleOptions" default="ToggleOptions::default()">
                            "Disabled, read-only, form and labelling settings: see "<Link href=toggle_options>"ToggleOptions"</Link>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-switch-return">
                    <p>
                        <Code inline=true>"UseSwitchReturn"</Code>" is an alias of "<Link href=toggle_return>"UseToggleReturn"</Link>
                        ": "<Code inline=true>"label_props"</Code>" for the label, "<Code inline=true>"input_props"</Code>
                        " for the input (with "<Code inline=true>"role=\"switch\""</Code>"), and the state signals "
                        <Code inline=true>"is_selected"</Code>", "<Code inline=true>"is_pressed"</Code>", "
                        <Code inline=true>"is_focus_visible"</Code>", ... to style the track."
                    </p>
                </Section>

                <Section title="Example" id="use-switch-example">
                    <p>
                        "The hook renders no state. The demo hides the input with "<Code inline=true>"visually_hidden_styles()"</Code>
                        " and puts the state signals on the label as data attributes, which its stylesheet selects on."
                    </p>
                    <Demo
                        description="Notifications switch drawn as a track, with disabled and read-only toggles"
                        source=include_str!("demos/switch.rs")
                        source_open=true
                    >
                        <SwitchDemo/>
                    </Demo>
                </Section>
            </Section>

            <Section title="use_toggle">
                <p>
                    "The behavior shared by checkboxes and switches, on an "<Code inline=true>"<input type=\"checkbox\">"</Code>
                    " without a role: pressing the input or its label toggles the state, plus validation, form reset and "
                    "focus-ring tracking. Use it for a toggle that is neither a checkbox nor a switch. It takes the same "
                    <Code inline=true>"UseToggleInput"</Code>" and returns the same "<Code inline=true>"UseToggleReturn"</Code>
                    " as "<Code inline=true>"use_switch"</Code>"."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let state = use_toggle_state(UseToggleStateInput::default());
                        let toggle = use_toggle(UseToggleInput::new(state));
                        let (label_attrs, label_styles) = toggle.label_props.into_parts();
                        let (input_attrs, input_styles) = toggle.input_props.into_parts();

                        view! {
                            <label {..label_attrs} style=label_styles>
                                <input {..input_attrs} style=input_styles/>
                                "Favorite"
                            </label>
                        }
                    "#)}
                </Code>
                <p>
                    "For a "<Code inline=true>"<button>"</Code>" with "<Code inline=true>"aria-pressed"</Code>", use "
                    <Link href=routes::doc::toggle_button::Hook.materialize()>"use_toggle_button"</Link>" instead."
                </p>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Tab">"Moves focus to the switch."</KeyRow>
                    <KeyRow keys="Space">"Turns the switch on or off."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Switch.materialize()>"Switch overview"</Link></li>
                <li><Link href=routes::doc::switch::Atom.materialize()>"Switch atom"</Link></li>
                <li><Link href=routes::doc::switch::Component.materialize()>"Switch component"</Link></li>
                <li><Link href=routes::doc::checkbox::Hook.materialize()>"Checkbox hooks"</Link></li>
                <li><Link href=routes::doc::toggle_button::Hook.materialize()>"Toggle button hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
