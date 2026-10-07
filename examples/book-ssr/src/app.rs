use std::time::Duration;

use leptonic::{
    atoms::prelude::{
        AnchorLink, Button, Dialog, LeptonicTheme, Link, ModalBackdrop, ModalContent, Switch,
        Theme, ThemeProvider, Toast, ToastCloseButton, ToastContent, ToastDescription, ToastRegion,
        ToastTitle, VisuallyHidden, use_theme,
    },
    components::prelude::{Leptonic, ToastRoot},
    hooks::{
        IntoAttrs, LandmarkController, LandmarkRole, LinkTarget, ToastOptions, ToastQueue,
        UseLandmarkInput, use_landmark,
    },
    signal_ls,
    utils::{
        CapturedElement,
        aria::{AriaExpanded, AriaHasPopup},
        focus::focus_element,
    },
};
use leptos::prelude::*;
use leptos_meta::{Link as MetaLink, Meta, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{components::Router, hooks::use_location};
use leptos_use::{use_document, use_media_query, use_window};

use crate::{kit::Icon, pages::documentation::doc_search::DocSearch, routes};

pub const LEPTOS_OUTPUT_NAME: &str = env!("LEPTOS_OUTPUT_NAME");

/// The documented leptonic version, shown in the app bar.
const VERSION_LABEL: &str = "v0.6.0 (main)";

const GITHUB_URL: &str = "https://github.com/lpotthast/leptonic";

/// Describes the book where a page has no description of its own (search engines, link previews).
pub const SITE_DESCRIPTION: &str = "Leptonic: accessible UI building blocks for Leptos \u{2014} hooks, atoms and themed components.";

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

    // The reader's theme, remembered in the browser.
    let (theme, set_theme) = signal_ls("theme", LeptonicTheme::default());
    let toasts = BookToasts(ToastQueue::new(None));
    provide_context(toasts);

    view! {
        <Meta name="theme-color" content="#e66956"/>

        <Stylesheet id="leptos" href=format!("/pkg/{LEPTOS_OUTPUT_NAME}.css")/>

        <MetaLink rel="icon" href="/res/icon/leptonic_x64.png"/>
        <MetaLink rel="apple-touch-icon" href="/res/icon/maskable_icon_x192.png"/>

        // Fallback; every page sets its own title (and description).
        <Title text="Leptonic"/>

        <ThemeProvider theme set_theme>
            <ComponentDemoContexts>
                <Router>
                    <Layout>
                        { routes::route_tree() }
                    </Layout>
                </Router>
            </ComponentDemoContexts>
            <BookToastRegion toasts/>
        </ThemeProvider>
    }
}

/// What the demos of leptonic's themed components (shown until the book moves them onto atoms) read from the
/// components' `Root`: the `Toasts` of a `ToastRoot`, and the `Leptonic` context (`Select` asks whether the device is
/// a desktop). The book itself doesn't use them.
#[component]
fn ComponentDemoContexts(children: Children) -> impl IntoView {
    let is_mobile_device = Signal::derive(|| {
        use_window().as_ref().is_some_and(|window| {
            window
                .navigator()
                .user_agent()
                .is_ok_and(|agent| agent.to_lowercase().contains("mobi"))
        })
    });
    provide_context(Leptonic {
        is_mobile_device,
        is_desktop_device: Signal::derive(move || !is_mobile_device.get()),
    });
    view! { <ToastRoot>{children()}</ToastRoot> }
}

/// A toast of the book: a title and a sentence.
#[derive(Debug, Clone)]
pub struct BookToast {
    pub title: String,
    pub description: String,
}

/// The book's toasts, shown at the bottom of the window. Pages show one with [`BookToasts::show`].
#[derive(Clone, Copy)]
pub struct BookToasts(ToastQueue<BookToast>);

impl BookToasts {
    /// Shows `toast` for a few seconds.
    pub fn show(self, toast: BookToast) {
        self.0.add(
            toast,
            ToastOptions {
                timeout: Some(Duration::from_secs(5)),
                ..ToastOptions::default()
            },
        );
    }
}

