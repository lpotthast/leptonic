use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::focus_within::FocusWithinDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseFocusWithin() -> impl IntoView {
    view! {
        <DocPage title="use_focus_within">
            <p>
                "The "<Code inline=true>"use_focus_within"</Code>" hook tracks whether focus is anywhere inside an element. "
                "Unlike "<Link href=routes::doc::focus::UseFocus.materialize()><Code inline=true>"use_focus"</Code></Link>
                ", which only reacts to the element itself, it fires when focus enters or leaves the whole element tree. "
                "See the "<Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>" for domain guidance."
            </p>

            <ReactAria hook="useFocusWithin"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseFocusWithinInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    " (enabled, no callbacks)."
                </p>

                <ApiTable kind=ApiKind::Input of="UseFocusWithinInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Ignores focus events while true. Turning it on while focus is inside ends the focus-within state."
                    </ApiRow>
                    <ApiRow name="on_focus_within" ty="Option<Callback<FocusWithinEvent>>" default="None">
                        "Called when focus enters the element or one of its descendants."
                    </ApiRow>
                    <ApiRow name="on_blur_within" ty="Option<Callback<FocusWithinEvent>>" default="None">
                        "Called when focus leaves the element and all of its descendants."
                    </ApiRow>
                    <ApiRow name="on_focus_within_change" ty="Option<Callback<bool>>" default="None">
                        "Called with the new state whenever focus enters or leaves the tree."
                    </ApiRow>
                </ApiTable>

                <p>
                    <Code inline=true>"FocusWithinEvent"</Code>" wraps the underlying "<Code inline=true>"web_sys::FocusEvent"</Code>
                    " in its "<Code inline=true>"event"</Code>" field."
                </p>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseFocusWithinReturn">
                    <ApiRow name="props" ty="UseFocusWithinProps">
                        "The "<Code inline=true>"focusin"</Code>" and "<Code inline=true>"focusout"</Code>
                        " listeners. Spread them onto the container with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                    <ApiRow name="is_focus_within" ty="Signal<bool>">"Whether focus is currently inside the container."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <p>
                    "Use "<Code inline=true>"is_focus_within"</Code>" for conditional styling, for example to highlight a form group "
                    "while one of its fields is focused:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let UseFocusWithinReturn { props, is_focus_within } =
                            use_focus_within(UseFocusWithinInput::default());

                        view! {
                            <div {..props.into_attrs()} class:field-group-active=is_focus_within>
                                <input type="text"/>
                                <button>"Submit"</button>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>"Click any element inside the container, then tab between them. The container stays focused-within."</p>

                <Demo
                    description="Focus-within state and event counts of a container with an input and two buttons"
                    source=include_str!("demos/focus_within.rs")
                >
                    <FocusWithinDemo/>
                </Demo>
            </Section>

            <Section title="Behavior">
                <ul>
                    <li>
                        "Listens to "<Code inline=true>"focusin"</Code>" and "<Code inline=true>"focusout"</Code>
                        ". Moving focus between children does not end the focus-within state."
                    </li>
                    <li>"Ignores events that bubble through portals from elements outside the container\u{2019}s DOM tree."</li>
                    <li>
                        "While focus is inside, a document-level "<Code inline=true>"focusin"</Code>" listener notices when focus "
                        "lands outside without a "<Code inline=true>"focusout"</Code>", e.g. because the focused element was removed."
                    </li>
                    <li>
                        "Firefox does not fire "<Code inline=true>"blur"</Code>" when a focused form element becomes disabled. "
                        "The hook dispatches a synthetic blur in that case."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::UseFocus.materialize()>"use_focus"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
