use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::focus::FocusDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseFocus() -> impl IntoView {
    view! {
        <DocPage title="use_focus">
            <p>
                "The "<Code inline=true>"use_focus"</Code>" hook tracks the focus of a single element. Its callbacks fire "
                "when the element itself receives or loses focus, not when a descendant does \u{2014} use "
                <Link href=routes::doc::focus::UseFocusWithin.materialize()><Code inline=true>"use_focus_within"</Code></Link>
                " for that. See the "<Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>" for domain guidance."
            </p>

            <ReactAria hook="useFocus"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseFocusInput"</Code>" does not implement "<Code inline=true>"Default"</Code>
                    ", so you set every field."
                </p>

                <ApiTable kind=ApiKind::Input of="UseFocusInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>">"Ignores all focus events while true."</ApiRow>
                    <ApiRow name="on_focus" ty="Option<Callback<FocusEvent>>">"Called when the element receives focus."</ApiRow>
                    <ApiRow name="on_blur" ty="Option<Callback<FocusEvent>>">"Called when the element loses focus."</ApiRow>
                    <ApiRow name="on_focus_change" ty="Option<Callback<bool>>">
                        "Called with the new focus state whenever the element gains or loses focus."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseFocusReturn">
                    <ApiRow name="props" ty="UseFocusProps">
                        "The "<Code inline=true>"focus"</Code>" and "<Code inline=true>"blur"</Code>
                        " listeners. Spread them onto the element with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r"
                        let (is_focused, set_is_focused) = signal(false);

                        let UseFocusReturn { props } = use_focus(UseFocusInput {
                            is_disabled: Signal::stored(false),
                            on_focus: None,
                            on_blur: None,
                            on_focus_change: Some(Callback::new(move |focused| set_is_focused.set(focused))),
                        });

                        view! {
                            <input {..props.into_attrs()}/>
                        }
                    ")}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Focus and blur events of a focusable div, with a disabled toggle" source=include_str!("demos/focus.rs")>
                    <FocusDemo/>
                </Demo>
            </Section>

            <Section title="Behavior">
                <ul>
                    <li>"Only fires when the element itself is focused or blurred, not its descendants."</li>
                    <li>
                        "Before calling "<Code inline=true>"on_focus"</Code>", the hook checks that "
                        <Code inline=true>"document.activeElement"</Code>" still is the element, in case an earlier focus "
                        "handler already moved focus elsewhere. The check pierces shadow roots."
                    </li>
                    <li>
                        "Firefox does not fire "<Code inline=true>"blur"</Code>" when a focused form element becomes disabled. "
                        "The hook watches the element and dispatches a synthetic blur in that case."
                    </li>
                    <li>"While "<Code inline=true>"is_disabled"</Code>" is true, no callback is called."</li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusWithin.materialize()>"use_focus_within"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
