use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TypographyListDemo() -> impl IntoView {
    view! {
        <Ul>
            <Li slot>"Flour"</Li>
            <Li slot>"Water"</Li>
            <Li slot>"Salt"</Li>
        </Ul>
    }
}
