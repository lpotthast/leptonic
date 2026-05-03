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
                    use leptonic::hooks::{use_button, UseButtonInput, UseButtonReturn};

                    let UseButtonReturn { props, .. } = use_button(UseButtonInput {
                        // ...
                    });

                    view! {
                        <div {..props.into_attrs()}>
                            "My custom button"
                        </div>
                    }
                "#)}
            </Code>

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
                Toc::Leaf { title: "Atoms", link: "#atoms" },
                Toc::Leaf { title: "Components", link: "#components" },
                Toc::Leaf { title: "Feature Flags", link: "#feature-flags" },
            ]
        }/>
    }
}
