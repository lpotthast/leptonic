use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageChip(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H1>"Chips"</Typography>

        <Chip color=ChipColor::Primary>"Primary"</Chip>
        <Chip color=ChipColor::Secondary>"Secondary"</Chip>
        <Chip color=ChipColor::Success>"Success"</Chip>
        <Chip color=ChipColor::Info>"Info"</Chip>
        <Chip color=ChipColor::Warn>"Warn"</Chip>
        <Chip color=ChipColor::Danger>"Danger"</Chip>
    }
}
