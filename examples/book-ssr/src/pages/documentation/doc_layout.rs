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

        // ── Interactions ──────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsCursor margin=Margin::Right(Size::Em(1.0))></Icon> "Interactions"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=routes::doc::hooks::UsePress.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_press"</Link>
                <Link href=routes::doc::hooks::UseHover.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_hover"</Link>
                <Link href=routes::doc::hooks::UseMove.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_move"</Link>
                <Link href=routes::doc::hooks::UseMoveWithin.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_move_within"</Link>
                <Link href=routes::doc::hooks::UseKeyboard.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_keyboard"</Link>
                <Link href=routes::doc::hooks::UseInteractOutside.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_interact_outside"</Link>
                <Link href=routes::doc::hooks::UseScrollWheel.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_scroll_wheel"</Link>
                <Link href=routes::doc::hooks::UsePreventScroll.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_prevent_scroll"</Link>
                <Link href=routes::doc::hooks::Dnd.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "Drag & Drop"</Link>
            </Stack>
        </DrawerSection>

        // ── Focus ─────────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsEye margin=Margin::Right(Size::Em(1.0))></Icon> "Focus"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=routes::doc::atoms::FocusScope.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="atom"/> "FocusScope"</Link>
                <Link href=routes::doc::hooks::UseFocus.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_focus"</Link>
                <Link href=routes::doc::hooks::UseFocusWithin.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_focus_within"</Link>
                <Link href=routes::doc::hooks::UseFocusable.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_focusable"</Link>
                <Link href=routes::doc::hooks::UseFocusManager.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_focus_manager"</Link>
                <Link href=routes::doc::hooks::UseHasTabbableChild.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_has_tabbable_child"</Link>
            </Stack>
            <DrawerSection level=3 header=move || "Focus Ring">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::atoms::FocusRing.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="atom"/> "FocusRing"</Link>
                    <Link href=routes::doc::hooks::UseFocusRing.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_focus_ring"</Link>
                </Stack>
            </DrawerSection>
        </DrawerSection>

        // ── Overlays ──────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsWindowStack margin=Margin::Right(Size::Em(1.0))></Icon> "Overlays"
        }>
            <DrawerSection level=3 header=move || "Overlay">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::hooks::UseOverlay.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_overlay"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Popover">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Popover.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Popover"</Link>
                    <Link href=routes::doc::atoms::Popover.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="atom"/> "Popover"</Link>
                    <Link href=routes::doc::hooks::UsePopover.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_popover"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Tooltip">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::hooks::UseTooltip.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_tooltip"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Modal">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Modal.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Modal"</Link>
                    <Link href=routes::doc::hooks::UseModal.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_modal"</Link>
                </Stack>
            </DrawerSection>
        </DrawerSection>

        // ── Input ─────────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsToggles margin=Margin::Right(Size::Em(1.0))></Icon> "Input"
        }>
            <DrawerSection level=3 header=move || "Button">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Button.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Button"</Link>
                    <Link href=routes::doc::atoms::Button.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="atom"/> "Button"</Link>
                    <Link href=routes::doc::hooks::UseButton.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_button"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Text Field">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Input.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Input"</Link>
                    <Link href=routes::doc::hooks::UseTextField.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_text_field"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Checkbox">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Checkbox.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Checkbox"</Link>
                    <Link href=routes::doc::hooks::UseCheckbox.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_checkbox"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Radio">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Radio.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Radio"</Link>
                    <Link href=routes::doc::hooks::UseRadio.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_radio"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Toggle">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Toggle.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Toggle"</Link>
                    <Link href=routes::doc::hooks::UseSwitch.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_switch"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Slider">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Slider.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Slider"</Link>
                    <Link href=routes::doc::atoms::Slider.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="atom"/> "Slider"</Link>
                    <Link href=routes::doc::hooks::UseSlider.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_slider"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Select">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Select.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Select"</Link>
                    <Link href=routes::doc::hooks::UseSelect.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_select"</Link>
                </Stack>
            </DrawerSection>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=routes::doc::hooks::UseLabel.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_label"</Link>
                <Link href=routes::doc::hooks::UseCombobox.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_combobox"</Link>
                <Link href=routes::doc::hooks::UseListbox.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_listbox"</Link>
                <Link href=routes::doc::hooks::UseMenu.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_menu"</Link>
                <Link href=routes::doc::hooks::Selection.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "selection"</Link>
                <Link href=routes::doc::components::ColorPicker.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Color Picker"</Link>
                <Link href=routes::doc::components::DateTime.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Date & Time"</Link>
                <Link href=routes::doc::components::TiptapEditor.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Tiptap Editor"</Link>
            </Stack>
        </DrawerSection>

        // ── Data Display ──────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsGrid margin=Margin::Right(Size::Em(1.0))></Icon> "Data Display"
        }>
            <DrawerSection level=3 header=move || "Grid">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Grid.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Grid"</Link>
                    <Link href=routes::doc::atoms::Grid.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="atom"/> "Grid"</Link>
                    <Link href=routes::doc::hooks::UseGrid.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_grid"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Table">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Table.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Table"</Link>
                    <Link href=routes::doc::hooks::UseTable.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_table"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Tree">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::hooks::UseTree.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_tree"</Link>
                </Stack>
            </DrawerSection>
        </DrawerSection>

        // ── Layout ────────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsColumnsGap margin=Margin::Right(Size::Em(1.0))></Icon> "Layout"
        }>
            <DrawerSection level=3 header=move || "Separator">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Separator.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Separator"</Link>
                    <Link href=routes::doc::hooks::UseSeparator.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_separator"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Tabs">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Tabs.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Tabs"</Link>
                    <Link href=routes::doc::hooks::UseTabs.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_tabs"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Collapsible">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Collapsible.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Collapsible"</Link>
                    <Link href=routes::doc::hooks::UseDisclosure.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_disclosure"</Link>
                </Stack>
            </DrawerSection>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=routes::doc::components::Stack.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Stack"</Link>
                <Link href=routes::doc::components::Skeleton.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Skeleton"</Link>
                <Link href=routes::doc::components::AppBar.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "App Bar"</Link>
                <Link href=routes::doc::components::Drawer.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Drawer"</Link>
                <Link href=routes::doc::hooks::UseToolbar.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_toolbar"</Link>
            </Stack>
        </DrawerSection>

        // ── Feedback ──────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsChatSquare margin=Margin::Right(Size::Em(1.0))></Icon> "Feedback"
        }>
            <DrawerSection level=3 header=move || "Progress">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Progress.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Progress"</Link>
                    <Link href=routes::doc::hooks::UseProgressBar.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_progress_bar"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Chip">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::components::Chip.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Chip"</Link>
                    <Link href=routes::doc::hooks::UseTag.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_tag"</Link>
                </Stack>
            </DrawerSection>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=routes::doc::components::Alert.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Alert"</Link>
                <Link href=routes::doc::components::Toast.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Toast"</Link>
                <Link href=routes::doc::components::Kbd.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Kbd"</Link>
                <Link href=routes::doc::hooks::UseMeter.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_meter"</Link>
            </Stack>
        </DrawerSection>

        // ── Navigation ────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsSignpost margin=Margin::Right(Size::Em(1.0))></Icon> "Navigation"
        }>
            <DrawerSection level=3 header=move || "Link">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::atoms::Link.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="atom"/> "Link"</Link>
                    <Link href=routes::doc::hooks::UseLink.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_link"</Link>
                </Stack>
            </DrawerSection>
            <DrawerSection level=3 header=move || "Anchor Link">
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                    <Link href=routes::doc::atoms::AnchorLink.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="atom"/> "AnchorLink"</Link>
                    <Link href=routes::doc::hooks::UseAnchorLink.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_anchor_link"</Link>
                </Stack>
            </DrawerSection>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=routes::doc::hooks::UseBreadcrumbs.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_breadcrumbs"</Link>
            </Stack>
        </DrawerSection>

        // ── General ───────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsCircleSquare margin=Margin::Right(Size::Em(1.0))></Icon> "General"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero attr:class="link-stack">
                <Link href=routes::doc::components::Typography.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Typography"</Link>
                <Link href=routes::doc::components::Icon.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Icon"</Link>
                <Link href=routes::doc::components::Callback.materialize() attr:class="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Callback"</Link>
            </Stack>
        </DrawerSection>
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
fn DocBadge(#[prop(into)] kind: &'static str) -> impl IntoView {
    view! {
        <span class=format!("doc-badge doc-badge-{kind}")>{kind}</span>
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
