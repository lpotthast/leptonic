use std::fmt::Display;

use leptos::*;
use leptos_icons::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::*;
use serde::{Deserialize, Serialize};

use leptonic::prelude::*;

use crate::pages::{documentation::{
    doc_root::DocRoutes, err404::PageErr404, overview::PageOverview,
}, welcome::PageWelcome};

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum AppTheme {
    Light,
    #[default]
    Dark,
}

impl Theme for AppTheme {
    fn name(&self) -> &'static str {
        match self {
            AppTheme::Light => "light",
            AppTheme::Dark => "dark",
        }
    }

    fn icon(&self) -> leptos_icons::Icon {
        match self {
            AppTheme::Light => BsIcon::BsSun.into(),
            AppTheme::Dark => BsIcon::BsMoon.into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AppRoutes {
    // Content
    Welcome,
    Doc,

    // Technical
    CatchAll,
}

impl AppRoutes {
    pub fn route(self) -> &'static str {
        match self {
            AppRoutes::Welcome => "",
            AppRoutes::Doc => "doc",
            AppRoutes::CatchAll => "*",
        }
    }
}

/// Required so that `Routes` variants can be used in `<Route path=Routes::Foo ...>` definitions.
impl Display for AppRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.route())
    }
}

/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo ...>` definitions.
impl ToHref for AppRoutes {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(move || self.route().to_owned())
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Title text="Leptonic"/>
        <Root default_theme=AppTheme::default()>
            <Router>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Layout/> }>
                        <Route path=AppRoutes::Welcome view=|cx| view! { cx, <PageWelcome/> }/>
                        <DocRoutes path=AppRoutes::Doc/>
                        <Route path=AppRoutes::CatchAll view=|cx| view! { cx, <PageErr404 /> }/>
                    </Route>
                </Routes>
            </Router>
        </Root>
    }
}

pub const APP_BAR_HEIGHT: Height = Height::Em(3.5);

#[component]
pub fn Layout(cx: Scope) -> impl IntoView {
    view! { cx,
        <AppBar id="app-bar" height=APP_BAR_HEIGHT>
            <div id="app-bar-content">
                <Stack id="links" orientation=StackOrientation::Horizontal spacing=0>
                    <Link href="">
                        <img src="/res/leptonic.svg" id="logo"/>
                    </Link>

                    <Link href=AppRoutes::Doc>
                        <Typography variant=TypographyVariant::H3 margin=Margin::Left(Size::Em(0.5))>
                            "Docs"
                        </Typography>
                    </Link>
                </Stack>

                <Quicksearch />

                <Stack id="actions" orientation=StackOrientation::Horizontal spacing=10 style="margin-right: 1em">
                    "v0.1"

                    <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>
                        <Icon id="github-icon" icon=BsIcon::BsGithub></Icon>
                    </LinkExt>

                    <ThemeToggle off=AppTheme::Light on=AppTheme::Dark/>
                </Stack>
            </div>
        </AppBar>

        <Box id="content" style=format!("margin-top: {APP_BAR_HEIGHT}; height: calc(100vh - {APP_BAR_HEIGHT}); max-height: calc(100vh - {APP_BAR_HEIGHT});")>
            // <Outlet/> will show nested child routes.
            <Outlet/>
        </Box>
    }
}
