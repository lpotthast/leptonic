use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    focus_manager_basic::FocusManagerBasicDemo, focus_manager_scope::FocusManagerScopeDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseFocusManager() -> impl IntoView {
    view! {
        <DocPage title="use_focus_manager">
            <p>
                "The "<Code inline=true>"use_focus_manager"</Code>" hook moves focus programmatically within a container: to the "
                "next, previous, first or last focusable element. "
                "See the "<Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>" for domain guidance."
            </p>

            <ReactAria hook="FocusScope"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseFocusManagerInput"</Code>" is an empty struct; pass "
                    <Code inline=true>"UseFocusManagerInput::default()"</Code>". All configuration is passed per call as "
                    <Code inline=true>"FocusManagerOptions"</Code>"."
                </p>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseFocusManagerReturn">
                    <ApiRow name="focus_manager" ty="FocusManager">
                        "Moves focus within the container, see "<a href="#focusmanager">"FocusManager"</a>". Cheap to clone."
                    </ApiRow>
                    <ApiRow name="props" ty="UseFocusManagerProps">
                        "Captures the container element. Spread it onto the container with "
                        <Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

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

                <p>
                    "The "<Code inline=true>"focus_*"</Code>" methods let the browser scroll the element into view."
                </p>

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
                            "Only consider elements reachable with Tab ("<Code inline=true>"tabindex >= 0"</Code>")."
                        </ApiRow>
                        <ApiRow name="accept" ty="Option<Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync>>" default="None">
                            "Skips elements for which the filter returns false."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let UseFocusManagerReturn { focus_manager, props } =
                            use_focus_manager(UseFocusManagerInput::default());

                        let on_key_down = move |e: KeyboardEvent| {
                            if e.key() == "ArrowRight" {
                                focus_manager.focus_next(FocusManagerOptions { wrap: true, ..Default::default() });
                            }
                        };

                        view! {
                            <div role="toolbar" {..props.into_attrs()} on:keydown=on_key_down>
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
                    "The buttons move focus within the container below. The element with "
                    <Code inline=true>"tabindex=\"-1\""</Code>" is skipped once you check \u{201c}Tabbable only\u{201d}."
                </p>

                <Demo
                    description="Buttons moving focus to the first, previous, next and last element of a container, with wrap and tabbable-only options"
                    source=include_str!("demos/focus_manager_basic.rs")
                >
                    <FocusManagerBasicDemo/>
                </Demo>
            </Section>

            <Section title="Tabbable Option">
                <p>
                    "By default, the focus manager visits every focusable element, including those with "
                    <Code inline=true>"tabindex=\"-1\""</Code>". Set "<Code inline=true>"tabbable: true"</Code>
                    " to only visit elements reachable with Tab:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        focus_manager.focus_next(FocusManagerOptions {
                            tabbable: true,
                            ..Default::default()
                        });
                    ")}
                </Code>
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

            <Section title="Focus Trapping">
                <p>
                    <Code inline=true>"use_focus_manager"</Code>" only moves focus when you call it. It does not trap focus or "
                    "handle Tab. To keep Tab inside a container, use the "
                    <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope atom"</Link>
                    ", which also restores and auto-focuses. Children of a "<Code inline=true>"FocusScope"</Code>
                    " get its "<Code inline=true>"FocusManager"</Code>" from the "<Code inline=true>"FocusScopeContext"</Code>"."
                </p>

                <p>"Tab through the container below. Focus wraps from the last element to the first, and back with Shift + Tab."</p>

                <Demo description="FocusScope with contain=true, trapping Tab inside a container" source=include_str!("demos/focus_manager_scope.rs")>
                    <FocusManagerScopeDemo/>
                </Demo>
            </Section>

            <Section title="Behavior">
                <ul>
                    <li>
                        "Elements are visited in document order. Unlike the browser\u{2019}s tab order, a positive "
                        <Code inline=true>"tabindex"</Code>" does not move an element to the front."
                    </li>
                    <li>"Disabled, hidden and inert elements are skipped."</li>
                    <li>"With "<Code inline=true>"tabbable: true"</Code>", the radio buttons of a group count as a single stop."</li>
                    <li>"The walk descends into shadow roots."</li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope atom"</Link></li>
                <li><Link href=routes::doc::focus::UseHasTabbableChild.materialize()>"use_has_tabbable_child"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
