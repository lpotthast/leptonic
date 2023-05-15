use leptonic::prelude::*;
use leptos::*;
use leptos_icons::BsIcon;

use crate::app::Routes;

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
                            <Link href=Routes::Button class="item">"Button"</Link>
                            <Link href=Routes::Tab class="item">"Tabs"</Link>
                            <Link href=Routes::Input class="item">"Inputs"</Link>
                            <Link href=Routes::DateTime class="item">"Date & Time"</Link>
                            <Link href=Routes::Collapsible class="item">"Collapsible"</Link>
                            <Link href=Routes::AppBar class="item">"App Bar"</Link>
                            <Link href=Routes::Drawer class="item">"Drawer"</Link>
                            <Link href=Routes::Toast class="item">"Toast"</Link>
                            <Link href=Routes::Modal class="item">"Modal"</Link>
                            <Link href=Routes::Alert class="item">"Alert"</Link>
                            <Link href=Routes::Typography class="item">"Typography"</Link>
                            <Link href=Routes::Icon class="item">"Icon"</Link>
                            <Link href=Routes::ProgressIndicator class="item">"Progress"</Link>
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
    }
}
