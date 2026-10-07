use indoc::indoc;
use leptos::prelude::*;

use super::demos::overlay::BasicOverlayDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseOverlay() -> impl IntoView {
    view! {
        <DocPage title="use_overlay">
            <p>
                "The "<Code inline=true>"use_overlay"</Code>" hook dismisses an overlay: on Escape, on a press outside of it or "
                "when focus leaves it. See the "<Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link>
                " for the other overlay building blocks."
            </p>

            <ReactAriaSource path="overlays/useOverlay.ts"/>

            <Section title="Input">
                <p>
                    "Pass a "<Code inline=true>"UseOverlayInput"</Code>" with every field named; the Default column gives the "
                    "value for fields you don\u{2019}t need."
                </p>

                <ApiTable kind=ApiKind::Input of="UseOverlayInput">
                    <ApiRow name="is_open" ty="Signal<bool>">"Whether the overlay is open. Required."</ApiRow>
                    <ApiRow name="on_close" ty="Callback<()>">"Called when the overlay should close. Required."</ApiRow>
                    <ApiRow name="is_dismissable" ty="Signal<bool>" default="false">
                        "Close the overlay when the user presses outside of it."
                    </ApiRow>
                    <ApiRow name="should_close_on_blur" ty="Signal<bool>" default="false">"Close the overlay when focus leaves it."</ApiRow>
                    <ApiRow name="is_keyboard_dismiss_disabled" ty="Signal<bool>" default="false">
                        "Don\u{2019}t close the overlay on Escape."
                    </ApiRow>
                    <ApiRow name="should_close_on_interact_outside" ty="Option<InteractOutsideFilter>" default="None">
                        "Decides for outside presses and blur whether the overlay closes: receives the element the user "
                        "interacted with (or that received focus) and returns "<Code inline=true>"true"</Code>" to close. "
                        <Code inline=true>"None"</Code>" closes on every outside interaction."
                    </ApiRow>
                    <ApiRow name="group" ty="Option<CapturedElement>" default="None">
                        "The element the overlay belongs to for the overlay stack and outside presses, when it is part of a group "
                        "(a submenu\u{2019}s popover in its root popover\u{2019}s container). "<Code inline=true>"None"</Code>
                        ": the overlay element."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseOverlayReturn">
                    <ApiRow name="props" ty="UseOverlayProps">
                        "The overlay\u{2019}s "<Code inline=true>"id"</Code>", its element capture and its "
                        <Code inline=true>"keydown"</Code>", "<Code inline=true>"focusin"</Code>" and "
                        <Code inline=true>"focusout"</Code>" handlers. Spread "<Code inline=true>"{..props.into_attrs()}"</Code>
                        " onto the overlay element."
                    </ApiRow>
                    <ApiRow name="id" ty="String">
                        "The overlay\u{2019}s id. Pass it to "
                        <Link href=routes::doc::overlay_behavior::UseOverlayTrigger.materialize()>"use_overlay_trigger"</Link>
                        " as "<Code inline=true>"overlay_id"</Code>"."
                    </ApiRow>
                    <ApiRow name="overlay_element" ty="CapturedElement">
                        "The overlay element once it is rendered, e.g. for "
                        <Link href=routes::doc::overlay_behavior::AriaHideOutside.materialize()>"aria_hide_outside"</Link>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::prelude::FocusScope, hooks::*};

                        let (is_open, set_is_open) = signal(false);

                        let UseOverlayReturn { props, .. } = use_overlay(UseOverlayInput {
                            is_open: is_open.into(),
                            on_close: Callback::new(move |()| set_is_open.set(false)),
                            is_dismissable: Signal::stored(true),
                            should_close_on_blur: Signal::stored(false),
                            is_keyboard_dismiss_disabled: Signal::stored(false),
                            should_close_on_interact_outside: None,
                            group: None,
                        });

                        // The overlay renders again on every opening, so its attributes are cloned per render.
                        let attrs = StoredValue::new(props.into_attrs());

                        view! {
                            <Show when=move || is_open.get()>
                                <div {..attrs.get_value()} role="dialog" aria-label="Filters">
                                    <FocusScope restore_focus=true auto_focus=true>"…"</FocusScope>
                                </div>
                            </Show>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Choose how the panel may be dismissed, then open it. A "
                    <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                    " moves focus into the panel, so the Escape key reaches it, and back to the button when it closes."
                </p>

                <Demo
                    description="A panel dismissed with Escape, a press outside or blur, as configured"
                    source=include_str!("demos/overlay.rs")
                >
                    <BasicOverlayDemo/>
                </Demo>
            </Section>

            <Section title="Dismissing">
                <Section title="Escape">
                    <p>
                        "Pressing "<Keys keys="Escape"/>" while focus is inside the overlay closes it, if it is the topmost "
                        "overlay. The key press stops there, so outer overlays stay open. Escape is ignored during an IME "
                        "composition. With "<Code inline=true>"is_keyboard_dismiss_disabled"</Code>" the key press bubbles on."
                    </p>
                    <p>
                        "The key handler sits on the overlay element, so move focus into the overlay when it opens, for example "
                        "with a "<Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>"."
                    </p>
                </Section>

                <Section title="Press Outside">
                    <p>
                        "With "<Code inline=true>"is_dismissable"</Code>", a press outside the overlay closes it if it was the "
                        "topmost overlay when the press started. Detection comes from "
                        <Link href=routes::doc::interactions::UseInteractOutside.materialize()>"use_interact_outside"</Link>
                        ". The press isn\u{2019}t cancelled: the outside element still gets its default action (a link "
                        "navigates, a checkbox toggles), but the event doesn\u{2019}t propagate to its handlers. "
                        <Code inline=true>"should_close_on_interact_outside"</Code>" lets you exclude elements, for example a "
                        "trigger that toggles the overlay itself."
                    </p>
                </Section>

                <Section title="Blur">
                    <p>
                        "With "<Code inline=true>"should_close_on_blur"</Code>", the overlay closes when focus moves to an "
                        "element outside of it, whether or not it is the topmost overlay. Focus moving into a child focus scope "
                        "(e.g. a menu opened from a dialog) or being lost to the page body (e.g. when switching browser tabs) "
                        "doesn\u{2019}t close it."
                    </p>
                </Section>

                <Section title="Overlay Stack">
                    <p>
                        "Open overlays are kept on a stack in the order they opened. Escape and outside presses only close the "
                        "topmost overlay, so a press in a nested overlay doesn\u{2019}t close its parent. The overlays of a "
                        <Code inline=true>"group"</Code>" share one entry."
                    </p>
                </Section>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Escape">"Closes the topmost overlay, unless keyboard dismissal is disabled."</KeyRow>
                </KeyboardTable>
            </Section>

            <Section title="Roles and Focus">
                <p>
                    "The hook doesn\u{2019}t manage focus, roles or names. Give the overlay a role (usually "
                    <Code inline=true>"dialog"</Code>") and a name, move focus into it with a "
                    <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>" (containing it for modal "
                    "overlays), connect the trigger with "
                    <Link href=routes::doc::overlay_behavior::UseOverlayTrigger.materialize()>"use_overlay_trigger"</Link>
                    " and add "<Link href=routes::doc::overlay_behavior::DismissButton.materialize()>"DismissButton"</Link>
                    "s for screen reader users without an Escape key."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UseOverlayTrigger.materialize()>"use_overlay_trigger"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UseOverlayPosition.materialize()>"use_overlay_position"</Link></li>
                <li><Link href=routes::doc::interactions::UseInteractOutside.materialize()>"use_interact_outside"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
