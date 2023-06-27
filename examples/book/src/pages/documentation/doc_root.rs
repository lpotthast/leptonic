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
use crate::pages::documentation::date_time::PageDateTime;
use crate::pages::documentation::drawer::PageDrawer;
use crate::pages::documentation::grid::PageGrid;
use crate::pages::documentation::icon::PageIcon;
use crate::pages::documentation::input::PageInput;
use crate::pages::documentation::installation::PageInstallation;
use crate::pages::documentation::link::PageLink;
use crate::pages::documentation::modal::PageModal;
use crate::pages::documentation::overview::PageOverview;
use crate::pages::documentation::progress::PageProgress;
use crate::pages::documentation::select::PageSelect;
use crate::pages::documentation::separator::PageSeparator;
use crate::pages::documentation::skeleton::PageSkeleton;
use crate::pages::documentation::slider::PageSlider;
use crate::pages::documentation::stack::PageStack;
use crate::pages::documentation::tab::PageTab;
use crate::pages::documentation::themes::PageThemes;
use crate::pages::documentation::tiptap_editor::PageTiptapEditor;
use crate::pages::documentation::toast::PageToast;
use crate::pages::documentation::toggle::PageToggle;
use crate::pages::documentation::transition::PageTransition;
use crate::pages::documentation::typography::PageTypography;
use crate::pages::documentation::usage::PageUsage;
use crate::APP_BAR_HEIGHT;
use crate::pages::err404::PageErr404;

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
    Collapsible,

    // Input
    Button,
    Input,
    TiptapEditor,
    DateTime,
    Slider,
    Select,
    Toggle,

    // Feedback
    Alert,
    Toast,
    Modal,
    Progress,
    Chip,

    // General
    Typography,
    Icon,
    Link,
    Anchor,
    Callback,

    // Animation
    Transition,

    // Technical
    CatchAll,
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
            DocRoutes::Tab => "tab",
            DocRoutes::Collapsible => "collapsible",

            DocRoutes::Button => "button",
            DocRoutes::Input => "input",
            DocRoutes::TiptapEditor => "tiptap-editor",
            DocRoutes::DateTime => "date-time",
            DocRoutes::Slider => "slider",
            DocRoutes::Select => "select",
            DocRoutes::Toggle => "toggle",

            DocRoutes::Alert => "alert",
            DocRoutes::Toast => "toast",
            DocRoutes::Modal => "modal",
            DocRoutes::Progress => "progress",
            DocRoutes::Chip => "chip",

            DocRoutes::Typography => "typography",
            DocRoutes::Icon => "icon",
            DocRoutes::Link => "link",
            DocRoutes::Anchor => "anchor",
            DocRoutes::Callback => "callback",

            DocRoutes::Transition => "transition",
            DocRoutes::CatchAll => "*",
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
pub fn DocRoutes<P>(cx: Scope, path: P) -> impl IntoView
where
    P: std::fmt::Display,
{
    view! {cx,
        <Route path=path view=|cx| view! {cx, <DocLayout/>}>
            <Route path="" view=|cx| view! { cx, <Redirect path=DocRoutes::Overview/> }/>
            <Route path=DocRoutes::Overview view=|cx| view! { cx, <PageOverview/> }/>
            <Route path=DocRoutes::Installation view=|cx| view! { cx, <PageInstallation/> }/>
            <Route path=DocRoutes::Usage view=|cx| view! { cx, <PageUsage/> }/>
            <Route path=DocRoutes::Themes view=|cx| view! { cx, <PageThemes/> }/>
            <Route path=DocRoutes::Changelog view=|cx| view! { cx, <PageChangelog/> }/>

            <Route path=DocRoutes::Stack view=|cx| view! { cx, <PageStack/> }/>
            <Route path=DocRoutes::Grid view=|cx| view! { cx, <PageGrid/> }/>
            <Route path=DocRoutes::Separator view=|cx| view! { cx, <PageSeparator/> }/>
            <Route path=DocRoutes::Skeleton view=|cx| view! { cx, <PageSkeleton/> }/>
            <Route path=DocRoutes::AppBar view=|cx| view! { cx, <PageAppBar/> }/>
            <Route path=DocRoutes::Drawer view=|cx| view! { cx, <PageDrawer/> }/>
            <Route path=DocRoutes::Tab view=|cx| view! { cx, <PageTab/> }/>
            <Route path=DocRoutes::Collapsible view=|cx| view! { cx, <PageCollapsible/> }/>

            <Route path=DocRoutes::Button view=|cx| view! { cx, <PageButton/> }/>
            <Route path=DocRoutes::Input view=|cx| view! { cx, <PageInput/> }/>
            <Route path=DocRoutes::TiptapEditor view=|cx| view! { cx, <PageTiptapEditor/> }/>
            <Route path=DocRoutes::DateTime view=|cx| view! { cx, <PageDateTime/> }/>
            <Route path=DocRoutes::Slider view=|cx| view! { cx, <PageSlider/> }/>
            <Route path=DocRoutes::Select view=|cx| view! { cx, <PageSelect/> }/>
            <Route path=DocRoutes::Toggle view=|cx| view! { cx, <PageToggle/> }/>

            <Route path=DocRoutes::Alert view=|cx| view! { cx, <PageAlert/> }/>
            <Route path=DocRoutes::Toast view=|cx| view! { cx, <PageToast/> }/>
            <Route path=DocRoutes::Modal view=|cx| view! { cx, <PageModal/> }/>
            <Route path=DocRoutes::Progress view=|cx| view! { cx, <PageProgress/> }/>
            <Route path=DocRoutes::Chip view=|cx| view! { cx, <PageChip/> }/>

            <Route path=DocRoutes::Typography view=|cx| view! { cx, <PageTypography/> }/>
            <Route path=DocRoutes::Icon view=|cx| view! { cx, <PageIcon/> }/>
            <Route path=DocRoutes::Link view=|cx| view! { cx, <PageLink/> }/>
            <Route path=DocRoutes::Anchor view=|cx| view! { cx, <PageAnchor/> }/>
            <Route path=DocRoutes::Callback view=|cx| view! { cx, <PageCallback/> }/>

            <Route path=DocRoutes::Transition view=|cx| view! { cx, <PageTransition/> }/>

            <Route path=DocRoutes::CatchAll view=|cx| view! { cx, <PageErr404 /> }/>
        </Route>
    }
}

