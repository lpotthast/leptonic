use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::prelude::Size;
use leptos::prelude::*;

#[component]
pub fn PageUseMeter() -> impl IntoView {
    let (disk_usage, _set_disk_usage) = signal(72.5);
    let (battery, _set_battery) = signal(45.0);

    let disk_meter = use_meter(UseMeterInput {
        value: disk_usage.into(),
        label: Some("Disk Usage".to_string()),
        min_value: 0.0,
        max_value: 100.0,
        ..Default::default()
    });

    let battery_meter = use_meter(UseMeterInput {
        value: battery.into(),
        label: Some("Battery".to_string()),
        min_value: 0.0,
        max_value: 100.0,
        format_options: Some(MeterFormatOptions {
            style: MeterFormatStyle::Percent,
            decimals: 0,
        }),
        ..Default::default()
    });

    view! {
        <Article>
            <h1 id="use_meter" class="anchor">
                "use_meter"
                <AnchorLink href="#use_meter" description="Direct link to article header"/>
            </h1>

            <p>"Hook for creating accessible meter/gauge components that represent a scalar value within a known range."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <Stack orientation=StackOrientation::Vertical spacing=Size::Em(1.5)>
                <div style="max-width: 300px;">
                    <div style="display: flex; justify-content: space-between; margin-bottom: 0.5em;">
                        <label id={disk_meter.label_props.id.clone()}>"Disk Usage"</label>
                        <span>{ move || disk_meter.value_label.get() }</span>
                    </div>
                    <div
                        {..disk_meter.meter_props}
                        style="height: 20px; background: #e0e0e0; border-radius: 4px; overflow: hidden;"
                    >
                        <div style=move || format!(
                            "height: 100%; width: {}%; background: {}; transition: width 0.3s;",
                            disk_meter.percentage.get(),
                            if disk_meter.percentage.get() > 80.0 { "#e53935" }
                            else if disk_meter.percentage.get() > 60.0 { "#fb8c00" }
                            else { "#43a047" }
                        )></div>
                    </div>
                </div>

                <div style="max-width: 300px;">
                    <div style="display: flex; justify-content: space-between; margin-bottom: 0.5em;">
                        <label id={battery_meter.label_props.id.clone()}>"Battery Level"</label>
                        <span>{ move || battery_meter.value_label.get() }</span>
                    </div>
                    <div
                        {..battery_meter.meter_props}
                        style="height: 20px; background: #e0e0e0; border-radius: 4px; overflow: hidden;"
                    >
                        <div style=move || format!(
                            "height: 100%; width: {}%; background: {}; transition: width 0.3s;",
                            battery_meter.percentage.get(),
                            if battery_meter.percentage.get() < 20.0 { "#e53935" }
                            else if battery_meter.percentage.get() < 50.0 { "#fb8c00" }
                            else { "#43a047" }
                        )></div>
                    </div>
                </div>
            </Stack>

            <Code>
                {indoc!(r#"
                    let UseMeterReturn { meter_props, label_props, percentage, value_label, .. } = use_meter(
                        UseMeterInput {
                            value: Signal::derive(|| 72.5),
                            label: Some("Disk Usage".to_string()),
                            min_value: 0.0,
                            max_value: 100.0,
                            ..Default::default()
                        }
                    );

                    view! {
                        <div>
                            <label id={label_props.id}>"Disk Usage"</label>
                            <span>{ move || value_label.get() }</span>
                        </div>
                        <div {..meter_props} style="height: 20px; background: #ddd;">
                            <div style=move || format!("width: {}%;", percentage.get())></div>
                        </div>
                    }
                "#)}
            </Code>

            <h2 id="meter-vs-progress" class="anchor">
                "Meter vs Progress Bar"
                <AnchorLink href="#meter-vs-progress" description="Direct link to meter vs progress"/>
            </h2>

            <p>"Meters and progress bars serve different purposes:"</p>
            <ul>
                <li><strong>"Meter"</strong>": Displays a static scalar value (disk usage, battery level, vote ratings)"</li>
                <li><strong>"Progress Bar"</strong>": Shows completion of a task over time (file upload, loading)"</li>
            </ul>

            <h2 id="format-options" class="anchor">
                "Format Options"
                <AnchorLink href="#format-options" description="Direct link to format options"/>
            </h2>

            <p>"Customize value display with format options:"</p>
            <Code>
                {indoc!(r"
                    UseMeterInput {
                        format_options: Some(MeterFormatOptions {
                            style: MeterFormatStyle::Percent, // or Decimal
                            decimals: 1,
                        }),
                        ..Default::default()
                    }
                ")}
            </Code>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"The hook automatically sets:"</p>
            <ul>
                <li><code>"role=\"meter\""</code></li>
                <li><code>"aria-valuenow"</code> " (current value)"</li>
                <li><code>"aria-valuemin"</code> " (minimum value)"</li>
                <li><code>"aria-valuemax"</code> " (maximum value)"</li>
                <li><code>"aria-valuetext"</code> " (human-readable value)"</li>
                <li><code>"aria-labelledby"</code> " (links to label)"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Semantic meter role"</li>
                <li>"Computed percentage"</li>
                <li>"Customizable value formatting"</li>
                <li>"Label association"</li>
                <li>"Full ARIA support"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_meter", link: "#use_meter" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Meter vs Progress", link: "#meter-vs-progress" },
                Toc::Leaf { title: "Format Options", link: "#format-options" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
