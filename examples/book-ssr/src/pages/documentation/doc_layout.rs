use leptonic::{components::prelude::*, prelude::*};
use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::app::AppLayoutContext;
use crate::app::APP_BAR_HEIGHT;
use crate::routes;

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
                <Link href=routes::doc::Overview.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Overview"</Link>
                <Link href=routes::doc::Installation.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Installation"</Link>
                <Link href=routes::doc::Themes.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Themes"</Link>
                <Link href=routes::doc::Changelog.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Changelog"</Link>
            </Stack>
        </DrawerSection>

        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Hooks"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=routes::doc::hooks::UsePress.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_press"</Link>
                <Link href=routes::doc::hooks::UseMove.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_move"</Link>
                <Link href=routes::doc::hooks::UseHover.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_hover"</Link>
                <Link href=routes::doc::hooks::UseButton.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_button"</Link>
                <Link href=routes::doc::hooks::UseOverlay.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_overlay"</Link>
                <Link href=routes::doc::hooks::UseAnchorLink.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_anchor_link"</Link>
            </Stack>
        </DrawerSection>

        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Atoms"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=routes::doc::atoms::Button.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Button"</Link>
                <Link href=routes::doc::atoms::Popover.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Popover"</Link>
                <Link href=routes::doc::atoms::AnchorLink.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"AnchorLink"</Link>
            </Stack>
        </DrawerSection>

        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Components"
        }>
            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsColumnsGap margin=Margin::Right(Size::Em(1.0))></Icon> "Layout"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Stack.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Stack"</Link>
                    <Link href=routes::doc::components::Grid.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Grid"</Link>
                    <Link href=routes::doc::components::Separator.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Separator"</Link>
                    <Link href=routes::doc::components::Skeleton.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Skeleton"</Link>
                    <Link href=routes::doc::components::AppBar.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"App Bar"</Link>
                    <Link href=routes::doc::components::Drawer.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Drawer"</Link>
                    <Link href=routes::doc::components::Tabs.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Tabs"</Link>
                    <Link href=routes::doc::components::Table.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Table"</Link>
                    <Link href=routes::doc::components::Collapsible.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Collapsible"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsToggles margin=Margin::Right(Size::Em(1.0))></Icon> "Input"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Button.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Button"</Link>
                    <Link href=routes::doc::components::Input.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Input"</Link>
                    <Link href=routes::doc::components::TiptapEditor.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Tiptap editor"</Link>
                    <Link href=routes::doc::components::DateTime.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Date & Time"</Link>
                    <Link href=routes::doc::components::Slider.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Slider"</Link>
                    <Link href=routes::doc::components::Select.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Select"</Link>
                    <Link href=routes::doc::components::Checkbox.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Checkbox"</Link>
                    <Link href=routes::doc::components::Radio.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Radio"</Link>
                    <Link href=routes::doc::components::Toggle.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Toggle"</Link>
                    <Link href=routes::doc::components::ColorPicker.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Color Picker"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsChatSquare margin=Margin::Right(Size::Em(1.0))></Icon> "Feedback"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Alert.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Alert"</Link>
                    <Link href=routes::doc::components::Toast.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Toast"</Link>
                    <Link href=routes::doc::components::Modal.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Modal"</Link>
                    <Link href=routes::doc::components::Progress.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Progress"</Link>
                    <Link href=routes::doc::components::Popover.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Popover"</Link>
                    <Link href=routes::doc::components::Chip.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Chip"</Link>
                    <Link href=routes::doc::components::Kbd.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Kbd"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsCircleSquare margin=Margin::Right(Size::Em(1.0))></Icon> "General"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Typography.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Typography"</Link>
                    <Link href=routes::doc::components::Icon.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Icon"</Link>
                    <Link href=routes::doc::components::Link.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Link"</Link>
                    <Link href=routes::doc::components::Callback.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Callback"</Link>
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
pub fn DrawerSection(
    level: u32,
    #[prop(into)] header: ViewFn,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="drawer-section" data-level=level>
            <div class="section-header">
                { header.run() }
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
