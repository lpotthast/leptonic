use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageArchitecture() -> impl IntoView {
    view! {
        <DocPage title="Hooks, Atoms & Components">
            <p>
                "Leptonic is built in three layers. Most UI capabilities are available as a hook, many also as an atom and as a "
                "component: pick the layer that gives you the control you need."
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
            </Section>

            <Section title="Hooks">
                <p>
                    "Hooks hold the interaction and accessibility logic: ARIA attributes, keyboard interaction, focus management. "
                    "They render nothing and bring no styles. A hook returns props that you spread onto your element. Leptonic\u{2019}s "
                    "hooks are ports of "
                    <LinkExt href="https://react-spectrum.adobe.com/react-aria/" target=LinkTarget::_Blank>"react-aria"</LinkExt>
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

                <Section title="Hook-owned state">
                    <p>
                        "Hooks with state create and own it. You set the initial value, read the state through a "
                        <Code inline=true>"Signal"</Code>", and change it through the callbacks the hook returns, so the hook "
                        "sees every change and can keep related state consistent. There are no controlled/uncontrolled prop pairs: "
                        "a signal is already shared, and you derive whatever you need from it."
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let UseToggleStateReturn { is_selected, toggle, .. } = use_toggle_state(false);

                            view! {
                                <Button on_press=move |_| toggle.run(())>
                                    {move || if is_selected.get() { "On" } else { "Off" }}
                                </Button>
                            }
                        "#)}
                    </Code>
                </Section>

                <Section title="Composing hooks">
                    <p>
                        "Most real widgets need more than one hook. There are two ways to combine them, depending on whether the "
                        "hooks describe the same thing or different things."
                    </p>

                    <p>
                        <b>"One hook configures another."</b>" A menu trigger is a button that also opens a menu. A spin button has two "
                        "stepper buttons that keep stepping while held. Hooks like "<Code inline=true>"use_menu_trigger"</Code>", "
                        <Code inline=true>"use_spin_button"</Code>" and "<Code inline=true>"use_number_field"</Code>
                        " therefore don\u{2019}t hand you DOM props for those buttons. They return a "<Code inline=true>"UseButtonInput"</Code>
                        ", and you render the button with "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                        ". Input structs implement "<Code inline=true>"Default"</Code>", so you can add your own settings with struct "
                        "update syntax:"
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let menu_trigger = use_menu_trigger(UseMenuTriggerInput { /* ... */ });

                            let button = use_button(UseButtonInput {
                                on_hover_start: Some(Callback::new(|_| { /* ... */ })),
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
                        <Code inline=true>"use_press"</Code>" and "<Code inline=true>"use_hover"</Code>
                        " on a custom widget, merge their props with the "<Code inline=true>"MergeWith"</Code>
                        " trait. Event handlers are chained so both run; for other attributes the last one wins:"
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::utils::MergeWith;

                            let press = use_press(UsePressInput { /* ... */ ..Default::default() });
                            let hover = use_hover(UseHoverInput { /* ... */ });

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
                    "Atoms are unstyled components that render a single HTML element with a hook\u{2019}s behavior. They give you "
                    "accessibility without manual hook setup, expose their state as "<Code inline=true>"data-*"</Code>
                    " attributes for your CSS, and fit into any design system."
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
                    "Components are ready-made, themed UI built on atoms and hooks. They carry leptonic\u{2019}s CSS classes and "
                    "design tokens ("<Code inline=true>"data-variant"</Code>", "<Code inline=true>"data-color"</Code>", "
                    <Code inline=true>"data-size"</Code>") and are styled by the "<Link href=routes::doc::Themes.materialize()>"themes"</Link>"."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::components::prelude::*;

                        view! {
                            <Button
                                on_press=move |_| { /* handle press */ }
                                variant=ButtonVariant::Filled
                                color=ButtonColor::Primary
                            >
                                "Click me"
                            </Button>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Feature Flags">
                <p>"Each layer is a cargo feature; each includes the layers below it:"</p>

                <DocTable headers=&["Feature", "Includes"]>
                    <TableRow><TableCell><Code inline=true>"hooks"</Code></TableCell><TableCell>"The hooks. Enabled by default."</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"atoms"</Code></TableCell><TableCell>"Atoms and hooks."</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"components"</Code></TableCell><TableCell>"Components, atoms and hooks."</TableCell></TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"full"</Code></TableCell>
                        <TableCell>"Everything, including the rich text editor, syntax highlighting, HTML sanitizing and clipboard support."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>
        </DocPage>
    }
}
