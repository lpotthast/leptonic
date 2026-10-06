use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::focusability::FocusabilityDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageFocusability() -> impl IntoView {
    view! {
        <DocPage title="focusability">
            <p>
                "The functions of "<Code inline=true>"leptonic::utils::focusability"</Code>" tell whether an element can take "
                "focus and whether "<Keys keys="Tab"/>" reaches it. Focus scopes, focus managers and "
                <Link href=routes::doc::focus::UseHasTabbableChild.materialize()>"use_has_tabbable_child"</Link>
                " decide with them; use them when you move focus yourself. See the "
                <Link href=routes::doc::Focus.materialize()>"Focus overview"</Link>" for the other focus building blocks."
            </p>

            <ReactAriaSource path="utils/isFocusable.ts"/>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::utils::focusability::{is_tabbable, TABBABLE_SELECTOR};

                        // The first element of a container that Tab would reach.
                        let first_tab_stop = container
                            .query_selector_all(TABBABLE_SELECTOR)
                            .ok()
                            .and_then(|nodes| {
                                (0..nodes.length())
                                    .filter_map(|index| nodes.get(index))
                                    .filter_map(|node| node.dyn_into::<web_sys::Element>().ok())
                                    .find(is_tabbable)
                            });
                    ")}
                </Code>
            </Section>

            <Section title="Demo">
                <p>"The results of both checks for a few elements:"</p>

                <Demo description="is_focusable and is_tabbable for a button, a disabled button, a button with tabindex -1, a link without href and an inert button" source=include_str!("demos/focusability.rs")>
                    <FocusabilityDemo/>
                </Demo>
            </Section>

            <Section title="is_focusable">
                <p>
                    <Code inline=true>"is_focusable(&Element) -> bool"</Code>" is true if the element matches "
                    <AnchorLink href="#focusable-selector">"FOCUSABLE_SELECTOR"</AnchorLink>", is visible ("
                    <Code inline=true>"is_element_visible"</Code>": no "<Code inline=true>"display: none"</Code>", "
                    <Code inline=true>"visibility: hidden"</Code>" or "<Code inline=true>"hidden"</Code>
                    " attribute on it or an ancestor, and not inside a closed "<Code inline=true>"<details>"</Code>
                    ") and is not inside an "<Code inline=true>"inert"</Code>" subtree ("<Code inline=true>"is_inert"</Code>")."
                </p>
            </Section>

            <Section title="is_tabbable">
                <p>
                    <Code inline=true>"is_tabbable(&Element) -> bool"</Code>" is true if the element matches "
                    <AnchorLink href="#tabbable-selector">"TABBABLE_SELECTOR"</AnchorLink>
                    ", is visible and not inert: it is focusable and has no negative "<Code inline=true>"tabindex"</Code>"."
                </p>
            </Section>

            <Section title="FOCUSABLE_SELECTOR">
                <p>
                    "A CSS selector for the elements that can take focus: enabled form controls, links and areas with "
                    <Code inline=true>"href"</Code>", "<Code inline=true>"summary"</Code>", embedded content ("
                    <Code inline=true>"iframe"</Code>", "<Code inline=true>"object"</Code>", "<Code inline=true>"embed"</Code>
                    "), media with controls, editable content and anything with a "<Code inline=true>"tabindex"</Code>
                    ", each unless "<Code inline=true>"hidden"</Code>". Selectors can\u{2019}t check visibility or "
                    <Code inline=true>"inert"</Code>": combine it with "<Code inline=true>"is_focusable"</Code>"."
                </p>
            </Section>

            <Section title="TABBABLE_SELECTOR">
                <p>
                    "The same elements as "<Code inline=true>"FOCUSABLE_SELECTOR"</Code>", without those with "
                    <Code inline=true>"tabindex=\"-1\""</Code>"."
                </p>
            </Section>

            <Section title="will_open_keyboard">
                <p>
                    <Code inline=true>"leptonic::utils::will_open_keyboard(&Element) -> bool"</Code>" tells whether focusing "
                    "the element opens the on-screen keyboard of a touch device: text-like inputs, "
                    <Code inline=true>"textarea"</Code>" and editable content do; checkboxes, radios, ranges, color and file "
                    "inputs and buttons don\u{2019}t. Use it to avoid moving focus to a text field after a touch, which would "
                    "cover half the screen with the keyboard."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Focus.materialize()>"Focus overview"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusManager.materialize()>"use_focus_manager"</Link></li>
                <li><Link href=routes::doc::focus::UseHasTabbableChild.materialize()>"use_has_tabbable_child"</Link></li>
                <li><Link href=format!("{}#focus-safely", routes::doc::focus::UseFocusable.materialize())>"focus_safely"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
