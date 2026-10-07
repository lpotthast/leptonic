use indoc::indoc;
use leptos::prelude::*;

use super::demos::button::ButtonDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomButton() -> impl IntoView {
    view! {
        <DocPage title="Button Atom">
            <p>
                "The "<Code inline=true>"Button"</Code>" atom renders an unstyled native "<Code inline=true>"<button>"</Code>
                " with the behavior of "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>". See the "
                <Link href=routes::doc::Button.materialize()>"Button overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::button::Hook.materialize()><Code inline=true>"use_button"</Code></Link>
                    ", which composes "<Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>", "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>", "
                    <Link href=routes::doc::focus::UseFocusable.materialize()>"use_focusable"</Link>", "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>" and "
                    <Link href=routes::doc::interactions::UseContextMenu.materialize()>"use_context_menu"</Link>"."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="atoms::button::Button">
                    <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">
                        "Called when the button is pressed."
                    </ApiRow>
                    <ApiRow name="on_hover_start, on_hover_end" ty="Option<Callback<HoverStartEvent>>, Option<Callback<HoverEndEvent>>" default="None">
                        "Called when a mouse or pen starts or stops hovering the button."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the button is disabled."</ApiRow>
                    <ApiRow name="button_type" ty="ButtonType" default="Button">
                        "The "<Code inline=true>"type"</Code>" attribute. The default doesn\u{2019}t submit forms, so a button "
                        "inside a form submits it only with "<Code inline=true>"ButtonType::Submit"</Code>"."
                    </ApiRow>
                    <ApiRow name="exclude_from_tab_order" ty="Signal<bool>" default="false">
                        "Skips the button when tabbing."
                    </ApiRow>
                    <ApiRow name="aria_haspopup" ty="Signal<Option<AriaHasPopup>>" default="None">
                        "The kind of popup the button opens."
                    </ApiRow>
                    <ApiRow name="aria_expanded" ty="Signal<Option<AriaExpanded>>" default="None">
                        "Whether the element the button controls is expanded."
                    </ApiRow>
                    <ApiRow name="aria_pressed" ty="Signal<Option<AriaPressed>>" default="None">
                        "The pressed state of a toggle button. For a ready-made one, see the "
                        <Link href=routes::doc::toggle_button::Atom.materialize()>"Toggle Button Atoms"</Link>"."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "Names the button when its content doesn\u{2019}t, as for an icon-only button."
                    </ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">
                        "The ids of the elements naming the button."
                    </ApiRow>
                    <ApiRow name="aria_describedby" ty="Signal<Option<String>>" default="None">
                        "The ids of the elements describing the button."
                    </ApiRow>
                    <ApiRow name="aria_controls" ty="Signal<Option<String>>" default="None">
                        "The ids of the elements the button controls."
                    </ApiRow>
                    <ApiRow name="aria_current" ty="Signal<Option<AriaCurrent>>" default="None">
                        "Marks the button as the current item of a set, such as the current page of a pagination."
                    </ApiRow>
                    <ApiRow name="form" ty="ButtonFormAttributes" default="ButtonFormAttributes::default()">
                        "For submit and reset buttons: "<Code inline=true>"form"</Code>" (the id of a form elsewhere), "
                        <Code inline=true>"form_action"</Code>", "<Code inline=true>"form_method"</Code>" ("<Code inline=true>"FormMethod"</Code>"), "
                        <Code inline=true>"form_enc_type"</Code>" ("<Code inline=true>"FormEncType"</Code>"), "<Code inline=true>"form_no_validate"</Code>", "
                        <Code inline=true>"form_target"</Code>" ("<Code inline=true>"LinkTarget"</Code>"), "<Code inline=true>"name"</Code>" and "
                        <Code inline=true>"value"</Code>"."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Classes and styles of the "<Code inline=true>"<button>"</Code>"."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The button content. Required."</ApiRow>
                </ApiTable>

                <Section title="ARIA Props">
                    <p>
                        "The "<Code inline=true>"aria_*"</Code>" props describe the button to assistive technology. Most "
                        "buttons need none: their text is their name. Use them when the content doesn\u{2019}t name the "
                        "button (an icon), when the button opens or controls another element, or when it marks the "
                        "current item of a set. The reactive ones take signals, so they follow your state:"
                    </p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{
                                atoms::prelude as atoms,
                                prelude::icondata,
                                utils::aria::{AriaCurrent, AriaExpanded},
                            };
                            use leptos::prelude::*;
                            use leptos_icons::Icon;

                            let is_open = RwSignal::new(false);
                            let is_closed = RwSignal::new(false);

                            view! {
                                // An icon-only button is named by `aria_label`.
                                <atoms::Button aria_label="Close" on_press=move |_| is_closed.set(true)>
                                    <Icon icon=icondata::BsXLg/>
                                </atoms::Button>

                                // A disclosure button: what it controls, and whether that is expanded.
                                <atoms::Button
                                    aria_controls=Some("details".to_owned())
                                    aria_expanded=Signal::derive(move || Some(AriaExpanded::from(is_open.get())))
                                    on_press=move |_| is_open.update(|open| *open = !*open)
                                >
                                    "Details"
                                </atoms::Button>
                                <div id="details" hidden=move || !is_open.get()>"The details."</div>

                                // The current page of a pagination.
                                <atoms::Button aria_current=Some(AriaCurrent::Page)>"3"</atoms::Button>
                            }
                        "#)}
                    </Code>
                    <p>
                        <Code inline=true>"aria_labelledby"</Code>" and "<Code inline=true>"aria_describedby"</Code>
                        " point to other elements that name or describe the button, by their ids. For a disclosure, the "
                        <Link href=routes::doc::disclosure::Atom.materialize()>"Disclosure Atoms"</Link>
                        " set these attributes for you."
                    </p>
                </Section>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude as atoms;
                        use leptos::{logging::log, prelude::*};

                        view! {
                            <atoms::Button classes="my-button" on_press=move |_| log!("pressed")>
                                "Press me"
                            </atoms::Button>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Unstyled button atom counting presses, with a disabled toggle" source=include_str!("demos/button.rs")>
                    <ButtonDemo/>
                </Demo>
            </Section>

            <Section title="Data Attributes">
                <p>"Set to "<Code inline=true>"true"</Code>" on the "<Code inline=true>"<button>"</Code>" while the state applies, absent otherwise:"</p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-pressed" ty="true">"The button is being pressed right now, by pointer or keyboard."</ApiRow>
                    <ApiRow name="data-hovered" ty="true">"A mouse or pen is over the button (not set for touch, nor while disabled)."</ApiRow>
                    <ApiRow name="data-focused" ty="true">"The button has focus, however it got it."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">
                        "The button has keyboard focus and should show a focus ring."
                    </ApiRow>
                    <ApiRow name="data-disabled" ty="true">
                        "The button is disabled. The native "<Code inline=true>"disabled"</Code>" attribute is set as well."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atom brings no styles. Its "<Code inline=true>"<button>"</Code>" has the default class "
                    <Code inline=true>"leptonic-Button"</Code>"; pass "<Code inline=true>"classes"</Code>" to add your own, "
                    "and render the content (text, an icon) as its children. Target the state with the data attributes. "
                    "Unlike "<Code inline=true>":hover"</Code>" and "<Code inline=true>":active"</Code>", they ignore the "
                    "emulated mouse events after a touch, and "<Code inline=true>"data-pressed"</Code>" also follows "
                    <Keys keys="Enter"/>" and "<Keys keys="Space"/>". The demo above uses this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .demo-atom-button {
                            padding: 0.5rem 1rem;
                            border: 1px solid var(--border);
                            border-radius: 6px;
                            background: var(--surface);
                            font: inherit;
                            cursor: pointer;
                        }
                        .demo-atom-button[data-hovered] { border-color: var(--accent); }
                        .demo-atom-button[data-pressed] { background: var(--border); }
                        .demo-atom-button[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                        .demo-atom-button[data-disabled] { opacity: 0.5; cursor: not-allowed; }
                    ")}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "Put the atom inside a trigger, and the trigger hands it its behavior through a "
                    <Link href=routes::doc::interactions::PressResponder.materialize()><Code inline=true>"PressResponder"</Code></Link>
                    ": inside a "<Code inline=true>"DialogTrigger"</Code>" ("<Link href=routes::doc::dialog::Atom.materialize()>"Dialog Atoms"</Link>
                    "), "<Code inline=true>"MenuTrigger"</Code>" ("<Link href=routes::doc::menu::Atom.materialize()>"Menu Atoms"</Link>
                    "), "<Code inline=true>"TooltipTrigger"</Code>" ("<Link href=routes::doc::tooltip::Atom.materialize()>"Tooltip Atoms"</Link>
                    ") or "<Code inline=true>"DisclosureTrigger"</Code>" ("<Link href=routes::doc::disclosure::Atom.materialize()>"Disclosure Atoms"</Link>
                    "), the button opens the overlay or panel and gets the matching ARIA attributes, without "
                    <Code inline=true>"on_press"</Code>"."
                </p>
                <p>
                    "An element that looks like a button but navigates is a link: use the "
                    <Code inline=true>"Link"</Code>" atom ("<Link href=routes::doc::link::Atom.materialize()>"Link Atoms"</Link>
                    ") and style it like your buttons."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Button.materialize()>"Button overview"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></li>
                <li><Link href=routes::doc::toggle_button::Atom.materialize()>"Toggle Button Atoms"</Link></li>
                <li><Link href=routes::doc::link::Atom.materialize()>"Link Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
