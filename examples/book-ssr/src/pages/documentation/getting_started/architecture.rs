use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageArchitecture() -> impl IntoView {
    view! {
        <DocPage title="Hooks, Atoms & Components">
            <p>
                "Leptonic is built in three layers. Most concepts (a button, a select, a table) are available as a hook, "
                "many also as an atom and as a component: pick the layer that gives you the control you need. The "
                <Link href=routes::doc::Overview.materialize()>"Overview"</Link>" explains how this book presents the "
                "layers of a concept."
            </p>

            <Section title="Layer Overview">
                <DocTable headers=&["Layer", "What you get", "When to use it"]>
                    <TableRow>
                        <TableCell><b>"Hook"</b></TableCell>
                        <TableCell>
                            "Behavior, ARIA attributes and interaction state (pressed, hovered, focus visible) for elements you "
                            "render. No elements, no styling."
                        </TableCell>
                        <TableCell>"Custom elements and full control over rendering."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><b>"Atom"</b></TableCell>
                        <TableCell>"A single semantic HTML element with the hook\u{2019}s behavior built in. No styling."</TableCell>
                        <TableCell>"Correct semantics and accessibility out of the box, styled by your own CSS."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><b>"Component"</b></TableCell>
                        <TableCell>"Themed, feature-rich UI with colors, variants and CSS variables."</TableCell>
                        <TableCell>"Standard UI. Use components unless you need more control."</TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "Each layer is a cargo feature that includes the layers below it; see "
                    <Link href=format!("{}#feature-flags", routes::doc::Installation.materialize())>"Feature Flags"</Link>"."
                </p>
            </Section>

            <Section title="Hooks">
                <p>
                    "Hooks hold the interaction and accessibility logic: ARIA attributes, keyboard interaction, focus management. "
                    "They render nothing and bring no styles. A hook returns props that you spread onto your element. Leptonic\u{2019}s "
                    "hooks are ports of "
                    <Link href="https://react-spectrum.adobe.com/react-aria/" target=LinkTarget::Blank>"react-aria"</Link>
                    "\u{2019}s hooks, with APIs adapted to Rust and Leptos."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::{ButtonElementType, UseButtonInput, UseButtonReturn, use_button};

                        let UseButtonReturn { props, .. } = use_button(UseButtonInput {
                            element_type: ButtonElementType::Other,
                            on_press: Some(Callback::new(|_| { /* handle press */ })),
                            ..Default::default()
                        });
                        let (attrs, styles) = props.into_parts();

                        view! {
                            <div {..attrs} style=styles>
                                "My custom button"
                            </div>
                        }
                    "#)}
                </Code>

                <Section title="Hook-Owned State">
                    <p>
                        "Hooks with state create and own it. You set the initial value, read the state through a "
                        <Code inline=true>"Signal"</Code>", and change it through the methods the hook returns, so the hook "
                        "sees every change and can keep related state consistent. To keep the state in your app instead, "
                        "bind it with a "
                        <Link href=format!("{}#valuebinding", routes::doc::Callbacks.materialize())>"ValueBinding"</Link>
                        " (a signal and a setter): changes still go through the hook, which then calls your setter."
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{
                                components::prelude::*,
                                hooks::{UseToggleStateInput, use_toggle_state},
                            };

                            let toggle = use_toggle_state(UseToggleStateInput {
                                default_selected: true,
                                ..Default::default()
                            });

                            view! {
                                <Button on_press=move |_| toggle.toggle()>
                                    {move || if toggle.is_selected.get() { "On" } else { "Off" }}
                                </Button>
                            }
                        "#)}
                    </Code>
                </Section>

                <Section title="Composing Hooks">
                    <p>
                        "Most concepts need more than one hook. There are two ways to combine them, depending on whether the "
                        "hooks describe the same thing or different things."
                    </p>

                    <p>
                        <b>"One hook configures another."</b>" A menu trigger is a button that also opens a menu. A spin button has two "
                        "stepper buttons that keep stepping while held. Hooks like "
                        <Link href=routes::doc::menu::Hook.materialize()><Code inline=true>"use_menu_trigger"</Code></Link>", "
                        <Link href=routes::doc::utilities::UseSpinButton.materialize()><Code inline=true>"use_spin_button"</Code></Link>
                        " and "<Link href=routes::doc::number_field::Hook.materialize()><Code inline=true>"use_number_field"</Code></Link>
                        " therefore don\u{2019}t hand you DOM props for those buttons. They return a "<Code inline=true>"UseButtonInput"</Code>
                        ", and you render the button with "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                        ". Input structs implement "<Code inline=true>"Default"</Code>" where every field has a sensible default, "
                        "so you can add your own settings with struct update syntax:"
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let menu_trigger = use_menu_trigger(UseMenuTriggerInput { /* ... */ });

                            let button = use_button(UseButtonInput {
                                on_hover_start: Some(Callback::new(|_| { /* prefetch the menu's data */ })),
                                ..menu_trigger.button
                            });
                            let (attrs, styles) = button.props.into_parts();

                            view! { <button {..attrs} style=styles>"Actions"</button> }
                        "#)}
                    </Code>

                    <p>
                        "This keeps exactly one press, focus and hover state machine per element. If both hooks attached their own "
                        "press handling to the same element, a single click would be processed twice, with two competing ideas of "
                        "whether the button is pressed."
                    </p>

                    <p>
                        <b>"Independent hooks on one element."</b>" When hooks add unrelated behavior to the same element, say "
                        <Link href=routes::doc::interactions::UsePress.materialize()><Code inline=true>"use_press"</Code></Link>
                        " and "<Link href=routes::doc::interactions::UseHover.materialize()><Code inline=true>"use_hover"</Code></Link>
                        " on an element of your own, merge their props with the "<Code inline=true>"MergeWith"</Code>
                        " trait. Event handlers are chained so both run; for other attributes the last one wins:"
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{hooks::*, utils::MergeWith};

                            let press = use_press(UsePressInput { /* ... */ ..Default::default() });
                            let hover = use_hover(UseHoverInput { /* ... */ ..Default::default() });

                            let (attrs, styles) = press.props.merge_with(hover.props).into_parts();

                            view! { <div {..attrs} style=styles>"Hover and press me"</div> }
                        "#)}
                    </Code>

                    <p>
                        "When in doubt, prefer input composition: if a hook hands you a "<Code inline=true>"UseButtonInput"</Code>
                        " (or another hook\u{2019}s input), pass it on rather than merging DOM props."
                    </p>
                </Section>
            </Section>

            <Section title="Atoms">
                <p>
                    "Atoms are unstyled Leptos components that render a single HTML element with a hook\u{2019}s behavior. "
                    "They give you accessibility without manual hook setup, expose their state as "
                    <Code inline=true>"data-*"</Code>" attributes for your CSS ("<Code inline=true>"data-pressed"</Code>", "
                    <Code inline=true>"data-focus-visible"</Code>", \u{2026}), and fit into any design system."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude as atoms;

                        view! {
                            <atoms::Button classes="my-button" on_press=move |_| { /* handle press */ }>
                                "My headless button"
                            </atoms::Button>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Components">
                <p>
                    "Components are ready-made, themed UI built on atoms and hooks. They carry leptonic\u{2019}s CSS classes "
                    "and data attributes for their look ("<Code inline=true>"data-variant"</Code>", "
                    <Code inline=true>"data-color"</Code>", "<Code inline=true>"data-size"</Code>") and are styled by the "
                    <Link href=routes::doc::Themes.materialize()>"themes"</Link>"."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::components::prelude::*;

                        view! {
                            <Button
                                on_press=move |_| { /* handle press */ }
                                variant=ButtonVariant::Outlined
                                color=ButtonColor::Success
                            >
                                "Save"
                            </Button>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="State Props">
                <p>
                    "Atoms and components take their state as two props: "<Code inline=true>"x"</Code>" for the value (a "
                    "plain value or any signal) and "<Code inline=true>"set_x"</Code>" to receive changes (an "
                    <Code inline=true>"RwSignal"</Code>", a "<Code inline=true>"WriteSignal"</Code>", a closure or a "
                    <Code inline=true>"Callback"</Code>", see "
                    <Link href=format!("{}#out", routes::doc::Callbacks.materialize())>"Out"</Link>"). The setter of an "
                    <Code inline=true>"is_x"</Code>" prop is "<Code inline=true>"set_x"</Code>": "
                    <Code inline=true>"is_selected"</Code>" and "<Code inline=true>"set_selected"</Code>", "
                    <Code inline=true>"is_open"</Code>" and "<Code inline=true>"set_open"</Code>"."
                </p>
                <ul>
                    <li>
                        "With both, the state is yours: the element shows "<Code inline=true>"x"</Code>" and asks for "
                        "changes through "<Code inline=true>"set_x"</Code>"."
                    </li>
                    <li>"With "<Code inline=true>"x"</Code>" alone, the element is read-only: changes go nowhere."</li>
                    <li>
                        "Without "<Code inline=true>"x"</Code>", the element keeps the state itself, starting at "
                        <Code inline=true>"default_x"</Code>"; "<Code inline=true>"set_x"</Code>" then receives every change."
                    </li>
                </ul>
                <p>
                    "Either way, "<Code inline=true>"on_change"</Code>" (for named states "<Code inline=true>"on_x_change"</Code>
                    ", e.g. "<Code inline=true>"on_open_change"</Code>") reports every change."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::components::prelude::*;

                        let accepted = RwSignal::new(false);

                        view! {
                            <Checkbox is_selected=accepted set_selected=accepted>"I accept the terms"</Checkbox>
                            <Button is_disabled=Signal::derive(move || !accepted.get())>"Continue"</Button>
                        }
                    "#)}
                </Code>
            </Section>
        </DocPage>
    }
}
