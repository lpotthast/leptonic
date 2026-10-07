use indoc::indoc;
use leptos::prelude::*;

use super::demos::spin_button::SpinButtonDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseSpinButton() -> impl IntoView {
    view! {
        <DocPage title="use_spin_button">
            <p>
                "A spin button is a control for picking a number from a range of discrete values, one step at a time: "
                <Keys keys="ArrowUp"/>" and "<Keys keys="ArrowDown"/>" step up and down, "<Keys keys="PageUp"/>" and "
                <Keys keys="PageDown"/>" take bigger steps, "<Keys keys="Home"/>" and "<Keys keys="End"/>" jump to the "
                "limits, and a pair of increment and decrement buttons let pointer users do the same. Think of the hour and "
                "minute fields of a time picker, or a quantity picker in a shop. It is a building block of the "
                <Link href=routes::doc::NumberField.materialize()>"Number Field"</Link>" and of date and time segments."
            </p>

            <p>
                "The "<Code inline=true>"use_spin_button"</Code>" hook implements this "
                <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/spinbutton/" target=LinkTarget::Blank>
                    "WAI-ARIA pattern"
                </Link>
                " for an element of your choice. It doesn\u{2019}t own the value: you keep it in your own signal and change it "
                "in the callbacks the hook calls. If you need a text input that people can also type numbers into, use "
                <Code inline=true>"use_number_field"</Code>" (see "
                <Link href=routes::doc::number_field::Hook.materialize()>"Number Field Hooks"</Link>
                "), which builds on this hook and adds parsing, formatting and validation."
            </p>

            <ReactAriaSource path="spinbutton/useSpinButton.ts"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseSpinButtonInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    ". A key is only handled when there is a callback for it. Keys without one are left alone and keep bubbling."
                </p>

                <ApiTable kind=ApiKind::Input of="UseSpinButtonInput">
                    <ApiRow name="value" ty="Signal<Option<f64>>" default="None">
                        "The current value, exposed as "<Code inline=true>"aria-valuenow"</Code>". "<Code inline=true>"None"</Code>
                        " (or NaN) means empty."
                    </ApiRow>
                    <ApiRow name="text_value" ty="Signal<Option<String>>" default="None">
                        "A readable form of the value, like \u{201c}2 cups\u{201d} or \u{201c}9 AM\u{201d}, exposed as "
                        <Code inline=true>"aria-valuetext"</Code>" and announced on change. "<Code inline=true>"None"</Code>
                        " uses "<Code inline=true>"value"</Code>". An empty string, or no "<Code inline=true>"value"</Code>" either, is "
                        "announced as \u{201c}Empty\u{201d}."
                    </ApiRow>
                    <ApiRow name="min_value, max_value" ty="Signal<Option<f64>>" default="None">
                        "The limits, exposed as "<Code inline=true>"aria-valuemin"</Code>" / "<Code inline=true>"aria-valuemax"</Code>
                        ". Holding a stepper button stops spinning once a limit is reached."
                    </ApiRow>
                    <ApiRow name="is_disabled, is_read_only, is_required" ty="Signal<bool>" default="false">
                        "Set the matching "<Code inline=true>"aria-*"</Code>" attributes. Disabled and read-only spin buttons "
                        "ignore the keyboard, and their stepper buttons are disabled."
                    </ApiRow>
                    <ApiRow name="on_increment, on_decrement" ty="Option<Callback<()>>" default="None">
                        "Step up or down by one step. Called for the arrow keys and the stepper buttons."
                    </ApiRow>
                    <ApiRow name="on_increment_page, on_decrement_page" ty="Option<Callback<()>>" default="None">
                        "Step by a page ("<Keys keys="PageUp"/>" / "<Keys keys="PageDown"/>"). Fall back to "
                        <Code inline=true>"on_increment"</Code>" / "<Code inline=true>"on_decrement"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_decrement_to_min, on_increment_to_max" ty="Option<Callback<()>>" default="None">
                        "Jump to the minimum ("<Keys keys="Home"/>") or maximum ("<Keys keys="End"/>")."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseSpinButtonReturn">
                    <ApiRow name="props" ty="UseSpinButtonProps">
                        "For the element showing the value: "<Code inline=true>"role=\"spinbutton\""</Code>", the "
                        <Code inline=true>"aria-value*"</Code>" attributes and the keyboard and focus handlers. Spread with "
                        <Code inline=true>"{..props.into_attrs()}"</Code>". Add a "<Code inline=true>"tabindex"</Code>
                        " and an accessible name yourself."
                    </ApiRow>
                    <ApiRow name="increment_button" ty="UseButtonInput">
                        "Configuration for the increment button, with the hold-to-spin behavior, disabled while the spin "
                        "button is disabled or read-only. Pass it to "
                        <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>"."
                    </ApiRow>
                    <ApiRow name="decrement_button" ty="UseButtonInput">
                        "Configuration for the decrement button. Pass it to "<Code inline=true>"use_button"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <p>
                    "The stepper buttons come as "<Code inline=true>"UseButtonInput"</Code>"s with the hold-to-spin behavior "
                    "already set up, and disabled with the spin button. Add a label with struct update syntax, and combine "
                    "their "<Code inline=true>"is_disabled"</Code>" with your own condition (here: the limit is reached) "
                    "before rendering them with "<Code inline=true>"use_button"</Code>":"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::*;

                        let (cups, set_cups) = signal(2.0_f64);
                        let step = move |delta: f64| set_cups.update(|v| *v = (*v + delta).clamp(0.0, 10.0));

                        let spin = use_spin_button(UseSpinButtonInput {
                            value: Signal::derive(move || Some(cups.get())),
                            min_value: Signal::stored(Some(0.0)),
                            max_value: Signal::stored(Some(10.0)),
                            on_increment: Some(Callback::new(move |()| step(1.0))),
                            on_decrement: Some(Callback::new(move |()| step(-1.0))),
                            ..Default::default()
                        });

                        let spin_disabled = spin.increment_button.is_disabled;
                        let (inc_attrs, inc_styles) = use_button(UseButtonInput {
                            aria_label: "More cups".into(),
                            is_disabled: Signal::derive(move || spin_disabled.get() || cups.get() >= 10.0),
                            allow_focus_when_disabled: true,
                            ..spin.increment_button
                        })
                        .props
                        .into_parts();
                        // ... the same for `spin.decrement_button`

                        view! {
                            <div {..spin.props.into_attrs()} tabindex="0" aria-label="Cups of coffee">
                                {move || cups.get()}
                            </div>
                            <button {..inc_attrs} style=inc_styles>"+"</button>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Focus the number and use "<Keys keys="ArrowUp"/>" / "<Keys keys="ArrowDown"/>", "<Keys keys="PageUp"/>
                    " / "<Keys keys="PageDown"/>" (five cups at a time), "<Keys keys="Home"/>" and "<Keys keys="End"/>
                    ", or hold one of the buttons."
                </p>
                <Demo
                    description="Custom spin button with a value from 0 to 10, two stepper buttons and a Disabled checkbox"
                    source=include_str!("demos/spin_button.rs")
                >
                    <SpinButtonDemo/>
                </Demo>
            </Section>

            <Section title="Stepper Buttons and Announcements">
                <p>
                    "Pressing a stepper button with a mouse or pen steps once right away. Keep holding it, and after a short "
                    "pause it keeps stepping until you let go or the value hits "<Code inline=true>"min_value"</Code>" / "
                    <Code inline=true>"max_value"</Code>". Touch waits a little longer before it starts spinning, since the "
                    "finger might be about to scroll: a quick tap steps once when you lift your finger, and sliding away to "
                    "scroll doesn\u{2019}t step at all. While a button is held, the context menu that long touches usually "
                    "open is suppressed."
                </p>

                <p>
                    "Whenever the value changes while the spin button or one of its buttons has focus, the new "
                    <Code inline=true>"text_value"</Code>" (or "<Code inline=true>"value"</Code>
                    ") is announced to screen readers. Negative numbers are announced with a real minus sign, so VoiceOver "
                    "reads \u{201c}minus\u{201d} even when a currency symbol sits in between."
                </p>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="ArrowUp / ArrowDown">"Step up / down."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Step up / down by a page."</KeyRow>
                    <KeyRow keys="Home / End">"Jump to the minimum / maximum."</KeyRow>
                </KeyboardTable>

                <p>"Holding a key keeps stepping. Keys with a modifier held are not handled."</p>
            </Section>

            <SeeAlso>
                <li>
                    <Link href=routes::doc::number_field::Hook.materialize()>"Number Field Hooks"</Link>
                    " \u{2014} a text input for numbers, built on this hook"
                </li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" \u{2014} renders the stepper buttons"</li>
                <li>
                    <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>
                    " \u{2014} the keyboard shortcuts behind the arrow keys"
                </li>
                <li><Link href=routes::doc::Slider.materialize()>"Slider"</Link>" \u{2014} for picking a value from a continuous range"</li>
            </SeeAlso>
        </DocPage>
    }
}
