use leptos::*;
use leptos_icons::*;
use leptos_meta::{provide_meta_context, Title, TitleProps};
use leptos_router::*;
use serde::{Deserialize, Serialize};

use leptonic::prelude::*;

use crate::pages::alert::PageAlert;
use crate::pages::alert::PageAlertProps;
use crate::pages::button::PageButton;
use crate::pages::button::PageButtonProps;
use crate::pages::collapsible::PageCollapsible;
use crate::pages::collapsible::PageCollapsibleProps;
use crate::pages::drawer::PageDrawer;
use crate::pages::drawer::PageDrawerProps;
use crate::pages::err404::PageErr404;
use crate::pages::err404::PageErr404Props;
use crate::pages::grid::PageGrid;
use crate::pages::grid::PageGridProps;
use crate::pages::icon::PageIcon;
use crate::pages::icon::PageIconProps;
use crate::pages::installation::PageInstallation;
use crate::pages::installation::PageInstallationProps;
use crate::pages::modal::PageModal;
use crate::pages::modal::PageModalProps;
use crate::pages::overview::PageOverview;
use crate::pages::overview::PageOverviewProps;
use crate::pages::stack::PageStack;
use crate::pages::stack::PageStackProps;
use crate::pages::tab::PageTab;
use crate::pages::tab::PageTabProps;
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
                        <Route path="overview" view=|cx| view! { cx, <PageOverview/> }/>
                        <Route path="installation" view=|cx| view! { cx, <PageInstallation/> }/>
                        <Route path="usage" view=|cx| view! { cx, <PageUsage/> }/>

                        <Route path="stack" view=|cx| view! { cx, <PageStack/> }/>
                        <Route path="grid" view=|cx| view! { cx, <PageGrid/> }/>

                        <Route path="button" view=|cx| view! { cx, <PageButton/> }/>
                        <Route path="tab" view=|cx| view! { cx, <PageTab/> }/>
                        <Route path="collapsible" view=|cx| view! { cx, <PageCollapsible/> }/>
                        <Route path="drawer" view=|cx| view! { cx, <PageDrawer/> }/>
                        <Route path="modal" view=|cx| view! { cx, <PageModal/> }/>
                        <Route path="alert" view=|cx| view! { cx, <PageAlert/> }/>
                        <Route path="typography" view=|cx| view! { cx, <PageTypography/> }/>
                        <Route path="icon" view=|cx| view! { cx, <PageIcon/> }/>

                        <Route path="*" view=|cx| view! { cx, <PageErr404 /> }/>
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

        <Drawer>
            <Stack orientation=StackOrientation::Vertical spacing=0 class="menu">
                <Quicksearch />

                <Collapsible
                    open=true
                    header=view! {cx,
                        <Icon icon=BsIcon::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Getting started"
                    }
                    body=view! {cx,
                        <Stack orientation=StackOrientation::Vertical spacing=0 class="menu nested dense">
                            <Link href="overview" class="item">"Overview"</Link>
                            <Link href="installation" class="item">"Installation"</Link>
                            <Link href="usage" class="item">"Usage"</Link>
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
                            <Link href="stack" class="item">"Stack"</Link>
                            <Link href="grid" class="item">"Grid"</Link>
                        </Stack>
                    }
                />

                <Collapsible
                    open=true
                    header=view! {cx,
                        <Icon icon=BsIcon::BsToggles margin=Margin::Right(Size::Em(1.0))></Icon> "Components"
                    }
                    body=view! {cx,
                        <Stack orientation=StackOrientation::Vertical spacing=0 class="menu nested dense">
                            <Link href="button" class="item">"Button"</Link>
                            <Link href="tab" class="item">"Tabs"</Link>
                            <Link href="collapsible" class="item">"Collapsible"</Link>
                            <Link href="drawer" class="item">"Drawer"</Link>
                            <Link href="modal" class="item">"Modal"</Link>
                            <Link href="alert" class="item">"Alert"</Link>
                            <Link href="typography" class="item">"Typography"</Link>
                            <Link href="icon" class="item">"Icon"</Link>
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
                            <Link href="fade" class="item">"Fade"</Link>
                            <Link href="grow" class="item">"Grow"</Link>
                            <Link href="slide" class="item">"Slide"</Link>
                            <Link href="zoom" class="item">"Zoom"</Link>
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
