use indoc::indoc;
use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

use super::demos::classes_and_styles_meter::ClassesAndStylesMeterDemo;
use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

#[component]
pub fn PageClassesAndStyles() -> impl IntoView {
    view! {
        <Article>
            <h1 id="classes-and-styles">
                "Classes & Styles"
                <AnchorLink
                    href="#classes-and-styles"
                    description="Direct link to section: Classes & Styles"
                />
            </h1>

            <p>
                "Every Leptonic atom and component takes a " <Code inline=true>"classes"</Code>
                " and a " <Code inline=true>"styles"</Code>
                " prop. Their types come from two small companion crates, "
                <LinkExt
                    href="https://github.com/lpotthast/leptos-classes"
                    target=LinkTarget::_Blank
                >
                    "leptos-classes"
                </LinkExt> " and "
                <LinkExt
                    href="https://github.com/lpotthast/leptos-styles"
                    target=LinkTarget::_Blank
                >
                    "leptos-styles"
                </LinkExt> ". " <Code inline=true>"Classes"</Code>
                " holds a list of class names and " <Code inline=true>"Styles"</Code>
                " holds inline style declarations. You can hand either one from component to component, add to it along the way, "
                "and both stay reactive until they are finally rendered."
            </p>

            <p>
                "Neither crate depends on Leptonic, so you can use them in any Leptos project. Leptonic re-exports them as "
                <Code inline=true>"leptonic::utils::classes"</Code> " and "
                <Code inline=true>"leptonic::utils::styles"</Code>
                ". The typed CSS values are re-exported as "
                <Code inline=true>"leptonic::utils::css"</Code> " and the property selectors as "
                <Code inline=true>"leptonic::utils::style"</Code> "."
            </p>

            <h2 id="the-problem">
                "The Problem"
                <AnchorLink href="#the-problem" description="Direct link to section: The Problem" />
            </h2>

            <p>
                "On a single element, Leptos already gives you everything you need: "
                <Code inline=true>"class=\"...\""</Code> " and "
                <Code inline=true>"class:name=signal"</Code> " for classes, "
                <Code inline=true>"style=\"...\""</Code> " and "
                <Code inline=true>"style:property=value"</Code>
                " for styles. These are attributes on one element, though. "
                "You can't bundle them into a single value and pass that value through several components."
            </p>

            <p>
                "In a component library, that's exactly what you need to do. Each layer wants to add a little of its own "
                "while keeping what the caller passed in. With plain strings, you end up concatenating by hand:"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    #[component]
                    fn Wrapper(
                        #[prop(optional)] extra_class: &'static str,
                        #[prop(optional)] extra_style: &'static str,
                    ) -> impl IntoView {
                        // Manual concatenation: fragile, not reactive, and nothing checks the CSS.
                        let class_str = format!("wrapper {extra_class}");
                        let style_str = format!("padding: 8px; {extra_style}");
                        view! { <div class=class_str style=style_str>"..."</div> }
                    }
                "#
                )}
            </Code>

            <p>
                "This gets messy quickly once conditions and deeper trees are involved. "
                <Code inline=true>"Classes"</Code> " and " <Code inline=true>"Styles"</Code>
                " give each of these concerns a proper type."
            </p>

            <h2 id="classes">
                "Classes"
                <AnchorLink href="#classes" description="Direct link to section: Classes" />
            </h2>

            <p>
                "A " <Code inline=true>"Classes"</Code> " value is an ordered list of class names. "
                "Each name is either always present or tied to a reactive condition."
            </p>

            <h3 id="classes-construction">"Construction"</h3>

            <p>
                "Most of the time you'll start from a string, or let the "
                <Code inline=true>"#[prop(into)]"</Code>
                " conversion do it for you. From there, you chain "
                <Code inline=true>".add()"</Code>
                " for static names, or use the builder if you prefer that style:"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    use leptonic::utils::classes::Classes;

                    // From a single name or an array of names.
                    let c = Classes::from("card");
                    let c = Classes::from(["card", "elevated"]);

                    // From a whitespace-separated string you only know at runtime.
                    let c = Classes::parse("card elevated");

                    // Chaining ...
                    let c = Classes::from("card").add("elevated");

                    // ... or the builder.
                    let c = Classes::builder().with("card").with("elevated").build();
                "#
                )}
            </Code>

            <p>
                "Each class name is a single token. An empty name or one containing whitespace panics. "
                "That's an easy one to trip over in props: " <Code inline=true>"classes=\"card elevated\""</Code>
                " panics, so write " <Code inline=true>"classes=[\"card\", \"elevated\"]"</Code>
                " instead. Adding the same name twice to one " <Code inline=true>"Classes"</Code>
                " value panics as well. If a class should depend on several things, combine them into one condition "
                "rather than registering the name twice. For untrusted runtime input, "
                <Code inline=true>"ClassName::try_new"</Code>
                " lets you validate without panicking."
            </p>

            <h3 id="classes-reactive">"Conditional Classes"</h3>

            <p>
                "Use " <Code inline=true>".add_reactive(name, when)"</Code>
                " for a class that should only be present some of the time. "
                "The condition can be a plain " <Code inline=true>"bool"</Code>
                ", a signal, a memo, or a closure. "
                "When exactly one of two classes should be active, use "
                <Code inline=true>".add_toggle()"</Code> " instead:"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    let (is_active, _) = signal(true);
                    let (is_disabled, _) = signal(false);
                    let (is_dark, _) = signal(true);

                    let classes = Classes::from("item")
                        .add_reactive("active", is_active)
                        .add_reactive("interactive", move || is_active.get() && !is_disabled.get())
                        .add_toggle(is_dark, "theme-dark", "theme-light");
                "#
                )}
            </Code>

            <h3 id="classes-drilling">"Prop Drilling"</h3>

            <p>
                "This is what the type is really for. Every layer receives a "
                <Code inline=true>"Classes"</Code>
                ", adds what it needs, and passes it on. Only the innermost component renders it with "
                <Code inline=true>"class=classes"</Code> ":"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    use leptonic::utils::classes::Classes;
                    use leptos::prelude::*;

                    /// The innermost component renders the accumulated classes.
                    #[component]
                    fn Inner(#[prop(into, optional)] classes: Classes) -> impl IntoView {
                        view! { <div class=classes>"Content"</div> }
                    }

                    /// An intermediate layer adds its own class.
                    #[component]
                    fn Middle(#[prop(into, optional)] classes: Classes) -> impl IntoView {
                        view! { <Inner classes=classes.add("middle-layer")/> }
                    }

                    /// The top layer creates the initial value.
                    #[component]
                    fn Outer() -> impl IntoView {
                        let (highlighted, _set_highlighted) = signal(false);
                        view! {
                            <Middle classes=Classes::from("outer-base").add_reactive("highlighted", highlighted)/>
                        }
                    }
                    // Renders class="outer-base middle-layer",
                    // or class="outer-base highlighted middle-layer" while `highlighted` is true.
                "#
                )}
            </Code>

            <p>
                "When you're handed two " <Code inline=true>"Classes"</Code>
                " values that were built independently, for example a prop and the result of a helper function, combine them with "
                <Code inline=true>".merge(other, MergeStrategy::default())"</Code>
                ". The default strategy keeps a class that appears in both if " <em>"either"</em>
                " condition holds. If you control both sides, just keep chaining "
                <Code inline=true>".add*()"</Code> " calls instead."
            </p>

            <h3 id="classes-ownership">"One Owner per Attribute"</h3>

            <p>
                <Code inline=true>"class=classes"</Code>
                " takes over the element's entire "
                <Code inline=true>"class"</Code>
                " attribute and rewrites all of it whenever something changes. A "
                <Code inline=true>"class:foo=..."</Code>
                " directive on the same element would be overwritten on the next update, so don't mix the two. "
                "Move the condition into the "
                <Code inline=true>"Classes"</Code>
                " value instead:"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    // Don't: the directive and `Classes` both write the `class` attribute.
                    view! { <div class=classes class:selected=is_selected/> }

                    // Do: one owner for the whole attribute.
                    view! { <div class=classes.add_reactive("selected", is_selected)/> }
                "#
                )}
            </Code>

            <h2 id="styles">
                "Styles" <AnchorLink href="#styles" description="Direct link to section: Styles" />
            </h2>

            <p>
                "A " <Code inline=true>"Styles"</Code>
                " value is a list of CSS declarations. Each one can be static, "
                "reactive, or only present some of the time. Declarations are typed: you pick a property through a "
                <em>"property selector"</em>
                " and pass it a value of the matching type. A padding can't end up with a color, "
                "a typo in a property name doesn't compile, and you never have to format a CSS string yourself."
            </p>

            <h3 id="styles-declarations">"Typed Declarations"</h3>

            <p>
                "Property selectors such as " <Code inline=true>"WidthProperty"</Code> " or "
                <Code inline=true>"BackgroundColorProperty"</Code> " build a declaration with "
                <Code inline=true>".declare(value)"</Code> ". The values come from the "
                <Code inline=true>"css"</Code> " module, with helpers like "
                <Code inline=true>"px()"</Code> ", " <Code inline=true>"rgb()"</Code> " and "
                <Code inline=true>"Padding::all()"</Code> ":"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    use leptonic::utils::{
                        css::{CssColor, CssColorName, Padding, px, rgb},
                        style::{BackgroundColorProperty, ColorProperty, PaddingProperty},
                        styles::Styles,
                    };

                    let styles = Styles::new()
                        .add(ColorProperty.declare(CssColor::Named(CssColorName::White)))
                        .add(BackgroundColorProperty.declare(rgb(74, 144, 217)))
                        .add(PaddingProperty.declare(Padding::all(px(16))));

                    // Renders style="color:white;background-color:rgb(74, 144, 217);padding:16px;"

                    // This does not compile: `padding` does not accept a color.
                    // PaddingProperty.declare(rgb(74, 144, 217));
                "#
                )}
            </Code>

            <p>
                "The same works with the builder ("
                <Code inline=true>"Styles::builder().with(...).build()"</Code>
                "). Because a single declaration, or an array of them, converts into "
                <Code inline=true>"Styles"</Code> ", you can also pass one straight into a "
                <Code inline=true>"styles"</Code> " prop, e.g. "
                <Code inline=true>"styles=OpacityProperty.declare(Opacity::new(0.5))"</Code> "."
            </p>

            <h3 id="styles-reactive">"Reactive and Optional Declarations"</h3>

            <p>
                "For a value that changes over time, use "
                <Code inline=true>".add_reactive()"</Code>
                " with a closure that builds the whole declaration. If the declaration should sometimes be left out entirely, use "
                <Code inline=true>".add_optional()"</Code> " and return an "
                <Code inline=true>"Option"</Code> ". While it returns "
                <Code inline=true>"None"</Code> ", the property doesn't show up in the "
                <Code inline=true>"style"</Code> " attribute at all:"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r"
                    use leptonic::utils::{
                        css::{CssDimension, NonNegativeLengthPercentage, Size, rgb, try_pct},
                        style::{BackgroundColorProperty, WidthProperty},
                        styles::Styles,
                    };

                    let (progress, _) = signal(42.0);
                    let (highlight, _) = signal(false);

                    let styles = Styles::new()
                        .add_reactive(move || WidthProperty.declare(size_pct(progress.get())))
                        .add_optional(move || {
                            highlight
                                .get()
                                .then(|| BackgroundColorProperty.declare(rgb(255, 240, 200)))
                        });

                    /// `width` takes a non-negative `Size`. Computed numbers can be NaN or negative,
                    /// so convert them fallibly and fall back to zero.
                    fn size_pct(value: f64) -> Size {
                        try_pct(value)
                            .ok()
                            .and_then(|dim| NonNegativeLengthPercentage::try_from(dim).ok())
                            .unwrap_or_else(|| NonNegativeLengthPercentage::new(CssDimension::Zero))
                            .into()
                    }
                "
                )}
            </Code>

            <p>
                "That helper isn't just ceremony. Functions like " <Code inline=true>"pct()"</Code>
                " and " <Code inline=true>"px()"</Code> " panic on non-finite numbers, and "
                <Code inline=true>"NonNegativeLengthPercentage::new"</Code>
                " panics on negative ones. For literals and values you know are in range, that's fine. "
                "For anything computed at runtime, like a slider position when "
                <Code inline=true>"min == max"</Code> ", use the " <Code inline=true>"try_"</Code>
                " variants and decide on a fallback."
            </p>

            <h3 id="styles-custom-properties">"Custom Properties"</h3>

            <p>
                "CSS custom properties (variables) are typed too. Declare one once with "
                <Code inline=true>"css_custom_property!"</Code>
                ", and it will only accept values of the type you gave it. This is a nice way to hand a live value to a stylesheet "
                "while the stylesheet stays in charge of how it's used:"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    use leptonic::utils::{
                        css::{CssColor, CssColorName, css_custom_property, rgb, var},
                        style::ColorProperty,
                        styles::Styles,
                    };

                    css_custom_property!(ACCENT: CssColor = "--accent");

                    // Set the variable ...
                    let styles = Styles::new().add(ACCENT.declare(rgb(230, 105, 86)));

                    // ... or reference it (a fallback of the same type is required).
                    let styles = Styles::new()
                        .add(ColorProperty.declare(var(&ACCENT, CssColor::Named(CssColorName::Black))));
                "#
                )}
            </Code>

            <h3 id="styles-unchecked">"The Unchecked Escape Hatch"</h3>

            <p>
                "The set of typed properties is intentionally growing step by step and doesn't cover all of CSS yet. "
                <Code inline=true>"position"</Code> ", " <Code inline=true>"display"</Code> ", "
                <Code inline=true>"transform"</Code> ", " <Code inline=true>"background"</Code>
                " gradients and many others aren't modeled. For those, there are explicitly "
                "named " <Code inline=true>"_unchecked"</Code>
                " methods that take the property name and value as plain strings. The long name is on purpose: "
                "it makes the spots that skip the type checks easy to find."
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    let (is_open, _) = signal(false);
                    let (gradient, _) = signal(String::from("linear-gradient(to right, red, blue)"));

                    let styles = Styles::new()
                        .add_unchecked("position", "absolute")
                        .add_optional_unchecked("transform", move || is_open.get().then_some("rotate(180deg)"))
                        // An always-present reactive unchecked value is written as an optional one that is always `Some`.
                        .add_optional_unchecked("background", move || Some(gradient.get()));
                "#
                )}
            </Code>

            <p>
                "Note that there is no conversion from "
                <Code inline=true>"(\"property\", \"value\")"</Code> " tuples or raw strings into "
                <Code inline=true>"Styles"</Code>
                ". Every unchecked declaration has to go through one of these methods."
            </p>

            <h3 id="styles-drilling">"Prop Drilling"</h3>

            <p>
                "Prop drilling works the same way as with classes. Each layer adds its declarations and passes the value on:"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    use leptonic::utils::{
                        css::{BorderCornerRadius, Padding, px, rgb},
                        style::{BackgroundColorProperty, BorderStartStartRadiusProperty, PaddingProperty},
                        styles::Styles,
                    };
                    use leptos::prelude::*;

                    #[component]
                    fn StyledBox(#[prop(into, optional)] styles: Styles) -> impl IntoView {
                        view! { <div style=styles>"Styled content"</div> }
                    }

                    #[component]
                    fn Card(#[prop(into, optional)] styles: Styles) -> impl IntoView {
                        view! {
                            <StyledBox styles=styles
                                .add(PaddingProperty.declare(Padding::all(px(16))))
                                .add(BorderStartStartRadiusProperty.declare(BorderCornerRadius::circular(px(8))))
                            />
                        }
                    }

                    #[component]
                    fn Page() -> impl IntoView {
                        view! { <Card styles=BackgroundColorProperty.declare(rgb(255, 255, 255))/> }
                    }
                    // Renders style="background-color:rgb(255, 255, 255);padding:16px;border-start-start-radius:8px;"
                "#
                )}
            </Code>

            <p>
                "Just like " <Code inline=true>"class=classes"</Code> ", "
                <Code inline=true>"style=styles"</Code> " owns the whole "
                <Code inline=true>"style"</Code> " attribute. Don't combine it with "
                <Code inline=true>"style:foo=..."</Code>
                " directives on the same element. Add the declaration to the "
                <Code inline=true>"Styles"</Code> " value instead."
            </p>

            <h2 id="merge">
                "Merge Priority"
                <AnchorLink href="#merge" description="Direct link to section: Merge Priority" />
            </h2>

            <p>
                <Code inline=true>"a.merge(b)"</Code>
                " combines two "
                <Code inline=true>"Styles"</Code>
                " values, with "
                <Code inline=true>"b"</Code>
                " acting as a lower-priority fallback layer. Conflicts are resolved per property: "
                "when "
                <Code inline=true>"a"</Code>
                " currently has a declaration for a property, it wins. When it doesn't, "
                <Code inline=true>"b"</Code>
                "'s declaration shows through. This is evaluated on every update, so a fallback "
                "reappears as soon as an optional declaration in "
                <Code inline=true>"a"</Code>
                " turns into "
                <Code inline=true>"None"</Code>
                "."
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    let defaults = Styles::new()
                        .add(ColorProperty.declare(CssColor::Named(CssColorName::Blue)))
                        .add(PaddingProperty.declare(Padding::all(px(16))));
                    let local = Styles::new()
                        .add(ColorProperty.declare(CssColor::Named(CssColorName::Red)));

                    // Renders style="color:red;padding:16px;"
                    let styles = local.merge(defaults);
                "#
                )}
            </Code>

            <p>
                "Leptonic's atoms use this to protect styles that hooks rely on. A hook may, for example, set "
                <Code inline=true>"touch-action: none"</Code>
                " to get pointer handling right. The atom puts the hook's styles first, so your own styles fill in "
                "everything else but can't accidentally override them:"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r"
                    // Inside an atom component (e.g., atoms/button.rs)
                    let (button_attrs, button_styles) = button_props.into_parts();

                    // Hook styles take precedence; user styles are the fallback layer.
                    let styles = button_styles.merge(styles);

                    view! {
                        <button {..button_attrs} class=classes style=styles>
                            {children()}
                        </button>
                    }
                "
                )}
            </Code>

            <p>
                "If you build your own components on top of Leptonic atoms, you usually won't need "
                <Code inline=true>".merge()"</Code> " yourself. Pass "
                <Code inline=true>"styles"</Code> " down and let the atom do the merging."
            </p>

            <h2 id="example">
                "Live Example"
                <AnchorLink href="#example" description="Direct link to section: Live Example" />
            </h2>

            <p>
                "Here's everything together. The meter's look is defined by CSS classes, and the "
                <Code inline=true>"complete"</Code>
                " class is toggled reactively. The only inline styles are the two values that really are dynamic: "
                "a typed " <Code inline=true>"width"</Code>
                " and a typed custom property for the fill color."
            </p>

            <DemoShell source=include_str!("demos/classes_and_styles_meter.rs")>
                <ClassesAndStylesMeterDemo />
            </DemoShell>

            <h2 id="css-values">
                "CSS Value Types"
                <AnchorLink
                    href="#css-values"
                    description="Direct link to section: CSS Value Types"
                />
            </h2>

            <p>
                "The value types follow the CSS grammar of the property they belong to, so you'll come across a handful of them. "
                "Most of them start from a " <Code inline=true>"CssDimension"</Code> ":"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r"
                    use leptonic::utils::css::{
                        CssDimension, LengthPercentageAuto, Margin, NonNegativeLengthPercentage, Size, em, pct,
                        px, rem, try_px,
                    };

                    let a: CssDimension = pct(100.0); // 100%
                    let b = em(1.0);                  // 1em
                    let c = px(800);                  // 800px
                    let d = rem(1.125);               // 1.125rem
                    let e = try_px(f64::NAN);         // Err(..) instead of a panic

                    // `width`, `height` & co. take a non-negative `Size`.
                    let width: Size = NonNegativeLengthPercentage::new(pct(50.0)).into();

                    // Insets (`top`, `left`, ...) and margins also accept `auto` and negative values.
                    let left = LengthPercentageAuto::from(px(-4));
                    let margin = Margin::Right(em(1.0).into());
                "
                )}
            </Code>

            <p>
                "Colors are built with " <Code inline=true>"rgb()"</Code> ", "
                <Code inline=true>"rgba()"</Code> ", " <Code inline=true>"hsl()"</Code> " or "
                <Code inline=true>"CssColor::Named(..)"</Code> ". Leptonic's own color types ("
                <Code inline=true>"RGB8"</Code> ", " <Code inline=true>"HSL"</Code>
                ") convert into " <Code inline=true>"CssColor"</Code> " with "
                <Code inline=true>".into()"</Code> "."
            </p>

            <h2 id="leptonic-usage">
                "How Leptonic Uses These Types"
                <AnchorLink
                    href="#leptonic-usage"
                    description="Direct link to section: How Leptonic Uses These Types"
                />
            </h2>

            <p>
                "Every Leptonic atom and component declares "
                <Code inline=true>"#[prop(into, optional)] classes: Classes"</Code> " and "
                <Code inline=true>"#[prop(into, optional)] styles: Styles"</Code> ". Thanks to "
                <Code inline=true>"into"</Code> ", you can pass a plain "
                <Code inline=true>"\"class-name\""</Code>
                ", an array of names, a single typed declaration, or a fully built value, whichever is handiest."
            </p>

            <p>
                "As described on the "
                <Link href=routes::doc::Architecture
                    .materialize()>"Hooks, Atoms & Components"</Link>
                " page, Leptonic is built in three layers. Each of them has its own part in this. "
                "Hooks return the attributes and styles they manage, such as ARIA attributes or "
                <Code inline=true>"touch-action"</Code> ". Atoms accept your "
                <Code inline=true>"classes"</Code> " and " <Code inline=true>"styles"</Code>
                ", merge the hook styles in front of yours, and render the element. "
                "Components add their theme class, for example "
                <Code inline=true>"\"leptonic-btn\""</Code>
                ", and pass everything down to the atom:"
            </p>

            <Code language=Language::Rust>
                {indoc!(
                    r#"
                    // Component layer: adds the theme class, passes everything to the atom.
                    <atoms::button::Button
                        classes=classes.add("leptonic-btn")
                        styles=styles
                    />

                    // Atom layer: hook styles first, then renders the element.
                    let (button_attrs, button_styles) = button_props.into_parts();
                    let styles = button_styles.merge(styles);

                    view! {
                        <button {..button_attrs} class=classes style=styles>
                            {children()}
                        </button>
                    }
                "#
                )}
            </Code>

            <h2 id="comparison">
                "Comparison with Native Leptos"
                <AnchorLink
                    href="#comparison"
                    description="Direct link to section: Comparison with Native Leptos"
                />
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
                        <td>
                            <Code inline=true>"class=\"foo\""</Code>
                        </td>
                        <td>
                            <Code inline=true>"classes=\"foo\""</Code>
                        </td>
                    </tr>
                    <tr>
                        <td>"Conditional class"</td>
                        <td>
                            <Code inline=true>"class:foo=signal"</Code>
                        </td>
                        <td>
                            <Code inline=true>".add_reactive(\"foo\", signal)"</Code>
                        </td>
                    </tr>
                    <tr>
                        <td>"Toggle between two classes"</td>
                        <td>"Two " <Code inline=true>"class:"</Code>" directives"</td>
                        <td>
                            <Code inline=true>".add_toggle(signal, \"a\", \"b\")"</Code>
                        </td>
                    </tr>
                    <tr>
                        <td>"Pass classes / styles as a prop"</td>
                        <td>"No built-in support"</td>
                        <td>
                            <Code inline=true>"classes: Classes"</Code>
                            ", "
                            <Code inline=true>"styles: Styles"</Code>
                        </td>
                    </tr>
                    <tr>
                        <td>"Multi-layer drilling"</td>
                        <td>"Manual string concatenation"</td>
                        <td>
                            <Code inline=true>".add(..)"</Code>
                            " at each layer"
                        </td>
                    </tr>
                    <tr>
                        <td>"Static inline style"</td>
                        <td>
                            <Code inline=true>"style=\"color: red\""</Code>
                        </td>
                        <td>
                            <Code inline=true>"ColorProperty.declare(..)"</Code>
                        </td>
                    </tr>
                    <tr>
                        <td>"Reactive style"</td>
                        <td>
                            <Code inline=true>"style:color=signal"</Code>
                        </td>
                        <td>
                            <Code inline=true>
                                ".add_reactive(move || ColorProperty.declare(..))"
                            </Code>
                        </td>
                    </tr>
                    <tr>
                        <td>"Merge hook and user styles"</td>
                        <td>"Not supported"</td>
                        <td>
                            <Code inline=true>"hook_styles.merge(user_styles)"</Code>
                        </td>
                    </tr>
                    <tr>
                        <td>"Checked property/value pairs"</td>
                        <td>"Strings only"</td>
                        <td>
                            "Property selectors and typed values, with "
                            <Code inline=true>"_unchecked"</Code> " as an opt-out"
                        </td>
                    </tr>
                </tbody>
            </table>

            <p>
                "When you're styling one element and nothing is passed between components, the native "
                <Code inline=true>"class=\"foo\""</Code> " and "
                <Code inline=true>"style:color=\"red\""</Code> " are perfectly fine. Use "
                <Code inline=true>"Classes"</Code> " and " <Code inline=true>"Styles"</Code>
                " where classes and styles need to travel through component boundaries and pick things up on the way. "
                "And for purely decorative styling, a CSS class in your stylesheet is usually still the best choice."
            </p>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf {
                    title: "Classes & Styles",
                    link: "#classes-and-styles",
                },
                Toc::Leaf {
                    title: "The Problem",
                    link: "#the-problem",
                },
                Toc::Leaf {
                    title: "Classes",
                    link: "#classes",
                },
                Toc::Leaf {
                    title: "Styles",
                    link: "#styles",
                },
                Toc::Leaf {
                    title: "Merge Priority",
                    link: "#merge",
                },
                Toc::Leaf {
                    title: "Live Example",
                    link: "#example",
                },
                Toc::Leaf {
                    title: "CSS Value Types",
                    link: "#css-values",
                },
                Toc::Leaf {
                    title: "How Leptonic Uses These Types",
                    link: "#leptonic-usage",
                },
                Toc::Leaf {
                    title: "Comparison with Native Leptos",
                    link: "#comparison",
                },
            ],
        } />
    }
}