/// The region of the book's toasts (leptonic's toast atoms), rendered at the end of the page while there are toasts.
#[component]
fn BookToastRegion(toasts: BookToasts) -> impl IntoView {
    view! {
        <ToastRegion queue=toasts.0 classes="book-toast-region" let:toast>
            <Toast toast=toast.clone() classes="book-toast">
                <ToastContent classes="book-toast-content">
                    <ToastTitle classes="book-toast-title">{toast.content.title.clone()}</ToastTitle>
                    <ToastDescription>{toast.content.description.clone()}</ToastDescription>
                </ToastContent>
                <ToastCloseButton classes="book-icon-button">
                    <Icon icon=icondata::BsXLg/>
                </ToastCloseButton>
            </Toast>
        </ToastRegion>
    }
}

/// Responsive layout state shared by the app shell and the documentation layout.
///
/// On large screens, the documentation navigation is a sidebar next to doc pages and the app bar shows every link. On
/// small screens (`is_small`), both are menus covering the page ([`MenuDrawer`]s), opened through the app bar.
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
        <AppBar>
            <div class="book-app-bar-group">
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
            </div>

            <div class="book-app-bar-group book-app-bar-end">
                <DocSearch/>
                {move || if is_small.get() {
                    view! {
                        <MenuButton label="Menu" icon=icondata::BsThreeDots open=ctx.main_menu_open/>
                    }.into_any()
                } else {
                    view! {
                        <Link href=routes::doc::Changelog.materialize() classes="book-version-link">{VERSION_LABEL}</Link>
                        <GithubLink/>
                        <ThemeToggle/>
                    }.into_any()
                }}
            </div>
        </AppBar>

        <div id="book-page">{children()}</div>

        <MenuDrawer
            side=MenuSide::Right
            label="Menu"
            is_open=Signal::derive(move || is_small.get() && ctx.main_menu_open.get())
            open=ctx.main_menu_open
        >
            <nav class="book-main-menu" aria-label="Main">
                <Link href=routes::Doc.materialize() classes="book-docs-link">"Docs"</Link>
                <Link href=routes::doc::Changelog.materialize() classes="book-version-link">{VERSION_LABEL}</Link>
                <GithubLink/>
                <ThemeToggle/>
            </nav>
        </MenuDrawer>
    }
}

/// The app bar: the page's `<header>`, registered as its banner landmark (F6 reaches it), fixed at the top.
#[component]
fn AppBar(children: Children) -> impl IntoView {
    let element = CapturedElement::new();
    let landmark = use_landmark(UseLandmarkInput {
        element,
        role: LandmarkRole::Banner,
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        focus: None,
    });

    view! {
        <header {..landmark.props.into_attrs()} {..element.attr()} id="book-app-bar">
            {children()}
        </header>
    }
}

/// "Skip to content": the first focusable element of every page, visible only while focused. Moves focus to the
/// page's `<main>`, so that the next Tab continues in the content.
#[component]
fn SkipLink() -> impl IntoView {
    let focus_main = move |_| {
        // The main landmark takes the focus as with Alt + F6; before hydration, the link's `#book-main` scrolls there.
        if LandmarkController::new().focus_main() {
            return;
        }
        if let Some(main) = use_document()
            .as_ref()
            .and_then(|document| document.get_element_by_id(MAIN_ID))
        {
            focus_element(&main, false);
        }
    };

    view! {
        <VisuallyHidden is_focusable=true classes="book-skip-link">
            <AnchorLink href=format!("#{MAIN_ID}") scroll_behavior=None on_press=focus_main>
                "Skip to content"
            </AnchorLink>
        </VisuallyHidden>
    }
}

