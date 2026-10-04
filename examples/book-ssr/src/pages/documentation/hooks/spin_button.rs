use indoc::indoc;
use leptonic::{components::prelude::*, utils::key::Key};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::spin_button::SpinButtonDemo;

#[component]
pub fn PageUseSpinButton() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_spin_button" class="anchor">
                "use_spin_button"
                <AnchorLink href="#use_spin_button" description="Direct link to article header"/>
            </h1>

            <p>
                "A spin button is a control for picking a number from a range of discrete values, one step at a time: "
                "the arrow keys step up and down, Page Up and Page Down take bigger steps, Home and End jump to the limits, "
                "and a pair of increment and decrement buttons let pointer users do the same. Think of the hour and minute "
                "fields of a time picker, or a quantity picker in a shop."
            </p>

            <p>
                "The "<Code inline=true>"use_spin_button"</Code>" hook implements this "
                <LinkExt href="https://www.w3.org/WAI/ARIA/apg/patterns/spinbutton/" target=LinkTarget::_Blank>
                    "WAI-ARIA pattern"
                </LinkExt>
                " for an element of your choice. It doesn\u{2019}t own the value: you keep it in your own signal and change it "
                "in the callbacks the hook calls. If you need a text input that people can also type numbers into, use "
                <Link href=crate::routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link>
                ", which builds on this hook and adds parsing, formatting and validation."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/spinbutton/useSpinButton.ts" target=LinkTarget::_Blank>
                    "useSpinButton"
                </LinkExt>
                "."
            </p>

            <h2 id="input" class="anchor">
                "Input"
                <AnchorLink href="#input" description="Direct link to section: Input"/>
            </h2>

            <p>
                <Code inline=true>"UseSpinButtonInput"</Code>" implements "<Code inline=true>"Default"</Code>
                ". A key is only handled when there is a callback for it. Keys without one are left alone and keep bubbling."
            </p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"value"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<Option<f64>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"The current value, exposed as "<Code inline=true>"aria-valuenow"</Code>". "<Code inline=true>"None"</Code>" (or NaN) means empty."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"text_value"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<Option<String>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"A readable form of the value, like \u{201c}2 cups\u{201d} or \u{201c}9 AM\u{201d}, exposed as "<Code inline=true>"aria-valuetext"</Code>" and announced on change. "<Code inline=true>"None"</Code>" uses "<Code inline=true>"value"</Code>"; an empty string is announced as \u{201c}Empty\u{201d}."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"min_value"</Code>", "<Code inline=true>"max_value"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<Option<f64>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"The limits, exposed as "<Code inline=true>"aria-valuemin"</Code>" / "<Code inline=true>"aria-valuemax"</Code>". Holding a stepper button stops spinning once a limit is reached."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"disabled"</Code>", "<Code inline=true>"read_only"</Code>", "<Code inline=true>"required"</Code></TableCell>
                            <TableCell><Code inline=true>"Signal<bool>"</Code></TableCell>
                            <TableCell><Code inline=true>"false"</Code></TableCell>
                            <TableCell>"Set the matching "<Code inline=true>"aria-*"</Code>" attributes. Disabled and read-only spin buttons ignore the keyboard."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_increment"</Code>", "<Code inline=true>"on_decrement"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<()>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Step up or down by one step. Called for the arrow keys and the stepper buttons."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_increment_page"</Code>", "<Code inline=true>"on_decrement_page"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<()>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Step by a page (Page Up / Page Down). Fall back to "<Code inline=true>"on_increment"</Code>" / "<Code inline=true>"on_decrement"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_decrement_to_min"</Code>", "<Code inline=true>"on_increment_to_max"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<()>>"</Code></TableCell>
                            <TableCell><Code inline=true>"None"</Code></TableCell>
                            <TableCell>"Jump to the minimum (Home) or maximum (End)."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="return" class="anchor">
                "Return"
                <AnchorLink href="#return" description="Direct link to section: Return"/>
            </h2>

            <p><Code inline=true>"UseSpinButtonReturn"</Code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"props"</Code></TableCell>
                            <TableCell><Code inline=true>"UseSpinButtonProps"</Code></TableCell>
                            <TableCell>
                                "For the element showing the value: "<Code inline=true>"role=\"spinbutton\""</Code>", the "
                                <Code inline=true>"aria-value*"</Code>" attributes and the keyboard and focus handlers. Spread with "
                                <Code inline=true>"{..props.into_attrs()}"</Code>". Add a "<Code inline=true>"tabindex"</Code>
                                " and an accessible name yourself."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"increment_button"</Code></TableCell>
                            <TableCell><Code inline=true>"UseButtonInput"</Code></TableCell>
                            <TableCell>"Configuration for the increment button. Pass it to "<Link href=crate::routes::doc::button::Hook.materialize()>"use_button"</Link>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"decrement_button"</Code></TableCell>
                            <TableCell><Code inline=true>"UseButtonInput"</Code></TableCell>
                            <TableCell>"Configuration for the decrement button. Pass it to "<Code inline=true>"use_button"</Code>"."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="example" class="anchor">
                "Example"
                <AnchorLink href="#example" description="Direct link to section: Example"/>
            </h2>

            <p>
                "The stepper buttons come as "<Code inline=true>"UseButtonInput"</Code>"s with the hold-to-spin behavior already set up. "
                "Everything else about them is up to you, so add a label and, if you like, a disabled state with struct update "
                "syntax before rendering them with "<Code inline=true>"use_button"</Code>":"
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
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

                    let (inc_attrs, inc_styles) = use_button(UseButtonInput {
                        aria_label: Some("More cups".into()),
                        disabled: Signal::derive(move || cups.get() >= 10.0),
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

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to section: Demo"/>
            </h2>

            <DemoShell
                source=include_str!("demos/spin_button.rs")
                description="Custom spin button with a value from 0 to 10 and two stepper buttons"
            >
                <SpinButtonDemo />
            </DemoShell>

            <h2 id="options" class="anchor">
                "Stepper buttons and announcements"
                <AnchorLink href="#options" description="Direct link to section: Stepper buttons and announcements"/>
            </h2>

            <p>
                "Pressing a stepper button with a mouse or pen steps once right away. Keep holding it, and after a short pause "
                "it keeps stepping until you let go or the value hits "<Code inline=true>"min_value"</Code>" / "
                <Code inline=true>"max_value"</Code>". Touch waits a little longer before it starts spinning, since the finger "
                "might be about to scroll: a quick tap steps once when you lift your finger, and sliding away to scroll "
                "doesn\u{2019}t step at all. While a button is held, the context menu that long touches usually open is suppressed."
            </p>

            <p>
                "Whenever the value changes while the spin button or one of its buttons has focus, the new "
                <Code inline=true>"text_value"</Code>" (or "<Code inline=true>"value"</Code>
                ") is announced to screen readers. Negative numbers are announced with a real minus sign, so VoiceOver reads "
                "\u{201c}minus\u{201d} even when a currency symbol sits in between."
            </p>

            <h2 id="keyboard" class="anchor">
                "Keyboard"
                <AnchorLink href="#keyboard" description="Direct link to section: Keyboard"/>
            </h2>

            <ul>
                <li><KbdKey key=Key::ArrowUp/>" / "<KbdKey key=Key::ArrowDown/>" \u{2014} Step up / down"</li>
                <li><KbdKey key=Key::PageUp/>" / "<KbdKey key=Key::PageDown/>" \u{2014} Step up / down by a page"</li>
                <li><KbdKey key=Key::Home/>" / "<KbdKey key=Key::End/>" \u{2014} Jump to the minimum / maximum"</li>
            </ul>

            <p>"Holding a key keeps stepping. Keys with a modifier held are not handled."</p>

            <h2 id="deviations" class="anchor">
                "Deviations from react-aria"
                <AnchorLink href="#deviations" description="Direct link to section: Deviations"/>
            </h2>

            <ul>
                <li>
                    <b>"Stepper buttons as button inputs."</b>" React-aria returns "<Code inline=true>"AriaButtonProps"</Code>
                    " for the buttons; leptonic returns "<Code inline=true>"UseButtonInput"</Code>", which you pass to "
                    <Code inline=true>"use_button"</Code>"."
                </li>
                <li>
                    <b>"No extra step after a touch hold."</b>" In react-aria, lifting a finger after the button already spun steps "
                    "once more. Leptonic remembers whether the press spun, so a tap steps once and a hold only spins."
                </li>
                <li>
                    <b>"Value text."</b>" "<Code inline=true>"text_value: None"</Code>" means \u{201c}derive it from "
                    <Code inline=true>"value"</Code>"\u{201d}, while "<Code inline=true>"Some(\"\")"</Code>" announces \u{201c}Empty\u{201d}."
                </li>
                <li>
                    <b>"English only."</b>" The \u{201c}Empty\u{201d} announcement is not localized yet."
                </li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link>" \u{2014} a text input for numbers, built on this hook"</li>
                <li><Link href=crate::routes::doc::button::Hook.materialize()>"use_button"</Link>" \u{2014} renders the stepper buttons"</li>
                <li><Link href=crate::routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>" \u{2014} the keyboard shortcuts behind the arrow keys"</li>
                <li><Link href=crate::routes::doc::Slider.materialize()>"Slider"</Link>" \u{2014} for picking a value from a continuous range"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_spin_button", link: "#use_spin_button" },
                Toc::Leaf { title: "Input", link: "#input" },
                Toc::Leaf { title: "Return", link: "#return" },
                Toc::Leaf { title: "Example", link: "#example" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Stepper buttons and announcements", link: "#options" },
                Toc::Leaf { title: "Keyboard", link: "#keyboard" },
                Toc::Leaf { title: "Deviations", link: "#deviations" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
