use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

use super::demos::color_swatch::ColorSwatchDemo;

#[component]
pub fn PageUseColorSwatch() -> impl IntoView {
    view! {
        <Article>
            <p style="margin-bottom: 0.5em;">
                <Link href=routes::doc::color::Hooks.materialize()>"\u{2190} Color Hooks Overview"</Link>
            </p>

            <h1 id="use_color_swatch" class="anchor">
                "use_color_swatch"
                <AnchorLink href="#use_color_swatch" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_color_swatch"</Code>
                " hook provides accessible ARIA props for a display-only color swatch."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useColorSwatch.html" target=LinkTarget::_Blank>
                    "useColorSwatch"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell
                source=include_str!("demos/color_swatch.rs")
                description="Display-only color swatch"
            >
                <ColorSwatchDemo />
            </DemoShell>

            <h2 id="api" class="anchor">
                "API"
                <AnchorLink href="#api" description="Direct link to API section"/>
            </h2>

            <h3>"Input"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorSwatchInput {
                        pub color: Signal<RGB8>,
                        pub color_name: Option<Signal<String>>,
                        pub aria_label: Option<String>,
                    }
                ")}
            </Code>

            <h3>"Return"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorSwatchReturn {
                        pub props: UseColorSwatchProps,
                        pub background_color: Signal<String>,
                    }
                ")}
            </Code>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"The hook automatically sets:"</p>
            <ul>
                <li><code>"role=\"img\""</code></li>
                <li><code>"aria-roledescription=\"color swatch\""</code></li>
                <li><code>"aria-label"</code> " (hex representation or custom label)"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=routes::doc::hooks::UseColorField.materialize()>"use_color_field"</Link>" - Hex color text input"</li>
                <li><Link href=routes::doc::hooks::UseColorArea.materialize()>"use_color_area"</Link>" - 2D color area"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_color_swatch", link: "#use_color_swatch" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "API", link: "#api" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
