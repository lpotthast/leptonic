use std::fmt::Display;

use leptonic::{components::prelude::*, prelude::*};
use leptos::*;
use leptos_router::*;

use crate::app::APP_BAR_HEIGHT;
use crate::app::{AppLayoutContext, AppRoutes};
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
use crate::pages::documentation::hooks::use_long_press::PageUseLongPress;

#[derive(Debug, Copy, Clone)]
pub enum DocRoutes {
    // Getting started
    Overview,
    Installation,
    Themes,
    Changelog,

    // Hooks
    UsePress,
    UseLongPress,
    UseMove,
    UseHover,
    UseButton,
    UseOverlay,
    UseAnchorLink,

    // Atoms
    AtomButton,
    AtomPopover,
    AtomAnchorLink,

    // Layout
    Stack,
    Grid,
    Separator,
    Skeleton,
    AppBar,
    Drawer,
    Tab,
    Table,
    Collapsible,

    // Input
    Button,
    Input,
    TiptapEditor,
    DateTime,
    Slider,
    Select,
    Checkbox,
    Radio,
    Toggle,
    ColorPicker,

    // Feedback
    Alert,
    Toast,
    Modal,
    Progress,
    Popover,
    Chip,
    Kbd,

    // General
    Typography,
    Icon,
    Link,
    Callback,

    // Animation
    //Transition,

    // Technical
    NotFound,
}

impl DocRoutes {
    pub const fn route(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Installation => "installation",
            Self::Themes => "themes",
            Self::Changelog => "changelog",

            Self::UsePress => "hooks/use-press",
            Self::UseLongPress => "hooks/use-long-press",
            Self::UseMove => "hooks/use-move",
            Self::UseHover => "hooks/use-hover",
            Self::UseButton => "hooks/use-button",
            Self::UseOverlay => "hooks/use-overlay",
            Self::UseAnchorLink => "hooks/use-anchor-link",

            Self::AtomButton => "atoms/button",
            Self::AtomPopover => "atoms/popover",
            Self::AtomAnchorLink => "atoms/anchor-link",

            Self::Stack => "components/stack",
            Self::Grid => "components/grid",
            Self::Separator => "components/separator",
            Self::Skeleton => "components/skeleton",
            Self::AppBar => "components/app-bar",
            Self::Drawer => "components/drawer",
            Self::Tab => "components/tabs",
            Self::Table => "components/table",
            Self::Collapsible => "components/collapsible",

            Self::Button => "components/button",
            Self::Input => "components/input",
            Self::TiptapEditor => "components/tiptap-editor",
            Self::DateTime => "components/date-time",
            Self::Slider => "components/slider",
            Self::Select => "components/select",
            Self::Checkbox => "components/checkbox",
            Self::Radio => "components/radio",
            Self::Toggle => "components/toggle",
            Self::ColorPicker => "components/color-picker",

            Self::Alert => "components/alert",
            Self::Toast => "components/toast",
            Self::Modal => "components/modal",
            Self::Progress => "components/progress",
            Self::Popover => "components/popover",
            Self::Chip => "components/chip",
            Self::Kbd => "components/kbd",

            Self::Typography => "components/typography",
            Self::Icon => "components/icon",
            Self::Link => "components/link",
            Self::Callback => "components/callback",

            //Self::Transition => "transition",
            Self::NotFound => "not-found", // Leptos requires this to be be named "*"!
        }
    }
}

/// Required so that `Routes` variants can be used in `<Route path=Routes::Foo ...>` definitions.
impl Display for DocRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.route())
    }
}

