use indoc::indoc;
use leptos::prelude::*;

use super::demos::scroll_wheel::ScrollWheelDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseScrollWheel() -> impl IntoView {
    view! {
        <DocPage title="use_scroll_wheel">
            <p>
                "The "<Code inline=true>"use_scroll_wheel"</Code>" hook turns mouse wheel and trackpad scrolling over an element "
                "into scroll deltas, without scrolling the page. See the "
                <Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>" to compare it with the other interaction building blocks."
            </p>

            <ReactAriaSource path="interactions/useScrollWheel.ts"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseScrollWheelInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    " (enabled, no callback)."
                </p>

                <ApiTable kind=ApiKind::Input of="UseScrollWheelInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                        "Ignore wheel events while "<Code inline=true>"true"</Code>". The page scrolls normally then."
                    </ApiRow>
                    <ApiRow name="on_scroll" ty="Option<Callback<ScrollEvent>>" default="None">
                        "Called for every wheel event with the "<Code inline=true>"ScrollEvent"</Code>"\u{2019}s "
                        <Code inline=true>"delta_x"</Code>" and "<Code inline=true>"delta_y"</Code>" (positive "
                        <Code inline=true>"delta_y"</Code>" means scrolling down)."
                    </ApiRow>
                </ApiTable>

            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseScrollWheelReturn">
                    <ApiRow name="props" ty="UseScrollWheelProps">
                        "The "<Code inline=true>"wheel"</Code>" handler. Spread it with "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::*;
                        use leptos::prelude::*;

                        let value = RwSignal::new(50.0_f64);

                        let UseScrollWheelReturn { props } = use_scroll_wheel(UseScrollWheelInput {
                            on_scroll: Some(Callback::new(move |e: ScrollEvent| {
                                value.update(|v| *v = (*v - e.delta_y * 0.1).clamp(0.0, 100.0));
                            })),
                            ..Default::default()
                        });

                        view! {
                            <div {..props.into_attrs()}>{move || format!("Value: {:.0}", value.get())}</div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Scrolling over a box adjusts a value between 0 and 100" source=include_str!("demos/scroll_wheel.rs")>
                    <ScrollWheelDemo/>
                </Demo>
            </Section>

            <Section title="Behavior">
                <ul>
                    <li>"The hook prevents the browser default and stops propagation, so the page doesn\u{2019}t scroll while you scroll the element."</li>
                    <li>
                        "Wheel events with "<Keys keys="Control"/>" held are zoom gestures (including trackpad pinch). The hook "
                        "ignores them, so the browser can zoom."
                    </li>
                    <li>"Deltas are passed on as the browser reports them. Their size depends on the device and the browser."</li>
                    <li>
                        "The element doesn\u{2019}t need focus: wheel events go to the element under the pointer. Give "
                        "keyboard users another way to change the value, as "<Link href=routes::doc::NumberField.materialize()>"Number Field"</Link>
                        " does with its arrow keys."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UsePreventScroll.materialize()>"use_prevent_scroll"</Link></li>
                <li><Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
