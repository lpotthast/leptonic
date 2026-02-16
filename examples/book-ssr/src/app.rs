use leptonic::{components::prelude::*, hooks::LinkTarget, prelude::*};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link as MetaLink, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{components::*, hooks::use_location, path};
use leptos_use::use_media_query;

use crate::{
    pages::{
        documentation::{
            atoms::{
                anchor_link::PageAtomAnchorLink, button::PageAtomButton,
                dismiss_button::PageAtomDismissButton, focus_ring::PageAtomFocusRing,
                focus_scope::PageAtomFocusScope, grid::PageAtomGrid, link::PageAtomLink,
                popover::PageAtomPopover, slider::PageAtomSlider,
            },
            components::{
                feedback::{
                    alert::PageAlert, chip::PageChip, kbd::PageKbd, modal::PageModal,
                    popover::PagePopover, progress::PageProgress, toast::PageToast,
                },
                general::{callback::PageCallback, icon::PageIcon, typography::PageTypography},
                input::{
                    button::PageButton, checkbox::PageCheckbox, color_picker::PageColorPicker,
                    date_time::PageDateTime, input_field::PageInput, radio::PageRadio,
                    select::PageSelect, slider::PageSlider, tiptap_editor::PageTiptapEditor,
                    toggle::PageToggle,
                },
                layout::{
                    app_bar::PageAppBar, collapsible::PageCollapsible, drawer::PageDrawer,
                    grid::PageGrid, separator::PageSeparator, skeleton::PageSkeleton,
                    stack::PageStack, tab::PageTab, table::PageTable,
                },
            },
            doc_layout::DocLayout,
            getting_started::{
                changelog::PageChangelog,
                event_propagation::PageEventPropagation,
                installation::PageInstallation, overview::PageOverview,
                themes::PageThemes,
            },
            hooks::{
                anchor_link::PageUseAnchorLink, breadcrumbs::PageUseBreadcrumbs,
                button::PageUseButton, checkbox::PageUseCheckboxHook, combobox::PageUseCombobox,
                disclosure::PageUseDisclosure, dnd::PageUseDnd, focus::PageUseFocus,
                focus_manager::PageUseFocusManager, focus_ring::PageUseFocusRing,
                focus_visible::PageUseFocusVisible, focus_within::PageUseFocusWithin,
                focusable::PageUseFocusable, grid::PageUseGrid,
                has_tabbable_child::PageUseHasTabbableChild, hover::PageUseHover,
                interact_outside::PageUseInteractOutside, keyboard::PageUseKeyboard,
                label::PageUseLabel, link::PageUseLink, listbox::PageUseListbox,
                menu::PageUseMenuHook, meter::PageUseMeter, modal::PageUseModalHook,
                overlay::PageUseOverlay,
                popover::PageUsePopoverHook, press::PageUsePress,
                prevent_scroll::PageUsePreventScroll, progress::PageUseProgressBar,
                r#move::PageUseMove, radio::PageUseRadioHook, scroll_wheel::PageUseScrollWheel,
                select::PageUseSelectHook, selection::PageUseSelection,
                separator::PageUseSeparatorHook, slider::PageUseSliderHook,
                switch::PageUseSwitchHook, table::PageUseTableHook, tabs::PageUseTabsHook,
                tag::PageUseTag, text_field::PageUseTextField, toolbar::PageUseToolbar,
                tooltip::PageUseTooltipHook, tree::PageUseTree,
            },
        },
        editor::ThemeEditor,
        err404::PageErr404,
        welcome::PageWelcome,
    },
    routes,
};

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
                            <Route path=routes::doc::EventPropagation.path() view=PageEventPropagation/>

                            <ParentRoute path=routes::doc::Hooks.path() view=||view! { <Outlet/> }>
                                <Route path=routes::doc::hooks::UsePress.path() view=PageUsePress/>
                                <Route path=routes::doc::hooks::UseMove.path() view=PageUseMove/>
                                <Route path=routes::doc::hooks::UseHover.path() view=PageUseHover/>
                                <Route path=routes::doc::hooks::UseFocus.path() view=PageUseFocus/>
                                <Route path=routes::doc::hooks::UseButton.path() view=PageUseButton/>
                                <Route path=routes::doc::hooks::UseOverlay.path() view=PageUseOverlay/>
                                <Route path=routes::doc::hooks::UseAnchorLink.path() view=PageUseAnchorLink/>
                                <Route path=routes::doc::hooks::UseKeyboard.path() view=PageUseKeyboard/>
                                <Route path=routes::doc::hooks::UseInteractOutside.path() view=PageUseInteractOutside/>
                                <Route path=routes::doc::hooks::UseScrollWheel.path() view=PageUseScrollWheel/>
                                <Route path=routes::doc::hooks::UsePreventScroll.path() view=PageUsePreventScroll/>
                                <Route path=routes::doc::hooks::UseFocusWithin.path() view=PageUseFocusWithin/>
                                <Route path=routes::doc::hooks::UseFocusable.path() view=PageUseFocusable/>
                                <Route path=routes::doc::hooks::UseFocusRing.path() view=PageUseFocusRing/>
                                <Route path=routes::doc::hooks::UseFocusVisible.path() view=PageUseFocusVisible/>
                                <Route path=routes::doc::hooks::UseFocusManager.path() view=PageUseFocusManager/>
                                <Route path=routes::doc::hooks::UseHasTabbableChild.path() view=PageUseHasTabbableChild/>
                                <Route path=routes::doc::hooks::Selection.path() view=PageUseSelection/>
                                <Route path=routes::doc::hooks::UseLabel.path() view=PageUseLabel/>
                                <Route path=routes::doc::hooks::UseCheckbox.path() view=PageUseCheckboxHook/>
                                <Route path=routes::doc::hooks::UseRadio.path() view=PageUseRadioHook/>
                                <Route path=routes::doc::hooks::UseTextField.path() view=PageUseTextField/>
                                <Route path=routes::doc::hooks::UseSwitch.path() view=PageUseSwitchHook/>
                                <Route path=routes::doc::hooks::UseSlider.path() view=PageUseSliderHook/>
                                <Route path=routes::doc::hooks::UseModal.path() view=PageUseModalHook/>
                                <Route path=routes::doc::hooks::UseTooltip.path() view=PageUseTooltipHook/>
                                <Route path=routes::doc::hooks::UseMenu.path() view=PageUseMenuHook/>
                                <Route path=routes::doc::hooks::UseListbox.path() view=PageUseListbox/>
                                <Route path=routes::doc::hooks::UseSelect.path() view=PageUseSelectHook/>
                                <Route path=routes::doc::hooks::UseCombobox.path() view=PageUseCombobox/>
                                <Route path=routes::doc::hooks::UseTabs.path() view=PageUseTabsHook/>
                                <Route path=routes::doc::hooks::UseTable.path() view=PageUseTableHook/>
                                <Route path=routes::doc::hooks::Dnd.path() view=PageUseDnd/>
                                <Route path=routes::doc::hooks::UseDisclosure.path() view=PageUseDisclosure/>
                                <Route path=routes::doc::hooks::UseProgressBar.path() view=PageUseProgressBar/>
                                <Route path=routes::doc::hooks::UseBreadcrumbs.path() view=PageUseBreadcrumbs/>
                                <Route path=routes::doc::hooks::UseLink.path() view=PageUseLink/>
                                <Route path=routes::doc::hooks::UseMeter.path() view=PageUseMeter/>
                                <Route path=routes::doc::hooks::UseSeparator.path() view=PageUseSeparatorHook/>
                                <Route path=routes::doc::hooks::UseTag.path() view=PageUseTag/>
                                <Route path=routes::doc::hooks::UseToolbar.path() view=PageUseToolbar/>
                                <Route path=routes::doc::hooks::UseTree.path() view=PageUseTree/>
                                <Route path=routes::doc::hooks::UseGrid.path() view=PageUseGrid/>
                                <Route path=routes::doc::hooks::UsePopover.path() view=PageUsePopoverHook/>
                            </ParentRoute>

                            <ParentRoute path=routes::doc::Atoms.path() view=||view! { <Outlet/> }>
                                <Route path=routes::doc::atoms::Button.path() view=PageAtomButton/>
                                <Route path=routes::doc::atoms::DismissButton.path() view=PageAtomDismissButton/>
                                <Route path=routes::doc::atoms::Popover.path() view=PageAtomPopover/>
                                <Route path=routes::doc::atoms::AnchorLink.path() view=PageAtomAnchorLink/>
                                <Route path=routes::doc::atoms::FocusScope.path() view=PageAtomFocusScope/>
                                <Route path=routes::doc::atoms::FocusRing.path() view=PageAtomFocusRing/>
                                <Route path=routes::doc::atoms::Grid.path() view=PageAtomGrid/>
                                <Route path=routes::doc::atoms::Slider.path() view=PageAtomSlider/>
                                <Route path=routes::doc::atoms::Link.path() view=PageAtomLink/>
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

    // TODO: Use
    let _search_options: Vec<QuicksearchOption> = vec![
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
        create_search_option(routes::doc::atoms::Link.materialize(), "Link"),
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

                            <LinkExt href="https://github.com/lpotthast/leptonic" target=LinkTarget::_Blank>
                                <Icon attr:id="github-icon" icon=icondata::BsGithub aria_label="GitHub icon"/>
                            </LinkExt>

                            <ThemeToggle off=LeptonicTheme::Light on=LeptonicTheme::Dark attr:style="margin-right: 1em"/>
                        }.into_any(),
                    } }
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
                <Stack orientation=StackOrientation::Vertical spacing=Size::Em(2.0) attr:class="menu">

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
