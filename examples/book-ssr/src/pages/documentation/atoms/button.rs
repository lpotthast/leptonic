use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::button::ButtonDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomButton() -> impl IntoView {
    view! {
        <DocPage title="Button atom">
            <p>
                "The "<Code inline=true>"Button"</Code>" atom renders a native "<Code inline=true>"<button>"</Code>
                " with the behavior of "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                " and no styling. See the "<Link href=routes::doc::Button.materialize()>"Button overview"</Link>
                " for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::button::Hook.materialize()><Code inline=true>"use_button"</Code></Link>
                    ", which composes "<Link href=routes::doc::interactions::UsePress.materialize()>"use_press"</Link>", "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>" and "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>"."
                </p>
            </Section>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="atoms::button::Button">
                    <ApiRow name="on_press" ty="Option<Callback<PressEvent>>" default="None">
                        "Called when the button is pressed."
                    </ApiRow>
                    <ApiRow name="on_hover_start, on_hover_end" ty="Option<Callback<..>>" default="None">
                        "Called when a mouse or pen starts or stops hovering the button."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the button is disabled."</ApiRow>
                    <ApiRow name="button_type" ty="ButtonType" default="Button">
                        "The "<Code inline=true>"type"</Code>" attribute. Defaults to "<Code inline=true>"button"</Code>
                        ", so buttons inside forms don\u{2019}t submit them unless asked to."
                    </ApiRow>
                    <ApiRow name="exclude_from_tab_order" ty="Signal<bool>" default="false">
                        "Skip the button when tabbing."
                    </ApiRow>
                    <ApiRow name="aria_haspopup" ty="Signal<Option<AriaHasPopup>>" default="None">
                        "The kind of popup the button opens."
                    </ApiRow>
                    <ApiRow name="aria_expanded" ty="Signal<Option<AriaExpanded>>" default="None">
                        "Whether the element the button controls is expanded."
                    </ApiRow>
                    <ApiRow name="aria_pressed" ty="Signal<Option<AriaPressed>>" default="None">
                        "The pressed state of a toggle button."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "Labels the button when its content doesn't (icon-only buttons)."
                    </ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">
                        "The ids of the elements labelling the button."
                    </ApiRow>
                    <ApiRow name="aria_describedby, aria_controls" ty="Signal<Option<String>>" default="None">
                        "The ids of the elements describing the button / controlled by it."
                    </ApiRow>
                    <ApiRow name="aria_current" ty="Signal<Option<AriaCurrent>>" default="None">
                        "Marks the button as the current item of a set, e.g. the current page of a pagination."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Classes and styles of the "<Code inline=true>"<button>"</Code>"."
                    </ApiRow>
                    <ApiRow name="children" ty="Children">"The button content."</ApiRow>
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
                            use leptonic::{atoms::prelude as atoms, utils::aria::{AriaCurrent, AriaExpanded}};

                            let (is_open, set_is_open) = signal(false);

                            view! {
                                // An icon-only button is named by `aria_label`.
                                <atoms::Button aria_label="Close" on_press=move |_| close()>
                                    <Icon icon=icondata::BsXLg/>
                                </atoms::Button>

                                // A disclosure button: what it controls, and whether that is expanded.
                                <atoms::Button
                                    aria_controls=Some("details".to_owned())
                                    aria_expanded=Signal::derive(move || Some(AriaExpanded::from(is_open.get())))
                                    on_press=move |_| set_is_open.update(|open| *open = !*open)
                                >
                                    "Details"
                                </atoms::Button>
                                <div id="details" hidden=move || !is_open.get()>"\u{2026}"</div>

                                // The current page of a pagination.
                                <atoms::Button aria_current=Some(AriaCurrent::Page)>"3"</atoms::Button>
                            }
                        "#)}
                    </Code>
                    <p>
                        <Code inline=true>"aria_labelledby"</Code>" and "<Code inline=true>"aria_describedby"</Code>
                        " point to other elements that name or describe the button, by their ids. A toggle button sets "
                        <Code inline=true>"aria_pressed"</Code>"; for a ready-made one, see the "
                        <Link href=routes::doc::toggle_button::Atom.materialize()>"ToggleButton atom"</Link>"."
                    </p>
                </Section>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude as atoms;

                        view! {
                            <atoms::Button classes="my-button" on_press=move |_| log!("pressed")>
                                "Press me"
                            </atoms::Button>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Unstyled button atom counting presses" source=include_str!("demos/button.rs")>
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
                    "The atom brings no styles. Pass "<Code inline=true>"classes"</Code>" and target its state with the data "
                    "attributes. Unlike "<Code inline=true>":hover"</Code>" and "<Code inline=true>":active"</Code>", they "
                    "ignore emulated mouse events after a touch, and "<Code inline=true>"data-pressed"</Code>" also follows "
                    <Keys keys="Enter"/>" and "<Keys keys="Space"/>":"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-button[data-hovered] { background: #e8eefc; }
                        .my-button[data-pressed] { background: #c9d6f8; }
                        .my-button[data-focus-visible] { outline: 2px solid royalblue; }
                        .my-button[data-disabled] { opacity: 0.5; }
                    ")}
                </Code>
            </Section>

            <Section title="LinkButton">
                <p>
                    <Code inline=true>"LinkButton"</Code>" renders a router link ("<Code inline=true>"<a>"</Code>
                    ") with button behavior. Use it for elements that look like buttons but navigate. It takes an "
                    <Code inline=true>"href"</Code>", an optional "<Code inline=true>"target"</Code>", "
                    <Code inline=true>"exact"</Code>" (whether the link is only marked active on an exact location match) and the "
                    "hover, disabled and popup props of "<Code inline=true>"Button"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Button.materialize()>"Button overview"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></li>
                <li><Link href=routes::doc::button::Component.materialize()>"Button component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
