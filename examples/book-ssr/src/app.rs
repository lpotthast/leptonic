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

use crate::{
    pages::documentation::doc_search::DocSearch,
    routes,
    sheet::{Sheet, SheetSide},
};

pub const LEPTOS_OUTPUT_NAME: &str = env!("LEPTOS_OUTPUT_NAME");

/// The documented leptonic version, shown in the app bar.
const VERSION_LABEL: &str = "v0.6.0 (main)";

const GITHUB_URL: &str = "https://github.com/lpotthast/leptonic";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                // Collects uncaught page errors (and Rust panic messages, which the panic hook logs before the wasm
                // traps), so that the browser tests (`tests/browser_test.rs`) can fail on them.
                <script>
                    "window.__pageErrors = [];
                    window.addEventListener('error', e => window.__pageErrors.push(String(e.message)));
                    window.addEventListener('unhandledrejection', e => window.__pageErrors.push(String(e.reason)));
                    const consoleError = console.error.bind(console);
                    console.error = (...args) => {
                        const message = args.map(String).join(' ');
                        if (message.includes('panicked at')) window.__pageErrors.push(message);
                        consoleError(...args);
                    };"
                </script>
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
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Meta name="description" content="Leptonic"/>
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

/// Responsive layout state shared by the app shell and the documentation layout.
///
/// On large screens, the documentation navigation is a sidebar next to doc pages and the app bar shows every link. On
/// small screens (`is_small`), both are menus covering the page ([`Sheet`]s), opened through the app bar.
#[derive(Debug, Clone, Copy)]
pub struct AppLayoutContext {
    pub is_small: Signal<bool>,
    pub doc_menu_open: RwSignal<bool>,
    pub main_menu_open: RwSignal<bool>,
}

impl AppLayoutContext {
    fn close_menus(&self) {
        self.doc_menu_open.set(false);
        self.main_menu_open.set(false);
    }
}

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let is_small = use_media_query("(max-width: 800px)");
    let location = use_location();
    let is_doc = Memo::new(move |_| location.pathname.get().starts_with("/doc"));

    let ctx = AppLayoutContext {
        is_small,
        doc_menu_open: RwSignal::new(false),
        main_menu_open: RwSignal::new(false),
    };
    provide_context(ctx);

    // The menus only exist on small screens, and close once the user navigated.
    Effect::watch(
        move || is_small.get(),
        move |_, _, _| ctx.close_menus(),
        false,
    );
    Effect::watch(
        move || location.pathname.get(),
        move |_, _, _| ctx.close_menus(),
        false,
    );

    let logo = move || {
        view! {
            <Link href=routes::Root.materialize()>
                <img src="/res/leptonic.svg" id="book-logo" alt="Leptonic logo"/>
            </Link>
        }
    };

    view! {
        <AppBar attr:id="book-app-bar" height=APP_BAR_HEIGHT>
            <div id="book-app-bar-content">
                <Stack orientation=StackOrientation::Horizontal spacing=CssDimension::Zero>
                    {move || match (is_doc.get(), is_small.get()) {
                        (false, true) => logo().into_any(),
                        (true, true) => view! {
                            <MenuButton label="Documentation menu" icon=icondata::BsList open=ctx.doc_menu_open/>
                            {logo}
                        }.into_any(),
                        (_, false) => view! {
                            {logo}
                            <Link href=routes::Doc.materialize() classes="docs-link">"Docs"</Link>
                        }.into_any(),
                    }}
                </Stack>

                <Stack orientation=StackOrientation::Horizontal spacing=em(1.0)>
                    <DocSearch/>
                    {move || if is_small.get() {
                        view! {
                            <MenuButton label="Menu" icon=icondata::BsThreeDots open=ctx.main_menu_open/>
                        }.into_any()
                    } else {
                        view! {
                            <Link href=routes::doc::Changelog.materialize()>{VERSION_LABEL}</Link>
                            <GithubLink/>
                            <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark classes="app-bar-theme-toggle"/>
                        }.into_any()
                    }}
                </Stack>
            </div>
        </AppBar>

        <main id="book-content">{children()}</main>

        <Sheet
            is_open=Signal::derive(move || is_small.get() && ctx.main_menu_open.get())
            on_close=move |()| ctx.main_menu_open.set(false)
            label="Menu"
            side=SheetSide::Right
        >
            <div class="book-main-menu">
                <GithubLink/>
                <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark/>
                <Link href=routes::doc::Changelog.materialize()>{VERSION_LABEL}</Link>
            </div>
        </Sheet>
    }
}

/// An app bar button opening a menu [`Sheet`].
#[component]
fn MenuButton(label: &'static str, icon: icondata::Icon, open: RwSignal<bool>) -> impl IntoView {
    view! {
        <Button
            on_press=move |_| open.set(true)
            variant=ButtonVariant::Flat
            aria_haspopup=Some(AriaHasPopup::Dialog)
            aria_expanded=Signal::derive(move || Some(AriaExpanded::from(open.get())))
            classes="book-icon-button"
            attr:aria-label=label
        >
            <Icon icon/>
        </Button>
    }
}

#[component]
fn GithubLink() -> impl IntoView {
    view! {
        <LinkExt href=GITHUB_URL target=LinkTarget::_Blank classes="github-link">
            <Icon icon=icondata::BsGithub aria_label="Leptonic on GitHub"/>
        </LinkExt>
    }
}