#[component]
pub fn DocLayout(cx: Scope) -> impl IntoView {
    let app_layout_context = expect_context::<AppLayoutContext>(cx);

    let drawer_class = move || match app_layout_context.is_small.get() {
        true => "mobile",
        false => "",
    };

    let close_doc_drawer_on_mobile = move || {
        let ctx = expect_context::<AppLayoutContext>(cx);
        if ctx.is_small.get_untracked() {
            ctx.close_doc_drawer();
        }
    };

    view! { cx,
        <Box id="doc-layout">
            <Drawer
                id="doc-drawer"
                shown=Signal::derive(cx, move || !app_layout_context.doc_drawer_closed.get())
                class=drawer_class
                style=format!("top: {APP_BAR_HEIGHT}")
            >
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="menu">

                    <DrawerSection header=move |cx| view! {cx,
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

                    <DrawerSection header=move |cx| view! {cx,
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
                            <Link href=DocRoutes::Collapsible class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Collapsible"</Link>
                        </Stack>
                    </DrawerSection>

                    <DrawerSection header=move |cx| view! {cx,
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
                        </Stack>
                    </DrawerSection>

                    <DrawerSection header=move |cx| view! {cx,
                        <Icon icon=BsIcon::BsChatSquare margin=Margin::Right(Size::Em(1.0))></Icon> "Feedback"
                    }>
                        <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                            <Link href=DocRoutes::Alert class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Alert"</Link>
                            <Link href=DocRoutes::Toast class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Toast"</Link>
                            <Link href=DocRoutes::Modal class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Modal"</Link>
                            <Link href=DocRoutes::Progress class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Progress"</Link>
                            <Link href=DocRoutes::Chip class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Chip"</Link>
                        </Stack>
                    </DrawerSection>

                    <DrawerSection header=move |cx| view! {cx,
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

                    <DrawerSection header=move |cx| view! {cx,
                        <Icon icon=BsIcon::BsArrowsMove margin=Margin::Right(Size::Em(1.0))></Icon> "Animation"
                    }>
                        <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                            <Link href=DocRoutes::Transition class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Transitions"</Link>
                        </Stack>
                    </DrawerSection>
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
pub fn DrawerSection<H, IV>(cx: Scope, header: H, children: Children) -> impl IntoView
where
    H: Fn(Scope) -> IV + 'static,
    IV: IntoView + 'static,
{
    view! {cx,
        <div class="drawer-section">
            <div class="section-header">
                { header(cx) }
            </div>
            { children(cx) }
        </div>
    }
}
