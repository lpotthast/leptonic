use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn ChipConceptDemo() -> impl IntoView {
    view! {
        <Chip>"Default"</Chip>
        <Chip color=Signal::from(ChipColor::Success)>"Success"</Chip>
        <Chip color=Signal::from(ChipColor::Danger)>"Danger"</Chip>
    }
}
