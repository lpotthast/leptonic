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
            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsCursor margin=Margin::Right(Size::Em(1.0))></Icon> "Interactions"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::hooks::UsePress.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_press"</Link>
                    <Link href=routes::doc::hooks::UseMove.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_move"</Link>
                    <Link href=routes::doc::hooks::UseMoveWithin.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_move_within"</Link>
                    <Link href=routes::doc::hooks::UseHover.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_hover"</Link>
                    <Link href=routes::doc::hooks::UseKeyboard.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_keyboard"</Link>
                    <Link href=routes::doc::hooks::UseLongPress.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_long_press"</Link>
                    <Link href=routes::doc::hooks::UseInteractOutside.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_interact_outside"</Link>
                    <Link href=routes::doc::hooks::UseScrollWheel.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_scroll_wheel"</Link>
                    <Link href=routes::doc::hooks::UsePreventScroll.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_prevent_scroll"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsEye margin=Margin::Right(Size::Em(1.0))></Icon> "Focus"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::hooks::UseFocus.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_focus"</Link>
                    <Link href=routes::doc::hooks::UseFocusWithin.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_focus_within"</Link>
                    <Link href=routes::doc::hooks::UseFocusable.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_focusable"</Link>
                    <Link href=routes::doc::hooks::UseFocusRing.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_focus_ring"</Link>
                    <Link href=routes::doc::hooks::UseFocusManager.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_focus_manager"</Link>
                    <Link href=routes::doc::hooks::UseHasTabbableChild.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_has_tabbable_child"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsUiChecks margin=Margin::Right(Size::Em(1.0))></Icon> "Forms"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::hooks::UseLabel.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_label"</Link>
                    <Link href=routes::doc::hooks::UseTextField.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_text_field"</Link>
                    <Link href=routes::doc::hooks::UseCheckbox.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_checkbox"</Link>
                    <Link href=routes::doc::hooks::UseRadio.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_radio"</Link>
                    <Link href=routes::doc::hooks::UseSwitch.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_switch"</Link>
                    <Link href=routes::doc::hooks::UseSlider.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_slider"</Link>
                    <Link href=routes::doc::hooks::Selection.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"selection"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsWindowStack margin=Margin::Right(Size::Em(1.0))></Icon> "Overlays"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::hooks::UseOverlay.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_overlay"</Link>
                    <Link href=routes::doc::hooks::UsePopover.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_popover"</Link>
                    <Link href=routes::doc::hooks::UseModal.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_modal"</Link>
                    <Link href=routes::doc::hooks::UseTooltip.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_tooltip"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsList margin=Margin::Right(Size::Em(1.0))></Icon> "Selection"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::hooks::UseMenu.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_menu"</Link>
                    <Link href=routes::doc::hooks::UseListbox.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_listbox"</Link>
                    <Link href=routes::doc::hooks::UseSelect.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_select"</Link>
                    <Link href=routes::doc::hooks::UseCombobox.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_combobox"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsGrid margin=Margin::Right(Size::Em(1.0))></Icon> "Data Display"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::hooks::UseTabs.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_tabs"</Link>
                    <Link href=routes::doc::hooks::UseTable.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_table"</Link>
                    <Link href=routes::doc::hooks::UseGrid.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_grid"</Link>
                    <Link href=routes::doc::hooks::UseTree.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_tree"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsInfoCircle margin=Margin::Right(Size::Em(1.0))></Icon> "Feedback"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::hooks::UseProgressBar.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_progress_bar"</Link>
                    <Link href=routes::doc::hooks::UseMeter.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_meter"</Link>
                </Stack>
            </DrawerSection>

            <DrawerSection level=2 header=move || view! {
                <Icon icon=icondata::BsPuzzle margin=Margin::Right(Size::Em(1.0))></Icon> "Other"
            }>
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::hooks::UseButton.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_button"</Link>
                    <Link href=routes::doc::hooks::UseAnchorLink.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_anchor_link"</Link>
                    <Link href=routes::doc::hooks::UseLink.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_link"</Link>
                    <Link href=routes::doc::hooks::UseBreadcrumbs.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_breadcrumbs"</Link>
                    <Link href=routes::doc::hooks::UseDisclosure.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_disclosure"</Link>
                    <Link href=routes::doc::hooks::UseSeparator.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_separator"</Link>
                    <Link href=routes::doc::hooks::UseTag.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_tag"</Link>
                    <Link href=routes::doc::hooks::UseToolbar.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_toolbar"</Link>
                    <Link href=routes::doc::hooks::Dnd.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"drag & drop"</Link>
                </Stack>
            </DrawerSection>
        </DrawerSection>

        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Atoms"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=routes::doc::atoms::Button.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Button"</Link>
                <Link href=routes::doc::atoms::Popover.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Popover"</Link>
                <Link href=routes::doc::atoms::AnchorLink.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"AnchorLink"</Link>
                <Link href=routes::doc::atoms::FocusScope.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"FocusScope"</Link>
                <Link href=routes::doc::atoms::FocusRing.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()>"FocusRing"</Link>
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
        <div id="doc-layout" style=move || format!(
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
        </div>
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
