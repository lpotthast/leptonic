use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::press_responder::PressResponderDemo;
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
pub fn PageAtomPressResponder() -> impl IntoView {
    view! {
        <Article>
            <h1 id="press-responder" class="anchor">
                "PressResponder"
                <AnchorLink href="#press-responder" description="Direct link to article header"/>
            </h1>

            <p>
                "PressResponder is a context-providing atom that lets parent components inject press behavior into "
                "descendant pressable elements without wrapping them in an extra DOM node. "
                "Any descendant that calls "<Code inline=true>"use_press"</Code>" automatically picks up the injected behavior. "
                "See the "<Link href=crate::routes::doc::interactions::UsePress.materialize()>"use_press hook"</Link>" for the underlying press interaction."
            </p>

            <p>
                "This pattern is commonly used by trigger components (e.g., menu triggers, dialog triggers) "
                "to make their child button open/close an overlay, without the button needing to know about the overlay."
            </p>

            <h2 id="how-it-works" class="anchor">
                "How It Works"
                <AnchorLink href="#how-it-works" description="Direct link to section: How It Works"/>
            </h2>

            <ul>
                <li>
                    <Code inline=true>"PressResponder"</Code>" provides a "<Code inline=true>"PressResponderContext"</Code>
                    " via Leptos context ("<Code inline=true>"provide_context"</Code>")."
                </li>
                <li>
                    <Code inline=true>"use_press"</Code>" automatically reads this context and merges its fields with the hook\u{2019}s local input."
                </li>
                <li>
                    <strong>"Callbacks chain:"</strong>" context callbacks fire first, then local callbacks. "
                    "Both the parent\u{2019}s and child\u{2019}s "<Code inline=true>"on_press"</Code>" will fire on a single press."
                </li>
                <li>
                    <strong>"Boolean fields merge with OR:"</strong>" if either parent or local sets a field to "<Code inline=true>"true"</Code>
                    ", the behavior is enabled. This includes "<Code inline=true>"disabled"</Code>", "
                    <Code inline=true>"prevent_focus_on_press"</Code>", "<Code inline=true>"should_cancel_on_pointer_exit"</Code>
                    ", and "<Code inline=true>"allow_text_selection_on_press"</Code>"."
                </li>
                <li>
                    <Code inline=true>"force_is_pressed"</Code>": OR \u{2014} either parent or local can force the pressed visual state "
                    "(e.g., a dialog trigger forces pressed appearance while the dialog is open)."
                </li>
                <li>
                    <Code inline=true>"registered"</Code>": set to "<Code inline=true>"true"</Code>" by "<Code inline=true>"use_press"</Code>
                    " when it reads the context. Allows the parent to detect whether a pressable child exists."
                </li>
            </ul>

            <h2 id="props" class="anchor">
                "Props"
                <AnchorLink href="#props" description="Direct link to section: Props"/>
            </h2>

            <p>"All props are optional. Only provide the fields you want to inject into descendant pressable elements."</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Prop"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><Code inline=true>"on_press"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<PressEvent>>"</Code></TableCell>
                            <TableCell>"Called when a complete press is detected"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_press_start"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<PressEvent>>"</Code></TableCell>
                            <TableCell>"Called when a press starts"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_press_end"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<PressEvent>>"</Code></TableCell>
                            <TableCell>"Called when a press ends (regardless of success)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_press_up"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<PressEvent>>"</Code></TableCell>
                            <TableCell>"Called when the pointer is released over the target"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"on_press_change"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Callback<bool>>"</Code></TableCell>
                            <TableCell>"Called when the pressed state changes"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"disabled"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Signal<bool>>"</Code></TableCell>
                            <TableCell>"Disables press on descendants (merges with OR)"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"force_is_pressed"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<Signal<bool>>"</Code></TableCell>
                            <TableCell>"Forces pressed visual state on descendants"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"prevent_focus_on_press"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<bool>"</Code></TableCell>
                            <TableCell>"Prevents focus when pressing descendants"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"should_cancel_on_pointer_exit"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<bool>"</Code></TableCell>
                            <TableCell>"Cancels press when pointer exits target"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"allow_text_selection_on_press"</Code></TableCell>
                            <TableCell><Code inline=true>"Option<bool>"</Code></TableCell>
                            <TableCell>"Allows text selection during press"</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"children"</Code></TableCell>
                            <TableCell><Code inline=true>"Children"</Code></TableCell>
                            <TableCell>"Child content (must contain a pressable descendant)"</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h2 id="example" class="anchor">
                "Example"
                <AnchorLink href="#example" description="Direct link to section: Example"/>
            </h2>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptonic::atoms::prelude::*;

                    // Parent injects an on_press handler via PressResponder.
                    // The child Button's own on_press still fires — both are chained.
                    view! {
                        <PressResponder
                            on_press=Callback::new(|_| log!("parent pressed"))
                            force_is_pressed=is_dialog_open
                        >
                            <Button on_press=move |_| { log!("child pressed") }>
                                "Open dialog"
                            </Button>
                        </PressResponder>
                    }
                "#)}
            </Code>

            <h2 id="demo" class="anchor">
                "Demo"
                <AnchorLink href="#demo" description="Direct link to section: Demo"/>
            </h2>

            <p>
                "Press the button below. Both the parent (PressResponder) and child (Pressable) handlers fire, "
                "with the parent\u{2019}s handler executing first."
            </p>

            <DemoShell description="PressResponder callback chaining" source=include_str!("demos/press_responder.rs")>
                <PressResponderDemo />
            </DemoShell>

            <h2 id="clear" class="anchor">
                "ClearPressResponder"
                <AnchorLink href="#clear" description="Direct link to section: ClearPressResponder"/>
            </h2>

            <p>
                <Code inline=true>"ClearPressResponder"</Code>" shadows any ancestor "<Code inline=true>"PressResponderContext"</Code>
                " with an empty context, preventing press behavior from leaking into overlay content."
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    // Buttons inside the overlay do NOT inherit the trigger's press handler.
                    <PressResponder on_press=toggle_overlay>
                        <Button>"Open"</Button>

                        <ClearPressResponder>
                            <OverlayContent>
                                <Button on_press=action>"Action"</Button>
                            </OverlayContent>
                        </ClearPressResponder>
                    </PressResponder>
                "#)}
            </Code>

            <h2 id="nesting" class="anchor">
                "Nesting"
                <AnchorLink href="#nesting" description="Direct link to section: Nesting"/>
            </h2>

            <p>
                "PressResponder supports nesting. When nested, callbacks from all levels are chained "
                "(outermost fires first). Boolean config fields use inner-wins semantics."
            </p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    // on_press fires: outer → inner → button's own handler
                    <PressResponder on_press=outer_handler>
                        <PressResponder on_press=inner_handler>
                            <Button on_press=button_handler>"Nested"</Button>
                        </PressResponder>
                    </PressResponder>
                "#)}
            </Code>

            <h2 id="context" class="anchor">
                "PressResponderContext"
                <AnchorLink href="#context" description="Direct link to section: PressResponderContext"/>
            </h2>

            <p>
                "The "<Code inline=true>"PressResponder"</Code>" component is a convenience wrapper around "
                <Code inline=true>"provide_context::<PressResponderContext>(...)"</Code>". "
                "You can also provide the context directly from a hook or custom component:"
            </p>

            <Code language=Language::Rust>
                {indoc!(r"
                    use leptonic::hooks::PressResponderContext;

                    // Provide context directly instead of using the PressResponder atom.
                    let registered = StoredValue::new_local(false);
                    provide_context(PressResponderContext {
                        on_press: Some(Callback::new(|_| { /* ... */ })),
                        force_is_pressed: Some(is_open.into()),
                        registered,
                        ..PressResponderContext::empty()
                    });
                ")}
            </Code>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UsePress.materialize()>"Hook deep-dive: use_press"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "PressResponder", link: "#press-responder" },
                Toc::Leaf { title: "How It Works", link: "#how-it-works" },
                Toc::Leaf { title: "Props", link: "#props" },
                Toc::Leaf { title: "Example", link: "#example" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "ClearPressResponder", link: "#clear" },
                Toc::Leaf { title: "Nesting", link: "#nesting" },
                Toc::Leaf { title: "PressResponderContext", link: "#context" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
