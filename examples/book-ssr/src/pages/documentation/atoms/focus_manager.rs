use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::focus_manager::FocusManagerDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomFocusManager() -> impl IntoView {
    view! {
        <DocPage title="FocusManager">
            <p>
                "The "<Code inline=true>"FocusManager"</Code>" atom renders a "<Code inline=true>"<div>"</Code>
                " and creates a focus manager for it: an object that moves focus to the next, previous, first or last focusable "
                "element inside the div. Use it for arrow-key navigation in custom widgets. See the "
                <Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>" for domain guidance."
            </p>

            <ReactAria hook="FocusScope"/>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::focus::UseFocusManager.materialize()><Code inline=true>"use_focus_manager"</Code></Link>
                    ". The atom spreads the hook\u{2019}s props on its div and hands the manager to its children."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="FocusManager">
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the div."</ApiRow>
                    <ApiRow name="children" ty="Fn(FocusManager) -> impl IntoView">
                        "Renders the content. Receives the "<Link href=format!("{}#focusmanager", routes::doc::focus::UseFocusManager.materialize())>"FocusManager"</Link>
                        ". Bind it with "<Code inline=true>"let:manager"</Code>" and pass it on to the components that need it."
                    </ApiRow>
                </ApiTable>
                <p>
                    "The atom also provides the "<Code inline=true>"FocusManager"</Code>" as context, so components deeper "
                    "inside can get it with "<Code inline=true>"expect_context::<FocusManager>()"</Code>"."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::prelude::*,
                            components::prelude::Button,
                            hooks::{FocusManager as Manager, FocusManagerOptions},
                        };

                        #[component]
                        fn NextButton(manager: Manager) -> impl IntoView {
                            view! {
                                <Button on_press=move |_| { manager.focus_next(FocusManagerOptions::default()); }>
                                    "Next"
                                </Button>
                            }
                        }

                        view! {
                            // `let:` binds the manager the atom passes to its children.
                            <FocusManager let:manager>
                                <NextButton manager/>
                                <input/>
                            </FocusManager>
                        }
                    "#)}
                </Code>
                <p>
                    "Each method takes "<Link href=format!("{}#focusmanageroptions", routes::doc::focus::UseFocusManager.materialize())>"FocusManagerOptions"</Link>
                    " ("<Code inline=true>"from"</Code>", "<Code inline=true>"wrap"</Code>", "<Code inline=true>"tabbable"</Code>
                    ", "<Code inline=true>"accept"</Code>") and returns the element it focused, if any."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Tab into the group, then use the arrow keys, "<Keys keys="Home"/>" and "<Keys keys="End"/>
                    ". All buttons stay in the tab order; for a single tab stop, also manage "<Code inline=true>"tabindex"</Code>"."
                </p>

                <Demo description="Buttons navigated with the arrow keys through a focus manager" source=include_str!("demos/focus_manager.rs") source_open=true>
                    <FocusManagerDemo/>
                </Demo>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
