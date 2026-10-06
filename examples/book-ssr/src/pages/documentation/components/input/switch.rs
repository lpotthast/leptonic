use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    switch_basic::SwitchBasicDemo, switch_disabled::SwitchDisabledDemo,
    switch_icons::SwitchIconsDemo, switch_sizes::SwitchSizesDemo,
    switch_stationary::SwitchStationaryDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageSwitch() -> impl IntoView {
    view! {
        <DocPage title="Switch Component">
            <p>
                "The themed "<Code inline=true>"Switch"</Code>" component, an on/off control with its label as children. See the "
                <Link href=routes::doc::Switch.materialize()>"Switch overview"</Link>" for concept guidance."
            </p>

            <Demo description="Airplane mode switch controlled by a signal, showing its state" source=include_str!("demos/switch_basic.rs")>
                <SwitchBasicDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="components::switch::Switch">
                    <ApiRow name="default_selected" ty="bool" default="false">"Whether the switch starts on."</ApiRow>
                    <ApiRow name="on_change" ty="Option<Callback<bool>>" default="None">
                        "Called when the switch is turned on or off, also while "<Code inline=true>"is_selected"</Code>
                        " controls the selection."
                    </ApiRow>
                    <ApiRow name="is_selected" ty="Option<Signal<bool>>" default="None">
                        "The selection (controlled): a value or any signal."
                    </ApiRow>
                    <ApiRow name="set_selected" ty="Option<Out<bool>>" default="None">
                        "Receives the new state: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables the switch."</ApiRow>
                    <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"Shows the state without allowing changes."</ApiRow>
                    <ApiRow name="name" ty="Option<String>" default="None">"The input\u{2019}s "<Code inline=true>"name"</Code>", for form submission."</ApiRow>
                    <ApiRow name="form_value" ty="Option<String>" default="None">
                        "The value submitted while on. Without it, the browser submits "<Code inline=true>"on"</Code>"."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"The accessible name, for a switch without children."</ApiRow>
                    <ApiRow name="size" ty="SwitchSize" default="Normal">
                        <Code inline=true>"Small"</Code>", "<Code inline=true>"Normal"</Code>" or "<Code inline=true>"Big"</Code>
                        ", see "<AnchorLink href="#sizes">"Sizes"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="variant" ty="SwitchVariant" default="Sliding">
                        <Code inline=true>"Sliding"</Code>" or "<Code inline=true>"Stationary"</Code>", see "<AnchorLink href="#variants">"Variants"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="icons" ty="Option<SwitchIcons>" default="None">
                        "Icons shown in the knob while off and on, see "<AnchorLink href="#icons">"Icons"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the "<Code inline=true>"<label>"</Code>"."</ApiRow>
                    <ApiRow name="children" ty="Option<Children>" default="None">"The label text."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="State">
                <p>
                    "A switch either keeps its own state, starting at "<Code inline=true>"default_selected"</Code>
                    " and reporting changes through "<Code inline=true>"on_change"</Code>", or shows your state, given as "
                    <Code inline=true>"is_selected"</Code>", and hands changes to "<Code inline=true>"set_selected"</Code>
                    ". The demo above passes one "<Code inline=true>"RwSignal"</Code>" as both: "
                    <Code inline=true>"is_selected=airplane_mode set_selected=airplane_mode"</Code>". The children are the label: always "
                    "give a switch one (or an "<Code inline=true>"aria_label"</Code>"), so screen readers can name it."
                </p>
            </Section>

            <Section title="Icons">
                <p>
                    "Pass a pair of icons as "<Code inline=true>"SwitchIcons"</Code>". The knob shows the "
                    <Code inline=true>"off"</Code>" icon while off and the "<Code inline=true>"on"</Code>" icon while on."
                </p>
                <Demo description="Night mode switch with sun and moon icons" source=include_str!("demos/switch_icons.rs")>
                    <SwitchIconsDemo/>
                </Demo>
            </Section>

            <Section title="Variants">
                <p>
                    <Code inline=true>"SwitchVariant::Sliding"</Code>" (the default) moves the knob along the track. A "
                    <Code inline=true>"Stationary"</Code>" switch is a single circle that changes its color and icon."
                </p>
                <Demo description="Stationary lock switch with lock icons" source=include_str!("demos/switch_stationary.rs")>
                    <SwitchStationaryDemo/>
                </Demo>
            </Section>

            <Section title="Sizes">
                <p>
                    <Code inline=true>"SwitchSize::Small"</Code>", "<Code inline=true>"Normal"</Code>" (the default) and "
                    <Code inline=true>"Big"</Code>" scale the track and the knob; the label keeps the surrounding font size. "
                    "Pick the size that matches the text and controls next to the switch."
                </p>
                <Demo description="Small, normal and big switches" source=include_str!("demos/switch_sizes.rs")>
                    <SwitchSizesDemo/>
                </Demo>
            </Section>

            <Section title="Disabled and Read-Only">
                <p>
                    "A disabled switch can\u{2019}t be focused or changed. A read-only switch can be focused, and screen readers "
                    "announce it as read-only, but it can\u{2019}t be changed either."
                </p>
                <Demo description="Backups switch with disabled and read-only checkboxes" source=include_str!("demos/switch_disabled.rs")>
                    <SwitchDisabledDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the switch to your design:"</p>
                <CssVariables prefix="--switch-" scss=theme_scss!("switch")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Switch.materialize()>"Switch overview"</Link></li>
                <li><Link href=routes::doc::switch::Hook.materialize()>"Switch Hooks"</Link></li>
                <li><Link href=routes::doc::switch::Atom.materialize()>"Switch Atom"</Link></li>
                <li><Link href=routes::doc::Themes.materialize()>"Themes"</Link>" (the "<Code inline=true>"ThemeToggle"</Code>" switch)"</li>
                <li><Link href=routes::doc::checkbox::Component.materialize()>"Checkbox Components"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
