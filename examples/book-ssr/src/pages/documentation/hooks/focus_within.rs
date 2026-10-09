use indoc::indoc;
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
                "See the "<Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>
                " to compare it with the other focus building blocks."
            </p>

            <ReactAria hook="useFocusWithin"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseFocusWithinInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    " (enabled, no callbacks)."
                </p>

                <ApiTable kind=ApiKind::Input of="UseFocusWithinInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Ignores focus events while true. Turning it on while focus is inside ends the focus-within state: "
                        <Code inline=true>"on_focus_within_change"</Code>" is called with "<Code inline=true>"false"</Code>
                        ", "<Code inline=true>"on_blur_within"</Code>" is not called."
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
                    "Use "<Code inline=true>"is_focus_within"</Code>" for conditional styling, for example to highlight a form "
                    "group while one of its fields is focused. The hook sets no attribute itself, so expose the state as one:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::focus::{UseFocusWithinInput, UseFocusWithinReturn, use_focus_within};

                        let UseFocusWithinReturn { props, is_focus_within } =
                            use_focus_within(UseFocusWithinInput::default());

                        view! {
                            <div
                                class="field-group"
                                data-focus-within=move || is_focus_within.get().then_some("true")
                                {..props.into_attrs()}
                            >
                                <input aria-label="Search"/>
                                <button>"Search"</button>
                            </div>
                        }
                    "#)}
                </Code>

                <Code language=Language::Css>
                    {indoc!(r"
                        .field-group { border: 2px solid var(--border); }
                        .field-group[data-focus-within] { border-color: var(--accent); }
                    ")}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Click into the field, then tab between the field and the buttons: focus stays within the group, so "
                    "neither callback fires. Tab out of the group to end the focus-within state."
                </p>

                <Demo
                    description="Focus-within state and enter and leave counts of a group with a field and two buttons"
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
                <li><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>" (its "<Code inline=true>"within"</Code>" mode)"</li>
            </SeeAlso>
        </DocPage>
    }
}
