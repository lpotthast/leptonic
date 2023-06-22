use leptonic::prelude::*;
use leptos::*;
use leptos_icons::BsIcon;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageDrawer(cx: Scope) -> impl IntoView {
    view! { cx,
        <H2>"Drawer"</H2>

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
                            <Link href=DocRoutes::Overview class="item">"Overview"</Link>
                            <Link href=DocRoutes::Installation class="item">"Installation"</Link>
                            <Link href=DocRoutes::Usage class="item">"Usage"</Link>
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
                            <Link href=DocRoutes::Stack class="item">"Stack"</Link>
                            <Link href=DocRoutes::Grid class="item">"Grid"</Link>
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
                            <Link href=DocRoutes::Button class="item">"Button"</Link>
                            <Link href=DocRoutes::Tab class="item">"Tabs"</Link>
                            <Link href=DocRoutes::Input class="item">"Inputs"</Link>
                            <Link href=DocRoutes::DateTime class="item">"Date & Time"</Link>
                            <Link href=DocRoutes::Collapsible class="item">"Collapsible"</Link>
                            <Link href=DocRoutes::AppBar class="item">"App Bar"</Link>
                            <Link href=DocRoutes::Drawer class="item">"Drawer"</Link>
                            <Link href=DocRoutes::Toast class="item">"Toast"</Link>
                            <Link href=DocRoutes::Modal class="item">"Modal"</Link>
                            <Link href=DocRoutes::Alert class="item">"Alert"</Link>
                            <Link href=DocRoutes::Typography class="item">"Typography"</Link>
                            <Link href=DocRoutes::Icon class="item">"Icon"</Link>
                            <Link href=DocRoutes::Progress class="item">"Progress"</Link>
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
                            <Link href=DocRoutes::Transition class="item">"Transitions"</Link>
                        </Stack>
                    }
                />
            </Stack>
        </Drawer>
    }
}
