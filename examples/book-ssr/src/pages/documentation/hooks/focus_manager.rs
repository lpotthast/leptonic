use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::focus_manager_basic::FocusManagerBasicDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseFocusManager() -> impl IntoView {
    view! {
        <DocPage title="use_focus_manager">
            <p>
                "The "<Code inline=true>"use_focus_manager"</Code>" hook moves focus programmatically within a container: to the "
                "next, previous, first or last focusable element. The "
                <Link href=routes::doc::focus::FocusManagerProvider.materialize()>"FocusManagerProvider"</Link>
                " atom renders such a container for you. See the "<Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>
                " to compare it with the other focus building blocks."
            </p>

            <ReactAria hook="FocusScope"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseFocusManagerInput"</Code>" is an empty struct; pass "
                    <Code inline=true>"UseFocusManagerInput::default()"</Code>". All configuration is passed per call as "
                    <AnchorLink href="#focusmanageroptions">"FocusManagerOptions"</AnchorLink>"."
                </p>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseFocusManagerReturn">
                    <ApiRow name="focus_manager" ty="FocusManager">
                        "Moves focus within the container, see "<AnchorLink href="#focusmanager">"FocusManager"</AnchorLink>
                        ". Cheap to clone."
                    </ApiRow>
                    <ApiRow name="props" ty="UseFocusManagerProps">
                        "Captures the container element. Spread it onto the container with "
                        <Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                </ApiTable>

                <Section title="FocusManager">
                    <p>
                        "Every method takes "<Code inline=true>"FocusManagerOptions"</Code>" and returns the element it found, or "
                        <Code inline=true>"None"</Code>" if there is none (or the container is not rendered yet)."
                    </p>

                    <DocTable headers=&["Method", "Description"]>
                        <TableRow>
                            <TableCell><Code inline=true>"focus_next"</Code>", "<Code inline=true>"focus_previous"</Code></TableCell>
                            <TableCell>
                                "Focus the next or previous focusable element after "<Code inline=true>"from"</Code>
                                ". If "<Code inline=true>"from"</Code>" is outside the container, start at its first or last element."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"focus_first"</Code>", "<Code inline=true>"focus_last"</Code></TableCell>
                            <TableCell>"Focus the first or last focusable element of the container."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"find_first"</Code>", "<Code inline=true>"find_last"</Code></TableCell>
                            <TableCell>"Find the first or last focusable element without focusing it, e.g. to focus it your own way."</TableCell>
                        </TableRow>
                    </DocTable>

                    <p>"The "<Code inline=true>"focus_*"</Code>" methods let the browser scroll the element into view."</p>
                </Section>

                <Section title="FocusManagerOptions">
                    <p><Code inline=true>"FocusManagerOptions"</Code>" implements "<Code inline=true>"Default"</Code>"."</p>

                    <ApiTable kind=ApiKind::Input of="FocusManagerOptions">
                        <ApiRow name="from" ty="Option<web_sys::Element>" default="None">
                            "The element to start from. "<Code inline=true>"None"</Code>" starts at "
                            <Code inline=true>"document.activeElement"</Code>"."
                        </ApiRow>
                        <ApiRow name="wrap" ty="bool" default="false">
                            "Whether "<Code inline=true>"focus_next"</Code>" and "<Code inline=true>"focus_previous"</Code>
                            " wrap around at the end and beginning."
                        </ApiRow>
                        <ApiRow name="tabbable" ty="bool" default="false">
                            "Only consider elements reachable with "<Keys keys="Tab"/>" ("<Code inline=true>"tabindex >= 0"</Code>")."
                        </ApiRow>
                        <ApiRow name="accept" ty="Option<Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync>>" default="None">
                            "Skips elements for which the filter returns false."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Example">
                <p>
                    "Arrow keys moving focus between the buttons of a group, with "
                    <Link href=routes::doc::interactions::UseKeyboard.materialize()>"use_keyboard"</Link>" handling the keys:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            hooks::*,
                            utils::keyboard_shortcut::{KeyboardShortcuts, Shortcut},
                        };

                        let UseFocusManagerReturn { focus_manager, props } =
                            use_focus_manager(UseFocusManagerInput::default());

                        let wrap = || FocusManagerOptions { wrap: true, ..Default::default() };
                        let next = focus_manager.clone();
                        let keyboard = use_keyboard(UseKeyboardInput {
                            shortcuts: Some(
                                KeyboardShortcuts::new()
                                    .on(Shortcut::key("ArrowRight"), move |_| { next.focus_next(wrap()); })
                                    .on(Shortcut::key("ArrowLeft"), move |_| { focus_manager.focus_previous(wrap()); }),
                            ),
                            allow_repeats: true,
                            ..Default::default()
                        });

                        view! {
                            <div role="group" aria-label="Clipboard" {..props.into_attrs()} {..keyboard.props.into_attrs()}>
                                <button>"Cut"</button>
                                <button>"Copy"</button>
                                <button>"Paste"</button>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The buttons move focus within the formatting group below. \u{201c}Strikethrough\u{201d} has "
                    <Code inline=true>"tabindex=\"-1\""</Code>": the focus manager visits it until you check "
                    "\u{201c}Tabbable only\u{201d}."
                </p>

                <Demo
                    description="Buttons moving focus to the first, previous, next and last element of a group, with wrap and tabbable-only options"
                    source=include_str!("demos/focus_manager_basic.rs")
                >
                    <FocusManagerBasicDemo/>
                </Demo>
            </Section>

            <Section title="Custom Filter">
                <p>"The "<Code inline=true>"accept"</Code>" filter skips elements you don\u{2019}t want to visit:"</p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use std::sync::Arc;

                        focus_manager.focus_next(FocusManagerOptions {
                            accept: Some(Arc::new(|el: &web_sys::Element| !el.class_list().contains("skip-focus"))),
                            ..Default::default()
                        });
                    "#)}
                </Code>
            </Section>

            <Section title="Focus Containment">
                <p>
                    <Code inline=true>"use_focus_manager"</Code>" only moves focus when you call it. It does not contain focus "
                    "or handle "<Keys keys="Tab"/>". To keep "<Keys keys="Tab"/>" inside a container, use the "
                    <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>
                    " atom, which also restores and auto-focuses. Children of a "<Code inline=true>"FocusScope"</Code>
                    " get its "<Code inline=true>"FocusManager"</Code>" from the "<Code inline=true>"FocusScopeContext"</Code>"."
                </p>
            </Section>

            <Section title="Behavior">
                <ul>
                    <li>
                        "Elements are visited in document order. Unlike the browser\u{2019}s tab order, a positive "
                        <Code inline=true>"tabindex"</Code>" does not move an element to the front."
                    </li>
                    <li>
                        "Disabled, hidden and inert elements are skipped (see "
                        <Link href=routes::doc::focus::Focusability.materialize()>"focusability"</Link>")."
                    </li>
                    <li>"With "<Code inline=true>"tabbable: true"</Code>", the radio buttons of a group count as a single stop."</li>
                    <li>"The walk descends into shadow roots."</li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::FocusManagerProvider.materialize()>"FocusManagerProvider"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
                <li><Link href=routes::doc::focus::UseHasTabbableChild.materialize()>"use_has_tabbable_child"</Link></li>
                <li><Link href=routes::doc::Toolbar.materialize()>"Toolbar"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
