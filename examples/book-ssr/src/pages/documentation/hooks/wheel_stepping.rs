use indoc::indoc;
use leptos::prelude::*;

use super::demos::wheel_stepping::WheelSteppingDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseWheelStepping() -> impl IntoView {
    view! {
        <DocPage title="use_wheel_stepping">
            <p>
                "The "<Code inline=true>"use_wheel_stepping"</Code>" hook lets the scroll wheel step a field\u{2019}s value "
                "while the field has focus: scrolling down increments, scrolling up decrements. "
                <Link href=routes::doc::number_field::Hook.materialize()>"Number Field"</Link>" and "
                <Link href=routes::doc::color_field::Hook.materialize()>"Color Field"</Link>" step this way. See the "
                <Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>
                " to compare it with the other interaction building blocks."
            </p>

            <ReactAriaSource path="numberfield/useNumberField.ts"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseWheelSteppingInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    " (enabled, editable, no callbacks)."
                </p>
                <ApiTable kind=ApiKind::Input of="UseWheelSteppingInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Whether the field is disabled: its focus isn\u{2019}t followed, and the wheel doesn\u{2019}t step."
                    </ApiRow>
                    <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"Whether the field is read-only: the wheel doesn\u{2019}t step."</ApiRow>
                    <ApiRow name="is_wheel_disabled" ty="bool" default="false">
                        "Whether the wheel leaves the value alone: the page scrolls instead."
                    </ApiRow>
                    <ApiRow name="on_increment, on_decrement" ty="Option<Callback<()>>" default="None">
                        "Called when the wheel scrolls down (increment) or up (decrement) while the field has focus."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseWheelSteppingReturn">
                    <ApiRow name="group" ty="Part<UseFocusWithinProps>">
                        "The element whose focus lets the wheel step: the field\u{2019}s group around the input (with "
                        "stepper buttons, say), or the input itself if there is none."
                    </ApiRow>
                    <ApiRow name="input" ty="Part<UseScrollWheelProps>">"The input the wheel steps over."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::interactions::{UseWheelSteppingInput, UseWheelSteppingReturn, use_wheel_stepping};
                        use leptos::prelude::*;

                        let rating = RwSignal::new(3_u8);

                        let UseWheelSteppingReturn { group, input } = use_wheel_stepping(UseWheelSteppingInput {
                            on_increment: Some(Callback::new(move |()| rating.update(|r| *r = (*r + 1).min(5)))),
                            on_decrement: Some(Callback::new(move |()| rating.update(|r| *r = r.saturating_sub(1)))),
                            ..UseWheelSteppingInput::default()
                        });

                        view! {
                            <div {..group.attrs()}>
                                <input aria-label="Rating" {..input.attrs()} prop:value=move || rating.get().to_string() />
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="A minutes field stepped by the scroll wheel while it has focus" source=include_str!("demos/wheel_stepping.rs")>
                    <WheelSteppingDemo/>
                </Demo>
            </Section>

            <Section title="Behavior">
                <ul>
                    <li>
                        "The wheel steps only while focus is within the group, so scrolling past an unfocused field "
                        "scrolls the page. While it steps, the page doesn\u{2019}t scroll."
                    </li>
                    <li>
                        "A mostly horizontal scroll, such as a trackpad gesture drifting sideways, doesn\u{2019}t step."
                    </li>
                    <li>
                        "Each wheel event steps once, whatever its size. A field that steps by a size of its own (a number "
                        "field\u{2019}s "<Code inline=true>"step"</Code>") does so in its callbacks."
                    </li>
                    <li>
                        "The wheel is no way to reach the value without a mouse: give the field keys as well, as "
                        <Link href=routes::doc::utilities::UseSpinButton.materialize()>"use_spin_button"</Link>" does."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::interactions::UseScrollWheel.materialize()>"use_scroll_wheel"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusWithin.materialize()>"use_focus_within"</Link></li>
                <li><Link href=routes::doc::number_field::Hook.materialize()>"Number Field Hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
