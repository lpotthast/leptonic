use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{meter_storage::MeterStorageDemo, meter_value_text::MeterValueTextDemo};
use crate::{kit::*, routes};

#[component]
pub fn PageMeterComponent() -> impl IntoView {
    view! {
        <DocPage title="Meter Component">
            <p>
                "The themed "<Code inline=true>"Meter"</Code>" component shows a value within a known range, such as a "
                "battery level or used storage, as a filled track with its label and value text above. See the "
                <Link href=routes::doc::Meter.materialize()>"Meter overview"</Link>" for concept guidance."
            </p>

            <Demo description="A storage meter with its own value text" source=include_str!("demos/meter_storage.rs")>
                <MeterStorageDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="components::meter::Meter">
                    <ApiRow name="value" ty="NumberSignal<T>">
                        "The value: a number or any signal of one, clamped to the range. The number type "
                        <Code inline=true>"T"</Code>" is inferred from it. Required."
                    </ApiRow>
                    <ApiRow name="min_value" ty="Option<Signal<T>>" default="0">"The start of the range."</ApiRow>
                    <ApiRow name="max_value" ty="Option<Signal<T>>" default="100">"The end of the range."</ApiRow>
                    <ApiRow name="format_options" ty="Option<Signal<NumberFormatOptions>>" default="percent">
                        "How the value text is formatted: a percent style formats the percentage, other styles the value."
                    </ApiRow>
                    <ApiRow name="value_label" ty="MaybeProp<String>" default="None">"Replaces the value text (e.g. \u{201c}3 of 4 GB\u{201d})."</ApiRow>
                    <ApiRow name="label" ty="Option<String>" default="None">"The visible label above the track. Without it, set "<Code inline=true>"aria_label"</Code>"."</ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the meter when it has no visible label."</ApiRow>
                    <ApiRow name="show_value" ty="bool" default="true">"Whether the value text shows next to the label."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Additional classes and styles."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Value Text">
                <p>
                    "By default, the value text is the percentage of the range, formatted for the user\u{2019}s locale. A "
                    <Code inline=true>"format_options"</Code>" style other than percent formats the value itself; "
                    <Code inline=true>"value_label"</Code>" replaces the text (see above). With "
                    <Code inline=true>"show_value=false"</Code>" the text isn\u{2019}t shown, but screen readers still "
                    "announce it."
                </p>

                <Demo
                    description="Meters with the default percentage, a decimal value and a hidden value text"
                    source=include_str!("demos/meter_value_text.rs")
                >
                    <MeterValueTextDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt meters to your design:"</p>
                <CssVariables prefix="--meter-" scss=theme_scss!("meter")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Meter.materialize()>"Meter overview"</Link></li>
                <li><Link href=routes::doc::meter::Hook.materialize()>"use_meter"</Link></li>
                <li><Link href=routes::doc::meter::Atom.materialize()>"Meter Atoms"</Link></li>
                <li><Link href=routes::doc::progress_bar::Component.materialize()>"Progress Bar Component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
