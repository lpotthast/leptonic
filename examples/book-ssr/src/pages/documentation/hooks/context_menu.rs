use indoc::indoc;
use leptos::prelude::*;

use super::demos::context_menu::ContextMenuDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseContextMenu() -> impl IntoView {
    view! {
        <DocPage title="use_context_menu">
            <p>
                "The "<Code inline=true>"use_context_menu"</Code>" hook tells you when the user asks for a context menu on "
                "an element, with the mouse, the keyboard, touch or a screen reader, and where. See the "
                <Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>
                " to compare it with the other interaction building blocks."
            </p>

            <ReactAriaSource path="interactions/useContextMenu.ts"/>

            <p>
                "A context menu is requested by a right click ("<Keys keys="Control"/>" + click on macOS), "
                <Keys keys="Shift + F10"/>", the context menu key, "<Keys keys="Control + Enter"/>" on macOS, or a long "
                "press on iOS. The hook prevents the browser\u{2019}s own context menu on the element and reports the "
                "position the browser gives the request: the pointer\u{2019}s for a right click or long press, a point the "
                "browser picks for "<Keys keys="Shift + F10"/>" and the context menu key, and the center of the element "
                "for "<Keys keys="Control + Enter"/>" on macOS."
            </p>
            <p>
                "You rarely call it yourself: a "<Link href=format!("{}#context-menus", routes::doc::menu::Atom.materialize())>
                "context menu "<Code inline=true>"MenuTrigger"</Code></Link>" opens its menu with it, and "
                <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" and "
                <Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link>" take an "
                <Code inline=true>"on_context_menu"</Code>" callback that uses it, including the long press."
            </p>

            <Section title="Input">
                <ApiTable kind=ApiKind::Input of="UseContextMenuInput">
                    <ApiRow name="on_context_menu" ty="Option<Callback<ContextMenuEvent>>" default="None">
                        "Called when a context menu is requested. Without it, the hook does nothing and the browser\u{2019}s "
                        "context menu stays."
                    </ApiRow>
                </ApiTable>

                <p><Code inline=true>"ContextMenuEvent"</Code>" describes the request:"</p>
                <ApiTable kind=ApiKind::Fields of="ContextMenuEvent">
                    <ApiRow name="target" ty="SendWrapper<Element>">"The element the context menu is for."</ApiRow>
                    <ApiRow name="point" ty="Point">"Pointer coordinates in the event target, in CSS pixels."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseContextMenuReturn">
                    <ApiRow name="props" ty="UseContextMenuProps">
                        "The "<Code inline=true>"contextmenu"</Code>" and "<Code inline=true>"keydown"</Code>" handlers. Spread "
                        "them onto the element with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                    <ApiRow name="long_press" ty="Option<LongPress>">"Long-press handlers and options, including the threshold and accessible description."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::interactions::{ContextMenuEvent, UseContextMenuInput, UseContextMenuReturn, use_context_menu};
                        use leptos::prelude::*;

                        let UseContextMenuReturn { props, .. } = use_context_menu(UseContextMenuInput {
                            on_context_menu: Some(Callback::new(move |e: ContextMenuEvent| {
                                open_menu_at(e.point.x, e.point.y);
                            })),
                        });

                        view! {
                            <div tabindex="0" {..props.into_attrs()}>"Right click me"</div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="A focusable area reporting where context menus are requested" source=include_str!("demos/context_menu.rs")>
                    <ContextMenuDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="Shift + F10 / ContextMenu">"Requests a context menu for the focused element."</KeyRow>
                    <KeyRow keys="Control + Enter">"Requests a context menu at the center of the focused element (macOS)."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=format!("{}#context-menus", routes::doc::menu::Atom.materialize())>"Menu Atoms"</Link>" (context menus)"</li>
                <li><Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></li>
                <li><Link href=routes::doc::interactions::PressResponder.materialize()>"PressResponder"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
