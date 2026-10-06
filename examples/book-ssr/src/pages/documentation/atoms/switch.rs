use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::switch::SwitchAtomDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomSwitch() -> impl IntoView {
    view! {
        <DocPage title="Switch Atom">
            <p>
                "The "<Code inline=true>"Switch"</Code>" atom renders an unstyled switch: a "<Code inline=true>"<label>"</Code>
                " around a visually hidden "<Code inline=true>"<input type=\"checkbox\" role=\"switch\">"</Code>
                " and your children, which draw the track and the label text. You style it through the data attributes it renders. "
                "See the "<Link href=routes::doc::Switch.materialize()>"Switch overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"Switch"</Code>" calls "<Link href=routes::doc::switch::Hook.materialize()>"use_switch"</Link>", "
                    <Link href=format!("{}#use-toggle-state", routes::doc::checkbox::Hook.materialize())>"use_toggle_state"</Link>
                    " (without "<Code inline=true>"state"</Code>") and "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>"."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::switch::Switch;

                        let wifi = RwSignal::new(true);

                        view! {
                            <Switch state=wifi classes="my-switch">
                                <span class="my-switch-track" aria-hidden="true">
                                    <span class="my-switch-thumb"></span>
                                </span>
                                "Wi-Fi"
                            </Switch>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo
                    description="Wi-Fi switch drawn with CSS, with disabled and read-only toggles"
                    source=include_str!("demos/switch.rs")
                >
                    <SwitchAtomDemo/>
                </Demo>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="atoms::switch::Switch">
                    <ApiRow name="default_selected" ty="bool" default="false">"Whether the switch starts on."</ApiRow>
                    <ApiRow name="on_change" ty="Option<Callback<bool>>" default="None">
                        "Called when the switch is turned on or off. Also called when "<Code inline=true>"state"</Code>
                        " is given."
                    </ApiRow>
                    <ApiRow name="state" ty="Option<ToggleState>" default="None">
                        "Binds the switch to your state, e.g. "<Code inline=true>"state=rw_signal"</Code>". Replaces "
                        <Code inline=true>"default_selected"</Code>"."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the switch."</ApiRow>
                    <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"The switch can be focused but not changed."</ApiRow>
                    <ApiRow name="is_required" ty="Signal<bool>" default="false">"Marks the switch as required."</ApiRow>
                    <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the switch invalid."</ApiRow>
                    <ApiRow name="validate" ty="Option<ValidateFn<bool>>" default="None">"Validates the selection."</ApiRow>
                    <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                        "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                        <Link href=routes::doc::atoms::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
                    </ApiRow>
                    <ApiRow name="name" ty="Option<String>" default="None">"The input\u{2019}s "<Code inline=true>"name"</Code>"."</ApiRow>
                    <ApiRow name="form_value" ty="Option<String>" default="None">
                        "The value submitted while on. Without it, the browser submits "<Code inline=true>"on"</Code>"."
                    </ApiRow>
                    <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the input belongs to, when it isn\u{2019}t inside it."</ApiRow>
                    <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id."</ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a switch without label text."</ApiRow>
                    <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Further labelling or describing elements."</ApiRow>
                    <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the switch when it mounts."</ApiRow>
                    <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the switch gains or loses focus."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<label>"</Code>"."</ApiRow>
                    <ApiRow name="children" ty="Option<Children>" default="None">"The track and the label text."</ApiRow>
                </ApiTable>
                <p>
                    "See "<Link href=format!("{}#toggleoptions", routes::doc::checkbox::Hook.materialize())>"ToggleOptions"</Link>
                    " for how these settings behave."
                </p>
            </Section>

            <Section title="Data Attributes">
                <p>"Set to "<Code inline=true>"true"</Code>" on the "<Code inline=true>"<label>"</Code>" while the state applies:"</p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-selected" ty="true">"The switch is on."</ApiRow>
                    <ApiRow name="data-pressed" ty="true">"The switch or its label is pressed."</ApiRow>
                    <ApiRow name="data-hovered" ty="true">"A mouse or pen is over the label."</ApiRow>
                    <ApiRow name="data-focused" ty="true">"The input has focus."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">"The input has keyboard focus: show a focus ring."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">"The switch is disabled."</ApiRow>
                    <ApiRow name="data-readonly" ty="true">"The switch is read-only."</ApiRow>
                    <ApiRow name="data-invalid" ty="true">"The switch is invalid."</ApiRow>
                    <ApiRow name="data-required" ty="true">"The switch is required."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>"Move the thumb and color the track from "<Code inline=true>"data-selected"</Code>":"</p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-switch { display: inline-flex; align-items: center; gap: 0.5em; cursor: pointer; }
                        .my-switch-track { width: 44px; height: 24px; padding: 2px; box-sizing: border-box; border-radius: 12px; background: gray; }
                        .my-switch-thumb { display: block; width: 20px; height: 20px; border-radius: 50%; background: white; transition: transform 0.2s; }
                        .my-switch[data-selected] .my-switch-track { background: royalblue; }
                        .my-switch[data-selected] .my-switch-thumb { transform: translateX(20px); }
                        .my-switch[data-focus-visible] .my-switch-track { outline: 2px solid royalblue; outline-offset: 2px; }
                    ")}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Switch.materialize()>"Switch overview"</Link></li>
                <li><Link href=routes::doc::switch::Hook.materialize()>"Switch hooks"</Link></li>
                <li><Link href=routes::doc::switch::Component.materialize()>"Switch component"</Link></li>
                <li><Link href=routes::doc::checkbox::Atom.materialize()>"Checkbox atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
