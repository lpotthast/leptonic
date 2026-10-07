use indoc::indoc;
use leptos::prelude::*;

use super::demos::focusable::FocusableDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseFocusable() -> impl IntoView {
    view! {
        <DocPage title="use_focusable">
            <p>
                "The "<Code inline=true>"use_focusable"</Code>" hook makes any element focusable: it manages the "
                <Code inline=true>"tabindex"</Code>", handles focus and keyboard events, supports auto focus and gives you a handle "
                "to focus the element programmatically. It combines "
                <Link href=routes::doc::focus::UseFocus.materialize()>"use_focus"</Link>" and "
                <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>". The "
                <Link href=routes::doc::focus::Focusable.materialize()>"Focusable"</Link>" atom applies it to a child element "
                "for you. See the "<Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>
                " to compare it with the other focus building blocks."
            </p>

            <ReactAriaSource path="interactions/useFocusable.tsx"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseFocusableInput"</Code>" implements "<Code inline=true>"Default"</Code>". It is "
                    <Code inline=true>"Clone"</Code>" but not "<Code inline=true>"Copy"</Code>
                    ", because it can own a set of keyboard shortcuts."
                </p>

                <ApiTable kind=ApiKind::Input of="UseFocusableInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Removes the "<Code inline=true>"tabindex"</Code>" and ignores focus and keyboard events."
                    </ApiRow>
                    <ApiRow name="auto_focus" ty="bool" default="false">"Focus the element when it mounts."</ApiRow>
                    <ApiRow name="exclude_from_tab_order" ty="Signal<bool>" default="false">
                        "Sets "<Code inline=true>"tabindex=\"-1\""</Code>": the element can still be focused by pointer or "
                        "programmatically, but not with "<Keys keys="Tab"/>"."
                    </ApiRow>
                    <ApiRow name="on_focus" ty="Option<Callback<FocusEvent>>" default="None">
                        "Called when the element receives focus."
                    </ApiRow>
                    <ApiRow name="on_blur" ty="Option<Callback<FocusEvent>>" default="None">
                        "Called when the element loses focus."
                    </ApiRow>
                    <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">
                        "Called with the new focus state."
                    </ApiRow>
                    <ApiRow name="on_key_down, on_key_up" ty="Option<Callback<KeyboardEventWrapper>>" default="None">
                        "Called when a key is pressed or released while the element has focus."
                    </ApiRow>
                    <ApiRow name="shortcuts" ty="Option<KeyboardShortcuts>" default="None">
                        "Keyboard shortcuts handled while the element has focus, as in "
                        <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>"."
                    </ApiRow>
                    <ApiRow name="allow_shortcut_repeats" ty="bool" default="false">
                        "Whether shortcuts also fire for auto-repeated key presses (a key held down)."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseFocusableReturn">
                    <ApiRow name="props" ty="UseFocusableProps">
                        "The "<Code inline=true>"tabindex"</Code>", the focus and keyboard listeners and the element capture. "
                        "Spread them onto the element with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                    <ApiRow name="focus_handle" ty="FocusHandle">"Focuses the element programmatically."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::button::Button, hooks::*};
                        use leptos::{logging::log, prelude::*};

                        // A scrollable region has to be focusable, so keyboard users can scroll it.
                        let UseFocusableReturn { props, focus_handle } = use_focusable(UseFocusableInput {
                            on_focus_change: Some(Callback::new(move |focused| log!("focused: {focused}"))),
                            ..Default::default()
                        });

                        view! {
                            <div role="region" aria-label="Terms" class="scrollable" {..props.into_attrs()}>
                                // long content
                            </div>
                            <Button on_press=move |_| focus_handle.focus()>"Read the terms"</Button>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The release notes scroll, so keyboard users must be able to focus them. Tab to them (or use the button) and "
                    "scroll with "<Keys keys="ArrowDown"/>". Disabled, the notes lose their "<Code inline=true>"tabindex"</Code>
                    "; excluded from the tab order, "<Keys keys="Tab"/>" skips them but the button still focuses them."
                </p>

                <Demo
                    description="A scrollable region made focusable, with a key log and disabled and tab order toggles"
                    source=include_str!("demos/focusable.rs")
                >
                    <FocusableDemo/>
                </Demo>
            </Section>

            <Section title="Programmatic Focus">
                <p>
                    <Code inline=true>"FocusHandle"</Code>" is "<Code inline=true>"Copy"</Code>
                    ", so you can move it into any closure. Its methods:"
                </p>

                <DocTable headers=&["Method", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"focus()"</Code></TableCell>
                        <TableCell>
                            "Focuses the element without scrolling. While a screen reader drives the interaction (virtual "
                            "modality), focus is deferred to avoid VoiceOver scroll issues during CSS transitions. Does nothing "
                            "before the element is rendered."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"has_element()"</Code></TableCell>
                        <TableCell>"Whether the element has been rendered. Always "<Code inline=true>"false"</Code>" during SSR."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"get_element()"</Code></TableCell>
                        <TableCell>"The element, read reactively: an effect re-runs once the element is rendered."</TableCell>
                    </TableRow>
                </DocTable>

                <p>"Use it to focus elements in response to user actions, in your own keyboard navigation, or to restore focus after a dialog closes."</p>

                <Section title="focus_safely">
                    <p>
                        <Code inline=true>"FocusHandle::focus"</Code>" calls "
                        <Code inline=true>"leptonic::utils::focus::focus_safely(&element)"</Code>", which you can also call "
                        "with any element. It focuses without scrolling the page. While a screen reader drives the "
                        "interaction, it waits until running CSS transitions have ended and focuses only if focus hasn\u{2019}t "
                        "moved elsewhere meanwhile. It does nothing for elements that aren\u{2019}t in the document, and during SSR."
                    </p>
                </Section>
            </Section>

            <Section title="Tab Index">
                <p>"The hook sets the "<Code inline=true>"tabindex"</Code>" attribute for you:"</p>

                <DocTable headers=&["State", "tabindex"]>
                    <TableRow><TableCell><Code inline=true>"is_disabled"</Code></TableCell><TableCell>"none (the element is not focusable)"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"exclude_from_tab_order"</Code></TableCell><TableCell><Code inline=true>"-1"</Code></TableCell></TableRow>
                    <TableRow><TableCell>"otherwise"</TableCell><TableCell><Code inline=true>"0"</Code></TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Auto Focus">
                <p>
                    "With "<Code inline=true>"auto_focus: true"</Code>", the element is focused as soon as it is rendered, also "
                    "after client-side navigation."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        let focusable = use_focusable(UseFocusableInput {
                            auto_focus: true,
                            ..Default::default()
                        });
                    ")}
                </Code>
            </Section>

            <Section title="FocusableContext">
                <p>
                    "A parent, such as a tooltip trigger, can add event handlers to a focusable child by providing a "
                    <Code inline=true>"FocusableContext"</Code>". The child\u{2019}s "<Code inline=true>"use_focusable"</Code>
                    " reads the context and runs the parent\u{2019}s handlers after its own. With "
                    <Code inline=true>"element"</Code>", the parent also receives the child\u{2019}s element."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::{hooks::FocusableContext, utils::EventHandler};

                        // In the parent:
                        provide_context(FocusableContext {
                            on_focus: Some(EventHandler::new(|_| { /* parent focus handler */ })),
                            ..Default::default()
                        });

                        // The child's use_focusable chains the context handlers.
                        // They are skipped while the child is disabled.
                    ")}
                </Code>

                <ApiTable kind=ApiKind::Fields of="FocusableContext">
                    <ApiRow name="on_focus, on_blur" ty="Option<EventHandler<FocusEvent>>">"Additional focus and blur handlers."</ApiRow>
                    <ApiRow name="on_keydown, on_keyup" ty="Option<EventHandler<KeyboardEvent>>">"Additional keyboard handlers."</ApiRow>
                    <ApiRow name="aria_describedby" ty="Option<Signal<Option<String>>>">
                        "Describes the element as well (e.g. a tooltip), added to its own description."
                    </ApiRow>
                    <ApiRow name="attrs" ty="Option<FocusableContextAttrs>">
                        "Further attributes for the element (e.g. pointer handlers), spread as given; "
                        <Code inline=true>"FocusableContextAttrs::new(|| attrs.into_any_attr())"</Code>"."
                    </ApiRow>
                    <ApiRow name="element" ty="Option<CapturedElement>">"Receives the focusable element as well."</ApiRow>
                </ApiTable>
                <p>"A disabled element gets none of the context\u{2019}s props."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::Focusable.materialize()>"Focusable"</Link></li>
                <li><Link href=routes::doc::focus::UseFocus.materialize()>"use_focus"</Link></li>
                <li><Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusVisible.materialize()>"use_focus_visible"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
