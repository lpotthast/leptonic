use indoc::indoc;
use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, toc::Toc},
    routes,
};

#[component]
pub fn PageClassesAndStyles() -> impl IntoView {
    view! {
        <Article>
            <h1 id="classes-and-styles">
                "Classes & Styles"
                <AnchorLink href="#classes-and-styles" description="Direct link to section: Classes & Styles"/>
            </h1>

            <p>
                "Leptonic provides two standalone crates for managing CSS in Leptos component hierarchies: "
                <LinkExt href="https://github.com/lpotthast/leptos-classes" target=LinkTarget::_Blank>"leptos-classes"</LinkExt>
                " and "
                <LinkExt href="https://github.com/lpotthast/leptos-styles" target=LinkTarget::_Blank>"leptos-styles"</LinkExt>
                ". They supply the " <Code inline=true>"Classes"</Code> " and " <Code inline=true>"Styles"</Code>
                " types — composable, prop-drillable, reactive wrappers for CSS class lists and inline styles."
            </p>

            <p>
                "Both crates are independent of Leptonic and can be used in any Leptos project. "
                "Use them directly via " <Code inline=true>"leptos_classes"</Code>
                " and " <Code inline=true>"leptos_styles"</Code> "."
            </p>

            <h2 id="the-problem">
                "The Problem"
                <AnchorLink href="#the-problem" description="Direct link to section: The Problem"/>
            </h2>

            <p>
                "Leptos provides " <Code inline=true>"class=\"...\""</Code>
                " for static class strings and " <Code inline=true>"class:name=signal"</Code>
                " for reactive conditional classes on a single element. "
                "The same applies to styles with " <Code inline=true>"style=\"...\""</Code>
                " and " <Code inline=true>"style:property=value"</Code> ". "
                "These work well for styling individual elements, but they are element-level directives — "
                "they cannot be packaged into a single prop value and passed through a multi-layer component hierarchy."
            </p>

            <p>
                "When building composable component trees, each layer often needs to add its own classes or styles "
                "while preserving those from the layer above. Without a dedicated type, this requires manual string concatenation:"
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    #[component]
                    fn Wrapper(
                        #[prop(optional)] extra_class: &'static str,
                        #[prop(optional)] extra_style: &'static str,
                    ) -> impl IntoView {
                        // Manual concatenation: fragile, no reactivity, no type safety.
                        let class_str = format!("wrapper {extra_class}");
                        let style_str = format!("padding: 8px; {extra_style}");
                        view! { <div class=class_str style=style_str>"..."</div> }
                    }
                "#)}
            </Code>

            <p>
                "This quickly becomes unwieldy with deeper component trees and reactive conditions. "
                <Code inline=true>"Classes"</Code> " and " <Code inline=true>"Styles"</Code>
                " solve this by providing a first-class type for each concern."
            </p>

            <h2 id="classes">
                "Classes"
                <AnchorLink href="#classes" description="Direct link to section: Classes"/>
            </h2>

            <p>
                "The " <Code inline=true>"Classes"</Code> " type represents a list of CSS class names, "
                "each optionally gated by a reactive " <Code inline=true>"Signal<bool>"</Code> " condition."
            </p>

            <h3 id="classes-construction">"Construction"</h3>

            <p>"Classes can be created from a variety of input types thanks to broad " <Code inline=true>"From"</Code> " implementations:"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptos_classes::Classes;

                    // From a static string
                    let c: Classes = "my-class".into();

                    // From an array of strings
                    let c: Classes = ["base", "extra"].into();

                    // From a conditional tuple (class, bool)
                    let c: Classes = ("active", true).into();

                    // From a reactive tuple (class, Signal<bool>)
                    let (is_active, _) = signal(true);
                    let c: Classes = ("active", is_active).into();

                    // Using the builder
                    let c = Classes::builder()
                        .with("base")
                        .with(("highlighted", is_active))
                        .build();
                "#)}
            </Code>

            <h3 id="classes-composition">"Composition"</h3>

            <p>
                "The " <Code inline=true>".add()"</Code> " method chains additional classes onto an existing value. "
                "This is the core mechanism for prop drilling — each component layer calls " <Code inline=true>".add()"</Code>
                " to contribute its own classes before passing the value further down."
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let classes = Classes::new()
                        .add("base")
                        .add(("highlighted", is_active))  // conditional
                        .add("always-on");
                "#)}
            </Code>

            <p>
                "For toggling between two classes based on a signal, use " <Code inline=true>".add_toggle()"</Code> ":"
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let (is_dark, _) = signal(true);
                    let classes = Classes::new()
                        .add("base")
                        .add_toggle(is_dark, "theme-dark", "theme-light");
                "#)}
            </Code>

            <h3 id="classes-drilling">"Prop Drilling"</h3>

            <p>"Here is a complete three-layer example showing how classes flow through a component hierarchy:"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptos::prelude::*;
                    use leptos_classes::Classes;

                    /// The innermost component renders classes into the DOM.
                    #[component]
                    fn Inner(#[prop(into, optional)] classes: Classes) -> impl IntoView {
                        view! { <div class=classes>"Content"</div> }
                    }

                    /// An intermediate layer adds its own class.
                    #[component]
                    fn Middle(#[prop(into, optional)] classes: Classes) -> impl IntoView {
                        view! { <Inner classes=classes.add("middle-layer")/> }
                    }

                    /// The top layer creates the initial Classes value.
                    #[component]
                    fn Outer() -> impl IntoView {
                        let (highlighted, _set_highlighted) = signal(false);
                        view! {
                            <Middle classes=Classes::builder()
                                .with("outer-base")
                                .with(("highlighted", highlighted))
                                .build()
                            />
                        }
                    }
                    // Resulting class attribute: "outer-base middle-layer"
                    // When highlighted is true: "outer-base highlighted middle-layer"
                "#)}
            </Code>

            <h2 id="styles">
                "Styles"
                <AnchorLink href="#styles" description="Direct link to section: Styles"/>
            </h2>

            <p>
                "The " <Code inline=true>"Styles"</Code> " type represents a list of CSS property-value pairs, "
                "each optionally reactive."
            </p>

            <h3 id="styles-construction">"Construction"</h3>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptos_styles::Styles;

                    // From a property-value tuple
                    let s: Styles = ("color", "red").into();

                    // From a CSS string (parsed at the colon)
                    let s: Styles = "color: red".into();

                    // From an array of tuples
                    let s: Styles = [("color", "red"), ("padding", "10px")].into();

                    // Using the builder
                    let s = Styles::builder()
                        .with("color", "red")
                        .with("padding", "10px")
                        .build();
                "#)}
            </Code>

            <h3 id="styles-composition">"Composition"</h3>

            <p>
                "The " <Code inline=true>".add()"</Code> " method takes a property and a value as separate arguments, "
                "making it easy to chain:"
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let styles = Styles::new()
                        .add("border-radius", "8px")
                        .add("padding", "16px");
                "#)}
            </Code>

            <p>
                "For pre-built " <Code inline=true>"StyleEntry"</Code> " values (e.g., tuples with reactive signals), "
                "use " <Code inline=true>".add_entry()"</Code> ":"
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let (color, _set_color) = signal(Some("blue".to_string()));
                    let styles = Styles::new()
                        .add("padding", "10px")
                        .add_optional("color", color);  // reactive, excluded when None
                "#)}
            </Code>

            <h3 id="styles-reactive">"Reactive Values"</h3>

            <p>
                "Style values can be reactive via " <Code inline=true>"Signal<Option<T>>"</Code>
                ". When the signal returns " <Code inline=true>"None"</Code>
                ", the property is excluded from the output entirely. "
                "You can also pass closures as values to " <Code inline=true>".add()"</Code>
                " — they are called reactively and their return value is used as the style value:"
            </p>

            <Code language=Language::Rust>
                {indoc!(r"
                    use leptonic::utils::css::pct;
                    use leptonic::utils::style::Style;

                    let (progress, _) = signal(42.0);
                    let styles = Styles::new()
                        .add(Style::Width, move || pct(progress.get()));
                ")}
            </Code>

            <h3 id="styles-drilling">"Prop Drilling"</h3>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptos::prelude::*;
                    use leptos_styles::Styles;

                    #[component]
                    fn StyledBox(#[prop(into, optional)] styles: Styles) -> impl IntoView {
                        view! { <div style=styles>"Styled content"</div> }
                    }

                    #[component]
                    fn Card(#[prop(into, optional)] styles: Styles) -> impl IntoView {
                        view! {
                            <StyledBox styles=styles
                                .add("border-radius", "8px")
                                .add("padding", "16px")
                            />
                        }
                    }

                    #[component]
                    fn Page() -> impl IntoView {
                        view! { <Card styles=("background", "white")/> }
                    }
                    // Resulting style: "background: white; border-radius: 8px; padding: 16px"
                "#)}
            </Code>

            <h2 id="merge">
                "The merge Pattern"
                <AnchorLink href="#merge" description="Direct link to section: The merge Pattern"/>
            </h2>

            <p>
                <Code inline=true>"Styles"</Code> " provides a " <Code inline=true>".merge(other)"</Code>
                " method where " <Code inline=true>"self"</Code> " takes precedence over " <Code inline=true>"other"</Code>
                ". Conflicting properties from " <Code inline=true>"other"</Code> " are silently dropped (with a warning in debug builds)."
            </p>

            <p>
                "This is critical for hooks that manage accessibility-sensitive styles internally. "
                "For example, a button hook may set " <Code inline=true>"touch-action: none"</Code>
                " to ensure correct pointer behavior. The merge pattern ensures that user-provided styles "
                "cannot accidentally override these hook-managed properties:"
            </p>

            <Code language=Language::Rust>
                {indoc!(r"
                    // Inside an atom component (e.g., atoms/button.rs)
                    let (button_attrs, button_styles) = button_props.into_parts();

                    // Hook styles take precedence. User styles are added
                    // only if they don't conflict.
                    let styles = button_styles.merge(styles);

                    view! {
                        <button {..button_attrs} class=classes style=styles>
                            {children()}
                        </button>
                    }
                ")}
            </Code>

            <p>
                "When writing your own components on top of Leptonic atoms, you generally do not need to call "
                <Code inline=true>".merge()"</Code> " yourself — just pass " <Code inline=true>"styles"</Code>
                " down and the atom layer handles merging with hook styles."
            </p>

            <h2 id="css-values">
                "CSS Value Types"
                <AnchorLink href="#css-values" description="Direct link to section: CSS Value Types"/>
            </h2>

            <p>
                "The " <Code inline=true>"leptos-styles"</Code> " crate also provides a typed CSS value system, "
                "re-exported through " <Code inline=true>"leptonic::utils::css"</Code> "."
            </p>

            <p>
                <Code inline=true>"CssDimension"</Code>
                " covers common size values with convenience constructors:"
            </p>

            <Code language=Language::Rust>
                {indoc!(r"
                    use leptonic::utils::css::{px, em, rem, pct};

                    let width = pct(100.0);     // 100%
                    let padding = em(1.0);       // 1em
                    let max_width = px(800);     // 800px
                    let font_size = rem(1.125);  // 1.125rem
                ")}
            </Code>

            <p>
                "The broader " <Code inline=true>"CssValue"</Code> " enum covers CSS keywords, colors, "
                "custom properties, and calc expressions:"
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptonic::utils::css::CssValue;

                    let auto = CssValue::Auto;                        // auto
                    let var = CssValue::Var("--accent".into(), None);  // var(--accent)
                "#)}
            </Code>

            <p>
                "The " <Code inline=true>"Style"</Code> " enum provides type-safe CSS property names, "
                "preventing typos. It can be used both with " <Code inline=true>".add()"</Code>
                " and the builder:"
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptonic::utils::style::Style;
                    use leptonic::utils::css::pct;
                    use leptos_styles::Styles;

                    let styles = Styles::new()
                        .add(Style::Width, pct(100.0))
                        .add(Style::Display, "flex");
                "#)}
            </Code>

            <h2 id="leptonic-usage">
                "How Leptonic Uses These Types"
                <AnchorLink href="#leptonic-usage" description="Direct link to section: How Leptonic Uses These Types"/>
            </h2>

            <p>
                "Every Leptonic component and atom accepts "
                <Code inline=true>"#[prop(into, optional)] classes: Classes"</Code> " and "
                <Code inline=true>"#[prop(into, optional)] styles: Styles"</Code> " props. "
                "The " <Code inline=true>"into"</Code> " attribute lets you pass a plain string, a tuple, "
                "an array, or a fully constructed " <Code inline=true>"Classes"</Code> " / "
                <Code inline=true>"Styles"</Code> " value — whichever is most convenient."
            </p>

            <p>
                "As described in the "
                <Link href=routes::doc::Architecture.materialize()>"Architecture"</Link>
                " page, Leptonic follows a three-layer hierarchy: hooks, atoms, and components. "
                "Classes and styles flow through these layers:"
            </p>

            <ul>
                <li>
                    <strong>"Hook layer"</strong>
                    ": Returns managed attributes and styles (e.g., "
                    <Code inline=true>"touch-action"</Code>", ARIA attributes)."
                </li>
                <li>
                    <strong>"Atom layer"</strong>
                    ": Accepts " <Code inline=true>"classes"</Code> " and " <Code inline=true>"styles"</Code>
                    " props, merges hook-managed styles with user-provided styles via "
                    <Code inline=true>".merge()"</Code> ", and renders the final element."
                </li>
                <li>
                    <strong>"Component layer"</strong>
                    ": Adds a theme class (e.g., " <Code inline=true>"\"leptonic-btn\""</Code>
                    ") via " <Code inline=true>".add()"</Code> " and passes everything down to the atom."
                </li>
            </ul>

            <p>"Example flow for a Button:"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    // Component layer: adds theme class, passes to atom
                    <atoms::button::Button
                        classes=classes.add("leptonic-btn")
                        styles=styles
                    />

                    // Atom layer: merges hook styles, renders element
                    let (button_attrs, button_styles) = button_props.into_parts();
                    let styles = button_styles.merge(styles);

                    view! {
                        <button {..button_attrs} class=classes style=styles>
                            {children()}
                        </button>
                    }
                "#)}
            </Code>

            <h2 id="comparison">
                "Comparison with Native Leptos"
                <AnchorLink href="#comparison" description="Direct link to section: Comparison with Native Leptos"/>
            </h2>

            <table>
                <thead>
                    <tr>
                        <th>"Capability"</th>
                        <th>"Native Leptos"</th>
                        <th>"Classes / Styles"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Static class"</td>
                        <td><Code inline=true>"class=\"foo\""</Code></td>
                        <td><Code inline=true>"classes=\"foo\""</Code></td>
                    </tr>
                    <tr>
                        <td>"Conditional class"</td>
                        <td><Code inline=true>"class:foo=signal"</Code></td>
                        <td><Code inline=true>".add((\"foo\", signal))"</Code></td>
                    </tr>
                    <tr>
                        <td>"Pass classes as prop"</td>
                        <td>"No built-in support"</td>
                        <td><Code inline=true>"#[prop(into, optional)] classes: Classes"</Code></td>
                    </tr>
                    <tr>
                        <td>"Multi-layer drilling"</td>
                        <td>"Manual string concatenation"</td>
                        <td><Code inline=true>".add()"</Code>" at each layer"</td>
                    </tr>
                    <tr>
                        <td>"Toggle between two classes"</td>
                        <td>"Two " <Code inline=true>"class:"</Code>" directives"</td>
                        <td><Code inline=true>".add_toggle(signal, \"a\", \"b\")"</Code></td>
                    </tr>
                    <tr>
                        <td>"Static inline style"</td>
                        <td><Code inline=true>"style=\"color: red\""</Code></td>
                        <td><Code inline=true>"styles=(\"color\", \"red\")"</Code></td>
                    </tr>
                    <tr>
                        <td>"Reactive style"</td>
                        <td><Code inline=true>"style:color=signal"</Code></td>
                        <td><Code inline=true>".add(Style::Color, signal)"</Code></td>
                    </tr>
                    <tr>
                        <td>"Merge hook + user styles"</td>
                        <td>"Not supported"</td>
                        <td><Code inline=true>".merge(other)"</Code></td>
                    </tr>
                    <tr>
                        <td>"Typed CSS values"</td>
                        <td>"Strings only"</td>
                        <td><Code inline=true>"px()"</Code>", "<Code inline=true>"em()"</Code>", "<Code inline=true>"Style::Width"</Code>" etc."</td>
                    </tr>
                </tbody>
            </table>

            <p>
                "If you are styling a single element without component composition, native Leptos directives like "
                <Code inline=true>"class=\"foo\""</Code> " and " <Code inline=true>"style:color=\"red\""</Code>
                " are perfectly fine. "
                <Code inline=true>"Classes"</Code> " and " <Code inline=true>"Styles"</Code>
                " become valuable when building reusable, multi-layer component hierarchies where "
                "classes and styles need to flow through and accumulate across component boundaries."
            </p>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Classes & Styles", link: "#classes-and-styles" },
                Toc::Leaf { title: "The Problem", link: "#the-problem" },
                Toc::Leaf { title: "Classes", link: "#classes" },
                Toc::Leaf { title: "Styles", link: "#styles" },
                Toc::Leaf { title: "The merge Pattern", link: "#merge" },
                Toc::Leaf { title: "CSS Value Types", link: "#css-values" },
                Toc::Leaf { title: "How Leptonic Uses These Types", link: "#leptonic-usage" },
                Toc::Leaf { title: "Comparison with Native Leptos", link: "#comparison" },
            ]
        }/>
    }
}