/// An app bar button opening a [`MenuDrawer`].
#[component]
fn MenuButton(label: &'static str, icon: icondata::Icon, open: RwSignal<bool>) -> impl IntoView {
    view! {
        <Button
            on_press=move |_| open.set(true)
            aria_haspopup=Some(AriaHasPopup::Dialog)
            aria_expanded=Signal::derive(move || Some(AriaExpanded::from(open.get())))
            aria_label=label
            classes="book-icon-button"
        >
            <Icon icon/>
        </Button>
    }
}

/// The side of the screen a [`MenuDrawer`] slides in from. Physical, also in right-to-left pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuSide {
    Left,
    Right,
}

/// A menu of small screens: a modal dialog covering the page from one side (leptonic's modal atoms), with a close
/// button at its top. Escape and a press outside close it; the focus stays inside while it is open and returns to
/// the button that opened it. It slides in and out (`data-entering`/`data-exiting`, see `_shell.scss`).
#[component]
pub fn MenuDrawer(
    side: MenuSide,
    /// The dialog's name.
    label: &'static str,
    #[prop(into)] is_open: Signal<bool>,
    /// The open state the menu's own controls set (closing).
    open: RwSignal<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let side = match side {
        MenuSide::Left => "book-menu-left",
        MenuSide::Right => "book-menu-right",
    };
    let children = StoredValue::new(children);
    view! {
        <ModalBackdrop is_open set_open=open is_dismissable=true classes="book-menu-backdrop">
            <ModalContent classes=["book-menu", side]>
                <Dialog aria_label=label classes="book-menu-dialog">
                    <div class="book-menu-header">
                        <Button
                            on_press=move |_| open.set(false)
                            aria_label="Close menu"
                            classes="book-icon-button"
                        >
                            <Icon icon=icondata::BsXLg/>
                        </Button>
                    </div>
                    {children.get_value()()}
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
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

/// Switches between the light and the dark theme (leptonic's `Switch` atom on the theme of the `ThemeProvider`),
/// showing the sun or the moon in its knob.
#[component]
fn ThemeToggle() -> impl IntoView {
    let theme = use_theme::<LeptonicTheme>()
        .expect("the book renders the theme toggle inside its `ThemeProvider`");
    let is_dark = Signal::derive(move || theme.theme().get() == LeptonicTheme::Dark);
    let set_dark = move |dark: bool| {
        theme.set_theme(if dark {
            LeptonicTheme::Dark
        } else {
            LeptonicTheme::Light
        });
    };

    view! {
        <Switch is_selected=is_dark set_selected=set_dark aria_label="Dark theme" classes="book-theme-toggle">
            <span class="book-theme-toggle-track" aria-hidden="true">
                <span class="book-theme-toggle-knob">
                    {move || {
                        let icon = if is_dark.get() { icondata::BsMoon } else { icondata::BsSun };
                        view! { <Icon icon/> }
                    }}
                </span>
            </span>
        </Switch>
    }
}

/// A page's `<main>` (with the id [`MAIN_ID`]), registered as the main landmark: F6 and Shift + F6 move between the
/// landmarks, Alt + F6 jumps here.
#[component]
pub fn MainLandmark(class: &'static str, children: Children) -> impl IntoView {
    let element = CapturedElement::new();
    let landmark = use_landmark(UseLandmarkInput {
        element,
        role: LandmarkRole::Main,
        aria_label: MaybeProp::default(),
        aria_labelledby: None,
        focus: None,
    });

    view! {
        <main {..landmark.props.into_attrs()} {..element.attr()} id=MAIN_ID class=class>
            {children()}
        </main>
    }
}

/// A `<nav>` of the shell, registered as a navigation landmark named `label`.
#[component]
pub fn NavLandmark(
    label: &'static str,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let element = CapturedElement::new();
    let landmark = use_landmark(UseLandmarkInput {
        element,
        role: LandmarkRole::Navigation,
        aria_label: label.into(),
        aria_labelledby: None,
        focus: None,
    });

    view! {
        <nav {..landmark.props.into_attrs()} {..element.attr()} id=id class=class>
            {children()}
        </nav>
    }
}
