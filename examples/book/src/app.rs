use std::fmt::Display;

use leptos::*;
use leptos_icons::BsIcon;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::*;
use leptos_use::use_media_query;

use leptonic::prelude::*;

use crate::pages::{documentation::doc_root::DocRoutes, err404::PageErr404, welcome::PageWelcome};

#[derive(Debug, Copy, Clone)]
pub enum AppRoutes {
    Welcome,
    Doc,
    NotFound,
}

impl AppRoutes {
    pub fn route(self) -> &'static str {
        match self {
            AppRoutes::Welcome => "",
            AppRoutes::Doc => "doc",
            AppRoutes::NotFound => "*", // Leptos requires this to be be named "*"!
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
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Leptonic"/>
        <Root default_theme=LeptonicTheme::default()>
            <Router>
                <Routes>
                    <Route path="" view=|| view! { <Layout/> }>
                        <Route path=AppRoutes::Welcome view=|| view! { <PageWelcome/> }/>
                        <DocRoutes path=AppRoutes::Doc/>
                        <Route path=AppRoutes::NotFound view=|| view! { <PageErr404 /> }/>
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
pub fn Layout() -> impl IntoView {
    let is_small = use_media_query("(max-width: 800px)");
    let router_context = use_router();
    let is_doc = create_memo(move |_| router_context.pathname().get().starts_with("/doc"));

    // The main drawer is only used on mobile / small screens!.
    let (main_drawer_closed, set_main_drawer_closed) = create_signal(true);
    let (doc_drawer_closed, set_doc_drawer_closed) = create_signal(false);

    // Always close the doc-drawer when the application is now small.
    // Always open the doc-drawer when the application is no longer small.
    create_effect(move |_| {
        if is_small.get() {
            set_doc_drawer_closed.set(true);
        } else {
            set_doc_drawer_closed.set(false);
        }
    });

    // Always close the main-drawer when the application is no longer small.
    create_effect(move |_| {
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

    provide_context(ctx);

    let search_options = vec![
        (
            "overview",
            QuicksearchOption {
                view: ViewProducer::new(move || {
                    view! {
                        <Link href=DocRoutes::Overview class="search-link">
                            "Overview"
                        </Link>
                    }
                }),
                on_select: Callback::new(move |()| {}),
            },
        ),
        (
            "installation",
            QuicksearchOption {
                view: ViewProducer::new(move || {
                    view! {
                        <Link href=DocRoutes::Installation class="search-link">
                            "Installation"
                        </Link>
                    }
                }),
                on_select: Callback::new(move |()| {}),
            },
        ),
        (
            "usage",
            QuicksearchOption {
                view: ViewProducer::new(move || {
                    view! {
                        <Link href=DocRoutes::Usage class="search-link">
                            "Usage"
                        </Link>
                    }
                }),
                on_select: Callback::new(move |()| {}),
            },
        ),
    ];

    let logo = move || {
        view! {
            <Link href="">
                <img src="/res/leptonic.svg" id="logo" alt="Leptonic logo"/>
            </Link>
        }
    };

    view! {
        <AppBar id="app-bar" height=APP_BAR_HEIGHT>
            <div id="app-bar-content">
                <Stack id="left" orientation=StackOrientation::Horizontal spacing=Size::Zero>
                    { move || match (is_doc.get(), is_small.get()) {
                        (false, true) => logo().into_view(),
                        (true, true) => view! {
                            <Icon id="mobile-menu-trigger" icon=BsIcon::BsList on:click=move |_| ctx.toggle_doc_drawer()/>
                            { logo }
                        }.into_view(),
                        (_, false) => view! {
                            { logo }
                            <Link href=AppRoutes::Doc>
                                <H3 style="margin: 0 0 0 0.5em">
                                    "Docs"
                                </H3>
                            </Link>
                        }.into_view(),
                    } }
                </Stack>

                <Stack id="center" orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                    <Quicksearch
                        id="quicksearch"
                        trigger=move |set_quicksearch| view! {
                            <QuicksearchTrigger id="quicksearch-trigger" set_quicksearch=set_quicksearch>
                                { move || match is_small.get() {
                                    true => view! { <Icon icon=BsIcon::BsSearch />}.into_view(),
                                    false => view! { "Search"}.into_view(),
                                } }
                            </QuicksearchTrigger>
                        }
                        query=move |search: String| {
                            if search.is_empty() {
                                return vec![];
                            }
                            let lower_search = search.to_lowercase();
                            // TODO: Re-enable in rc3
                            //search_options.iter()
                            //    .filter(|it| it.0.to_lowercase().contains(&lower_search))
                            //    .map(|it| it.1.clone())
                            //    .collect::<Vec<_>>()
                            Vec::new()
                        }
                    />
                </Stack>

                <Stack id="right" orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                    { move || match is_small.get() {
                        true => view! {
                            <Icon id="mobile-menu-trigger" icon=BsIcon::BsThreeDots on:click=move |_| ctx.toggle_main_drawer()/>
                        }.into_view(),
                        false => view! {
                            <Link href=DocRoutes::Changelog>"v0.3.0"</Link>

                            <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>
                                <Icon id="github-icon" icon=BsIcon::BsGithub aria_label="GitHub icon"/>
                            </LinkExt>

                            <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark style="margin-right: 1em"/>
                        }.into_view(),
                    } }
                </Stack>
            </div>
        </AppBar>

        <Box id="content" style=format!("height: calc(var(--leptonic-vh, 100vh) - {APP_BAR_HEIGHT}); max-height: calc(var(--leptonic-vh, 100vh) - {APP_BAR_HEIGHT});")>
            // <Outlet/> will show nested child routes.
            <Outlet/>

            <Drawer id="main-drawer" shown=Signal::derive(move || !main_drawer_closed.get()) side=DrawerSide::Right style=format!("top: {APP_BAR_HEIGHT}")>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Em(2.0) class="menu">

                    <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank style="font-size: 3em;">
                        <Icon id="github-icon" icon=BsIcon::BsGithub/>
                    </LinkExt>

                    <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark style="margin-right: 1em"/>

                    "Currently - v0.3.0"
                </Stack>
            </Drawer>
        </Box>
    }
}
