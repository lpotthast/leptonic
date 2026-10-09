use indoc::indoc;
use leptos::prelude::*;

use super::demos::focusable::FocusableDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomFocusable() -> impl IntoView {
    view! {
        <DocPage title="Focusable">
            <p>
                "The "<Code inline=true>"Focusable"</Code>" atom makes its child element focusable: it adds a "
                <Code inline=true>"tabindex"</Code>" and focus and keyboard handlers to the child and renders no element of "
                "its own. Use it for elements that should be reachable by keyboard without being buttons, most often an "
                "icon that shows a tooltip. See the "<Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>
                " for the other focus building blocks."
            </p>

            <ReactAriaSource path="interactions/useFocusable.tsx"/>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::focus::UseFocusable.materialize()><Code inline=true>"use_focusable"</Code></Link>
                    ". The atom puts the hook\u{2019}s attributes and handlers onto its child. A "<Code inline=true>"tabindex"</Code>
                    " the child sets itself wins. A surrounding trigger applies its attributes to the child as well, through "
                    "the hook\u{2019}s "<Link href=format!("{}#focusablecontext", routes::doc::focus::UseFocusable.materialize())>"FocusableContext"</Link>
                    ": a "<Link href=routes::doc::tooltip::Atom.materialize()><Code inline=true>"TooltipTrigger"</Code></Link>
                    " makes it its trigger, with "<Code inline=true>"aria-describedby"</Code>" pointing at the tooltip."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Focusable">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Whether the element can\u{2019}t be focused: the atom removes the "<Code inline=true>"tabindex"</Code>
                        " it manages, and a surrounding trigger\u{2019}s attributes don\u{2019}t apply."
                    </ApiRow>
                    <ApiRow name="exclude_from_tab_order" ty="Signal<bool>" default="false">
                        "Whether the element is skipped when tabbing; it stays focusable by click or script."
                    </ApiRow>
                    <ApiRow name="auto_focus" ty="bool" default="false">
                        "Whether to focus the element when it mounts, once it has its "<Code inline=true>"tabindex"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">"Called when the element gains or loses focus."</ApiRow>
                    <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called with whether the element has focus."</ApiRow>
                    <ApiRow name="on_key_down, on_key_up" ty="Option<Callback<KeyboardEventWrapper>>" default="None">
                        "Called on key events while the element has focus."
                    </ApiRow>
                    <ApiRow name="children" ty="TypedChildren<V>">"The element to make focusable. Required."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::{focusable::Focusable, tooltip::{Tooltip, TooltipTrigger}},
                        };

                        view! {
                            <TooltipTrigger>
                                <Focusable>
                                    <span role="img" aria-label="Info">"ⓘ"</span>
                                </Focusable>
                                <Tooltip>"More details"</Tooltip>
                            </TooltipTrigger>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Tab to the info icon: it takes focus and shows its tooltip, as it does on hover. Disabled, "<Keys keys="Tab"/>" skips it."
                </p>

                <Demo description="An info icon that shows a tooltip on hover and keyboard focus, with a disabled toggle" source=include_str!("demos/focusable.rs")>
                    <FocusableDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>
                    "The atom renders no data attributes. Style keyboard focus with "<Code inline=true>":focus-visible"</Code>
                    ", or wrap the child in a "<Link href=routes::doc::focus::FocusRing.materialize()>"FocusRing"</Link>
                    " for "<Code inline=true>"data-focus-visible"</Code>":"
                </p>

                <Code language=Language::Css>
                    {indoc!(r"
                        .info-icon:focus-visible { outline: 2px solid var(--focus); outline-offset: 2px; }
                    ")}
                </Code>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A focusable element must tell assistive technology what it is. Give the child an interactive ARIA role "
                    "(e.g. "<Code inline=true>"button"</Code>" or "<Code inline=true>"link"</Code>"), make it an image ("
                    <Code inline=true>"role=\"img\""</Code>" with an "<Code inline=true>"aria-label"</Code>", an "
                    <Code inline=true>"<img>"</Code>" or an "<Code inline=true>"<svg>"</Code>"), or give it one of the roles "
                    <Code inline=true>"tabpanel"</Code>", "<Code inline=true>"meter"</Code>" or "
                    <Code inline=true>"progressbar"</Code>", so that its name and its tooltip are announced. In development builds, the atom warns about a child without such a role. For an "
                    "element that performs an action, use a "<Link href=routes::doc::Button.materialize()>"button"</Link>" instead."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link></li>
                <li><Link href=routes::doc::focus::FocusRing.materialize()>"FocusRing"</Link></li>
                <li><Link href=routes::doc::tooltip::Atom.materialize()>"Tooltip Atoms"</Link></li>
                <li><Link href=format!("{}#pressable", routes::doc::interactions::PressResponder.materialize())>"Pressable"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
