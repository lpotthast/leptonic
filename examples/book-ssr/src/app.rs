use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;
use leptos_meta::{
    provide_meta_context, Link as MetaLink, Meta, MetaTags, Style, Stylesheet, Title,
};
use leptos_router::components::*;
use leptos_router::hooks::use_location;
use leptos_router::{AsPath, StaticSegment};
use leptos_use::use_media_query;

use crate::pages::documentation::doc_root::doc_routes;
use crate::pages::{editor::ThemeEditor, err404::PageErr404, welcome::PageWelcome};

pub const LEPTOS_OUTPUT_NAME: &str = env!("LEPTOS_OUTPUT_NAME");

/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo.render() ...>` definitions.
pub trait SegmentRenderer {
    fn to_href(&self) -> String;
}

// AppRoutes::Doc.route()

impl SegmentRenderer for () {
    fn to_href(&self) -> String {
        "".to_string()
    }
}

impl SegmentRenderer for (StaticSegment<&'static str>,) {
    fn to_href(&self) -> String {
        let (a,) = self;
        let a = a.0.as_path();
        format!("{a}")
    }
}

impl SegmentRenderer for (StaticSegment<&'static str>, StaticSegment<&'static str>) {
    fn to_href(&self) -> String {
        let (a, b) = self;
        let a = a.0.as_path();
        let b = b.0.as_path();
        format!("{a}/{b}")
    }
}

pub mod app_routes {
    use leptos_router::{path, StaticSegment};

    pub const WELCOME: () = path!("");
    pub const DOC: (StaticSegment<&str>,) = path!("doc");
    pub const THEME_EDITOR: (StaticSegment<&str>,) = path!("theme-editor");
    pub const NOT_FOUND: (StaticSegment<&str>,) = path!("not-found");
}

/*
/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo ...>` definitions.
impl ToHref for AppRoutes {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(move || format!("/{}", self.route()))
    }
}
*/

// Sourced from: https://fonts.googleapis.com/css?family=Roboto&display=swap
const FONT: &'static str = r#"
/* cyrillic-ext */
@font-face {
    font-family: 'Roboto';
    font-style: normal;
    font-weight: 400;
    font-display: swap;
    src: url(font/roboto/v30/KFOmCnqEu92Fr1Mu72xKOzY.woff2) format('woff2');
    unicode-range: U+0460-052F, U+1C80-1C88, U+20B4, U+2DE0-2DFF, U+A640-A69F, U+FE2E-FE2F;
}

/* cyrillic */
@font-face {
    font-family: 'Roboto';
    font-style: normal;
    font-weight: 400;
    font-display: swap;
    src: url(font/roboto/v30/KFOmCnqEu92Fr1Mu5mxKOzY.woff2) format('woff2');
    unicode-range: U+0301, U+0400-045F, U+0490-0491, U+04B0-04B1, U+2116;
}

/* greek-ext */
@font-face {
    font-family: 'Roboto';
    font-style: normal;
    font-weight: 400;
    font-display: swap;
    src: url(font/roboto/v30/KFOmCnqEu92Fr1Mu7mxKOzY.woff2) format('woff2');
    unicode-range: U+1F00-1FFF;
}

/* greek */
@font-face {
    font-family: 'Roboto';
    font-style: normal;
    font-weight: 400;
    font-display: swap;
    src: url(font/roboto/v30/KFOmCnqEu92Fr1Mu4WxKOzY.woff2) format('woff2');
    unicode-range: U+0370-03FF;
}

/* vietnamese */
@font-face {
    font-family: 'Roboto';
    font-style: normal;
    font-weight: 400;
    font-display: swap;
    src: url(font/roboto/v30/KFOmCnqEu92Fr1Mu7WxKOzY.woff2) format('woff2');
    unicode-range: U+0102-0103, U+0110-0111, U+0128-0129, U+0168-0169, U+01A0-01A1, U+01AF-01B0, U+0300-0301, U+0303-0304, U+0308-0309, U+0323, U+0329, U+1EA0-1EF9, U+20AB;
}

/* latin-ext */
@font-face {
    font-family: 'Roboto';
    font-style: normal;
    font-weight: 400;
    font-display: swap;
    src: url(font/roboto/v30/KFOmCnqEu92Fr1Mu7GxKOzY.woff2) format('woff2');
    unicode-range: U+0100-02AF, U+0304, U+0308, U+0329, U+1E00-1E9F, U+1EF2-1EFF, U+2020, U+20A0-20AB, U+20AD-20CF, U+2113, U+2C60-2C7F, U+A720-A7FF;
}

/* latin */
@font-face {
    font-family: 'Roboto';
    font-style: normal;
    font-weight: 400;
    font-display: swap;
    src: url(font/roboto/v30/KFOmCnqEu92Fr1Mu4mxK.woff2) format('woff2');
    unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}
