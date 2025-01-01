use std::fmt::Debug;

use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute, Redirect, Route, Routes};
use leptos_router::*;

use crate::app::AppLayoutContext;
use crate::app::SegmentRenderer;
use crate::app::{app_routes, APP_BAR_HEIGHT};
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

pub mod doc_routes {
    use leptos_router::{path, StaticSegment};

    // Getting started
    pub const OVERVIEW: (StaticSegment<&str>,) = path!("overview");
    pub const INSTALLATION: (StaticSegment<&str>,) = path!("installation");
    pub const THEMES: (StaticSegment<&str>,) = path!("themes");
    pub const CHANGELOG: (StaticSegment<&str>,) = path!("changelog");

    // Hooks
    pub const USE_PRESS: (StaticSegment<&str>, StaticSegment<&str>) = path!("hooks/use-press");
    pub const USE_MOVE: (StaticSegment<&str>, StaticSegment<&str>) = path!("hooks/use-move");
    pub const USE_HOVER: (StaticSegment<&str>, StaticSegment<&str>) = path!("hooks/use-hover");
    pub const USE_BUTTON: (StaticSegment<&str>, StaticSegment<&str>) = path!("hooks/use-button");
    pub const USE_OVERLAY: (StaticSegment<&str>, StaticSegment<&str>) = path!("hooks/use-overlay");
    pub const USE_ANCHORLINK: (StaticSegment<&str>, StaticSegment<&str>) =
        path!("hooks/use-anchor-link");

    // Atoms
    pub const ATOM_BUTTON: (StaticSegment<&str>, StaticSegment<&str>) = path!("atoms/button");
    pub const ATOM_POPOVER: (StaticSegment<&str>, StaticSegment<&str>) = path!("atoms/popover");
    pub const ATOM_ANCHORLINK: (StaticSegment<&str>, StaticSegment<&str>) =
        path!("atoms/anchor-link");

    // Layout
    pub const STACK: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/stack");
    pub const GRID: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/grid");
    pub const SEPARATOR: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/separator");
    pub const SKELETON: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/skeleton");
    pub const APP_BAR: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/app-bar");
    pub const DRAWER: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/drawer");
    pub const TAB: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/tabs");
    pub const TABLE: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/table");
    pub const COLLAPSIBLE: (StaticSegment<&str>, StaticSegment<&str>) =
        path!("components/collapsible");

    // Input
    pub const BUTTON: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/button");
    pub const INPUT: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/input");
    pub const TIPTAP_EDITOR: (StaticSegment<&str>, StaticSegment<&str>) =
        path!("components/tiptap-editor");
    pub const DATETIME: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/date-time");
    pub const SLIDER: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/slider");
    pub const SELECT: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/select");
    pub const CHECKBOX: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/checkbox");
    pub const RADIO: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/radio");
    pub const TOGGLE: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/toggle");
    pub const COLOR_PICKER: (StaticSegment<&str>, StaticSegment<&str>) =
        path!("components/color-picker");

    // Feedback
    pub const ALERT: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/alert");
    pub const TOAST: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/toast");
    pub const MODAL: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/modal");
    pub const PROGRESS: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/progress");
    pub const POPOVER: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/popover");
    pub const CHIP: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/chip");
    pub const KBD: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/kbd");

    // General
    pub const TYPOGRAPHY: (StaticSegment<&str>, StaticSegment<&str>) =
        path!("components/typography");
    pub const ICON: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/icon");
    pub const LINK: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/link");
    pub const CALLBACK: (StaticSegment<&str>, StaticSegment<&str>) = path!("components/callback");

    // Animation
    //Transition,

    // Technical
    pub const NOT_FOUND: (StaticSegment<&str>,) = path!("not-found");
}

