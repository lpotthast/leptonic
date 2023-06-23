use leptonic::prelude::*;
use leptos::*;
use leptos_icons::BsIcon;

use crate::app::AppTheme;

#[component]
pub fn PageAppBar(cx: Scope) -> impl IntoView {
    let app_bar_height = Height::Em(3.0);

    view! { cx,
        <H1>"App Bar"</H1>

        <div style="position: relative;">
            <AppBar height=app_bar_height>
                <Link href="">
                    <Typography variant=TypographyVariant::H3 margin=Margin::Left(Size::Em(1.0))>
                        "Leptonic  -  v0.1"
                    </Typography>
                </Link>
                <Stack orientation=StackOrientation::Horizontal spacing=10 style="margin-right: 1em">
                    <Icon icon=BsIcon::BsGithub></Icon>
                    <ThemeToggle off=AppTheme::Light on=AppTheme::Dark/>
                </Stack>
            </AppBar>
        </div>
    }
}
