use indoc::indoc;
use leptos::prelude::*;

use super::demos::has_tabbable_child::HasTabbableChildDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseHasTabbableChild() -> impl IntoView {
    view! {
        <DocPage title="use_has_tabbable_child">
            <p>
                "The "<Code inline=true>"use_has_tabbable_child"</Code>" hook tells you whether an element contains anything "
                "you can reach with "<Keys keys="Tab"/>". Use it to decide whether a container should be a tab stop itself, or to make sure a "
                "focus trap has something to focus. "
                "See the "<Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>
                " to compare it with the other focus building blocks."
            </p>

            <ReactAriaSource path="focus/useHasTabbableChild.ts"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseHasTabbableChildInput"</Code>" implements "<Code inline=true>"Default"</Code>"."
                </p>

                <ApiTable kind=ApiKind::Input of="UseHasTabbableChildInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Stops observing the container. "<Code inline=true>"has_tabbable_child"</Code>" is "
                        <Code inline=true>"false"</Code>" while disabled."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseHasTabbableChildReturn">
                    <ApiRow name="has_tabbable_child" ty="Signal<bool>">
                        "Whether the container has at least one tabbable descendant."
                    </ApiRow>
                    <ApiRow name="props" ty="UseHasTabbableChildProps">
                        "Captures the container element. Spread it onto the container with "
                        <Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <p>
                    "A common pattern makes a container a tab stop only while it has no tabbable children. Otherwise it gets "
                    <Code inline=true>"tabindex=\"-1\""</Code>": it can still be focused programmatically, but "<Keys keys="Tab"/>" goes straight "
                    "to its children."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::focus::{UseHasTabbableChildInput, UseHasTabbableChildReturn, use_has_tabbable_child};

                        let UseHasTabbableChildReturn { has_tabbable_child, props } =
                            use_has_tabbable_child(UseHasTabbableChildInput::default());

                        view! {
                            <div
                                role="group"
                                aria-label="Attachments"
                                tabindex=move || if has_tabbable_child.get() { -1 } else { 0 }
                                {..props.into_attrs()}
                            >
                                <button>"Child button"</button>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The demo applies the pattern above. Hide the button and the field, and the group itself becomes a tab stop. "
                    "Disabled, the hook reports no tabbable child."
                </p>

                <Demo
                    description="Group that becomes a tab stop when its button and field are hidden, with a disabled toggle"
                    source=include_str!("demos/has_tabbable_child.rs")
                >
                    <HasTabbableChildDemo/>
                </Demo>
            </Section>

            <Section title="How It Works">
                <p>
                    "The hook checks the container once it is rendered and then watches its subtree with a "
                    <Code inline=true>"MutationObserver"</Code>". It reacts to added and removed elements anywhere below the "
                    "container and to changes of the "<Code inline=true>"tabindex"</Code>" and "<Code inline=true>"disabled"</Code>
                    " attributes. Each change triggers a rescan with the same tabbability rules the focus manager uses ("
                    <Link href=routes::doc::focus::Focusability.materialize()>"is_tabbable"</Link>"): disabled, hidden and "
                    "inert elements and elements with "<Code inline=true>"tabindex=\"-1\""</Code>" don\u{2019}t count."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusManager.materialize()>"create_focus_manager"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
                <li><Link href=routes::doc::focus::Focusability.materialize()>"focusability"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
