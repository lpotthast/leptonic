use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::separator_decorative::SeparatorDecorativeDemo;
use super::demos::separator_horizontal::SeparatorHorizontalDemo;
use super::demos::separator_vertical::SeparatorVerticalDemo;

#[component]
pub fn PageUseSeparatorHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_separator" class="anchor">
                "use_separator"
                <AnchorLink href="#use_separator" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_separator"</Code>" hook creates accessible separators that divide content visually and semantically. "
                "See the "<Link href=crate::routes::doc::Separator.materialize()>"Separator overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useSeparator.html" target=LinkTarget::_Blank>
                    "useSeparator"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <h3>"Horizontal Separator (hr element)"</h3>
            <DemoShell source=include_str!("demos/separator_horizontal.rs")>
                <SeparatorHorizontalDemo />
            </DemoShell>

            <h3>"Vertical Separator"</h3>
            <DemoShell source=include_str!("demos/separator_vertical.rs")>
                <SeparatorVerticalDemo />
            </DemoShell>

            <h3>"Decorative Separator (div element)"</h3>
            <DemoShell source=include_str!("demos/separator_decorative.rs")>
                <SeparatorDecorativeDemo />
            </DemoShell>

            <Code language=Language::Rust>
                {indoc!(r#"
                    // Using <hr> element (no role needed)
                    let hr_sep = use_separator(UseSeparatorInput {
                        orientation: SeparatorOrientation::Horizontal,
                        element_type: SeparatorElementType::Hr,
                    });

                    // Using <div> element (role="separator" is added)
                    let div_sep = use_separator(UseSeparatorInput {
                        orientation: SeparatorOrientation::Vertical,
                        element_type: SeparatorElementType::Div,
                    });

                    view! {
                        <hr {..hr_sep.separator_props.into_attrs()} />
                        <div {..div_sep.separator_props.into_attrs()}></div>
                    }
                "#)}
            </Code>

            <h2 id="element-types" class="anchor">
                "Element Types"
                <AnchorLink href="#element-types" description="Direct link to element types"/>
            </h2>

            <p>"The separator can be rendered using different elements:"</p>
            <ul>
                <li><code>"SeparatorElementType::Hr"</code> " - Semantic " <code>"<hr>"</code> " element (default)"</li>
                <li><code>"SeparatorElementType::Div"</code> " - Div with role=\"separator\""</li>
                <li><code>"SeparatorElementType::Span"</code> " - Span with role=\"separator\""</li>
            </ul>

            <p>
                "When using " <code>"<hr>"</code> ", no additional ARIA attributes are needed because the element is inherently semantic. "
                "For other elements, the hook adds " <code>"role=\"separator\""</code> " and " <code>"aria-orientation"</code> "."
            </p>

            <h2 id="orientations" class="anchor">
                "Orientations"
                <AnchorLink href="#orientations" description="Direct link to orientations"/>
            </h2>

            <ul>
                <li><code>"SeparatorOrientation::Horizontal"</code> " - Divides content vertically (default)"</li>
                <li><code>"SeparatorOrientation::Vertical"</code> " - Divides content horizontally"</li>
            </ul>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"For non-hr elements, the hook sets:"</p>
            <ul>
                <li><code>"role=\"separator\""</code></li>
                <li><code>"aria-orientation"</code> " (horizontal or vertical)"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Semantic " <code>"<hr>"</code> " support"</li>
                <li>"Custom element support with proper ARIA"</li>
                <li>"Horizontal and vertical orientations"</li>
                <li>"Minimal, focused API"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Separator.materialize()>"Separator overview"</Link></li>
                <li><Link href=crate::routes::doc::separator::Component.materialize()>"Separator component"</Link></li>
                <li><Link href=crate::routes::doc::LayoutCategory.materialize()>"Layout domain"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_separator", link: "#use_separator" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Element Types", link: "#element-types" },
                Toc::Leaf { title: "Orientations", link: "#orientations" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
