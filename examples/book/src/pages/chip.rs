use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageChip(cx: Scope) -> impl IntoView {
    let (dismissed, set_dismissed) = create_signal(cx, false);

    view! { cx,
        <Typography variant=TypographyVariant::H1>"Chips"</Typography>

        <Code>
            {indoc!(r#"
                <Chip color=ChipColor::Primary>"Primary"</Chip>
            "#)}
        </Code>

        <Chip color=ChipColor::Primary>"Primary"</Chip>
        <Chip color=ChipColor::Secondary>"Secondary"</Chip>
        <Chip color=ChipColor::Success>"Success"</Chip>
        <Chip color=ChipColor::Info>"Info"</Chip>
        <Chip color=ChipColor::Warn>"Warn"</Chip>
        <Chip color=ChipColor::Danger>"Danger"</Chip>

        <H2>"Dismissible chips"</H2>

        <P>"As chips are often used to convey mutable state, we allow chips to be dismissible."</P>
        <P>"Dismissible chips display an X icon which lets the user dismiss the chip."</P>
        <P>"The component embedding the chip is responsible of actually removing it, e.g. not rendering it again."</P>

        <Code>
            {indoc!(r#"
                <Chip
                    color=ChipColor::Secondary
                    dismissible=Callback::new(cx, move |_| set_dismissed.set(true))
                >
                    "Dismissible"
                </Chip>
            "#)}
        </Code>

        <Show
            when=move || !dismissed.get()
            fallback=move |cx| view! {cx, <Button on_click=move |_| set_dismissed.set(false)>"Reveal chip"</Button>}
        >
            <Chip color=ChipColor::Secondary dismissible=Callback::new(cx, move |()| set_dismissed.set(true))>
                "Dismissible"
            </Chip>
        </Show>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --chip-primary-text-color
                --chip-primary-text-color-hover
                --chip-primary-background-color
                --chip-primary-background-color-hover
                --chip-secondary-text-color
                --chip-secondary-text-color-hover
                --chip-secondary-background-color
                --chip-secondary-background-color-hover
                --chip-success-text-color
                --chip-success-text-color-hover
                --chip-success-background-color
                --chip-success-background-color-hover
                --chip-info-text-color
                --chip-info-text-color-hover
                --chip-info-background-color
                --chip-info-background-color-hover
                --chip-warn-text-color
                --chip-warn-text-color-hover
                --chip-warn-background-color
                --chip-warn-background-color-hover
                --chip-danger-text-color
                --chip-danger-text-color-hover
                --chip-danger-background-color
                --chip-danger-background-color-hover
            "#)}
        </Code>
    }
}
