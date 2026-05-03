use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ChipColorsDemo() -> impl IntoView {
    view! {
        <Chip color=ChipColor::Primary>"Primary"</Chip>
        <Chip color=ChipColor::Secondary>"Secondary"</Chip>
        <Chip color=ChipColor::Success>"Success"</Chip>
        <Chip color=ChipColor::Info>"Info"</Chip>
        <Chip color=ChipColor::Warn>"Warn"</Chip>
        <Chip color=ChipColor::Danger>"Danger"</Chip>
    }
}
