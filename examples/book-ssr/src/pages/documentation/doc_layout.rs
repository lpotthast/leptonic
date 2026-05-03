use leptonic::{
    components::prelude::*,
    prelude::*,
    utils::css::{CssDimension, em},
};
use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::{app::AppLayoutContext, routes};

#[component]
pub fn DocLayout() -> impl IntoView {
    let app_layout_context = expect_context::<AppLayoutContext>();

    let close_doc_drawer_on_mobile = move || {
        if app_layout_context.is_small.get_untracked() {
            app_layout_context.close_doc_drawer();
        }
    };

    let drawer_content = view! {
        <DrawerSection level=1 header=move || view! {
            <Icon icon=icondata::BsBook margin=Margin::Right(em(1.0))></Icon> "Getting started"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=CssDimension::Zero classes="link-stack">
                <Link href=routes::doc::Overview.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Overview"</Link>
                <Link href=routes::doc::Installation.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Installation"</Link>
                <Link href=routes::doc::Themes.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Themes"</Link>
                <Link href=routes::doc::Changelog.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Changelog"</Link>
                <Link href=routes::doc::EventPropagation.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Event Propagation"</Link>
                <Link href=routes::doc::Architecture.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Hooks, Atoms & Components"</Link>
                <Link href=routes::doc::ClassesAndStyles.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Classes & Styles"</Link>
            </Stack>
        </DrawerSection>

        // ── Interactions ──────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Link href=routes::doc::Interactions.materialize() classes="drawer-domain-link"
                on:click=move |_| close_doc_drawer_on_mobile()>
                <Icon icon=icondata::BsCursor margin=Margin::Right(em(1.0))/>
                "Interactions"
            </Link>
        }>
            <Stack orientation=StackOrientation::Vertical spacing=CssDimension::Zero classes="link-stack">
                <Link href=routes::doc::interactions::UsePress.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_press"</Link>
                <Link href=routes::doc::interactions::PressResponder.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"PressResponder"</Link>
                <Link href=routes::doc::interactions::UseHover.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_hover"</Link>
                <Link href=routes::doc::interactions::UseMove.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_move"</Link>
                <Link href=routes::doc::interactions::UseKeyboard.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_keyboard"</Link>
                <Link href=routes::doc::interactions::UseInteractOutside.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_interact_outside"</Link>
                <Link href=routes::doc::interactions::UseScrollWheel.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_scroll_wheel"</Link>
                <Link href=routes::doc::interactions::UsePreventScroll.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_prevent_scroll"</Link>
                <Link href=routes::doc::interactions::Dnd.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Drag & Drop"</Link>
            </Stack>
        </DrawerSection>

        // ── Focus ─────────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Link href=routes::doc::Focus.materialize() classes="drawer-domain-link"
                on:click=move |_| close_doc_drawer_on_mobile()>
                <Icon icon=icondata::BsEye margin=Margin::Right(em(1.0))/>
                "Focus"
            </Link>
        }>
            <Stack orientation=StackOrientation::Vertical spacing=CssDimension::Zero classes="link-stack">
                <Link href=routes::doc::focus::UseFocus.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_focus"</Link>
                <Link href=routes::doc::focus::UseFocusWithin.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_focus_within"</Link>
                <Link href=routes::doc::focus::UseFocusable.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_focusable"</Link>
                <Link href=routes::doc::focus::UseFocusManager.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_focus_manager"</Link>
                <Link href=routes::doc::focus::UseHasTabbableChild.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_has_tabbable_child"</Link>
                <Link href=routes::doc::focus::UseFocusRing.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_focus_ring"</Link>
                <Link href=routes::doc::focus::UseFocusVisible.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_focus_visible"</Link>
                <Link href=routes::doc::focus::FocusScope.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"FocusScope"</Link>
                <Link href=routes::doc::focus::FocusRing.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"FocusRing"</Link>
            </Stack>
        </DrawerSection>

        // ── Overlays ──────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Link href=routes::doc::Overlays.materialize() classes="drawer-domain-link"
                on:click=move |_| close_doc_drawer_on_mobile()>
                <Icon icon=icondata::BsWindowStack margin=Margin::Right(em(1.0))/>
                "Overlays"
            </Link>
        }>
            <Stack orientation=StackOrientation::Vertical spacing=CssDimension::Zero classes="link-stack">
                <Link href=routes::doc::overlays::UseOverlay.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"use_overlay"</Link>
                <Link href=routes::doc::overlays::DismissButton.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"DismissButton"</Link>
            </Stack>
        </DrawerSection>

        // ── Selection ─────────────────────────────────────────────
        <div class="drawer-section" data-level="1">
            <Link href=routes::doc::SelectionDomain.materialize() classes="drawer-domain-link"
                on:click=move |_| close_doc_drawer_on_mobile()>
                <Icon icon=icondata::BsCheckSquare margin=Margin::Right(em(1.0))/>
                "Selection"
            </Link>
        </div>

        // ── Input ───────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Link href=routes::doc::InputCategory.materialize() classes="drawer-domain-link"
                on:click=move |_| close_doc_drawer_on_mobile()>
                <Icon icon=icondata::BsToggles margin=Margin::Right(em(1.0))/>
                "Input"
            </Link>
        }>
            <Stack orientation=StackOrientation::Vertical spacing=CssDimension::Zero classes="link-stack">
                // Concepts (alphabetical)
                <Link href=routes::doc::Button.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Button"</Link>
                <Link href=routes::doc::Checkbox.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Checkbox"</Link>
                <Link href=routes::doc::Color.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Color"</Link>
                <Link href=routes::doc::Combobox.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Combobox"</Link>
                <Link href=routes::doc::Listbox.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Listbox"</Link>
                <Link href=routes::doc::Radio.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Radio"</Link>
                <Link href=routes::doc::Select.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Select"</Link>
                <Link href=routes::doc::Slider.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Slider"</Link>
                <Link href=routes::doc::TextField.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Text Field"</Link>
                <Link href=routes::doc::Toggle.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Toggle"</Link>
                // Standalone items (alphabetical, with badges)
                <Link href=routes::doc::components::DateTime.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Date & Time"</Link>
                <Link href=routes::doc::components::TiptapEditor.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Tiptap Editor"</Link>
                <Link href=routes::doc::hooks::UseLabel.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_label"</Link>
            </Stack>
        </DrawerSection>

        // ── Data Display ────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Link href=routes::doc::DataDisplay.materialize() classes="drawer-domain-link"
                on:click=move |_| close_doc_drawer_on_mobile()>
                <Icon icon=icondata::BsGrid margin=Margin::Right(em(1.0))/>
                "Data Display"
            </Link>
        }>
            <Stack orientation=StackOrientation::Vertical spacing=CssDimension::Zero classes="link-stack">
                // Concepts (alphabetical)
                <Link href=routes::doc::Grid.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Grid"</Link>
                <Link href=routes::doc::Table.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Table"</Link>
                // Standalone items
                <Link href=routes::doc::hooks::UseTree.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_tree"</Link>
            </Stack>
        </DrawerSection>

        // ── Layout ──────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Link href=routes::doc::LayoutCategory.materialize() classes="drawer-domain-link"
                on:click=move |_| close_doc_drawer_on_mobile()>
                <Icon icon=icondata::BsColumnsGap margin=Margin::Right(em(1.0))/>
                "Layout"
            </Link>
        }>
            <Stack orientation=StackOrientation::Vertical spacing=CssDimension::Zero classes="link-stack">
                // Concepts (alphabetical)
                <Link href=routes::doc::Collapsible.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Collapsible"</Link>
                <Link href=routes::doc::Separator.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Separator"</Link>
                <Link href=routes::doc::Tabs.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Tabs"</Link>
                // Standalone items (alphabetical, with badges)
                <Link href=routes::doc::components::AppBar.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "App Bar"</Link>
                <Link href=routes::doc::components::Drawer.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Drawer"</Link>
                <Link href=routes::doc::components::Skeleton.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Skeleton"</Link>
                <Link href=routes::doc::components::Stack.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Stack"</Link>
                <Link href=routes::doc::hooks::UseToolbar.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_toolbar"</Link>
            </Stack>
        </DrawerSection>

        // ── Feedback ────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Link href=routes::doc::Feedback.materialize() classes="drawer-domain-link"
                on:click=move |_| close_doc_drawer_on_mobile()>
                <Icon icon=icondata::BsChatSquare margin=Margin::Right(em(1.0))/>
                "Feedback"
            </Link>
        }>
            <Stack orientation=StackOrientation::Vertical spacing=CssDimension::Zero classes="link-stack">
                // Concepts (alphabetical)
                <Link href=routes::doc::Chip.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Chip"</Link>
                <Link href=routes::doc::Modal.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Modal"</Link>
                <Link href=routes::doc::Popover.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Popover"</Link>
                <Link href=routes::doc::Progress.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Progress"</Link>
                <Link href=routes::doc::Tooltip.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Tooltip"</Link>
                // Standalone items (alphabetical, with badges)
                <Link href=routes::doc::components::Alert.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Alert"</Link>
                <Link href=routes::doc::components::Kbd.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Kbd"</Link>
                <Link href=routes::doc::components::Toast.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Toast"</Link>
                <Link href=routes::doc::hooks::UseMeter.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_meter"</Link>
            </Stack>
        </DrawerSection>

        // ── Navigation ──────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Link href=routes::doc::Navigation.materialize() classes="drawer-domain-link"
                on:click=move |_| close_doc_drawer_on_mobile()>
                <Icon icon=icondata::BsSignpost margin=Margin::Right(em(1.0))/>
                "Navigation"
            </Link>
        }>
            <Stack orientation=StackOrientation::Vertical spacing=CssDimension::Zero classes="link-stack">
                // Concepts (alphabetical)
                <Link href=routes::doc::Link.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Link"</Link>
                <Link href=routes::doc::Menu.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()>"Menu"</Link>
                // Standalone items
                <Link href=routes::doc::hooks::UseBreadcrumbs.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="hook"/> "use_breadcrumbs"</Link>
            </Stack>
        </DrawerSection>

        // ── General ─────────────────────────────────────────────
        <DrawerSection level=1 header=move || view! {
            <Link href=routes::doc::General.materialize() classes="drawer-domain-link"
                on:click=move |_| close_doc_drawer_on_mobile()>
                <Icon icon=icondata::BsCircleSquare margin=Margin::Right(em(1.0))/>
                "General"
            </Link>
        }>
            <Stack orientation=StackOrientation::Vertical spacing=CssDimension::Zero classes="link-stack">
                <Link href=routes::doc::components::Typography.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Typography"</Link>
                <Link href=routes::doc::components::Icon.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Icon"</Link>
                <Link href=routes::doc::components::Callback.materialize() classes="item" on:click=move |_| close_doc_drawer_on_mobile()><DocBadge kind="comp"/> "Callback"</Link>
            </Stack>
        </DrawerSection>
    };

    view! {
        <div id="doc-layout" style=move || format!(
            "margin-left: {}em; margin-right: {}em;",
            if !app_layout_context.is_small.get() && !app_layout_context.doc_drawer_closed.get() {
                16 // desktop: drawer open
            } else {
                0 // mobile: drawer overlays, or desktop: drawer closed
            },
            if app_layout_context.is_medium.get() { 0 } else { 12 },
        )>
            <Drawer
                side=DrawerSide::Left
                attr:id="doc-drawer"
                shown=Signal::derive(move || !app_layout_context.doc_drawer_closed.get())
            >
                <Stack orientation=StackOrientation::Vertical spacing=CssDimension::Zero classes="menu">
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
#[allow(clippy::needless_pass_by_value)]
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
