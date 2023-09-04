use std::fmt::Display;

use leptonic::prelude::*;
use leptos::*;
use leptos_icons::BsIcon;
use leptos_router::*;

use crate::app::{AppLayoutContext, AppRoutes};
use crate::pages::documentation::alert::PageAlert;
use crate::pages::documentation::anchor::PageAnchor;
use crate::pages::documentation::app_bar::PageAppBar;
use crate::pages::documentation::button::PageButton;
use crate::pages::documentation::callback::PageCallback;
use crate::pages::documentation::changelog::PageChangelog;
use crate::pages::documentation::chip::PageChip;
use crate::pages::documentation::collapsible::PageCollapsible;
use crate::pages::documentation::color_picker::PageColorPicker;
use crate::pages::documentation::date_time::PageDateTime;
use crate::pages::documentation::drawer::PageDrawer;
use crate::pages::documentation::grid::PageGrid;
use crate::pages::documentation::icon::PageIcon;
use crate::pages::documentation::input::PageInput;
use crate::pages::documentation::installation::PageInstallation;
use crate::pages::documentation::kbd::PageKbd;
use crate::pages::documentation::link::PageLink;
use crate::pages::documentation::modal::PageModal;
use crate::pages::documentation::overview::PageOverview;
use crate::pages::documentation::popover::PagePopover;
use crate::pages::documentation::progress::PageProgress;
use crate::pages::documentation::select::PageSelect;
use crate::pages::documentation::separator::PageSeparator;
use crate::pages::documentation::skeleton::PageSkeleton;
use crate::pages::documentation::slider::PageSlider;
use crate::pages::documentation::stack::PageStack;
use crate::pages::documentation::tab::PageTab;
use crate::pages::documentation::table::PageTable;
use crate::pages::documentation::themes::PageThemes;
use crate::pages::documentation::tiptap_editor::PageTiptapEditor;
use crate::pages::documentation::toast::PageToast;
use crate::pages::documentation::toggle::PageToggle;
use crate::pages::documentation::transition::PageTransition;
use crate::pages::documentation::typography::PageTypography;
use crate::pages::documentation::usage::PageUsage;
use crate::APP_BAR_HEIGHT;

#[derive(Debug, Copy, Clone)]
pub enum DocRoutes {
    // Getting started
    Overview,
    Installation,
    Usage,
    Themes,
    Changelog,

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
    Anchor,
    Callback,

    // Animation
    Transition,

    // Technical
    NotFound,
}