// You can define other routes in their own component.
// Use a #[component(transparent)] that returns a <Route/>.
#[component(transparent)]
pub fn DocRoutes<Segments>(path: Segments) -> impl IntoView
where
    Segments: PossibleRouteMatch + Debug + Clone + Send + Sync + 'static,
{
    // TODO: This should be a helper function.
    let mut segs = Vec::new();
    doc_routes::OVERVIEW.generate_path(&mut segs);
    let mut overview_path = String::new();
    for seg in segs {
        overview_path.push_str(seg.as_raw_str());
    }

    view! {
        <Routes fallback=|| "Not found.">
            <ParentRoute path=path view=DocLayout>
                <Route path=path!("") view=move || view! { <Redirect path=overview_path.clone()/> }/>

                <Route path=doc_routes::OVERVIEW view=PageOverview/>
                <Route path=doc_routes::INSTALLATION view=PageInstallation/>
                <Route path=doc_routes::THEMES view=PageThemes/>
                <Route path=doc_routes::CHANGELOG view=PageChangelog/>

                <Route path=doc_routes::USE_PRESS view=PageUsePress/>
                <Route path=doc_routes::USE_MOVE view=PageUseMove/>
                <Route path=doc_routes::USE_HOVER view=PageUseHover/>
                <Route path=doc_routes::USE_BUTTON view=PageUseButton/>
                <Route path=doc_routes::USE_OVERLAY view=PageUseOverlay/>
                <Route path=doc_routes::USE_ANCHORLINK view=PageUseAnchorLink/>

                <Route path=doc_routes::ATOM_BUTTON view=PageAtomButton/>
                <Route path=doc_routes::ATOM_POPOVER view=PageAtomPopover/>
                <Route path=doc_routes::ATOM_ANCHORLINK view=PageAtomAnchorLink/>

                <Route path=doc_routes::STACK view=PageStack/>
                <Route path=doc_routes::GRID view=PageGrid/>
                <Route path=doc_routes::SEPARATOR view=PageSeparator/>
                <Route path=doc_routes::SKELETON view=PageSkeleton/>
                <Route path=doc_routes::APP_BAR view=PageAppBar/>
                <Route path=doc_routes::DRAWER view=PageDrawer/>
                <Route path=doc_routes::TAB view=PageTab/>
                <Route path=doc_routes::TABLE view=PageTable/>
                <Route path=doc_routes::COLLAPSIBLE view=PageCollapsible/>

                <Route path=doc_routes::BUTTON view=PageButton/>
                <Route path=doc_routes::INPUT view=PageInput/>
                <Route path=doc_routes::TIPTAP_EDITOR view=PageTiptapEditor/>
                <Route path=doc_routes::DATETIME view=PageDateTime/>
                <Route path=doc_routes::SLIDER view=PageSlider/>
                <Route path=doc_routes::SELECT view=PageSelect/>
                <Route path=doc_routes::CHECKBOX view=PageCheckbox/>
                <Route path=doc_routes::RADIO view=PageRadio/>
                <Route path=doc_routes::TOGGLE view=PageToggle/>
                <Route path=doc_routes::COLOR_PICKER view=PageColorPicker/>

                <Route path=doc_routes::ALERT view=PageAlert/>
                <Route path=doc_routes::TOAST view=PageToast/>
                <Route path=doc_routes::MODAL view=PageModal/>
                <Route path=doc_routes::PROGRESS view=PageProgress/>
                <Route path=doc_routes::POPOVER view=PagePopover/>
                <Route path=doc_routes::CHIP view=PageChip/>
                <Route path=doc_routes::KBD view=PageKbd/>

                <Route path=doc_routes::TYPOGRAPHY view=PageTypography/>
                <Route path=doc_routes::ICON view=PageIcon/>
                <Route path=doc_routes::LINK view=PageLink/>
                <Route path=doc_routes::CALLBACK view=PageCallback/>

                //<Route path=doc_routes::Transition view=PageTransition/>

                <Route path=doc_routes::NOT_FOUND view=|| view! { <Redirect path=app_routes::NOT_FOUND.to_href() /> }/>
            </ParentRoute>
        </Routes>
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn DocLayout() -> impl IntoView {
    let app_layout_context = expect_context::<AppLayoutContext>();

    let drawer_class = move || match app_layout_context.is_small.get() {
        true => "mobile",
        false => "",
    };

    let close_doc_drawer_on_mobile = move || {
        if app_layout_context.is_small.get_untracked() {
            app_layout_context.close_doc_drawer();
        }
    };

    let drawer_content = view! {
        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Getting started"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=doc_routes::OVERVIEW.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Overview"</Link>
                <Link href=doc_routes::INSTALLATION.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Installation"</Link>
                <Link href=doc_routes::THEMES.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Themes"</Link>
                <Link href=doc_routes::CHANGELOG.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Changelog"</Link>
            </Stack>
        </DrawerSection>

        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Hooks"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=doc_routes::USE_PRESS.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_press"</Link>
                <Link href=doc_routes::USE_MOVE.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_move"</Link>
                <Link href=doc_routes::USE_HOVER.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_hover"</Link>
                <Link href=doc_routes::USE_BUTTON.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_button"</Link>
                <Link href=doc_routes::USE_OVERLAY.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_overlay"</Link>
                <Link href=doc_routes::USE_ANCHORLINK.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_anchor_link"</Link>
            </Stack>
        </DrawerSection>

        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Atoms"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=doc_routes::ATOM_BUTTON.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Button"</Link>
                <Link href=doc_routes::ATOM_POPOVER.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Popover"</Link>
                <Link href=doc_routes::ATOM_ANCHORLINK.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"AnchorLink"</Link>
            </Stack>
        </DrawerSection>

        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Components"
        }>
            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsColumnsGap margin=Margin::Right(Size::Em(1.0))></Icon> "Layout"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=doc_routes::STACK.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Stack"</Link>
                    <Link href=doc_routes::GRID.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Grid"</Link>
                    <Link href=doc_routes::SEPARATOR.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Separator"</Link>
                    <Link href=doc_routes::SKELETON.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Skeleton"</Link>
                    <Link href=doc_routes::APP_BAR.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"App Bar"</Link>
                    <Link href=doc_routes::DRAWER.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Drawer"</Link>
                    <Link href=doc_routes::TAB.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Tabs"</Link>
                    <Link href=doc_routes::TABLE.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Table"</Link>
                    <Link href=doc_routes::COLLAPSIBLE.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Collapsible"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsToggles margin=Margin::Right(Size::Em(1.0))></Icon> "Input"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=doc_routes::BUTTON.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Button"</Link>
                    <Link href=doc_routes::INPUT.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Input"</Link>
                    <Link href=doc_routes::TIPTAP_EDITOR.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Tiptap editor"</Link>
                    <Link href=doc_routes::DATETIME.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Date & Time"</Link>
                    <Link href=doc_routes::SLIDER.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Slider"</Link>
                    <Link href=doc_routes::SELECT.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Select"</Link>
                    <Link href=doc_routes::CHECKBOX.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Checkbox"</Link>
                    <Link href=doc_routes::RADIO.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Radio"</Link>
                    <Link href=doc_routes::TOGGLE.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Toggle"</Link>
                    <Link href=doc_routes::COLOR_PICKER.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Color Picker"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsChatSquare margin=Margin::Right(Size::Em(1.0))></Icon> "Feedback"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=doc_routes::ALERT.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Alert"</Link>
                    <Link href=doc_routes::TOAST.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Toast"</Link>
                    <Link href=doc_routes::MODAL.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Modal"</Link>
                    <Link href=doc_routes::PROGRESS.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Progress"</Link>
                    <Link href=doc_routes::POPOVER.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Popover"</Link>
                    <Link href=doc_routes::CHIP.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Chip"</Link>
                    <Link href=doc_routes::KBD.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Kbd"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsCircleSquare margin=Margin::Right(Size::Em(1.0))></Icon> "General"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=doc_routes::TYPOGRAPHY.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Typography"</Link>
                    <Link href=doc_routes::ICON.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Icon"</Link>
                    <Link href=doc_routes::LINK.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Link"</Link>
                    <Link href=doc_routes::CALLBACK.to_href() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Callback"</Link>
                </Stack>
            </DrawerSection>
        </DrawerSection>

        //<DrawerSection level=2 header=move || view! {
        //    <Icon icon=icondata::BsArrowsMove margin=Margin::Right(Size::Em(1.0))></Icon> "Animation"
        //}>
        //    <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
        //        <Link href=DocRoutes::Transition class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Transitions"</Link>
        //    </Stack>
        //</DrawerSection>
    };

    view! {
        <Box attr:id="doc-layout" attr:style=move || format!(
            "margin-left: {}em; margin-right: {}em;",
            match app_layout_context.doc_drawer_closed.get() {
                true => 0,
                false => 16,
            },
            match app_layout_context.is_medium.get() {
                true => 0,
                false => 12,
            },
        )>
            <Drawer
                side=DrawerSide::Left
                attr:id="doc-drawer"
                shown=Signal::derive(move || !app_layout_context.doc_drawer_closed.get())
                attr:class=drawer_class
                attr:style=format!("position: fixed; left: 0; top: {APP_BAR_HEIGHT}; bottom: 0;")
            >
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="menu">
                    { drawer_content }
                </Stack>
            </Drawer>

            // <Outlet/> will show nested child routes.
            <Outlet/>
        </Box>
    }
}

#[component]
pub fn DrawerSection<H, IV>(level: u32, header: H, children: Children) -> impl IntoView
where
    H: Fn() -> IV + 'static,
    IV: IntoView + 'static,
{
    view! {
        <div class="drawer-section" data-level=level>
            <div class="section-header">
                { header() }
            </div>
            { children() }
        </div>
    }
}

#[component]
#[allow(dead_code)]
pub fn New() -> impl IntoView {
    view! {
        <Chip attr:style="color: var(--primary-color); background-color: transparent; margin: 0; padding: 0;">
            "NEW"
        </Chip>
    }
}
