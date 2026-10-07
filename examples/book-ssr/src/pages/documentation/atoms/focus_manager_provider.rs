use indoc::indoc;
use leptos::prelude::*;

use super::demos::focus_manager_provider::FocusManagerProviderDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomFocusManagerProvider() -> impl IntoView {
    view! {
        <DocPage title="FocusManagerProvider">
            <p>
                "The "<Code inline=true>"FocusManagerProvider"</Code>" atom renders a "<Code inline=true>"<div>"</Code>
                " and creates a focus manager for it: an object that moves focus to the next, previous, first or last focusable "
                "element inside the div. Use it for arrow-key navigation in your own composite controls. See the "
                <Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>" for the other focus building blocks."
            </p>

            <ReactAria hook="FocusScope"/>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::focus::UseFocusManager.materialize()><Code inline=true>"use_focus_manager"</Code></Link>
                    ". The atom spreads the hook\u{2019}s props on its div and hands the manager to its children. It renders "
                    "no data attributes."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="FocusManagerProvider">
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the div."</ApiRow>
                    <ApiRow name="children" ty="Fn(FocusManager) -> V">
                        "Renders the content. Receives the "<Link href=format!("{}#focusmanager", routes::doc::focus::UseFocusManager.materialize())>"FocusManager"</Link>
                        ". Bind it with "<Code inline=true>"let:manager"</Code>" and pass it on to whatever needs it. Required."
                    </ApiRow>
                </ApiTable>
                <p>
                    "The atom also provides the "<Code inline=true>"FocusManager"</Code>" as context, so Leptos components deeper "
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
                            <FocusManagerProvider let:manager>
                                <NextButton manager/>
                                <input/>
                            </FocusManagerProvider>
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
                    "Tab into the group, then use "<Keys keys="ArrowLeft"/>", "<Keys keys="ArrowRight"/>", "
                    <Keys keys="Home"/>" and "<Keys keys="End"/>". "
                    <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>
                    " handles the keys. All buttons stay in the tab order; for a single tab stop, also manage "
                    <Code inline=true>"tabindex"</Code>", as a "<Link href=routes::doc::Toolbar.materialize()>"Toolbar"</Link>" does."
                </p>

                <Demo description="Buttons navigated with the arrow keys, Home and End through a focus manager" source=include_str!("demos/focus_manager_provider.rs") source_open=true>
                    <FocusManagerProviderDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>
                    "Style the div through "<Code inline=true>"classes"</Code>" and its children through their own attributes. The "
                    "atom renders no data attributes; for a ring around the whole group while one of its children has keyboard "
                    "focus, wrap the content in an element with "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>" in "
                    <Code inline=true>"within"</Code>" mode."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
                <li><Link href=routes::doc::Toolbar.materialize()>"Toolbar"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
