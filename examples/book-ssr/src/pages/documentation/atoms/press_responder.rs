use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::press_responder::PressResponderDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomPressResponder() -> impl IntoView {
    view! {
        <DocPage title="PressResponder">
            <p>
                <Code inline=true>"PressResponder"</Code>" lets a parent inject press behavior into the pressable elements "
                "inside it, without wrapping them in an extra DOM node. Every descendant that calls "
                <Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>
                " picks the behavior up. See the "<Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>
                " for the other interaction building blocks. This page also documents "
                <AnchorLink href="#pressable"><Code inline=true>"Pressable"</Code></AnchorLink>" and "
                <AnchorLink href="#clearpressresponder"><Code inline=true>"ClearPressResponder"</Code></AnchorLink>"."
            </p>

            <p>
                "Trigger atoms use this pattern: a menu or dialog trigger makes its child button open the overlay, "
                "without the button knowing about the overlay."
            </p>

            <ReactAriaSource path="interactions/PressResponder.tsx"/>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"PressResponder"</Code>" provides a "<Code inline=true>"PressResponderContext"</Code>
                    ", which "<Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>" reads and merges "
                    "with its own input. The "<AnchorLink href="#pressable"><Code inline=true>"Pressable"</Code></AnchorLink>
                    " atom calls "<Code inline=true>"use_press"</Code>" and "
                    <Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>" for its child."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::prelude::*, hooks::PressEvent};
                        use leptos::{logging::log, prelude::*};

                        let is_dialog_open = RwSignal::new(false);

                        // Both handlers run on a single press: the PressResponder's first, then the Button's.
                        view! {
                            <PressResponder on_press=move |_: PressEvent| log!("parent pressed") force_is_pressed=is_dialog_open>
                                <Button on_press=move |_: PressEvent| log!("child pressed")>"Open dialog"</Button>
                            </PressResponder>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Press the button: the handler of the "<Code inline=true>"PressResponder"</Code>" runs first, then the one "
                    "of the "<Code inline=true>"Pressable"</Code>". Disabling the "<Code inline=true>"PressResponder"</Code>
                    " stops the presses of the button inside, although the "<Code inline=true>"Pressable"</Code>" itself is enabled. "
                    "As in react-aria, the responder\u{2019}s disabled state only reaches the press handling: the "
                    <Code inline=true>"Pressable"</Code>" gets neither "<Code inline=true>"aria-disabled"</Code>" nor "
                    <Code inline=true>"data-disabled"</Code>", so disable the element itself when it should look and announce as disabled."
                </p>

                <Demo
                    description="PressResponder chaining its on_press before the Pressable\u{2019}s and disabling it"
                    source=include_str!("demos/press_responder.rs")
                >
                    <PressResponderDemo/>
                </Demo>
            </Section>

            <Section title="PressResponder">
                <Section title="Props" id="press-responder-props">
                    <p>"All props are optional. Only set what you want to inject into the pressable descendants."</p>

                    <ApiTable kind=ApiKind::Props of="PressResponder">
                        <ApiRow name="on_context_menu" ty="Option<Callback<ContextMenuEvent>>" default="None">
                            "Called when the pressable element requests a context menu, through "
                            <Link href=routes::doc::interactions::UseContextMenu.materialize()>"use_context_menu"</Link>
                            " (a context menu "<Code inline=true>"MenuTrigger"</Code>" uses it)."
                        </ApiRow>
                        <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">
                            "Called when a press completes."
                        </ApiRow>
                        <ApiRow name="on_press_start" ty="Option<Callback<PressEvent>>" default="None">
                            "Called when a press starts."
                        </ApiRow>
                        <ApiRow name="on_press_end" ty="Option<Callback<PressEvent>>" default="None">
                            "Called when a press ends, whether it completed or not."
                        </ApiRow>
                        <ApiRow name="on_press_up" ty="Option<Callback<PressEvent>>" default="None">
                            "Called when the pointer is released over the target."
                        </ApiRow>
                        <ApiRow name="on_press_change" ty="Option<Callback<bool>>" default="None">
                            "Called when the pressed state changes."
                        </ApiRow>
                        <ApiRow name="on_long_press_start, on_long_press, on_long_press_end" ty="Option<Callback<LongPressEvent>>" default="None">
                            "Called when a long press starts, completes or ends, chained like the press callbacks."
                        </ApiRow>
                        <ApiRow name="long_press_accessibility_description" ty="MaybeProp<String>" default="None">
                            "Describes the long-press action to assistive technology (the element\u{2019}s own description wins)."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Option<Signal<bool>>" default="None">
                            "Disables pressing the descendants."
                        </ApiRow>
                        <ApiRow name="force_is_pressed" ty="Option<Signal<bool>>" default="None">
                            "Shows the descendants as pressed, e.g. while the overlay a trigger opened is visible."
                        </ApiRow>
                        <ApiRow name="prevent_focus_on_press" ty="Option<Signal<bool>>" default="None">
                            "Don\u{2019}t focus the descendants when they are pressed."
                        </ApiRow>
                        <ApiRow name="should_cancel_on_pointer_exit" ty="Option<Signal<bool>>" default="None">
                            "Cancel a press when the pointer leaves the target."
                        </ApiRow>
                        <ApiRow name="allow_text_selection_on_press" ty="Option<Signal<bool>>" default="None">
                            "Allow selecting text while pressing."
                        </ApiRow>
                        <ApiRow name="trigger" ty="Option<PressResponderTrigger>" default="None">
                            "An overlay trigger\u{2019}s props for the pressable "<Code inline=true>"Button"</Code>": "<Code inline=true>"aria-expanded"</Code>", "
                            <Code inline=true>"aria-controls"</Code>" and the element the overlay is positioned at (used by "
                            <Code inline=true>"DialogTrigger"</Code>" and "<Code inline=true>"MenuTrigger"</Code>")."
                        </ApiRow>
                        <ApiRow name="shortcuts" ty="Option<KeyboardShortcuts>" default="None">
                            "Keyboard shortcuts for the pressable "<Code inline=true>"Button"</Code>", handled after its own (a "
                            <Code inline=true>"MenuTrigger"</Code>"\u{2019}s: the arrow keys open the menu)."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">"Required. Content containing the pressable descendants."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="How It Works">
                    <ul>
                        <li>
                            <Code inline=true>"PressResponder"</Code>" provides a "<Code inline=true>"PressResponderContext"</Code>
                            " through Leptos context."
                        </li>
                        <li><Code inline=true>"use_press"</Code>" reads the context and merges it with its own input."</li>
                        <li>
                            <b>"Callbacks chain."</b>" The context\u{2019}s callbacks run first, then the local ones, so both "
                            <Code inline=true>"on_press"</Code>" handlers fire on a single press."
                        </li>
                        <li>
                            <b>"Flags combine with OR."</b>" If either the context or the local input enables "
                            <Code inline=true>"is_disabled"</Code>", "<Code inline=true>"force_is_pressed"</Code>", "
                            <Code inline=true>"prevent_focus_on_press"</Code>", "<Code inline=true>"should_cancel_on_pointer_exit"</Code>
                            " or "<Code inline=true>"allow_text_selection_on_press"</Code>", it is enabled."
                        </li>
                        <li>
                            <b>"Registration."</b>" "<Code inline=true>"use_press"</Code>" sets the context\u{2019}s "
                            <Code inline=true>"registered"</Code>" flag when it reads the context. In development builds, a "
                            <Code inline=true>"PressResponder"</Code>" without a pressable descendant logs a warning."
                        </li>
                        <li>
                            "Disabling only stops the presses: the descendant keeps its focusability and its ARIA attributes, "
                            "so mark it disabled yourself when it should look and be announced disabled (the demo sets "
                            <Code inline=true>"aria-disabled"</Code>")."
                        </li>
                    </ul>
                </Section>

                <Section title="Nesting">
                    <p>
                        "When "<Code inline=true>"PressResponder"</Code>"s are nested, the callbacks of all levels chain, outermost "
                        "first. For the other props, the innermost "<Code inline=true>"PressResponder"</Code>" that sets them wins."
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(r#"
                            // on_press runs outer_handler, inner_handler, then button_handler.
                            <PressResponder on_press=outer_handler>
                                <PressResponder on_press=inner_handler>
                                    <Button on_press=button_handler>"Nested"</Button>
                                </PressResponder>
                            </PressResponder>
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="Pressable">
                <p>
                    <Code inline=true>"Pressable"</Code>" makes its child element pressable without rendering an element of its "
                    "own: it calls "<Code inline=true>"use_press"</Code>" and "<Code inline=true>"use_focusable"</Code>
                    " and puts the handlers onto the child itself, next to the child\u{2019}s own. The child becomes focusable "
                    "(a "<Code inline=true>"tabindex"</Code>" of its own wins) and needs an interactive role, such as a "
                    <Code inline=true>"<button>"</Code>" or a "<Code inline=true>"<span role=\"button\">"</Code>
                    "; development builds warn otherwise. Inside a "<Code inline=true>"DialogTrigger"</Code>", it also gets the "
                    "trigger\u{2019}s "<Code inline=true>"aria-expanded"</Code>" and "<Code inline=true>"aria-controls"</Code>"."
                </p>

                <Section title="Props" id="pressable-props">
                    <ApiTable kind=ApiKind::Props of="Pressable">
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                            "Whether pressing is disabled. A "<Code inline=true>"tabindex"</Code>" the atom manages is removed "
                            "then; a native control like "<Code inline=true>"<button>"</Code>" stays focusable."
                        </ApiRow>
                        <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">"Called when a press completes."</ApiRow>
                        <ApiRow name="on_press_start" ty="Option<Callback<PressEvent>>" default="None">"Called when a press starts."</ApiRow>
                        <ApiRow name="on_press_end" ty="Option<Callback<PressEvent>>" default="None">"Called when a press ends, completed or not."</ApiRow>
                        <ApiRow name="on_press_up" ty="Option<Callback<PressEvent>>" default="None">"Called when the press is released over the element."</ApiRow>
                        <ApiRow name="on_press_change" ty="Option<Callback<bool>>" default="None">"Called when the pressed state changes."</ApiRow>
                        <ApiRow name="children" ty="TypedChildren<V>">"Required. The pressable element."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ClearPressResponder">
                <p>
                    <Code inline=true>"ClearPressResponder"</Code>" shadows any "<Code inline=true>"PressResponderContext"</Code>
                    " above it with an empty one, so the press behavior of a trigger doesn\u{2019}t leak into the overlay it opens."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        // Buttons inside the overlay don't inherit the trigger's press handler.
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
            </Section>

            <Section title="PressResponderContext">
                <p>
                    <Code inline=true>"PressResponder"</Code>" is a convenience wrapper around "
                    <Code inline=true>"provide_context::<PressResponderContext>(..)"</Code>". You can provide the context "
                    "directly from a hook or your own Leptos component. Unlike a nested "<Code inline=true>"PressResponder"</Code>
                    ", this replaces an outer context instead of chaining it."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::hooks::PressResponderContext;
                        use leptos::prelude::*;

                        provide_context(PressResponderContext {
                            on_press: Some(Callback::new(|_| { /* ... */ })),
                            force_is_pressed: Some(is_open.into()),
                            ..PressResponderContext::empty()
                        });
                    ")}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=routes::doc::focus::Focusable.materialize()>"Focusable"</Link></li>
                <li><Link href=routes::doc::button::Atom.materialize()>"Button Atoms"</Link></li>
                <li><Link href=routes::doc::dialog::Atom.materialize()>"Dialog Atoms"</Link>" ("<Code inline=true>"DialogTrigger"</Code>")"</li>
            </SeeAlso>
        </DocPage>
    }
}
