use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageButton(cx: Scope) -> impl IntoView {
    let (disabled, set_disabled) = create_signal(cx, false);
    view! { cx,
        <Typography variant=TypographyVariant::H2>"Buttons"</Typography>

        <div>
            <Button on_click=move |_| {} variant=ButtonVariant::Flat>"Flat"</Button>
            <Button on_click=move |_| {} variant=ButtonVariant::Outlined>"Outlined"</Button>
            <Button on_click=move |_| {} variant=ButtonVariant::Filled>"Filled"</Button>
        </div>

        <h2>"Button group"</h2>

        <ButtonGroup>
            <Button on_click=move |_| {} variant=ButtonVariant::Filled>"Button 1"</Button>
            <Button on_click=move |_| {} variant=ButtonVariant::Filled>"Button 2"</Button>
            <Button on_click=move |_| {} variant=ButtonVariant::Filled>"Button 3"</Button>
        </ButtonGroup>

        <Typography variant=TypographyVariant::H3>"Disabled"</Typography>

        <Toggle on=disabled set_on=set_disabled/>

        <Button on_click=move |_| {} variant=ButtonVariant::Filled disabled=true>"Disabled"</Button>
        <Button on_click=move |_| {} variant=ButtonVariant::Filled disabled=disabled>"Disabled"</Button>
        <Button on_click=move |_| {} variant=ButtonVariant::Filled disabled=Signal::derive(cx, move || !disabled.get())>"Disabled"</Button>
    }
}
