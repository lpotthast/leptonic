use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageArchitecture() -> impl IntoView {
    view! {
        <Article>
            <h1 id="architecture" class="anchor">
                "Hooks, Atoms & Components"
                <AnchorLink href="#architecture" description="Direct link to article header"/>
            </h1>

            <p>
                "Leptonic follows a three-layer architecture that gives you the level of control you need. "
                "Every UI capability is available as a hook, an atom, or a full component \u{2014} pick the layer that fits your use case."
            </p>

            <h2 id="layer-overview" class="anchor">
                "Layer Overview"
                <AnchorLink href="#layer-overview" description="Direct link to section: Layer Overview"/>
            </h2>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Layer"</TableHeaderCell>
                            <TableHeaderCell>"What you get"</TableHeaderCell>
                            <TableHeaderCell>"When to use it"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><b>"Hook"</b></TableCell>
                            <TableCell>"ARIA attributes, interaction state signals (press, hover, focus-ring), keyboard activation. No DOM element, no styling."</TableCell>
                            <TableCell>"Building custom behavior into non-standard elements. Full control over rendering."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><b>"Atom"</b></TableCell>
                            <TableCell>"A semantic HTML element with all hook behavior baked in. No visual styling."</TableCell>
                            <TableCell>"You want correct semantics and accessibility out of the box, but want to apply your own CSS."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><b>"Component"</b></TableCell>
                            <TableCell>"Fully themed element with colors, variants, groups, and CSS variables."</TableCell>
                            <TableCell>"Standard UI. Use this unless you need lower-level control."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="hooks" class="anchor">
                "Hooks"
                <AnchorLink href="#hooks" description="Direct link to section: Hooks"/>
            </h2>

            <p>"Low-level interaction and accessibility logic."</p>

            <ul>
                <li>"Pure functions returning props/attributes to spread on elements."</li>
                <li>"Handle accessibility (correct ARIA attributes, keyboard interaction, focus management)."</li>
                <li>"No rendering: you control the DOM."</li>
                <li>"No CSS classes or other design tokens."</li>
            </ul>

            <p><b>"Use when:"</b>" You need maximum flexibility and control over your elements."</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptonic::hooks::{use_button, ButtonElementType, UseButtonInput, UseButtonReturn};

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

            <h3 id="composing-hooks" class="anchor">
                "Composing hooks"
                <AnchorLink href="#composing-hooks" description="Direct link to section: Composing hooks"/>
            </h3>

            <p>
                "Most real widgets need more than one hook. There are two ways to combine them, and which one fits depends "
                "on whether the hooks describe the same thing or different things."
            </p>

            <p>
                <b>"One hook configures another."</b>" A menu trigger is a button that also opens a menu. A spin button has two "
                "stepper buttons that keep stepping while held. Hooks like "<Code inline=true>"use_menu_trigger"</Code>", "
                <Code inline=true>"use_spin_button"</Code>" and "<Code inline=true>"use_number_field"</Code>
                " therefore don\u{2019}t hand you DOM props for those buttons. They return a "<Code inline=true>"UseButtonInput"</Code>
                ", and you render the button with "<Code inline=true>"use_button"</Code>
                ". Input structs implement "<Code inline=true>"Default"</Code>", so you can add your own settings with struct update syntax:"
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
                "whether the button is pressed. It is also how react-aria works: "<Code inline=true>"useMenuTrigger"</Code>
                " returns "<Code inline=true>"AriaButtonProps"</Code>", which are then passed to "<Code inline=true>"useButton"</Code>"."
            </p>

            <p>
                <b>"Independent hooks on one element."</b>" When hooks add unrelated behavior to the same element, say "
                <Code inline=true>"use_press"</Code>" and "<Code inline=true>"use_hover"</Code>
                " on a custom widget, merge their props with the "<Code inline=true>"MergeWith"</Code>
                " trait. Event handlers are chained so both run, and for other attributes the last one wins:"
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

            <h2 id="atoms" class="anchor">
                "Atoms"
                <AnchorLink href="#atoms" description="Direct link to section: Atoms"/>
            </h2>

            <p>"Headless, single-element components that wrap hooks."</p>

            <ul>
                <li>"Provide accessibility and interaction behavior out of the box without manual hook setup."</li>
                <li>"Render one HTML element each: easy to compose and style."</li>
                <li>"Only minimal design tokens (no classes or custom styling-related data attributes)."</li>
                <li>"Easy to integrate into custom design systems."</li>
            </ul>

            <p><b>"Use when:"</b>" Building custom design systems that need accessibility without Leptonic\u{2019}s visual design."</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptonic::atoms::button::Button;

                    view! {
                        <Button on_press=move |_| { /* handle press */ }>
                            "My headless button"
                        </Button>
                    }
                "#)}
            </Code>

            <h2 id="components" class="anchor">
                "Components"
                <AnchorLink href="#components" description="Direct link to section: Components"/>
            </h2>

            <p>"Pre-built, styled components ready for production use."</p>

            <ul>
                <li>"Include CSS classes for leptonic-theme."</li>
                <li>"Include design tokens (data-variant, data-color, data-size)."</li>
                <li>"Feature-rich with complex behavior."</li>
                <li>"Built on atoms and hooks."</li>
            </ul>

            <p><b>"Use when:"</b>" You want Leptonic\u{2019}s design system with minimal configuration."</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptonic::components::button::{Button, ButtonVariant, ButtonColor};

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

            <h2 id="feature-flags" class="anchor">
                "Feature Flags"
                <AnchorLink href="#feature-flags" description="Direct link to section: Feature Flags"/>
            </h2>

            <p>"The library supports feature flags to control which layers are included:"</p>

            <ul>
                <li><Code inline=true>"hooks"</Code>" \u{2014} Low-level interaction hooks (default)"</li>
                <li><Code inline=true>"atoms"</Code>" \u{2014} Headless base components (requires hooks)"</li>
                <li><Code inline=true>"components"</Code>" \u{2014} Full pre-built components (requires atoms)"</li>
                <li><Code inline=true>"full"</Code>" \u{2014} All features combined"</li>
            </ul>

            <p>"Feature hierarchy: "<Code inline=true>"hooks"</Code>" \u{2192} "<Code inline=true>"atoms"</Code>" \u{2192} "<Code inline=true>"components"</Code></p>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Hooks, Atoms & Components", link: "#architecture" },
                Toc::Leaf { title: "Layer Overview", link: "#layer-overview" },
                Toc::Leaf { title: "Hooks", link: "#hooks" },
                Toc::Leaf { title: "Composing hooks", link: "#composing-hooks" },
                Toc::Leaf { title: "Atoms", link: "#atoms" },
                Toc::Leaf { title: "Components", link: "#components" },
                Toc::Leaf { title: "Feature Flags", link: "#feature-flags" },
            ]
        }/>
    }
}
