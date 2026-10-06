use leptonic::{components::prelude::*, utils::number_formatter::NumberFormatOptions};
use leptos::prelude::*;

#[component]
pub fn MeterValueTextDemo() -> impl IntoView {
    view! {
        // The default: the percentage of the range.
        <Meter value=68 label="Battery" />
        // A non-percent style formats the value itself.
        <Meter value=1_250 max_value=2_000 label="Points" format_options=NumberFormatOptions::default() />
        // No visible value text; screen readers still announce "40%".
        <Meter value=40 label="Profile complete" show_value=false />
    }
}
