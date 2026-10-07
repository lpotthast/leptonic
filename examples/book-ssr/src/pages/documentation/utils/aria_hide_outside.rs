use indoc::indoc;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageAriaHideOutside() -> impl IntoView {
    view! {
        <DocPage title="aria_hide_outside">
            <p>
                "The "<Code inline=true>"aria_hide_outside"</Code>" function hides everything on the page except the given "
                "elements from assistive technology, so that screen reader users stay inside an open modal overlay. See the "
                <Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link>
                " for the other overlay building blocks."
            </p>

            <ReactAriaSource path="overlays/ariaHideOutside.ts"/>

            <Section title="Usage">
                <p>
                    <Code inline=true>"aria_hide_outside(targets: &[web_sys::Element], options: AriaHideOutsideOptions)"</Code>
                    " walks the page, hides every element that doesn\u{2019}t contain one of the targets and returns a function that "
                    "shows them again. Call that function exactly once, when the overlay closes. It is a browser function: on the "
                    "server it does nothing."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::{hooks::*, utils::{AriaHideOutsideOptions, aria_hide_outside}};

                        let (is_open, set_is_open) = signal(false);
                        let UseOverlayReturn { props, overlay_element, .. } = use_overlay(UseOverlayInput {
                            is_open: is_open.into(),
                            on_close: Callback::new(move |()| set_is_open.set(false)),
                            is_dismissable: Signal::stored(false),
                            should_close_on_blur: Signal::stored(false),
                            is_keyboard_dismiss_disabled: Signal::stored(false),
                            should_close_on_interact_outside: None,
                            group: None,
                        });


                        // While open, hide the rest of the page; show it again when the overlay closes.
                        let undo = StoredValue::new_local(None::<Box<dyn FnOnce()>>);
                        let restore = move || undo.update_value(|undo| if let Some(undo) = undo.take() { undo() });
                        Effect::new(move |_| {
                            restore();
                            if let Some(overlay) = is_open.get().then(|| overlay_element.get()).flatten() {
                                let overlay: web_sys::Element = (*overlay).clone();
                                undo.set_value(Some(aria_hide_outside(&[overlay], AriaHideOutsideOptions::default())));
                            }
                        });
                        on_cleanup(restore);
                    ")}
                </Code>
            </Section>

            <Section title="AriaHideOutsideOptions">
                <ApiTable kind=ApiKind::Fields of="AriaHideOutsideOptions">
                    <ApiRow name="root" ty="Option<web_sys::Element>" default="None">
                        "The element to start hiding from. "<Code inline=true>"None"</Code>": the document body."
                    </ApiRow>
                    <ApiRow name="mode" ty="HideMode" default="AriaHidden">
                        "How elements are hidden: "<Code inline=true>"AriaHidden"</Code>" sets "<Code inline=true>"aria-hidden=\"true\""</Code>
                        " (the page stays usable with a pointer, as behind a combo box\u{2019}s popover), "<Code inline=true>"Inert"</Code>
                        " sets "<Code inline=true>"inert"</Code>" (also not interactive, for modal overlays). Import it as "
                        <Code inline=true>"leptonic::utils::HideMode"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Behavior">
                <ul>
                    <li>
                        "A "<Code inline=true>"MutationObserver"</Code>" hides elements added to the page while the overlay is open."
                    </li>
                    <li>
                        "Calls nest: an overlay opened from another one (a popover in a modal) takes over, and the outer "
                        "overlay\u{2019}s hiding resumes when it closes. An element hidden by several calls stays hidden until the last "
                        "of them is undone."
                    </li>
                    <li>
                        "Live regions of the "<Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live announcer"</Link>
                        " (marked "<Code inline=true>"data-live-announcer"</Code>") and elements marked "
                        <Code inline=true>"data-leptonic-top-layer"</Code>" stay visible, so announcements still reach screen readers."
                    </li>
                    <li>"Elements the page hid itself are left alone and stay hidden afterwards."</li>
                </ul>
            </Section>

            <Section title="keep_visible">
                <p>
                    <Code inline=true>"keep_visible(element: &web_sys::Element)"</Code>" keeps an element visible within the "
                    "current hiding, for content that opens outside the hiding overlay but belongs to it (a non-modal popover "
                    "opened from a modal, a modal while it animates in). It returns a function that undoes it, or "
                    <Code inline=true>"None"</Code>" when nothing is hidden or the element is already visible."
                </p>
            </Section>

            <Section title="Used By">
                <p>
                    <Link href=format!("{}#use-modal-backdrop", routes::doc::modal::Hook.materialize())>"use_modal_backdrop"</Link>
                    " and modal popovers ("<Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link>
                    ") make the rest of the page inert; non-modal popovers call "<Code inline=true>"keep_visible"</Code>
                    ". The combo box hides the page with "<Code inline=true>"aria-hidden"</Code>" while its list is open, and "
                    "keyboard drag and drop hides everything but the drop targets."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link></li>
                <li><Link href=routes::doc::Modal.materialize()>"Modal overview"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
