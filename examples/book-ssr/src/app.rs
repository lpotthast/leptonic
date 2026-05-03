use leptonic::{
    components::prelude::*,
    hooks::LinkTarget,
    prelude::*,
    utils::css::{CssDimension, em},
};
use leptos::prelude::*;
use leptos_meta::{Link as MetaLink, Meta, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{components::Router, hooks::use_location};
use leptos_use::use_media_query;

use crate::{pages::documentation::doc_search::DocSearch, routes};

pub const LEPTOS_OUTPUT_NAME: &str = env!("LEPTOS_OUTPUT_NAME");

//noinspection DuplicatedCode
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>

                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
                <link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap" rel="stylesheet"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
#[allow(clippy::let_unit_value)]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Meta name="description" content="Leptonic"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Stylesheet id="leptos" href=format!("/pkg/{LEPTOS_OUTPUT_NAME}.css")/>

        <MetaLink rel="icon" href="/res/icon/leptonic_x64.png"/>
        <MetaLink rel="apple-touch-icon" href="/res/icon/maskable_icon_x192.png"/>

        <Title text="Leptonic"/>

        <Root default_theme=LeptonicTheme::default()>
            <Router>
                <Layout>
                    { routes::route_tree() }
                </Layout>
            </Router>
        </Root>
    }
}

pub const APP_BAR_HEIGHT: Height = CssDimension::em(3.5);

#[derive(Debug, Clone, Copy)]
pub struct AppLayoutContext {
    pub is_small: Signal<bool>,
    pub is_medium: Signal<bool>,
    pub main_drawer_closed: Signal<bool>,
    set_main_drawer_closed: WriteSignal<bool>,
    pub doc_drawer_closed: Signal<bool>,
    set_doc_drawer_closed: WriteSignal<bool>,
}

impl AppLayoutContext {
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
pub fn Layout(children: Children) -> impl IntoView {
    let is_small = use_media_query("(max-width: 800px)");
    let is_medium = use_media_query("(max-width: 1200px)");

    let location = use_location();

    let is_doc = Memo::new(move |_| location.pathname.get().starts_with("/doc"));

    // The main drawer is only used on mobile / small screens!.
    let (main_drawer_closed, set_main_drawer_closed) = signal(true);
    let (doc_drawer_closed, set_doc_drawer_closed) = signal(true);

    let ctx = AppLayoutContext {
        is_small,
        is_medium,
        main_drawer_closed: main_drawer_closed.into(),
        set_main_drawer_closed,
        doc_drawer_closed: doc_drawer_closed.into(),
        set_doc_drawer_closed,
    };

    provide_context(ctx);

    // Close/open doc drawer based on route.
    Effect::new(move |_| {
        if !is_doc.get() {
            set_doc_drawer_closed.set(true);
        } else if !is_small.get() {
            set_doc_drawer_closed.set(false);
        }
    });

    // Close doc drawer when screen becomes small; open when large.
    Effect::new(move |_| {
        if is_small.get() {
            set_doc_drawer_closed.set(true);
        } else {
            set_doc_drawer_closed.set(false);
        }
    });

    // Close main drawer when screen is no longer small.
    Effect::new(move |_| {
        if !is_small.get() {
            set_main_drawer_closed.set(true);
        }
    });

    let logo = move || {
        view! {
            <Link href="">
                <img src="/res/leptonic.svg" id="logo" alt="Leptonic logo"/>
            </Link>
        }
    };

    view! {
        <AppBar attr:id="app-bar" height=APP_BAR_HEIGHT>
            <div id="app-bar-content">
                <Stack attr:id="left" orientation=StackOrientation::Horizontal spacing=CssDimension::Zero>
                    { move || match (is_doc.get(), is_small.get()) {
                        (false, true) => logo().into_any(),
                        (true, true) => view! {
                            <Icon attr:id="mobile-menu-trigger" icon=icondata::BsList on:click=move |_| ctx.toggle_doc_drawer()/>
                            { logo }
                        }.into_any(),
                        (_, false) => view! {
                            { logo }
                            <Link href=routes::Doc.materialize()>
                                <h3 style="margin: 0 0 0 0.5em">
                                    "Docs"
                                </h3>
                            </Link>
                        }.into_any(),
                    } }
                </Stack>

                <Stack attr:id="right" orientation=StackOrientation::Horizontal spacing=em(1.0)>
                    <DocSearch/>
                    { move || if is_small.get() { view! {
                        <Icon attr:id="mobile-menu-trigger" icon=icondata::BsThreeDots on:click=move |_| ctx.toggle_main_drawer()/>
                    }.into_any() } else { view! {
                        <Link href=routes::doc::Changelog.materialize()>"v0.6.0 (main)"</Link>

                        <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkTarget::_Blank>
                            <Icon attr:id="github-icon" icon=icondata::BsGithub aria_label="GitHub icon"/>
                        </LinkExt>

                        <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark attr:style="margin-right: 1em"/>
                    }.into_any() } }
                </Stack>
            </div>
        </AppBar>

        <main
            id="content"
            style="color: var(--main-color); background-color: var(--main-background-color);"
            aria-hidden=move || { ((is_doc.get() && is_small.get() && !doc_drawer_closed.get()) || !main_drawer_closed.get()).to_string() }
        >
            { children() }

            <Drawer
                attr:id="main-drawer"
                attr:style=format!("top: {APP_BAR_HEIGHT}")
                shown=Signal::derive(move || !main_drawer_closed.get())
                side=DrawerSide::Right
            >
                <Stack orientation=StackOrientation::Vertical spacing=em(2.0) attr:class="menu">

                    <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkTarget::_Blank attr:style="font-size: 3em;">
                        <Icon attr:id="github-icon" icon=icondata::BsGithub/>
                    </LinkExt>

                    <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark attr:style="margin-right: 1em"/>

                    "Currently - v0.6.0 (main)"
                </Stack>
            </Drawer>
        </main>
    }
}
