use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUseSeparatorHook() -> impl IntoView {
    let horizontal_sep = use_separator(UseSeparatorInput {
        orientation: SeparatorOrientation::Horizontal,
        element_type: SeparatorElementType::Hr,
    });

    let vertical_sep = use_separator(UseSeparatorInput {
        orientation: SeparatorOrientation::Vertical,
        element_type: SeparatorElementType::Div,
    });

    let div_sep = use_separator(UseSeparatorInput {
        orientation: SeparatorOrientation::Horizontal,
        element_type: SeparatorElementType::Div,
    });

    view! {
        <Article>
            <h1 id="use_separator" class="anchor">
                "use_separator"
                <AnchorLink href="#use_separator" description="Direct link to article header"/>
            </h1>

            <p>"Hook for creating accessible separators that divide content visually and semantically."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <h3>"Horizontal Separator (hr element)"</h3>
            <div style="margin: 1em 0;">
                <p>"Content above the separator"</p>
                <hr {..horizontal_sep.separator_props} style="border: none; border-top: 1px solid #ccc; margin: 1em 0;"/>
                <p>"Content below the separator"</p>
            </div>

            <h3>"Vertical Separator"</h3>
            <div style="display: flex; align-items: center; gap: 1em; margin: 1em 0;">
                <span>"Left content"</span>
                <div
                    {..vertical_sep.separator_props}
                    style="width: 1px; height: 24px; background: #ccc;"
                ></div>
                <span>"Right content"</span>
            </div>

            <h3>"Decorative Separator (div element)"</h3>
            <div style="margin: 1em 0;">
                <p>"This separator is rendered as a div with role=\"separator\""</p>
                <div
                    {..div_sep.separator_props}
                    style="height: 2px; background: linear-gradient(90deg, transparent, #ccc, transparent); margin: 1em 0;"
                ></div>
                <p>"Content continues..."</p>
            </div>

            <Code>
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
                        <hr {..hr_sep.separator_props} />
                        <div {..div_sep.separator_props}></div>
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
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_separator", link: "#use_separator" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Element Types", link: "#element-types" },
                Toc::Leaf { title: "Orientations", link: "#orientations" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
