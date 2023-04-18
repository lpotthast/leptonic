use leptonic::prelude::*;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn PageIcon(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H2>"Icons"</Typography>

        <Icon icon=BsIcon::BsFolderFill/>
        <Icon icon=BsIcon::BsFolder/>
    }
}
