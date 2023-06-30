use leptonic::prelude::*;
use leptos::*;
use leptos_icons::BsIcon;

#[component]
pub fn PageAppBar(cx: Scope) -> impl IntoView {
    let app_bar_height = Height::Em(3.0);

    view! { cx,
        <H1>"App Bar"</H1>

        <div style="position: relative;">
            <AppBar height=app_bar_height>
                <Link href="">
                    <H3>"Leptonic  -  v0.1"</H3>
                </Link>
                <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0) style="margin-right: 1em">
                    <Icon icon=BsIcon::BsGithub></Icon>
                    <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark/>
                </Stack>
            </AppBar>
        </div>
    }
}
