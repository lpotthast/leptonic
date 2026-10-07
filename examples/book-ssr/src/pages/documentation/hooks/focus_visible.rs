use indoc::indoc;
use leptos::prelude::*;

use super::demos::focus_visible::FocusVisibleDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseFocusVisible() -> impl IntoView {
    view! {
        <DocPage title="use_focus_visible">
            <p>
                "The "<Code inline=true>"use_focus_visible"</Code>" hook tracks whether focus should currently be made visible, "
                "for example with a focus ring. Focus is visible while the user navigates with the keyboard or assistive "
                "technology, and hidden after pointer interaction. The state is global, not tied to an element; to know whether "
                "a particular element should show a ring, use "
                <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>". "
                "See the "<Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>
                " to compare it with the other focus building blocks."
            </p>

            <ReactAria hook="useFocusVisible"/>

            <Section title="Input">
                <p><Code inline=true>"UseFocusVisibleInput"</Code>" implements "<Code inline=true>"Default"</Code>"."</p>

                <ApiTable kind=ApiKind::Input of="UseFocusVisibleInput">
                    <ApiRow name="auto_focus" ty="bool" default="false">
                        "Makes "<Code inline=true>"focus_should_be_visible"</Code>" start out "<Code inline=true>"true"</Code>
                        ", on the server and in the browser, whatever the modality. Set it when the element is focused on mount, "
                        "so the ring shows right away. It does not focus anything."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Stops following modality changes. While true, the hook unsubscribes and its signals keep their "
                        "last value. When it turns false again, the signals catch up with the current modality."
                    </ApiRow>
                    <ApiRow name="is_text_input" ty="bool" default="false">
                        "Use text input rules: only "<Keys keys="Tab"/>" and "<Keys keys="Escape"/>" make focus visible, see "<AnchorLink href="#text-input-rules">"Text Input Rules"</AnchorLink>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseFocusVisibleReturn">
                    <ApiRow name="focus_should_be_visible" ty="Signal<bool>">
                        "Whether focus should be visible: true for every modality except "<Code inline=true>"Pointer"</Code>"."
                    </ApiRow>
                    <ApiRow name="modality" ty="Signal<Modality>">"The current interaction modality."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::*;

                        let UseFocusVisibleReturn { focus_should_be_visible, .. } =
                            use_focus_visible(UseFocusVisibleInput::default());

                        view! {
                            <button data-focus-visible=move || focus_should_be_visible.get().then_some("true")>
                                "Button"
                            </button>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Click the button, then tab away and back. Its outline only shows while the modality is not "
                    "\u{201c}Pointer\u{201d}. While \u{201c}Disabled\u{201d} is checked, the readout keeps its last value."
                </p>

                <Demo description="Button whose focus outline follows the current interaction modality, with a disabled toggle" source=include_str!("demos/focus_visible.rs")>
                    <FocusVisibleDemo/>
                </Demo>
            </Section>

            <Section title="Modality">
                <p>"The hook listens to keyboard, pointer and focus events on the window and derives the modality from them:"</p>

                <DocTable headers=&["Modality", "Set when", "Focus visible"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Keyboard"</Code></TableCell>
                        <TableCell>
                            "A key is pressed. Modifier keys alone and shortcuts with "<Keys keys="Control"/>", "<Keys keys="Meta"/>
                            " (or "<Keys keys="Alt"/>" outside macOS) don\u{2019}t count."
                        </TableCell>
                        <TableCell>"yes"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Pointer"</Code></TableCell>
                        <TableCell>"The mouse, a pen or a finger presses down."</TableCell>
                        <TableCell>"no"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Virtual"</Code></TableCell>
                        <TableCell>"A screen reader or other assistive technology clicks or moves focus."</TableCell>
                        <TableCell>"yes"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Unknown"</Code></TableCell>
                        <TableCell>"No interaction happened yet."</TableCell>
                        <TableCell>"yes"</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Text Input Rules">
                <p>
                    "Typing into a text field should not make focus visible. While a text input has focus, only "<Keys keys="Tab"/>" and "<Keys keys="Escape"/>" "
                    "switch to keyboard modality for subscribers. With "<Code inline=true>"is_text_input: true"</Code>
                    ", the hook applies this rule regardless of the focused element. Use it for elements that aren\u{2019}t text "
                    "inputs themselves but should behave like one, e.g. the segments of a date field."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        let focus_visible = use_focus_visible(UseFocusVisibleInput {
                            is_text_input: true,
                            ..Default::default()
                        });
                    ")}
                </Code>
            </Section>

            <Section title="Disabling">
                <p>"Set "<Code inline=true>"is_disabled"</Code>" while the element using the hook is hidden or inactive, to avoid needless updates:"</p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        let focus_visible = use_focus_visible(UseFocusVisibleInput {
                            is_disabled: Signal::derive(move || !is_panel_visible.get()),
                            ..Default::default()
                        });
                    ")}
                </Code>
            </Section>

            <Section title="use_interaction_modality">
                <p>
                    <Code inline=true>"use_interaction_modality()"</Code>" returns the current modality as a "
                    <Code inline=true>"Signal<Option<Modality>>"</Code>", updated as it changes. It is "
                    <Code inline=true>"None"</Code>" during server-side rendering. Use it when you need the modality itself, "
                    "e.g. to open a menu with its first item focused only after a keyboard interaction."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::hooks::*;

                        let modality = use_interaction_modality();
                        let by_keyboard = move || modality.get() == Some(Modality::Keyboard);
                    ")}
                </Code>
            </Section>

            <Section title="get_modality">
                <p>
                    <Code inline=true>"get_modality() -> Modality"</Code>" reads the current modality once, without tracking, e.g. "
                    "in an event handler. It is "<Code inline=true>"Unknown"</Code>" during SSR and before any hook started "
                    "tracking the modality."
                </p>
            </Section>

            <Section title="set_modality">
                <p>
                    <Code inline=true>"set_modality(Modality)"</Code>" sets the modality and notifies all subscribers. The next "
                    "user interaction overrides it again. It does nothing during SSR."
                </p>
            </Section>

            <Section title="add_window_focus_tracking">
                <p>
                    <Code inline=true>"add_window_focus_tracking(Option<&HtmlElement>) -> Box<dyn FnOnce()>"</Code>" tracks the "
                    "modality in another window, e.g. an iframe, identified by an element in it, and returns a function that "
                    "stops tracking it. "<Code inline=true>"tear_down_window_focus_tracking(Option<&HtmlElement>)"</Code>
                    " stops tracking the window containing the element as well. The main window is tracked automatically."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
                <li><Link href=routes::doc::focus::FocusRing.materialize()>"FocusRing"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
