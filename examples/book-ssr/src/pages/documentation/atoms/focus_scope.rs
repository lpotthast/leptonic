use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::focus_scope::FocusScopeDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomFocusScope() -> impl IntoView {
    view! {
        <DocPage title="FocusScope">
            <p>
                "The "<Code inline=true>"FocusScope"</Code>" atom contains, restores and auto-focuses focus for dialogs, menus "
                "and other overlays, and gives its children a "<Code inline=true>"FocusManager"</Code>
                " to move focus programmatically. See the "<Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>
                " for the other focus building blocks."
            </p>

            <ReactAria hook="FocusScope"/>

            <Section title="Hooks Used">
                <p>
                    "The "<Code inline=true>"FocusManager"</Code>" of "
                    <Link href=routes::doc::focus::UseFocusManager.materialize()><Code inline=true>"use_focus_manager"</Code></Link>
                    ", which also decides what counts as focusable."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="FocusScope">
                    <ApiRow name="contain" ty="Signal<bool>" default="false">
                        "Keep focus inside the scope: "<Keys keys="Tab"/>" and "<Keys keys="Shift + Tab"/>" wrap around, and "
                        "focus that leaves the scope (by a click or programmatically) is moved back. May change while the scope "
                        "is mounted, e.g. a non-modal popover starts containing focus once a dialog is inside. Always give "
                        "keyboard users a way out of a containing scope, such as a close button or "<Keys keys="Escape"/>"."
                    </ApiRow>
                    <ApiRow name="restore_focus" ty="bool" default="false">
                        "When the scope unmounts, focus the element that was focused when it mounted."
                    </ApiRow>
                    <ApiRow name="auto_focus" ty="bool" default="false">
                        "Focus the first tabbable element (or, if there is none, the first focusable one) when the scope mounts, "
                        "unless focus is already inside."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Classes and styles of the wrapper "<Code inline=true>"<div>"</Code>"."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The scope\u{2019}s content. Required."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude::FocusScope;

                        view! {
                            <FocusScope contain=true restore_focus=true auto_focus=true>
                                <input type="text" aria-label="First name"/>
                                <input type="text" aria-label="Last name"/>
                                <button type="submit">"Submit"</button>
                            </FocusScope>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Open the form: focus moves to the first field. "<Keys keys="Tab"/>" cycles through the form without "
                    "leaving it. Submit or cancel, and focus returns to \u{201c}Open form\u{201d}."
                </p>

                <Demo
                    description="Form in a FocusScope with contain, restore_focus and auto_focus"
                    source=include_str!("demos/focus_scope.rs")
                >
                    <FocusScopeDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>
                    "The scope renders a wrapper "<Code inline=true>"<div>"</Code>" with "<Code inline=true>"display: contents"</Code>
                    ", so it does not affect the layout of its children. Style the children, or a container you put inside the scope."
                </p>
            </Section>

            <Section title="FocusScopeContext">
                <p>
                    <Code inline=true>"FocusScope"</Code>" provides a "<Code inline=true>"FocusScopeContext"</Code>
                    ". Its "<Code inline=true>"focus_manager"</Code>" moves focus within the scope, with the methods described on the "
                    <Link href=routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link>" page."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::focus_scope::FocusScopeContext,
                            components::prelude::Button,
                            hooks::FocusManagerOptions,
                        };

                        #[component]
                        fn NextButton() -> impl IntoView {
                            let ctx = expect_context::<FocusScopeContext>();

                            view! {
                                <Button on_press=move |_| {
                                    ctx.focus_manager.focus_next(FocusManagerOptions { wrap: true, ..Default::default() });
                                }>
                                    "Next"
                                </Button>
                            }
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Composition">
                <ul>
                    <li>
                        "Scopes nest. Only the innermost containing scope handles "<Keys keys="Tab"/>", and an outer scope does "
                        "not pull focus out of an inner one."
                    </li>
                    <li>
                        "Before restoring focus, the scope dispatches the cancelable "<Code inline=true>"leptonic-focus-scope-restore"</Code>
                        " event ("<Code inline=true>"RESTORE_FOCUS_EVENT"</Code>"). Call "<Code inline=true>"prevent_default()"</Code>
                        " on it to restore focus yourself."
                    </li>
                    <li>
                        "Without "<Code inline=true>"contain"</Code>", a scope with "<Code inline=true>"restore_focus"</Code>
                        " sends "<Keys keys="Tab"/>" to the element after the one focus will return to, so you can tab out of "
                        "an overlay naturally."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link></li>
                <li><Link href=routes::doc::focus::FocusManagerProvider.materialize()>"FocusManagerProvider"</Link></li>
                <li><Link href=routes::doc::focus::UseHasTabbableChild.materialize()>"use_has_tabbable_child"</Link></li>
                <li><Link href=routes::doc::dialog::Atom.materialize()>"Dialog Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
