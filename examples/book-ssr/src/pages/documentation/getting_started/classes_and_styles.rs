use indoc::indoc;
use leptonic::{components::prelude::*, hooks::LinkTarget};
use leptos::prelude::*;

use super::demos::classes_and_styles_meter::ClassesAndStylesMeterDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageClassesAndStyles() -> impl IntoView {
    view! {
        <DocPage title="Classes & Styles">
            <p>
                "Leptonic\u{2019}s atoms and components take a " <Code inline=true>"classes"</Code>
                " and a " <Code inline=true>"styles"</Code>
                " prop for the element they render. Their types come from two small companion crates, "
                <Link
                    href="https://github.com/lpotthast/leptos-classes"
                    target=LinkTarget::Blank
                >
                    "leptos-classes"
                </Link> " and "
                <Link
                    href="https://github.com/lpotthast/leptos-styles"
                    target=LinkTarget::Blank
                >
                    "leptos-styles"
                </Link> ". " <Code inline=true>"Classes"</Code>
                " holds a list of class names and " <Code inline=true>"Styles"</Code>
                " holds inline style declarations. You can hand either one from one Leptos component to the next, add to it along the way, "
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

            <Section title="The Problem">
                <p>
                    "On a single element, Leptos already gives you everything you need: "
                    <Code inline=true>"class=\"...\""</Code> " and "
                    <Code inline=true>"class:name=signal"</Code> " for classes, "
                    <Code inline=true>"style=\"...\""</Code> " and "
                    <Code inline=true>"style:property=value"</Code>
                    " for styles. These are attributes on one element, though. "
                    "You can\u{2019}t bundle them into a single value and pass that value through several Leptos components."
                </p>

                <p>
                    "In a UI library, that\u{2019}s exactly what you need to do. Each layer wants to add a little of its own "
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

            </Section>
            <Section title="Classes">
                <p>
                    "A " <Code inline=true>"Classes"</Code> " value is an ordered list of class names. "
                    "Each name is either always present or tied to a reactive condition."
                </p>

                <Section title="Construction">
                    <p>
                        "Most of the time you\u{2019}ll start from a string, or let the "
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
                        "That\u{2019}s an easy one to trip over in props: " <Code inline=true>"classes=\"card elevated\""</Code>
                        " panics, so write " <Code inline=true>"classes=[\"card\", \"elevated\"]"</Code>
                        " instead. Adding the same name twice to one " <Code inline=true>"Classes"</Code>
                        " value panics as well. If a class should depend on several things, combine them into one condition "
                        "rather than registering the name twice. For untrusted runtime input, "
                        <Code inline=true>"ClassName::try_new"</Code>
                        " lets you validate without panicking."
                    </p>

                </Section>
                <Section title="Conditional Classes">
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

                </Section>
                <Section title="Prop Drilling" id="classes-drilling">
                    <p>
                        "This is what the type is really for. Every layer receives a "
                        <Code inline=true>"Classes"</Code>
                        ", adds what it needs, and passes it on. Only the innermost Leptos component renders it with "
                        <Code inline=true>"class=classes"</Code> ":"
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(
                            r#"
                            use leptonic::utils::classes::Classes;
                            use leptos::prelude::*;

                            /// The innermost Leptos component renders the accumulated classes.
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
                        "When you\u{2019}re handed two " <Code inline=true>"Classes"</Code>
                        " values that were built independently, for example a prop and the result of a helper function, combine them with "
                        <Code inline=true>".merge(other, MergeStrategy::default())"</Code>
                        ". The default strategy keeps a class that appears in both if " <em>"either"</em>
                        " condition holds. If you control both sides, just keep chaining "
                        <Code inline=true>".add*()"</Code> " calls instead."
                    </p>

                </Section>
                <Section title="One Owner per Attribute">
                    <p>
                        <Code inline=true>"class=classes"</Code>
                        " takes over the element\u{2019}s entire "
                        <Code inline=true>"class"</Code>
                        " attribute and rewrites all of it whenever something changes. A "
                        <Code inline=true>"class:foo=..."</Code>
                        " directive on the same element would be overwritten on the next update, so don\u{2019}t mix the two. "
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

                </Section>
            </Section>
            <Section title="Styles">
                <p>
                    "A " <Code inline=true>"Styles"</Code>
                    " value is a list of CSS declarations. Each one can be static, "
                    "reactive, or only present some of the time. Declarations are typed: you pick a property through a "
                    <em>"property selector"</em>
                    " and pass it a value of the matching type. A padding can\u{2019}t end up with a color, "
                    "a typo in a property name doesn\u{2019}t compile, and you never have to format a CSS string yourself."
                </p>

                <Section title="Typed Declarations">
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

                </Section>
                <Section title="Reactive and Optional Declarations">
                    <p>
                        "For a value that changes over time, use "
                        <Code inline=true>".add_reactive()"</Code>
                        " with a closure that builds the whole declaration. If the declaration should sometimes be left out entirely, use "
                        <Code inline=true>".add_optional()"</Code> " and return an "
                        <Code inline=true>"Option"</Code> ". While it returns "
                        <Code inline=true>"None"</Code> ", the property doesn\u{2019}t show up in the "
                        <Code inline=true>"style"</Code> " attribute at all:"
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(
                            r"
                            use leptonic::utils::{
                                css::{computed_pct, computed_size, rgb},
                                style::{BackgroundColorProperty, WidthProperty},
                                styles::Styles,
                            };

                            let (progress, _) = signal(42.0);
                            let (highlight, _) = signal(false);

                            let styles = Styles::new()
                                .add_reactive(move || WidthProperty.declare(computed_size(computed_pct(progress.get()))))
                                .add_optional(move || {
                                    highlight
                                        .get()
                                        .then(|| BackgroundColorProperty.declare(rgb(255, 240, 200)))
                                });
                        "
                        )}
                    </Code>

                    <p>
                        "Functions like " <Code inline=true>"pct()"</Code> " and " <Code inline=true>"px()"</Code>
                        " panic on non-finite numbers, and " <Code inline=true>"NonNegativeLengthPercentage::new"</Code>
                        " panics on negative ones. For literals and values you know are in range, that\u{2019}s fine. "
                        "For anything computed at runtime, like a slider position when " <Code inline=true>"min == max"</Code>
                        ", use the helpers in " <Code inline=true>"leptonic::utils::css"</Code> ", which never panic:"
                    </p>

                    <DocTable headers=&["Helper", "Returns"]>
                        <TableRow>
                            <TableCell><Code inline=true>"computed_pct(f64)"</Code></TableCell>
                            <TableCell>"A percentage; "<Code inline=true>"0px"</Code>" for NaN and infinite numbers."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"computed_px(f64)"</Code></TableCell>
                            <TableCell>"A pixel length; "<Code inline=true>"0px"</Code>" for NaN and infinite numbers."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"computed_size(CssDimension)"</Code></TableCell>
                            <TableCell>
                                "A "<Code inline=true>"Size"</Code>" for "<Code inline=true>"width"</Code>", "
                                <Code inline=true>"height"</Code>" & co.; negative values become "<Code inline=true>"0px"</Code>"."
                            </TableCell>
                        </TableRow>
                    </DocTable>

                    <p>
                        "For other fallbacks, use the " <Code inline=true>"try_"</Code>
                        " variants ("<Code inline=true>"try_pct"</Code>", "<Code inline=true>"try_px"</Code>", \u{2026}) "
                        "and decide yourself."
                    </p>

                </Section>
                <Section title="Custom Properties">
                    <p>
                        "CSS custom properties (variables) are typed too. Declare one once with "
                        <Code inline=true>"css_custom_property!"</Code>
                        ", and it will only accept values of the type you gave it. This is a nice way to hand a live value to a stylesheet "
                        "while the stylesheet stays in charge of how it\u{2019}s used:"
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

                </Section>
                <Section title="The Unchecked Escape Hatch">
                    <p>
                        "The set of typed properties is intentionally growing step by step and doesn\u{2019}t cover all of CSS yet. "
                        <Code inline=true>"position"</Code> ", " <Code inline=true>"display"</Code> ", "
                        <Code inline=true>"transform"</Code> ", " <Code inline=true>"background"</Code>
                        " gradients and many others aren\u{2019}t modeled. For those, there are explicitly "
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

                </Section>
                <Section title="Prop Drilling" id="styles-drilling">
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
                        <Code inline=true>"style"</Code> " attribute. Don\u{2019}t combine it with "
                        <Code inline=true>"style:foo=..."</Code>
                        " directives on the same element. Add the declaration to the "
                        <Code inline=true>"Styles"</Code> " value instead."
                    </p>

                </Section>
            </Section>
            <Section title="Merge Priority">
                <p>
                    <Code inline=true>"a.merge(b)"</Code>
                    " combines two "
                    <Code inline=true>"Styles"</Code>
                    " values, with "
                    <Code inline=true>"b"</Code>
                    " acting as a lower-priority fallback layer. Conflicts are resolved per property: "
                    "when "
                    <Code inline=true>"a"</Code>
                    " currently has a declaration for a property, it wins. When it doesn\u{2019}t, "
                    <Code inline=true>"b"</Code>
                    "\u{2019}s declaration shows through. This is evaluated on every update, so a fallback "
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
                    "Leptonic\u{2019}s atoms use this to protect styles that hooks rely on. A hook may, for example, set "
                    <Code inline=true>"touch-action: none"</Code>
                    " to get pointer handling right. The atom puts the hook\u{2019}s styles first, so your own styles fill in "
                    "everything else but can\u{2019}t accidentally override them:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(
                        r"
                        // Inside an atom (e.g., atoms/button.rs)
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
                    "If you build your own Leptos components on top of leptonic atoms, you usually won\u{2019}t need "
                    <Code inline=true>".merge()"</Code> " yourself. Pass "
                    <Code inline=true>"styles"</Code> " down and let the atom do the merging."
                </p>

            </Section>
            <Section title="Live Example">
                <p>
                    "Here\u{2019}s everything together, on a "<Link href=routes::doc::meter::Atom.materialize()>"Meter"</Link>
                    " atom. Its look is defined by CSS classes; a reactive class adds stripes when the storage is almost "
                    "full. The only inline style is the one value that really is dynamic: the fill color, a typed "
                    <Code inline=true>"background-color"</Code>" that references the theme\u{2019}s status colors through "
                    "typed custom properties. The fill\u{2019}s width comes from the atom, which merges it in front of these "
                    "styles."
                </p>

                <Demo description="Storage meter with a reactive class and a typed reactive fill color" source=include_str!("demos/classes_and_styles_meter.rs")>
                    <ClassesAndStylesMeterDemo />
                </Demo>

            </Section>
            <Section title="CSS Value Types">
                <p>
                    "The value types follow the CSS grammar of the property they belong to, so you\u{2019}ll come across a handful of them. "
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
                    <Code inline=true>"leptonic::prelude"</Code>" re-exports the types leptonic\u{2019}s own props use: "
                    <Code inline=true>"Width"</Code>" and "<Code inline=true>"Height"</Code>" (both "
                    <Code inline=true>"CssDimension"</Code>"), "<Code inline=true>"Margin"</Code>", "
                    <Code inline=true>"Padding"</Code>" and "<Code inline=true>"FontWeight"</Code>"."
                </p>

                <p>
                    "Colors are built with " <Code inline=true>"rgb()"</Code> ", "
                    <Code inline=true>"rgba()"</Code> ", " <Code inline=true>"hsl()"</Code> " or "
                    <Code inline=true>"CssColor::Named(..)"</Code> ". Leptonic\u{2019}s own color types ("
                    <Code inline=true>"RGB8"</Code> ", " <Code inline=true>"HSL"</Code>
                    ") convert into " <Code inline=true>"CssColor"</Code> " with "
                    <Code inline=true>".into()"</Code> "."
                </p>

            </Section>
            <Section title="How Leptonic Uses These Types">
                <p>
                    "Atoms and components declare "
                    <Code inline=true>"#[prop(into, optional)] classes: Classes"</Code> " and "
                    <Code inline=true>"#[prop(into, optional)] styles: Styles"</Code> ". Thanks to "
                    <Code inline=true>"into"</Code> ", you can pass a plain "
                    <Code inline=true>"\"class-name\""</Code>
                    ", an array of names, a single typed declaration, or a fully built value, whichever is handiest. "
                    "Those that render no element of their own don\u{2019}t take them: atoms that give their child "
                    "behavior ("<Code inline=true>"Pressable"</Code>", "<Code inline=true>"Hoverable"</Code>", "
                    <Code inline=true>"Focusable"</Code>", "<Code inline=true>"PressResponder"</Code>", the "
                    <Code inline=true>"*Trigger"</Code>" atoms), providers ("<Code inline=true>"Root"</Code>", "
                    <Code inline=true>"ThemeProvider"</Code>", "<Code inline=true>"ToastRoot"</Code>"), and "
                    <Code inline=true>"Toast"</Code>" and "<Code inline=true>"AlertIcon"</Code>"."
                </p>

                <p>
                    "Each of the "<Link href=routes::doc::Architecture.materialize()>"three layers"</Link>" has its own "
                    "part in this. Hooks return the attributes and styles they manage, such as ARIA attributes or "
                    <Code inline=true>"touch-action"</Code> ". Atoms accept your "
                    <Code inline=true>"classes"</Code> " and " <Code inline=true>"styles"</Code>
                    ", merge the hook styles in front of yours (see "<AnchorLink href="#merge-priority">"Merge Priority"</AnchorLink>
                    "), and render the element. Components add their theme class, for example "
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
                    "#
                    )}
                </Code>

            </Section>
            <Section title="Comparison with Native Leptos">
                <DocTable headers=&["Capability", "Native Leptos", "Classes / Styles"]>
                    <TableRow>
                        <TableCell>"Static class"</TableCell>
                        <TableCell>
                            <Code inline=true>"class=\"foo\""</Code>
                        </TableCell>
                        <TableCell>
                            <Code inline=true>"classes=\"foo\""</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Conditional class"</TableCell>
                        <TableCell>
                            <Code inline=true>"class:foo=signal"</Code>
                        </TableCell>
                        <TableCell>
                            <Code inline=true>".add_reactive(\"foo\", signal)"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Toggle between two classes"</TableCell>
                        <TableCell>"Two " <Code inline=true>"class:"</Code>" directives"</TableCell>
                        <TableCell>
                            <Code inline=true>".add_toggle(signal, \"a\", \"b\")"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Pass classes / styles as a prop"</TableCell>
                        <TableCell>"No built-in support"</TableCell>
                        <TableCell>
                            <Code inline=true>"classes: Classes"</Code>
                            ", "
                            <Code inline=true>"styles: Styles"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Multi-layer drilling"</TableCell>
                        <TableCell>"Manual string concatenation"</TableCell>
                        <TableCell>
                            <Code inline=true>".add(..)"</Code>
                            " at each layer"
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Static inline style"</TableCell>
                        <TableCell>
                            <Code inline=true>"style=\"color: red\""</Code>
                        </TableCell>
                        <TableCell>
                            <Code inline=true>"ColorProperty.declare(..)"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Reactive style"</TableCell>
                        <TableCell>
                            <Code inline=true>"style:color=signal"</Code>
                        </TableCell>
                        <TableCell>
                            <Code inline=true>
                                ".add_reactive(move || ColorProperty.declare(..))"
                            </Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Merge hook and user styles"</TableCell>
                        <TableCell>"Not supported"</TableCell>
                        <TableCell>
                            <Code inline=true>"hook_styles.merge(user_styles)"</Code>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Checked property/value pairs"</TableCell>
                        <TableCell>"Strings only"</TableCell>
                        <TableCell>
                            "Property selectors and typed values, with "
                            <Code inline=true>"_unchecked"</Code> " as an opt-out"
                        </TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "When you\u{2019}re styling one element and nothing is passed between Leptos components, the native "
                    <Code inline=true>"class=\"foo\""</Code> " and "
                    <Code inline=true>"style:color=\"red\""</Code> " are perfectly fine. Use "
                    <Code inline=true>"Classes"</Code> " and " <Code inline=true>"Styles"</Code>
                    " where classes and styles need to travel through Leptos component boundaries and pick things up on the way. "
                    "And for purely decorative styling, a CSS class in your stylesheet is usually still the best choice."
                </p>
            </Section>
        </DocPage>
    }
}