"#;

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

    let router = view! {
        <Router>
            <Routes
                //fallback=|| "Page not found"
                fallback=move || view! {
                    <Layout>
                        <PageErr404/>
                    </Layout>
                }
            >
                //<ParentRoute path=path!("/") view=move || view! { <Layout/> }.into_any()>
                    <Route path=app_routes::WELCOME view=PageWelcome/>
                    //<DocRoutes path=app_routes::DOC/>
                    <Route path=app_routes::THEME_EDITOR view=ThemeEditor/>
                    <Route path=app_routes::NOT_FOUND view=PageErr404/>
                //</ParentRoute>
            </Routes>
        </Router>
    };

    view! {
        <Meta name="description" content="Leptonic"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Stylesheet id="leptos" href=format!("/pkg/{LEPTOS_OUTPUT_NAME}.css")/>
        <Style>
            { FONT }
        </Style>

        <MetaLink rel="icon" href="/res/icon/leptonic_x64.png"/>
        <MetaLink rel="apple-touch-icon" href="/res/icon/maskable_icon_x192.png"/>

        <Title text="Leptonic"/>

        <Root default_theme=LeptonicTheme::default()>
            { router }
        </Root>
    }
}

pub const APP_BAR_HEIGHT: Height = Height::Em(3.5);

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
#[allow(clippy::too_many_lines)]
pub fn Layout(#[prop(optional)] children: Option<Children>) -> impl IntoView {
    let is_small = use_media_query("(max-width: 800px)");
    let is_medium = use_media_query("(max-width: 1200px)");

    let location = use_location();

    let is_doc = Memo::new(move |_| location.pathname.get().starts_with("/doc"));

    // The main drawer is only used on mobile / small screens!.
    let (main_drawer_closed, set_main_drawer_closed) = signal(true);
    let (doc_drawer_closed, set_doc_drawer_closed) = signal(false);

    // Make sure the doc_drawer is closed whenever we leave a documentation route.
    Effect::new(move |_| {
        if !is_doc.get() {
            set_doc_drawer_closed.set(true);
        } else {
            if !is_small.get() {
                set_doc_drawer_closed.set(false);
            }
        }
    });

    // Always close the doc-drawer when the application is now small.
    // Always open the doc-drawer when the application is no longer small.
    Effect::new(move |_| {
        if is_small.get() {
            set_doc_drawer_closed.set(true);
        } else {
            set_doc_drawer_closed.set(false);
        }
    });

    // Always close the main-drawer when the application is no longer small.
    Effect::new(move |_| {
        if !is_small.get() {
            set_main_drawer_closed.set(true);
        }
    });

    let ctx = AppLayoutContext {
        is_small,
        is_medium,
        main_drawer_closed: main_drawer_closed.into(),
        set_main_drawer_closed,
        doc_drawer_closed: doc_drawer_closed.into(),
        set_doc_drawer_closed,
    };

    provide_context(ctx);

    let search_options = vec![
        create_search_option(doc_routes::OVERVIEW.to_href(), "Overview"),
        create_search_option(doc_routes::INSTALLATION.to_href(), "Installation"),
        create_search_option(doc_routes::THEMES.to_href(), "Themes"),
        create_search_option(doc_routes::CHANGELOG.to_href(), "Changelog"),
        create_search_option(doc_routes::STACK.to_href(), "Grid"),
        create_search_option(doc_routes::SEPARATOR.to_href(), "Separator"),
        create_search_option(doc_routes::SKELETON.to_href(), "App Bar"),
        create_search_option(doc_routes::DRAWER.to_href(), "Drawer"),
        create_search_option(doc_routes::TAB.to_href(), "Tabs"),
        create_search_option(doc_routes::TABLE.to_href(), "Table"),
        create_search_option(doc_routes::COLLAPSIBLE.to_href(), "Collapsible"),
        create_search_option(doc_routes::BUTTON.to_href(), "Button"),
        create_search_option(doc_routes::INPUT.to_href(), "Input"),
        create_search_option(doc_routes::TIPTAP_EDITOR.to_href(), "Tiptap Editor"),
        create_search_option(doc_routes::DATETIME.to_href(), "Date & Time"),
        create_search_option(doc_routes::SLIDER.to_href(), "Slider"),
        create_search_option(doc_routes::SELECT.to_href(), "Select"),
        create_search_option(doc_routes::COLOR_PICKER.to_href(), "Color Picker"),
        create_search_option(doc_routes::ALERT.to_href(), "Alert"),
        create_search_option(doc_routes::TOAST.to_href(), "Toast"),
        create_search_option(doc_routes::MODAL.to_href(), "Modal"),
        create_search_option(doc_routes::PROGRESS.to_href(), "Progress"),
        create_search_option(doc_routes::POPOVER.to_href(), "Popover"),
        create_search_option(doc_routes::CHIP.to_href(), "Chip"),
        create_search_option(doc_routes::KBD.to_href(), "Keyboard"),
        create_search_option(doc_routes::TYPOGRAPHY.to_href(), "Typography"),
        create_search_option(doc_routes::ICON.to_href(), "Icon"),
        create_search_option(doc_routes::LINK.to_href(), "Link"),
        create_search_option(doc_routes::CALLBACK.to_href(), "Callback"),
        //create_search_option(doc_routes::TRANSITION.to_href(), "Transition"),
    ];

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
                <Stack attr:id="left" orientation=StackOrientation::Horizontal spacing=Size::Zero>
                    { move || match (is_doc.get(), is_small.get()) {
                        (false, true) => logo().into_any(),
                        (true, true) => view! {
                            <Icon attr:id="mobile-menu-trigger" icon=icondata::BsList on:click=move |_| ctx.toggle_doc_drawer()/>
                            { logo }
                        }.into_any(),
                        (_, false) => view! {
                            { logo }
                            <Link href=app_routes::DOC.to_href()>
                                <h3 style="margin: 0 0 0 0.5em">
                                    "Docs"
                                </h3>
                            </Link>
                            //<Link href=AppRoutes::ThemeEditor>
                            //    <H3 style="margin: 0 0 0 0.5em">
                            //        "Theme Editor"
                            //    </h3>
                            //</Link>
                        }.into_any(),
                    } }
                </Stack>

                <Stack attr:id="center" orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                    <Quicksearch
                        attr:id="quicksearch"
                        trigger=move |set_quicksearch| view! {
                            <QuicksearchTrigger attr:id="quicksearch-trigger" set_quicksearch=set_quicksearch>
                                { move || match is_small.get() {
                                    true => view! { <Icon icon=icondata::BsSearch />}.into_any(),
                                    false => view! { "Search"}.into_any(),
                                } }
                            </QuicksearchTrigger>
                        }
                        query=move |search: String| {
                            if search.is_empty() {
                                return vec![];
                            }
                            let lower_search = search.to_lowercase();
                            search_options.iter()
                                .filter(|it| it.label.to_lowercase().contains(&lower_search))
                                .cloned()
                                .collect::<Vec<_>>()
                        }
                    />
                </Stack>

                <Stack attr:id="right" orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                    { move || match is_small.get() {
                        true => view! {
                            <Icon attr:id="mobile-menu-trigger" icon=icondata::BsThreeDots on:click=move |_| ctx.toggle_main_drawer()/>
                        }.into_any(),
                        false => view! {
                            <Link href=doc_routes::CHANGELOG.to_href()>"v0.6.0 (main)"</Link>

                            <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>
                                <Icon attr:id="github-icon" icon=icondata::BsGithub aria_label="GitHub icon"/>
                            </LinkExt>

                            <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark attr:style="margin-right: 1em"/>
                        }.into_any(),
                    } }
                </Stack>
            </div>
        </AppBar>

        <Box
            attr:id="content"
            attr:aria-hidden=move || { ((is_doc.get() && is_small.get() && !doc_drawer_closed.get()) || !main_drawer_closed.get()).to_string() }
        >
            {
                match children {
                    Some(children) => {
                        children()
                    },
                    None => view! {
                        // <Outlet/> will show nested child routes.
                        <Outlet/>
                    }.into_any(),
                }
            }

            <Drawer
                attr:id="main-drawer"
                attr:style=format!("top: {APP_BAR_HEIGHT}")
                shown=Signal::derive(move || !main_drawer_closed.get())
                side=DrawerSide::Right
            >
                <Stack orientation=StackOrientation::Vertical spacing=Size::Em(2.0) attr:class="menu">

                    <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank attr:style="font-size: 3em;">
                        <Icon attr:id="github-icon" icon=icondata::BsGithub/>
                    </LinkExt>

                    <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark attr:style="margin-right: 1em"/>

                    "Currently - v0.6.0 (main)"
                </Stack>
            </Drawer>
        </Box>
    }
}

fn create_search_option(
    route: impl ToHref + Send + Sync + Clone + 'static,
    label: &'static str,
) -> QuicksearchOption {
    QuicksearchOption {
        label: label.into(),
        view: ViewProducer::new(move || {
            {
                view! {
                    <Link href=route.clone() attr:class="search-link">
                        { label }
                    </Link>
                }
            }
            .into_any()
        }),
        on_select: Callback::new(|()| ()),
    }
}
