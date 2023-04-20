use std::fmt::Display;

use leptos::*;
use leptos_icons::*;
use leptos_meta::{provide_meta_context, Title, TitleProps};
use leptos_router::*;
use serde::{Deserialize, Serialize};

use leptonic::prelude::*;

use crate::pages::alert::PageAlert;
use crate::pages::alert::PageAlertProps;
use crate::pages::app_bar::PageAppBar;
use crate::pages::app_bar::PageAppBarProps;
use crate::pages::button::PageButton;
use crate::pages::button::PageButtonProps;
use crate::pages::collapsible::PageCollapsible;
use crate::pages::collapsible::PageCollapsibleProps;
use crate::pages::date_time::PageDateTime;
use crate::pages::date_time::PageDateTimeProps;
use crate::pages::drawer::PageDrawer;
use crate::pages::drawer::PageDrawerProps;
use crate::pages::err404::PageErr404;
use crate::pages::err404::PageErr404Props;
use crate::pages::grid::PageGrid;
use crate::pages::grid::PageGridProps;
use crate::pages::icon::PageIcon;
use crate::pages::icon::PageIconProps;
use crate::pages::input::PageInput;
use crate::pages::input::PageInputProps;
use crate::pages::installation::PageInstallation;
use crate::pages::installation::PageInstallationProps;
use crate::pages::modal::PageModal;
use crate::pages::modal::PageModalProps;
use crate::pages::overview::PageOverview;
use crate::pages::overview::PageOverviewProps;
use crate::pages::progress_indicator::PageProgressIndicator;
use crate::pages::progress_indicator::PageProgressIndicatorProps;
use crate::pages::select::PageSelect;
use crate::pages::select::PageSelectProps;
use crate::pages::separator::PageSeparator;
use crate::pages::separator::PageSeparatorProps;
use crate::pages::skeleton::PageSkeleton;
use crate::pages::skeleton::PageSkeletonProps;
use crate::pages::slider::PageSlider;
use crate::pages::slider::PageSliderProps;
use crate::pages::stack::PageStack;
use crate::pages::stack::PageStackProps;
use crate::pages::tab::PageTab;
use crate::pages::tab::PageTabProps;
use crate::pages::toast::PageToast;
use crate::pages::toast::PageToastProps;
use crate::pages::transition::PageTransition;
use crate::pages::transition::PageTransitionProps;
use crate::pages::typography::PageTypography;
use crate::pages::typography::PageTypographyProps;
use crate::pages::usage::PageUsage;
use crate::pages::usage::PageUsageProps;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum AppTheme {
    #[default]
    Light,
    Dark,
}

impl Theme for AppTheme {
    fn name(&self) -> &'static str {
        match self {
            AppTheme::Light => "light",
            AppTheme::Dark => "dark",
        }
    }

    fn icon(&self) -> leptos_icons::Icon {
        match self {
            AppTheme::Light => BsIcon::BsSun.into(),
            AppTheme::Dark => BsIcon::BsMoon.into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Routes {
    // Getting started
    Overview,
    Installation,
    Usage,

    // Layout
    Stack,
    Grid,
    Skeleton,
    AppBar,
    Drawer,

    // Components
    Button,
    Tab,
    Input,
    DateTime,
    Collapsible,
    Toast,
    Modal,
    Alert,
    Typography,
    Icon,
    ProgressIndicator,
    Slider,
    Separator,
    Select,

    // Animation
    Transition,

    // Technical
    CatchAll,
}

impl Routes {
    pub fn route(self) -> &'static str {
        match self {
            Routes::Overview => "/overview",
            Routes::Installation => "/installation",
            Routes::Usage => "/usage",

            Routes::Stack => "/stack",
            Routes::Grid => "/grid",
            Routes::Skeleton => "/skeleton",
            Routes::AppBar => "/app-bar",
            Routes::Drawer => "/drawer",

            Routes::Button => "/button",
            Routes::Tab => "/tab",
            Routes::Input => "/input",
            Routes::DateTime => "/date-time",
            Routes::Collapsible => "/collapsible",
            Routes::Toast => "/toast",
            Routes::Modal => "/modal",
            Routes::Alert => "/alert",
            Routes::Typography => "/typography",
            Routes::Icon => "/icon",
            Routes::ProgressIndicator => "/progress-indicator",
            Routes::Slider => "/slider",
            Routes::Separator => "/separator",
            Routes::Select => "/select",
            Routes::Transition => "/transition",
            Routes::CatchAll => "*",
        }
    }
}

/// Required so that `Routes` variants can be used in `<Route path=Routes::Foo ...>` definitions.
impl Display for Routes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.route())
    }
}

