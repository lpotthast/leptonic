use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::meter_battery::MeterBatteryDemo;
use super::demos::meter_disk::MeterDiskDemo;

#[component]
pub fn PageUseMeter() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_meter" class="anchor">
                "use_meter"
                <AnchorLink href="#use_meter" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_meter"</Code>" hook is a standalone hook for creating accessible meter/gauge components that represent a scalar value within a known range."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useMeter.html" target=LinkTarget::_Blank>
                    "useMeter"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell source=include_str!("demos/meter_disk.rs")>
                <MeterDiskDemo />
            </DemoShell>

            <DemoShell source=include_str!("demos/meter_battery.rs")>
                <MeterBatteryDemo />
            </DemoShell>

            <Code language=Language::Rust>
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
            <Code language=Language::Rust>
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

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::DataDisplay.materialize()>"Data Display domain"</Link></li>
                <li><Link href=crate::routes::doc::Progress.materialize()>"Progress"</Link></li>
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
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