impl DocRoutes {
    pub fn route(self) -> &'static str {
        match self {
            DocRoutes::Overview => "overview",
            DocRoutes::Installation => "installation",
            DocRoutes::Usage => "usage",
            DocRoutes::Themes => "themes",
            DocRoutes::Changelog => "changelog",

            DocRoutes::Stack => "stack",
            DocRoutes::Grid => "grid",
            DocRoutes::Separator => "separator",
            DocRoutes::Skeleton => "skeleton",
            DocRoutes::AppBar => "app-bar",
            DocRoutes::Drawer => "drawer",
            DocRoutes::Tab => "tabs",
            DocRoutes::Table => "table",
            DocRoutes::Collapsible => "collapsible",

            DocRoutes::Button => "button",
            DocRoutes::Input => "input",
            DocRoutes::TiptapEditor => "tiptap-editor",
            DocRoutes::DateTime => "date-time",
            DocRoutes::Slider => "slider",
            DocRoutes::Select => "select",
            DocRoutes::Toggle => "toggle",
            DocRoutes::ColorPicker => "color-picker",

            DocRoutes::Alert => "alert",
            DocRoutes::Toast => "toast",
            DocRoutes::Modal => "modal",
            DocRoutes::Progress => "progress",
            DocRoutes::Popover => "popover",
            DocRoutes::Chip => "chip",
            DocRoutes::Kbd => "kbd",

            DocRoutes::Typography => "typography",
            DocRoutes::Icon => "icon",
            DocRoutes::Link => "link",
            DocRoutes::Anchor => "anchor",
            DocRoutes::Callback => "callback",

            DocRoutes::Transition => "transition",
            DocRoutes::NotFound => "*", // Leptos requires this to be be named "*"!
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
pub fn DocRoutes<P>(path: P) -> impl IntoView
where
    P: std::fmt::Display,
{
    view! {
        <Route path=path view=|| view! { <DocLayout/>}>
            <Route path="" view=|| view! { <Redirect path=DocRoutes::Overview/> }/>
            <Route path=DocRoutes::Overview view=|| view! { <PageOverview/> }/>
            <Route path=DocRoutes::Installation view=|| view! { <PageInstallation/> }/>
            <Route path=DocRoutes::Usage view=|| view! { <PageUsage/> }/>
            <Route path=DocRoutes::Themes view=|| view! { <PageThemes/> }/>
            <Route path=DocRoutes::Changelog view=|| view! { <PageChangelog/> }/>

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
            <Route path=DocRoutes::Anchor view=|| view! { <PageAnchor/> }/>
            <Route path=DocRoutes::Callback view=|| view! { <PageCallback/> }/>

            <Route path=DocRoutes::Transition view=|| view! { <PageTransition/> }/>

            <Route path=DocRoutes::NotFound view=|| view! { <Redirect path=AppRoutes::NotFound.to_href()() /> }/>
        </Route>
    }
}

#[component]
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
        <DrawerSection header=move || view! {
            <Icon icon=BsIcon::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Getting started"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                <Link href=DocRoutes::Overview class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Overview"</Link>
                <Link href=DocRoutes::Installation class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Installation"</Link>
                <Link href=DocRoutes::Usage class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Usage"</Link>
                <Link href=DocRoutes::Themes class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Themes"</Link>
                <Link href=DocRoutes::Changelog class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Changelog"</Link>
            </Stack>
        </DrawerSection>

        <DrawerSection header=move || view! {
            <Icon icon=BsIcon::BsColumnsGap margin=Margin::Right(Size::Em(1.0))></Icon> "Layout"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                <Link href=DocRoutes::Stack class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Stack"</Link>
                <Link href=DocRoutes::Grid class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Grid"</Link>
                <Link href=DocRoutes::Separator class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Separator"</Link>
                <Link href=DocRoutes::Skeleton class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Skeleton"</Link>
                <Link href=DocRoutes::AppBar class="item" on:click=move |_| close_doc_drawer_on_mobile()>"App Bar"</Link>
                <Link href=DocRoutes::Drawer class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Drawer"</Link>
                <Link href=DocRoutes::Tab class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Tabs"</Link>
                <div class="item">
                    <Link href=DocRoutes::Table on:click=move |_| close_doc_drawer_on_mobile()>"Table"</Link>
                    <New/>
                </div>
                <Link href=DocRoutes::Collapsible class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Collapsible"</Link>
            </Stack>
        </DrawerSection>

        <DrawerSection header=move || view! {
            <Icon icon=BsIcon::BsToggles margin=Margin::Right(Size::Em(1.0))></Icon> "Input"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                <Link href=DocRoutes::Button class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Button"</Link>
                <Link href=DocRoutes::Input class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Input"</Link>
                <Link href=DocRoutes::TiptapEditor class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Tiptap editor"</Link>
                <Link href=DocRoutes::DateTime class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Date & Time"</Link>
                <Link href=DocRoutes::Slider class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Slider"</Link>
                <Link href=DocRoutes::Select class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Select"</Link>
                <Link href=DocRoutes::Toggle class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Toggle"</Link>
                <div class="item">
                    <Link href=DocRoutes::ColorPicker on:click=move |_| close_doc_drawer_on_mobile()>"Color Picker"</Link>
                    <New/>
                </div>
            </Stack>
        </DrawerSection>

        <DrawerSection header=move || view! {
            <Icon icon=BsIcon::BsChatSquare margin=Margin::Right(Size::Em(1.0))></Icon> "Feedback"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                <Link href=DocRoutes::Alert class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Alert"</Link>
                <Link href=DocRoutes::Toast class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Toast"</Link>
                <Link href=DocRoutes::Modal class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Modal"</Link>
                <Link href=DocRoutes::Progress class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Progress"</Link>
                <Link href=DocRoutes::Popover class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Popover"</Link>
                <Link href=DocRoutes::Chip class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Chip"</Link>
                <div class="item">
                    <Link href=DocRoutes::Kbd on:click=move |_| close_doc_drawer_on_mobile()>"Kbd"</Link>
                    <New/>
                </div>
            </Stack>
        </DrawerSection>

        <DrawerSection header=move || view! {
            <Icon icon=BsIcon::BsCircleSquare margin=Margin::Right(Size::Em(1.0))></Icon> "General"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                <Link href=DocRoutes::Typography class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Typography"</Link>
                <Link href=DocRoutes::Icon class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Icon"</Link>
                <Link href=DocRoutes::Link class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Link"</Link>
                <Link href=DocRoutes::Anchor class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Anchor"</Link>
                <Link href=DocRoutes::Callback class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Callback"</Link>
            </Stack>
        </DrawerSection>

        <DrawerSection header=move || view! {
            <Icon icon=BsIcon::BsArrowsMove margin=Margin::Right(Size::Em(1.0))></Icon> "Animation"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                <Link href=DocRoutes::Transition class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Transitions"</Link>
            </Stack>
        </DrawerSection>
    };

    view! {
        <Box id="doc-layout">
            <Drawer
                side=DrawerSide::Left
                id="doc-drawer"
                shown=Signal::derive(move || !app_layout_context.doc_drawer_closed.get())
                class=drawer_class
                style=format!("top: {APP_BAR_HEIGHT}")
            >
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="menu">
                    { drawer_content }
                </Stack>
            </Drawer>

            <Box id="doc-content">
                // <Outlet/> will show nested child routes.
                <Outlet/>
            </Box>
        </Box>
    }
}

#[component]
pub fn DrawerSection<H, IV>(header: H, children: Children) -> impl IntoView
where
    H: Fn() -> IV + 'static,
    IV: IntoView + 'static,
{
    view! {
        <div class="drawer-section">
            <div class="section-header">
                { header() }
            </div>
            { children() }
        </div>
    }
}

#[component]
pub fn New() -> impl IntoView {
    view! {
        <Chip style="color: var(--primary-color); background-color: transparent; margin: 0; padding: 0;">
            "NEW"
        </Chip>
    }
}