/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo ...>` definitions.
impl ToHref for Routes {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(move || self.route().to_owned())
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Title text="Leptonic"/>
        <Root default_theme=AppTheme::default()>
            <Router>
                <Layout>
                    <Routes>
                        <Route path="" view=|cx| view! { cx, <PageOverview/> }/>
                        <Route path=Routes::Overview view=|cx| view! { cx, <PageOverview/> }/>
                        <Route path=Routes::Installation view=|cx| view! { cx, <PageInstallation/> }/>
                        <Route path=Routes::Usage view=|cx| view! { cx, <PageUsage/> }/>

                        <Route path=Routes::Stack view=|cx| view! { cx, <PageStack/> }/>
                        <Route path=Routes::Grid view=|cx| view! { cx, <PageGrid/> }/>
                        <Route path=Routes::Skeleton view=|cx| view! { cx, <PageSkeleton/> }/>
                        <Route path=Routes::AppBar view=|cx| view! { cx, <PageAppBar/> }/>
                        <Route path=Routes::Drawer view=|cx| view! { cx, <PageDrawer/> }/>

                        <Route path=Routes::Button view=|cx| view! { cx, <PageButton/> }/>
                        <Route path=Routes::Tab view=|cx| view! { cx, <PageTab/> }/>
                        <Route path=Routes::Input view=|cx| view! { cx, <PageInput/> }/>
                        <Route path=Routes::DateTime view=|cx| view! { cx, <PageDateTime/> }/>
                        <Route path=Routes::Collapsible view=|cx| view! { cx, <PageCollapsible/> }/>
                        <Route path=Routes::Toast view=|cx| view! { cx, <PageToast/> }/>
                        <Route path=Routes::Modal view=|cx| view! { cx, <PageModal/> }/>
                        <Route path=Routes::Alert view=|cx| view! { cx, <PageAlert/> }/>
                        <Route path=Routes::Typography view=|cx| view! { cx, <PageTypography/> }/>
                        <Route path=Routes::Icon view=|cx| view! { cx, <PageIcon/> }/>
                        <Route path=Routes::ProgressIndicator view=|cx| view! { cx, <PageProgressIndicator/> }/>
                        <Route path=Routes::Slider view=|cx| view! { cx, <PageSlider/> }/>
                        <Route path=Routes::Separator view=|cx| view! { cx, <PageSeparator/> }/>
                        <Route path=Routes::Select view=|cx| view! { cx, <PageSelect/> }/>

                        <Route path=Routes::Transition view=|cx| view! { cx, <PageTransition/> }/>

                        <Route path=Routes::CatchAll view=|cx| view! { cx, <PageErr404 /> }/>
                    </Routes>
                </Layout>
            </Router>
        </Root>
    }
}

#[component]
pub fn Layout(cx: Scope, children: Children) -> impl IntoView {
    let app_bar_height = Height::Em(2.75);

    view! { cx,
        <AppBar height=app_bar_height>
            <Link href="">
                <Typography variant=TypographyVariant::H3 margin=Margin::Left(Size::Em(1.0))>
                    "Leptonic  -  v0.1"
                </Typography>
            </Link>
            <Stack orientation=StackOrientation::Horizontal spacing=10 style="margin-right: 1em">
                <ThemeToggle off=AppTheme::Light on=AppTheme::Dark/>
                <Icon icon=BsIcon::BsFolder></Icon>
                <Icon icon=BsIcon::BsPower></Icon>
            </Stack>
        </AppBar>

        <Drawer margin=Margin::Top(app_bar_height)>
            <Stack orientation=StackOrientation::Vertical spacing=0 class="menu">
                <Quicksearch />

                <Collapsible
                    open=true
                    header=view! {cx,
                        <Icon icon=BsIcon::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Getting started"
                    }
                    body=view! {cx,
                        <Stack orientation=StackOrientation::Vertical spacing=0 class="menu nested dense">
                            <Link href=Routes::Overview class="item">"Overview"</Link>
                            <Link href=Routes::Installation class="item">"Installation"</Link>
                            <Link href=Routes::Usage class="item">"Usage"</Link>
                        </Stack>
                    }
                />

                <Collapsible
                    open=true
                    header=view! {cx,
                        <Icon icon=BsIcon::BsColumnsGap margin=Margin::Right(Size::Em(1.0))></Icon> "Layout"
                    }
                    body=view! {cx,
                        <Stack orientation=StackOrientation::Vertical spacing=0 class="menu nested dense">
                            <Link href=Routes::Stack class="item">"Stack"</Link>
                            <Link href=Routes::Grid class="item">"Grid"</Link>
                            <Link href=Routes::Separator class="item">"Separator"</Link>
                            <Link href=Routes::Skeleton class="item">"Skeleton"</Link>
                            <Link href=Routes::AppBar class="item">"App Bar"</Link>
                            <Link href=Routes::Drawer class="item">"Drawer"</Link>
                            <Link href=Routes::Tab class="item">"Tabs"</Link>
                            <Link href=Routes::Collapsible class="item">"Collapsible"</Link>
                        </Stack>
                    }
                />

                <Collapsible
                    open=true
                    header=view! {cx,
                        <Icon icon=BsIcon::BsToggles margin=Margin::Right(Size::Em(1.0))></Icon> "Input"
                    }
                    body=view! {cx,
                        <Stack orientation=StackOrientation::Vertical spacing=0 class="menu nested dense">
                            <Link href=Routes::Button class="item">"Button"</Link>
                            <Link href=Routes::Input class="item">"Inputs"</Link>
                            <Link href=Routes::DateTime class="item">"Date & Time"</Link>
                            <Link href=Routes::Slider class="item">"Slider"</Link>
                            <Link href=Routes::Select class="item">"Select"</Link>
                        </Stack>
                    }
                />

                <Collapsible
                    open=true
                    header=view! {cx,
                        <Icon icon=BsIcon::BsChatSquare margin=Margin::Right(Size::Em(1.0))></Icon> "Feedback"
                    }
                    body=view! {cx,
                        <Stack orientation=StackOrientation::Vertical spacing=0 class="menu nested dense">
                            <Link href=Routes::Alert class="item">"Alert"</Link>
                            <Link href=Routes::Toast class="item">"Toast"</Link>
                            <Link href=Routes::Modal class="item">"Modal"</Link>
                            <Link href=Routes::ProgressIndicator class="item">"Progress"</Link>
                        </Stack>
                    }
                />

                <Collapsible
                    open=true
                    header=view! {cx,
                        <Icon icon=BsIcon::BsChatSquare margin=Margin::Right(Size::Em(1.0))></Icon> "General"
                    }
                    body=view! {cx,
                        <Stack orientation=StackOrientation::Vertical spacing=0 class="menu nested dense">
                            <Link href=Routes::Typography class="item">"Typography"</Link>
                            <Link href=Routes::Icon class="item">"Icon"</Link>
                        </Stack>
                    }
                />

                <Collapsible
                    open=true
                    header=view! {cx,
                        <Icon icon=BsIcon::BsArrowsMove margin=Margin::Right(Size::Em(1.0))></Icon> "Animation"
                    }
                    body=view! {cx,
                        <Stack orientation=StackOrientation::Vertical spacing=0 class="menu nested dense">
                            <Link href=Routes::Transition class="item">"Transitions"</Link>
                        </Stack>
                    }
                />
            </Stack>
        </Drawer>

        <Box id="content" style=format!("margin-top: {app_bar_height};")>
            { children(cx) }
        </Box>
    }
}
