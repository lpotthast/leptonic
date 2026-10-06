use leptonic::{
    atoms::prelude::{AnchorLink as AnchorLinkAtom, VisuallyHidden},
    components::prelude::*,
    hooks::LinkTarget,
    prelude::*,
    utils::{css::em, focus::focus_element},
};
use leptos::prelude::*;
use leptos_meta::{Link as MetaLink, Meta, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{components::Router, hooks::use_location};
use leptos_use::{use_document, use_media_query};

use crate::{
    pages::documentation::doc_search::DocSearch,
    routes,
    sheet::{Sheet, SheetSide},
};

pub const LEPTOS_OUTPUT_NAME: &str = env!("LEPTOS_OUTPUT_NAME");

/// The documented leptonic version, shown in the app bar.
const VERSION_LABEL: &str = "v0.6.0 (main)";

const GITHUB_URL: &str = "https://github.com/lpotthast/leptonic";

/// Describes the book where a page has no description of its own (search engines, link previews).
pub const SITE_DESCRIPTION: &str =
    "Leptonic: accessible UI building blocks for Leptos \u{2014} hooks, atoms and themed components.";

/// Id of every page's `<main>`, the target of the skip link.
pub const MAIN_ID: &str = "book-main";

/// The widest screen with the small-screen layout: the navigation and the app bar links become menus.
///
/// Coupled with `$small` in `style/book/_theme.scss`, which switches the layout in CSS: change both together.
pub const SMALL_SCREEN_MAX_WIDTH: &str = "800px";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        // The theme provider mirrors the current theme onto `<html>` once hydrated. Until then, the server's default
        // theme applies, so that the page background (on `<body>`, outside the provider) has its theme colors from
        // the start.
        <html lang="en" data-theme=LeptonicTheme::default().name()>
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
                // The weights of the style guide: 400 text, 600 emphasis, 700 headings, and italic text.
                <link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,400;0,600;0,700;1,400&display=swap" rel="stylesheet"/>
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
        <Meta name="theme-color" content="#e66956"/>

        <Stylesheet id="leptos" href=format!("/pkg/{LEPTOS_OUTPUT_NAME}.css")/>

        <MetaLink rel="icon" href="/res/icon/leptonic_x64.png"/>
        <MetaLink rel="apple-touch-icon" href="/res/icon/maskable_icon_x192.png"/>

        // Fallback; every page sets its own title (and description).
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

/// The shell around every page: the skip link, the app bar (the page's `<header>`) and the main menu of small
/// screens. Pages render their own `<main id="book-main">` (see [`MAIN_ID`]), so that it holds only their content.
#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let is_small = use_media_query(format!("(width <= {SMALL_SCREEN_MAX_WIDTH})"));
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
            <Link href=routes::Root.materialize() classes="book-logo-link">
                <img src="/res/leptonic.svg" id="book-logo" alt="Leptonic start page"/>
            </Link>
        }
    };

    view! {
        // The app bar (a `<header>`) is layered by leptonic's theme; the skip link shows inside it while focused.
        <AppBar attr:id="book-app-bar">
            <div id="book-app-bar-content">
                <Stack orientation=StackOrientation::Horizontal spacing=em(0.5)>
                    <SkipLink/>
                    {move || match (is_doc.get(), is_small.get()) {
                        (false, true) => logo().into_any(),
                        (true, true) => view! {
                            <MenuButton label="Documentation menu" icon=icondata::BsList open=ctx.doc_menu_open/>
                            {logo}
                        }.into_any(),
                        (_, false) => view! {
                            {logo}
                            <Link href=routes::Doc.materialize() classes="book-docs-link">"Docs"</Link>
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
                            <Link href=routes::doc::Changelog.materialize() classes="book-version-link">{VERSION_LABEL}</Link>
                            <GithubLink/>
                            <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark classes="book-theme-toggle"/>
                        }.into_any()
                    }}
                </Stack>
            </div>
        </AppBar>

        <div id="book-page">{children()}</div>

        <Sheet
            is_open=Signal::derive(move || is_small.get() && ctx.main_menu_open.get())
            on_close=move |()| ctx.main_menu_open.set(false)
            label="Menu"
            side=SheetSide::Right
        >
            <nav class="book-main-menu" aria-label="Main">
                <Link href=routes::Doc.materialize() classes="book-docs-link">"Docs"</Link>
                <Link href=routes::doc::Changelog.materialize() classes="book-version-link">{VERSION_LABEL}</Link>
                <GithubLink/>
                <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark classes="book-theme-toggle"/>
            </nav>
        </Sheet>
    }
}

/// "Skip to content": the first focusable element of every page, visible only while focused. Moves focus to the
/// page's `<main>`, so that the next Tab continues in the content.
#[component]
fn SkipLink() -> impl IntoView {
    let focus_main = move |_| {
        if let Some(main) = use_document()
            .as_ref()
            .and_then(|document| document.get_element_by_id(MAIN_ID))
        {
            focus_element(&main, false);
        }
    };

    view! {
        <VisuallyHidden is_focusable=true classes="book-skip-link">
            <AnchorLinkAtom href=format!("#{MAIN_ID}") scroll_behavior=None on_press=focus_main>
                "Skip to content"
            </AnchorLinkAtom>
        </VisuallyHidden>
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
        <Link href=GITHUB_URL target=LinkTarget::Blank classes="book-github-link">
            <Icon icon=icondata::BsGithub aria_label="Leptonic on GitHub"/>
        </Link>
    }
}
