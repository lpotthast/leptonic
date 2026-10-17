use indoc::indoc;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageUseHideOutside() -> impl IntoView {
    view! {
        <DocPage title="use_hide_outside">
            <p>
                "The "<Code inline=true>"use_hide_outside"</Code>" hook makes the page outside an open overlay inert: hidden "
                "from assistive technology and not interactive. That is what makes an overlay modal. "
                <Link href=format!("{}#use-modal-backdrop", routes::doc::modal::Hook.materialize())>"use_modal_backdrop"</Link>
                " and "<Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link>" call it; call it yourself for "
                "an overlay you build from "<Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link>
                ". See the "<Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link>
                " for the other overlay building blocks."
            </p>

            <ReactAriaSource path="overlays/usePopover.ts"/>

            <Section title="Input">
                <ApiTable kind=ApiKind::Input of="UseHideOutsideInput">
                    <ApiRow name="is_open" ty="Signal<bool>">
                        "Whether the overlay is open. The page is restored when it closes, or when the calling component "
                        "unmounts. Required."
                    </ApiRow>
                    <ApiRow name="overlay" ty="CapturedElement">
                        "The overlay, captured by the hook rendering it ("<Code inline=true>"use_overlay"</Code>"\u{2019}s "
                        <Code inline=true>"overlay_element"</Code>"). Nothing happens until it is captured, so the overlay itself is "
                        "never made inert. Pass the element containing a group of overlays (a menu and its submenus) to keep them "
                        "all usable. Required."
                    </ApiRow>
                    <ApiRow name="behavior" ty="Signal<HideOutsideBehavior>">
                        <Code inline=true>"Inert"</Code>" makes the page inert (a modal overlay); "<Code inline=true>"KeepVisible"</Code>
                        " leaves the page alone and only keeps the overlay usable should another modal overlay hide the page (a "
                        "non-modal popover opened from a modal, or a modal while it animates in). Required."
                    </ApiRow>
                </ApiTable>
                <p>"The hook returns nothing."</p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::overlay::{
                            HideOutsideBehavior, UseHideOutsideInput, UseOverlayInput, UseOverlayReturn, use_hide_outside,
                            use_overlay,
                        };
                        use leptos::prelude::*;

                        // `state` from `use_overlay_trigger_state`.
                        let is_open = state.is_open;
                        let UseOverlayReturn { overlay, overlay_element, .. } = use_overlay(UseOverlayInput {
                            is_open,
                            on_close: Callback::new(move |()| state.close()),
                            is_dismissable: Signal::stored(true),
                            should_close_on_blur: Signal::stored(false),
                            is_keyboard_dismiss_disabled: Signal::stored(false),
                            should_close_on_interact_outside: None,
                            group: None,
                        });
                        use_hide_outside(UseHideOutsideInput {
                            is_open,
                            overlay: overlay_element,
                            behavior: Signal::stored(HideOutsideBehavior::Inert),
                        });

                        view! {
                            <Show when=move || is_open.get()>
                                <div {..overlay.attrs()}>"Only this is usable while open."</div>
                            </Show>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Behavior">
                <ul>
                    <li>
                        "Nested overlays stack: a modal opened from a modal hides the first one too, and each restores what it hid "
                        "when it closes, in any order."
                    </li>
                    <li>
                        "Elements added to the page while the overlay is open are made inert as well; live regions and elements "
                        "marked "<Code inline=true>"data-leptonic-top-layer"</Code>" stay usable."
                    </li>
                    <li>
                        "Switching "<Code inline=true>"behavior"</Code>" while open takes effect right away: a modal that "
                        "finished entering hides the page."
                    </li>
                    <li>
                        "It is built on "<Link href=routes::doc::overlay_behavior::AriaHideOutside.materialize()>"aria_hide_outside"</Link>
                        " and "<Code inline=true>"keep_visible"</Code>". During server-side rendering it does nothing."
                    </li>
                    <li>
                        "Inert content can\u{2019}t be focused or clicked: render the overlay\u{2019}s own controls, such as its close "
                        "button, inside the overlay."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::AriaHideOutside.materialize()>"aria_hide_outside"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UsePreventScroll.materialize()>"use_prevent_scroll"</Link></li>
                <li><Link href=routes::doc::Modal.materialize()>"Modal overview"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
