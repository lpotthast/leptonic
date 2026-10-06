use indoc::indoc;
use leptonic::components::prelude::*;
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
                <Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <ReactAria hook="useScrollWheel"/>

            <Section title="Input">
                <ApiTable kind=ApiKind::Input of="UseScrollWheelInput">
                    <ApiRow name="is_disabled" ty="Signal<bool>">
                        "Ignore wheel events while "<Code inline=true>"true"</Code>". The page scrolls normally then."
                    </ApiRow>
                    <ApiRow name="on_scroll" ty="Option<Callback<ScrollEvent>>">
                        "Called for every wheel event with the "<Code inline=true>"ScrollEvent"</Code>"\u{2019}s "
                        <Code inline=true>"delta_x"</Code>" and "<Code inline=true>"delta_y"</Code>" (positive "
                        <Code inline=true>"delta_y"</Code>" means scrolling down)."
                    </ApiRow>
                </ApiTable>

                <p><Code inline=true>"UseScrollWheelInput"</Code>" has no "<Code inline=true>"Default"</Code>", so you set both fields."</p>
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
                        let (value, set_value) = signal(50.0f64);

                        let scroll_wheel = use_scroll_wheel(UseScrollWheelInput {
                            is_disabled: Signal::stored(false),
                            on_scroll: Some(Callback::new(move |e: ScrollEvent| {
                                set_value.update(|v| *v = (*v - e.delta_y * 0.1).clamp(0.0, 100.0));
                            })),
                        });

                        view! {
                            <div {..scroll_wheel.props.into_attrs()} tabindex="0">
                                {move || format!("Value: {:.0}", value.get())}
                            </div>
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
                        "Wheel events with "<Code inline=true>"Ctrl"</Code>" held are zoom gestures (including trackpad pinch). The hook "
                        "ignores them, so the browser can zoom."
                    </li>
                    <li>"Deltas are passed on as the browser reports them. Their size depends on the device and the browser."</li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::interactions::UsePreventScroll.materialize()>"use_prevent_scroll"</Link></li>
                <li><Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
