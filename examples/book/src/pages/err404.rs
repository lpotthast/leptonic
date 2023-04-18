use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageErr404(cx: Scope) -> impl IntoView {
    view! { cx,
        <Typography variant=TypographyVariant::H2>"404 Not Found"</Typography>

        <Link href="">
            <Button on_click=move |_| {} variant=ButtonVariant::Filled>
                "Back"
            </Button>
        </Link>
    }
}
