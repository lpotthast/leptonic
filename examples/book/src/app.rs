use std::fmt::Display;

use leptos::*;
use leptos_icons::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::*;
use leptos_use::use_media_query;
use serde::{Deserialize, Serialize};

use leptonic::prelude::*;

use crate::pages::{documentation::doc_root::DocRoutes, err404::PageErr404, welcome::PageWelcome};

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum AppTheme {
    #[default]
    Light,
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
        Box::new(move || format!("/{}", self.route()))
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

#[derive(Debug, Clone, Copy)]
pub struct AppLayoutContext {
    pub is_small: Signal<bool>,
    pub main_drawer_closed: Signal<bool>,
    set_main_drawer_closed: WriteSignal<bool>,
    pub doc_drawer_closed: Signal<bool>,
    set_doc_drawer_closed: WriteSignal<bool>,
}

impl AppLayoutContext {
    #[allow(unused)]
    pub fn close_main_drawer(&self) {
        self.set_main_drawer_closed.set(true);
    }

    pub fn close_doc_drawer(&self) {
        self.set_doc_drawer_closed.set(true);
    }

    pub fn toggle_main_drawer(&self) {
        let currently_closed = self.main_drawer_closed.get_untracked();
        self.set_main_drawer_closed.set(!currently_closed);
        if currently_closed {
            self.close_doc_drawer();
        }
    }

    pub fn toggle_doc_drawer(&self) {
        let currently_closed = self.doc_drawer_closed.get_untracked();
        self.set_doc_drawer_closed.set(!currently_closed);
        if currently_closed {
            self.close_main_drawer();
        }
    }
}

#[component]
pub fn Layout(cx: Scope) -> impl IntoView {
    let is_small = use_media_query(cx, "(max-width: 800px)");
    let router_context = use_router(cx);
    let is_doc = Signal::derive(cx, move || {
        router_context.pathname().get().starts_with("/doc")
    });

    // The main drawer is only used on mobile / small screens!.
    let (main_drawer_closed, set_main_drawer_closed) = create_signal(cx, true);
    let (doc_drawer_closed, set_doc_drawer_closed) = create_signal(cx, false);

    // Always re-close the doc-drawer when the application is now small.
    // Always re-open the doc-drawer when the application is no longer small.
    create_effect(cx, move |_| {
        if is_small.get() {
            set_doc_drawer_closed.set(true);
        } else {
            set_doc_drawer_closed.set(false);
        }
    });

    // Always close the main-drawer when the application is no longer small.
    create_effect(cx, move |_| {
        if !is_small.get() {
            set_main_drawer_closed.set(true);
        }
    });

    let ctx = AppLayoutContext {
        is_small,
        main_drawer_closed: main_drawer_closed.into(),
        set_main_drawer_closed,
        doc_drawer_closed: doc_drawer_closed.into(),
        set_doc_drawer_closed,
    };

    provide_context(cx, ctx.clone());

    let search_options = vec![
        (
            "overview",
            QuicksearchOption {
                view: create_callback(cx, move |cx| {
                    view! {cx,
                        <Link href=DocRoutes::Overview class="search-link">
                            "Overview"
                        </Link>
                    }
                    .into_view(cx)
                }),
                on_select: create_callback(cx, move |_| {}),
            },
        ),
        (
            "installation",
            QuicksearchOption {
                view: create_callback(cx, move |cx| {
                    view! {cx,
                        <Link href=DocRoutes::Installation class="search-link">
                            "Installation"
                        </Link>
                    }
                    .into_view(cx)
                }),
                on_select: create_callback(cx, move |_| {}),
            },
        ),
        (
            "usage",
            QuicksearchOption {
                view: create_callback(cx, move |cx| {
                    view! {cx,
                        <Link href=DocRoutes::Usage class="search-link">
                            "Usage"
                        </Link>
                    }
                    .into_view(cx)
                }),
                on_select: create_callback(cx, move |_| {}),
            },
        ),
    ];

    view! { cx,
        <AppBar id="app-bar" height=APP_BAR_HEIGHT>
            <div id="app-bar-content">
                <Stack id="left" orientation=StackOrientation::Horizontal spacing=Size::Zero>
                    { move || match (is_doc.get(), is_small.get()) {
                        (false, true) => view! {cx,
                            <Link href="">
                                <img src="/res/leptonic.svg" id="logo"/>
                            </Link>
                        }.into_view(cx),
                        (true, true) => view! {cx,
                            <Icon id="mobile-menu-trigger" icon=BsIcon::BsList on:click=move |_| ctx.toggle_doc_drawer()/>
                            <Link href="">
                                <img src="/res/leptonic.svg" id="logo"/>
                            </Link>
                        }.into_view(cx),
                        (_, false) => view! {cx,
                            <Link href="">
                                <img src="/res/leptonic.svg" id="logo"/>
                            </Link>

                            <Link href=AppRoutes::Doc>
                                <Typography variant=TypographyVariant::H3 margin=Margin::Left(Size::Em(0.5))>
                                    "Docs"
                                </Typography>
                            </Link>
                        }.into_view(cx),
                    } }
                </Stack>

                <Stack id="center" orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                    <Quicksearch
                        id="quicksearch"
                        trigger=move |cx, set_quicksearch| view! {cx,
                            <QuicksearchTrigger id="quicksearch-trigger" set_quicksearch=set_quicksearch>
                                { move || match is_small.get() {
                                    true => view! {cx, <Icon icon=BsIcon::BsSearch />}.into_view(cx),
                                    false => view! {cx, "Search"}.into_view(cx),
                                } }
                            </QuicksearchTrigger>
                        }
                        query=create_callback(cx, move |search: String| {
                            if search.is_empty() {
                                return vec![];
                            }
                            let lower_search = search.to_lowercase();
                            search_options.iter()
                                .filter(|it| it.0.to_lowercase().contains(&lower_search))
                                .map(|it| it.1.clone())
                                .collect::<Vec<_>>()
                        })
                    />
                </Stack>

                <Stack id="right" orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                    { move || match (is_doc.get(), is_small.get()) {
                        (_, true) => view! {cx,
                            <Icon id="mobile-menu-trigger" icon=BsIcon::BsThreeDots on:click=move |_| ctx.toggle_main_drawer()/>
                        }.into_view(cx),
                        (_, false) => view! {cx,
                            "v0.1"

                            <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>
                                <Icon id="github-icon" icon=BsIcon::BsGithub/>
                            </LinkExt>

                            <ThemeToggle off=AppTheme::Light on=AppTheme::Dark style="margin-right: 1em"/>
                        }.into_view(cx),
                    } }
                </Stack>
            </div>
        </AppBar>

        <Box id="content" style=format!("margin-top: {APP_BAR_HEIGHT}; height: calc(var(--leptonic-vh, 100vh) - {APP_BAR_HEIGHT}); max-height: calc(var(--leptonic-vh, 100vh) - {APP_BAR_HEIGHT});")>
            // <Outlet/> will show nested child routes.
            <Outlet/>

            <Drawer id="main-drawer" shown=Signal::derive(cx, move || !main_drawer_closed.get()) side=DrawerSide::Right style=format!("top: {APP_BAR_HEIGHT}")>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="menu">
                    "v0.1"

                    <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>
                        <Icon id="github-icon" icon=BsIcon::BsGithub/>
                    </LinkExt>

                    <ThemeToggle off=AppTheme::Light on=AppTheme::Dark style="margin-right: 1em"/>
                </Stack>
            </Drawer>
        </Box>
    }
}
