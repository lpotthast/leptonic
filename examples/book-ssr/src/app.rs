use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link as MetaLink, Meta, MetaTags, Stylesheet, Title};
use leptos_router::components::*;
use leptos_router::hooks::use_location;
use leptos_router::path;
use leptos_use::use_media_query;

use crate::pages::documentation::atoms::anchor_link::PageAtomAnchorLink;
use crate::pages::documentation::atoms::button::PageAtomButton;
use crate::pages::documentation::atoms::popover::PageAtomPopover;
use crate::pages::documentation::components::feedback::alert::PageAlert;
use crate::pages::documentation::components::feedback::chip::PageChip;
use crate::pages::documentation::components::feedback::kbd::PageKbd;
use crate::pages::documentation::components::feedback::modal::PageModal;
use crate::pages::documentation::components::feedback::popover::PagePopover;
use crate::pages::documentation::components::feedback::progress::PageProgress;
use crate::pages::documentation::components::feedback::toast::PageToast;
use crate::pages::documentation::components::general::callback::PageCallback;
use crate::pages::documentation::components::general::icon::PageIcon;
use crate::pages::documentation::components::general::link::PageLink;
use crate::pages::documentation::components::general::typography::PageTypography;
use crate::pages::documentation::components::input::button::PageButton;
use crate::pages::documentation::components::input::checkbox::PageCheckbox;
use crate::pages::documentation::components::input::color_picker::PageColorPicker;
use crate::pages::documentation::components::input::date_time::PageDateTime;
use crate::pages::documentation::components::input::input_field::PageInput;
use crate::pages::documentation::components::input::radio::PageRadio;
use crate::pages::documentation::components::input::select::PageSelect;
use crate::pages::documentation::components::input::slider::PageSlider;
use crate::pages::documentation::components::input::tiptap_editor::PageTiptapEditor;
use crate::pages::documentation::components::input::toggle::PageToggle;
use crate::pages::documentation::components::layout::app_bar::PageAppBar;
use crate::pages::documentation::components::layout::collapsible::PageCollapsible;
use crate::pages::documentation::components::layout::drawer::PageDrawer;
use crate::pages::documentation::components::layout::grid::PageGrid;
use crate::pages::documentation::components::layout::separator::PageSeparator;
use crate::pages::documentation::components::layout::skeleton::PageSkeleton;
use crate::pages::documentation::components::layout::stack::PageStack;
use crate::pages::documentation::components::layout::tab::PageTab;
use crate::pages::documentation::components::layout::table::PageTable;
use crate::pages::documentation::doc_layout::DocLayout;
use crate::pages::documentation::getting_started::changelog::PageChangelog;
use crate::pages::documentation::getting_started::installation::PageInstallation;
use crate::pages::documentation::getting_started::overview::PageOverview;
use crate::pages::documentation::getting_started::themes::PageThemes;
use crate::pages::documentation::hooks::anchor_link::PageUseAnchorLink;
use crate::pages::documentation::hooks::button::PageUseButton;
use crate::pages::documentation::hooks::hover::PageUseHover;
use crate::pages::documentation::hooks::overlay::PageUseOverlay;
use crate::pages::documentation::hooks::press::PageUsePress;
use crate::pages::documentation::hooks::r#move::PageUseMove;
use crate::pages::{editor::ThemeEditor, err404::PageErr404, welcome::PageWelcome};
use crate::pages::documentation::hooks::focus::PageUseFocus;
use crate::routes;

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
                    <Routes fallback=PageErr404>
                        <Route path=routes::Root.path() view=PageWelcome/>
                        <ParentRoute path=routes::Doc.path() view=DocLayout>
                            <Route path=path!("/") view=|| view! { <Redirect path=routes::doc::Overview.materialize()/> }/>

                            <Route path=routes::doc::Overview.path() view=PageOverview/>
                            <Route path=routes::doc::Installation.path() view=PageInstallation/>
                            <Route path=routes::doc::Themes.path() view=PageThemes/>
                            <Route path=routes::doc::Changelog.path() view=PageChangelog/>

                            <ParentRoute path=routes::doc::Hooks.path() view=||view! { <Outlet/> }>
                                <Route path=routes::doc::hooks::UsePress.path() view=PageUsePress/>
                                <Route path=routes::doc::hooks::UseMove.path() view=PageUseMove/>
                                <Route path=routes::doc::hooks::UseHover.path() view=PageUseHover/>
                                <Route path=routes::doc::hooks::UseFocus.path() view=PageUseFocus/>
                                <Route path=routes::doc::hooks::UseButton.path() view=PageUseButton/>
                                <Route path=routes::doc::hooks::UseOverlay.path() view=PageUseOverlay/>
                                <Route path=routes::doc::hooks::UseAnchorLink.path() view=PageUseAnchorLink/>
                            </ParentRoute>

                            <ParentRoute path=routes::doc::Atoms.path() view=||view! { <Outlet/> }>
                                <Route path=routes::doc::atoms::Button.path() view=PageAtomButton/>
                                <Route path=routes::doc::atoms::Popover.path() view=PageAtomPopover/>
                                <Route path=routes::doc::atoms::AnchorLink.path() view=PageAtomAnchorLink/>
                            </ParentRoute>

                            <ParentRoute path=routes::doc::Components.path() view=||view! { <Outlet/> }>
                                <Route path=routes::doc::components::Stack.path() view=PageStack/>
                                <Route path=routes::doc::components::Grid.path() view=PageGrid/>
                                <Route path=routes::doc::components::Separator.path() view=PageSeparator/>
                                <Route path=routes::doc::components::Skeleton.path() view=PageSkeleton/>
                                <Route path=routes::doc::components::AppBar.path() view=PageAppBar/>
                                <Route path=routes::doc::components::Drawer.path() view=PageDrawer/>
                                <Route path=routes::doc::components::Tabs.path() view=PageTab/>
                                <Route path=routes::doc::components::Table.path() view=PageTable/>
                                <Route path=routes::doc::components::Collapsible.path() view=PageCollapsible/>

                                <Route path=routes::doc::components::Button.path() view=PageButton/>
                                <Route path=routes::doc::components::Input.path() view=PageInput/>
                                <Route path=routes::doc::components::TiptapEditor.path() view=PageTiptapEditor/>
                                <Route path=routes::doc::components::DateTime.path() view=PageDateTime/>
                                <Route path=routes::doc::components::Slider.path() view=PageSlider/>
                                <Route path=routes::doc::components::Select.path() view=PageSelect/>
                                <Route path=routes::doc::components::Checkbox.path() view=PageCheckbox/>
                                <Route path=routes::doc::components::Radio.path() view=PageRadio/>
                                <Route path=routes::doc::components::Toggle.path() view=PageToggle/>
                                <Route path=routes::doc::components::ColorPicker.path() view=PageColorPicker/>

                                <Route path=routes::doc::components::Alert.path() view=PageAlert/>
                                <Route path=routes::doc::components::Toast.path() view=PageToast/>
                                <Route path=routes::doc::components::Modal.path() view=PageModal/>
                                <Route path=routes::doc::components::Progress.path() view=PageProgress/>
                                <Route path=routes::doc::components::Popover.path() view=PagePopover/>
                                <Route path=routes::doc::components::Chip.path() view=PageChip/>
                                <Route path=routes::doc::components::Kbd.path() view=PageKbd/>

                                <Route path=routes::doc::components::Typography.path() view=PageTypography/>
                                <Route path=routes::doc::components::Icon.path() view=PageIcon/>
                                <Route path=routes::doc::components::Link.path() view=PageLink/>
                                <Route path=routes::doc::components::Callback.path() view=PageCallback/>

                                //<Route path=routes::doc::components::Transition view=PageTransition/>
                            </ParentRoute>
                        </ParentRoute>
                        <Route path=routes::ThemeEditor.path() view=ThemeEditor/>
                        <Route path=routes::NotFound.path() view=PageErr404/>
                    </Routes>
                </Layout>
            </Router>
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
pub fn Layout(children: Children) -> impl IntoView {
    let is_small = use_media_query("(max-width: 800px)");
    let is_medium = use_media_query("(max-width: 1200px)");

    let location = use_location();

    let is_doc = Memo::new(move |_| location.pathname.get().starts_with("/doc"));

    // The main drawer is only used on mobile / small screens!.
    let (main_drawer_closed, set_main_drawer_closed) = signal(true);
    let (doc_drawer_closed, set_doc_drawer_closed) = signal(false);

    /*
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
    */

    let ctx = AppLayoutContext {
        is_small,
        is_medium,
        main_drawer_closed: main_drawer_closed.into(),
        set_main_drawer_closed,
        doc_drawer_closed: doc_drawer_closed.into(),
        set_doc_drawer_closed,
    };

    provide_context(ctx);

    let search_options: Vec<QuicksearchOption> = vec![
        create_search_option(routes::doc::Overview.materialize(), "Overview"),
        create_search_option(routes::doc::Installation.materialize(), "Installation"),
        create_search_option(routes::doc::Themes.materialize(), "Themes"),
        create_search_option(routes::doc::Changelog.materialize(), "Changelog"),
        create_search_option(routes::doc::components::Stack.materialize(), "Grid"),
        create_search_option(
            routes::doc::components::Separator.materialize(),
            "Separator",
        ),
        create_search_option(routes::doc::components::Skeleton.materialize(), "App Bar"),
        create_search_option(routes::doc::components::Drawer.materialize(), "Drawer"),
        create_search_option(routes::doc::components::Tabs.materialize(), "Tabs"),
        create_search_option(routes::doc::components::Table.materialize(), "Table"),
        create_search_option(
            routes::doc::components::Collapsible.materialize(),
            "Collapsible",
        ),
        create_search_option(routes::doc::components::Button.materialize(), "Button"),
        create_search_option(routes::doc::components::Input.materialize(), "Input"),
        create_search_option(
            routes::doc::components::TiptapEditor.materialize(),
            "Tiptap Editor",
        ),
        create_search_option(
            routes::doc::components::DateTime.materialize(),
            "Date & Time",
        ),
        create_search_option(routes::doc::components::Slider.materialize(), "Slider"),
        create_search_option(routes::doc::components::Select.materialize(), "Select"),
        create_search_option(
            routes::doc::components::ColorPicker.materialize(),
            "Color Picker",
        ),
        create_search_option(routes::doc::components::Alert.materialize(), "Alert"),
        create_search_option(routes::doc::components::Toast.materialize(), "Toast"),
        create_search_option(routes::doc::components::Modal.materialize(), "Modal"),
        create_search_option(routes::doc::components::Progress.materialize(), "Progress"),
        create_search_option(routes::doc::components::Popover.materialize(), "Popover"),
        create_search_option(routes::doc::components::Chip.materialize(), "Chip"),
        create_search_option(routes::doc::components::Kbd.materialize(), "Keyboard"),
        create_search_option(
            routes::doc::components::Typography.materialize(),
            "Typography",
        ),
        create_search_option(routes::doc::components::Icon.materialize(), "Icon"),
        create_search_option(routes::doc::components::Link.materialize(), "Link"),
        create_search_option(routes::doc::components::Callback.materialize(), "Callback"),
        //create_search_option(routes::doc::components::Transition.materialize(), "Transition"),
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
                            <Link href=routes::Doc.materialize()>
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

                /*
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
                */

                <Stack attr:id="right" orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                    { move || match is_small.get() {
                        true => view! {
                            <Icon attr:id="mobile-menu-trigger" icon=icondata::BsThreeDots on:click=move |_| ctx.toggle_main_drawer()/>
                        }.into_any(),
                        false => view! {
                            <Link href=routes::doc::Changelog.materialize()>"v0.6.0 (main)"</Link>

                            <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkExtTarget::Blank>
                                <Icon attr:id="github-icon" icon=icondata::BsGithub aria_label="GitHub icon"/>
                            </LinkExt>

                            <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark attr:style="margin-right: 1em"/>
                        }.into_any(),
                    } }
                </Stack>
            </div>
        </AppBar>

        <div
            id="content"
            aria-hidden=move || { ((is_doc.get() && is_small.get() && !doc_drawer_closed.get()) || !main_drawer_closed.get()).to_string() }
        >
            { children() }

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
        </div>
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
