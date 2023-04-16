use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageButton(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Buttons"</h2>

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
    }
}