/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo ...>` definitions.
impl ToHref for DocRoutes {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(move || format!("/{}/{}", AppRoutes::Doc.route(), self.route()))
    }
}

// You can define other routes in their own component.
// Use a #[component(transparent)] that returns a <Route/>.
#[component(transparent)]
pub fn DocRoutes<P: Display>(path: P) -> impl IntoView {
    view! {
        <Route path=path view=|| view! { <DocLayout/>}>
            <Route path="" view=|| view! { <Redirect path=DocRoutes::Overview/> }/>
            <Route path=DocRoutes::Overview view=|| view! { <PageOverview/> }/>
            <Route path=DocRoutes::Installation view=|| view! { <PageInstallation/> }/>
            <Route path=DocRoutes::Themes view=|| view! { <PageThemes/> }/>
            <Route path=DocRoutes::Changelog view=|| view! { <PageChangelog/> }/>

            <Route path=DocRoutes::UsePress view=|| view! { <PageUsePress/> }/>
            <Route path=DocRoutes::UseLongPress view=|| view! { <PageUseLongPress/> }/>
            <Route path=DocRoutes::UseMove view=|| view! { <PageUseMove/> }/>
            <Route path=DocRoutes::UseHover view=|| view! { <PageUseHover/> }/>
            <Route path=DocRoutes::UseButton view=|| view! { <PageUseButton/> }/>
            <Route path=DocRoutes::UseOverlay view=|| view! { <PageUseOverlay/> }/>
            <Route path=DocRoutes::UseAnchorLink view=|| view! { <PageUseAnchorLink/> }/>

            <Route path=DocRoutes::AtomButton view=|| view! { <PageAtomButton/> }/>
            <Route path=DocRoutes::AtomPopover view=|| view! { <PageAtomPopover/> }/>
            <Route path=DocRoutes::AtomAnchorLink view=|| view! { <PageAtomAnchorLink/> }/>

            <Route path=DocRoutes::Stack view=|| view! { <PageStack/> }/>
            <Route path=DocRoutes::Grid view=|| view! { <PageGrid/> }/>
            <Route path=DocRoutes::Separator view=|| view! { <PageSeparator/> }/>
            <Route path=DocRoutes::Skeleton view=|| view! { <PageSkeleton/> }/>
            <Route path=DocRoutes::AppBar view=|| view! { <PageAppBar/> }/>
            <Route path=DocRoutes::Drawer view=|| view! { <PageDrawer/> }/>
            <Route path=DocRoutes::Tab view=|| view! { <PageTab/> }/>
            <Route path=DocRoutes::Table view=|| view! { <PageTable/> }/>
            <Route path=DocRoutes::Collapsible view=|| view! { <PageCollapsible/> }/>

            <Route path=DocRoutes::Button view=|| view! { <PageButton/> }/>
            <Route path=DocRoutes::Input view=|| view! { <PageInput/> }/>
            <Route path=DocRoutes::TiptapEditor view=|| view! { <PageTiptapEditor/> }/>
            <Route path=DocRoutes::DateTime view=|| view! { <PageDateTime/> }/>
            <Route path=DocRoutes::Slider view=|| view! { <PageSlider/> }/>
            <Route path=DocRoutes::Select view=|| view! { <PageSelect/> }/>
            <Route path=DocRoutes::Checkbox view=|| view! { <PageCheckbox/> }/>
            <Route path=DocRoutes::Radio view=|| view! { <PageRadio/> }/>
            <Route path=DocRoutes::Toggle view=|| view! { <PageToggle/> }/>
            <Route path=DocRoutes::ColorPicker view=|| view! { <PageColorPicker/> }/>

            <Route path=DocRoutes::Alert view=|| view! { <PageAlert/> }/>
            <Route path=DocRoutes::Toast view=|| view! { <PageToast/> }/>
            <Route path=DocRoutes::Modal view=|| view! { <PageModal/> }/>
            <Route path=DocRoutes::Progress view=|| view! { <PageProgress/> }/>
            <Route path=DocRoutes::Popover view=|| view! { <PagePopover/> }/>
            <Route path=DocRoutes::Chip view=|| view! { <PageChip/> }/>
            <Route path=DocRoutes::Kbd view=|| view! { <PageKbd/> }/>

            <Route path=DocRoutes::Typography view=|| view! { <PageTypography/> }/>
            <Route path=DocRoutes::Icon view=|| view! { <PageIcon/> }/>
            <Route path=DocRoutes::Link view=|| view! { <PageLink/> }/>
            <Route path=DocRoutes::Callback view=|| view! { <PageCallback/> }/>

            //<Route path=DocRoutes::Transition view=|| view! { <PageTransition/> }/>

            <Route path=DocRoutes::NotFound view=|| view! { <Redirect path=AppRoutes::NotFound.to_href()() /> }/>
        </Route>
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
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                <Link href=DocRoutes::Overview class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Overview"</Link>
                <Link href=DocRoutes::Installation class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Installation"</Link>
                <Link href=DocRoutes::Themes class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Themes"</Link>
                <Link href=DocRoutes::Changelog class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Changelog"</Link>
            </Stack>
        </DrawerSection>

        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Hooks"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                <Link href=DocRoutes::UsePress class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_press"</Link>
                <Link href=DocRoutes::UseLongPress class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_long_press"</Link>
                <Link href=DocRoutes::UseMove class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_move"</Link>
                <Link href=DocRoutes::UseHover class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_hover"</Link>
                <Link href=DocRoutes::UseButton class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_button"</Link>
                <Link href=DocRoutes::UseOverlay class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_overlay"</Link>
                <Link href=DocRoutes::UseAnchorLink class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_anchor_link"</Link>
            </Stack>
        </DrawerSection>

        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Atoms"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                <Link href=DocRoutes::AtomButton class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Button"</Link>
                <Link href=DocRoutes::AtomPopover class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Popover"</Link>
                <Link href=DocRoutes::AtomAnchorLink class="item" on:click=move |_| close_doc_drawer_on_mobile()>"AnchorLink"</Link>
            </Stack>
        </DrawerSection>

        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Components"
        }>
            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsColumnsGap margin=Margin::Right(Size::Em(1.0))></Icon> "Layout"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                    <Link href=DocRoutes::Stack class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Stack"</Link>
                    <Link href=DocRoutes::Grid class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Grid"</Link>
                    <Link href=DocRoutes::Separator class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Separator"</Link>
                    <Link href=DocRoutes::Skeleton class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Skeleton"</Link>
                    <Link href=DocRoutes::AppBar class="item" on:click=move |_| close_doc_drawer_on_mobile()>"App Bar"</Link>
                    <Link href=DocRoutes::Drawer class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Drawer"</Link>
                    <Link href=DocRoutes::Tab class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Tabs"</Link>
                    <Link href=DocRoutes::Table class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Table"</Link>
                    <Link href=DocRoutes::Collapsible class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Collapsible"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsToggles margin=Margin::Right(Size::Em(1.0))></Icon> "Input"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                    <Link href=DocRoutes::Button class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Button"</Link>
                    <Link href=DocRoutes::Input class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Input"</Link>
                    <Link href=DocRoutes::TiptapEditor class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Tiptap editor"</Link>
                    <Link href=DocRoutes::DateTime class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Date & Time"</Link>
                    <Link href=DocRoutes::Slider class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Slider"</Link>
                    <Link href=DocRoutes::Select class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Select"</Link>
                    <Link href=DocRoutes::Checkbox class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Checkbox"</Link>
                    <Link href=DocRoutes::Radio class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Radio"</Link>
                    <Link href=DocRoutes::Toggle class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Toggle"</Link>
                    <Link href=DocRoutes::ColorPicker class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Color Picker"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsChatSquare margin=Margin::Right(Size::Em(1.0))></Icon> "Feedback"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                    <Link href=DocRoutes::Alert class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Alert"</Link>
                    <Link href=DocRoutes::Toast class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Toast"</Link>
                    <Link href=DocRoutes::Modal class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Modal"</Link>
                    <Link href=DocRoutes::Progress class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Progress"</Link>
                    <Link href=DocRoutes::Popover class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Popover"</Link>
                    <Link href=DocRoutes::Chip class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Chip"</Link>
                    <Link href=DocRoutes::Kbd class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Kbd"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsCircleSquare margin=Margin::Right(Size::Em(1.0))></Icon> "General"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                    <Link href=DocRoutes::Typography class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Typography"</Link>
                    <Link href=DocRoutes::Icon class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Icon"</Link>
                    <Link href=DocRoutes::Link class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Link"</Link>
                    <Link href=DocRoutes::Callback class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Callback"</Link>
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
        <Box id="doc-layout" style=move || format!(
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
                id="doc-drawer"
                shown=Signal::derive(move || !app_layout_context.doc_drawer_closed.get())
                class=drawer_class
                style=format!("position: fixed; left: 0; top: {APP_BAR_HEIGHT}; bottom: 0;")
            >
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="menu">
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
        <Chip style="color: var(--primary-color); background-color: transparent; margin: 0; padding: 0;">
            "NEW"
        </Chip>
    }
}
