use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    focus_ring_keyboard::FocusRingKeyboardDemo, focus_ring_within::FocusRingWithinDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseFocusRing() -> impl IntoView {
    view! {
        <DocPage title="use_focus_ring">
            <p>
                "The "<Code inline=true>"use_focus_ring"</Code>" hook decides when an element should show a focus ring: while it "
                "is focused and the user navigates with the keyboard, but not after a mouse click or a tap. "
                "See the "<Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>
                " to compare it with the other focus building blocks."
            </p>

            <ReactAria hook="useFocusRing"/>

            <Section title="Input">
                <p><Code inline=true>"UseFocusRingInput"</Code>" implements "<Code inline=true>"Default"</Code>"."</p>

                <ApiTable kind=ApiKind::Input of="UseFocusRingInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Ignores focus events while true. Disabling the hook while the element is focused resets "
                        <Code inline=true>"is_focused"</Code>" and "<Code inline=true>"is_focus_visible"</Code>" to "
                        <Code inline=true>"false"</Code>"."
                    </ApiRow>
                    <ApiRow name="within" ty="bool" default="false">
                        "Track focus anywhere inside the element instead of on the element itself, see "
                        <AnchorLink href="#within-mode">"Within Mode"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="auto_focus" ty="bool" default="false">
                        "Set this when the element is focused on mount: the ring then starts out visible. It does not focus the "
                        "element itself."
                    </ApiRow>
                    <ApiRow name="is_text_input" ty="bool" default="false">
                        "Use text input rules: only "<Keys keys="Tab"/>" and "<Keys keys="Escape"/>" make focus visible, see "<AnchorLink href="#text-input-mode">"Text Input Mode"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="on_focus" ty="Option<Callback<FocusEvent>>" default="None">
                        "Called when the element (or, in within mode, the subtree) receives focus."
                    </ApiRow>
                    <ApiRow name="on_blur" ty="Option<Callback<FocusEvent>>" default="None">
                        "Called when the element (or, in within mode, the subtree) loses focus."
                    </ApiRow>
                    <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">
                        "Called with the new focus state."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseFocusRingReturn">
                    <ApiRow name="props" ty="UseFocusRingProps">
                        "The focus listeners and the "<Code inline=true>"data-focus-visible"</Code>
                        " attribute. Spread them onto the element with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                    <ApiRow name="is_focused" ty="Signal<bool>">
                        "Whether the element is focused (in within mode: whether focus is inside it)."
                    </ApiRow>
                    <ApiRow name="is_focus_visible" ty="Signal<bool>">
                        "Whether the focus ring should be shown: the element is focused and the last interaction was not a pointer."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::*;

                        let focus_ring = use_focus_ring(UseFocusRingInput::default());

                        view! {
                            <button class="my-button" {..focus_ring.props.into_attrs()}>"Focus me"</button>
                        }
                    "#)}
                </Code>

                <p>
                    "The hook sets "<Code inline=true>"data-focus-visible=\"true\""</Code>" while the ring should show, so the "
                    "styling lives in your CSS:"
                </p>

                <Code language=Language::Css>
                    {indoc!(r"
                        .my-button:focus { outline: none; }
                        .my-button[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                    ")}
                </Code>

                <p>
                    "You don\u{2019}t compute styles in Rust, and all focus rings of your application are styled in one "
                    "place. "<Code inline=true>"is_focus_visible"</Code>" is still there when you need the state in code."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Click the button: it is focused, but shows no ring. Tab away and back: now the ring shows. Check "
                    "\u{201c}Disabled\u{201d} while the button is focused, and both states reset."
                </p>

                <Demo
                    description="Button that shows a focus ring only for keyboard focus, with its focus state and a disabled toggle"
                    source=include_str!("demos/focus_ring_keyboard.rs")
                >
                    <FocusRingKeyboardDemo/>
                </Demo>
            </Section>

            <Section title="Within Mode">
                <p>
                    "With "<Code inline=true>"within: true"</Code>", the hook listens to "<Code inline=true>"focusin"</Code>" and "
                    <Code inline=true>"focusout"</Code>" (via "
                    <Link href=routes::doc::focus::UseFocusWithin.materialize()>"use_focus_within"</Link>
                    ") and sets "<Code inline=true>"data-focus-visible"</Code>" on the container while any descendant has keyboard "
                    "focus. Use it for groups of controls, such as a search field with a button."
                </p>

                <Demo
                    description="Group showing one focus ring while its field or button has keyboard focus"
                    source=include_str!("demos/focus_ring_within.rs")
                >
                    <FocusRingWithinDemo/>
                </Demo>
            </Section>

            <Section title="Text Input Mode">
                <p>
                    "Typing in a text field is keyboard interaction, but it should not make focus visible. With "
                    <Code inline=true>"is_text_input: true"</Code>", only "<Keys keys="Tab"/>" and "<Keys keys="Escape"/>" do. Use it for elements that aren\u{2019}t text inputs themselves but should "
                    "follow text input rules, e.g. the segments of a date field."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        let focus_ring = use_focus_ring(UseFocusRingInput {
                            is_text_input: true,
                            ..Default::default()
                        });
                    ")}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::FocusRing.materialize()>"FocusRing"</Link>" (the atom applying this hook to its child)"</li>
                <li><Link href=routes::doc::focus::UseFocusVisible.materialize()>"use_focus_visible"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
