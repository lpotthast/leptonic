use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUseTabsHook() -> impl IntoView {
    // State to track which tab is selected
    let (selected_tab, set_selected_tab) = signal(Some("account".to_string()));

    // State to track which tab is focused (for roving tabindex)
    let (focused_tab_index, set_focused_tab_index) = signal(0usize);

    // Tab labels for display
    let tab_labels = ["Account", "Password", "Notifications"];

    // Set up the main tabs hook
    let tabs = use_tabs(UseTabsInput {
        selected_key: selected_tab.into(),
        default_selected_key: Some("account".to_string()),
        is_disabled: false.into(),
        orientation: TabsOrientation::Horizontal,
        activation_mode: TabsActivationMode::Automatic,
        on_selection_change: Some(Callback::new(move |key: String| {
            set_selected_tab.set(Some(key));
        })),
    });

    let id_base = tabs.id_base.clone();
    let select_tab = tabs.select_tab;
    let orientation = tabs.orientation;
    let activation_mode = tabs.activation_mode;

    // Set up the tab list hook with keyboard navigation callbacks
    let tab_list = use_tab_list(UseTabListInput {
        id_base: id_base.clone(),
        orientation,
        is_disabled: false.into(),
        label: Some("Settings".to_string()),
        on_focus_next: Some(Callback::new(move |_| {
            set_focused_tab_index.update(|i| *i = (*i + 1) % 3);
        })),
        on_focus_previous: Some(Callback::new(move |_| {
            set_focused_tab_index.update(|i| *i = if *i == 0 { 2 } else { *i - 1 });
        })),
        on_focus_first: Some(Callback::new(move |_| {
            set_focused_tab_index.set(0);
        })),
        on_focus_last: Some(Callback::new(move |_| {
            set_focused_tab_index.set(2);
        })),
    });

    // Set up individual tab hooks
    let id_base_for_tab0 = id_base.clone();
    let tab_account = use_tab(UseTabInput {
        tab_key: "account".to_string(),
        id_base: id_base_for_tab0,
        is_selected: Signal::derive(move || selected_tab.get() == Some("account".to_string())),
        is_disabled: false.into(),
        is_focused: Signal::derive(move || focused_tab_index.get() == 0),
        activation_mode,
        on_select: Some(Callback::new(move |_| {
            select_tab.run("account".to_string());
        })),
        on_focus: Some(Callback::new(move |_| {
            set_focused_tab_index.set(0);
        })),
    });

    let id_base_for_tab1 = id_base.clone();
    let tab_password = use_tab(UseTabInput {
        tab_key: "password".to_string(),
        id_base: id_base_for_tab1,
        is_selected: Signal::derive(move || selected_tab.get() == Some("password".to_string())),
        is_disabled: false.into(),
        is_focused: Signal::derive(move || focused_tab_index.get() == 1),
        activation_mode,
        on_select: Some(Callback::new(move |_| {
            select_tab.run("password".to_string());
        })),
        on_focus: Some(Callback::new(move |_| {
            set_focused_tab_index.set(1);
        })),
    });

    let id_base_for_tab2 = id_base.clone();
    let tab_notifications = use_tab(UseTabInput {
        tab_key: "notifications".to_string(),
        id_base: id_base_for_tab2,
        is_selected: Signal::derive(move || {
            selected_tab.get() == Some("notifications".to_string())
        }),
        is_disabled: false.into(),
        is_focused: Signal::derive(move || focused_tab_index.get() == 2),
        activation_mode,
        on_select: Some(Callback::new(move |_| {
            select_tab.run("notifications".to_string());
        })),
        on_focus: Some(Callback::new(move |_| {
            set_focused_tab_index.set(2);
        })),
    });

    // Set up tab panel hooks
    let id_base_for_panel0 = id_base.clone();
    let panel0 = use_tab_panel(UseTabPanelInput {
        panel_key: "account".to_string(),
        id_base: id_base_for_panel0,
        is_selected: Signal::derive(move || selected_tab.get() == Some("account".to_string())),
    });

    let id_base_for_panel1 = id_base.clone();
    let panel1 = use_tab_panel(UseTabPanelInput {
        panel_key: "password".to_string(),
        id_base: id_base_for_panel1,
        is_selected: Signal::derive(move || selected_tab.get() == Some("password".to_string())),
    });

    let id_base_for_panel2 = id_base.clone();
    let panel2 = use_tab_panel(UseTabPanelInput {
        panel_key: "notifications".to_string(),
        id_base: id_base_for_panel2,
        is_selected: Signal::derive(move || {
            selected_tab.get() == Some("notifications".to_string())
        }),
    });

    view! {
        <Article>
            <h1 id="tabs" class="anchor">
                "Tabs Hooks"
                <AnchorLink href="#tabs" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible tabbed interfaces with keyboard navigation and ARIA support."</p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <div style="margin: 1em 0;" {..tabs.props.into_attrs()}>
                // Tab list
                <div
                    {..tab_list.props.into_attrs()}
                    style="display: flex; border-bottom: 2px solid #ddd;"
                >
                    <button
                        {..tab_account.props.into_attrs()}
                        style=move || format!(
                            "padding: 0.75em 1.5em; border: none; background: transparent; cursor: pointer; font-size: 1em; position: relative; transition: all 0.2s; {}",
                            if tab_account.is_selected.get() {
                                "color: var(--brand-color); font-weight: bold; border-bottom: 2px solid var(--brand-color); margin-bottom: -2px;"
                            } else {
                                "color: #666;"
                            }
                        )
                    >
                        {tab_labels[0]}
                    </button>
                    <button
                        {..tab_password.props.into_attrs()}
                        style=move || format!(
                            "padding: 0.75em 1.5em; border: none; background: transparent; cursor: pointer; font-size: 1em; position: relative; transition: all 0.2s; {}",
                            if tab_password.is_selected.get() {
                                "color: var(--brand-color); font-weight: bold; border-bottom: 2px solid var(--brand-color); margin-bottom: -2px;"
                            } else {
                                "color: #666;"
                            }
                        )
                    >
                        {tab_labels[1]}
                    </button>
                    <button
                        {..tab_notifications.props.into_attrs()}
                        style=move || format!(
                            "padding: 0.75em 1.5em; border: none; background: transparent; cursor: pointer; font-size: 1em; position: relative; transition: all 0.2s; {}",
                            if tab_notifications.is_selected.get() {
                                "color: var(--brand-color); font-weight: bold; border-bottom: 2px solid var(--brand-color); margin-bottom: -2px;"
                            } else {
                                "color: #666;"
                            }
                        )
                    >
                        {tab_labels[2]}
                    </button>
                </div>

                // Tab panels
                <div
                    {..panel0.props.into_attrs()}
                    style=move || format!(
                        "padding: 1.5em; border: 1px solid #ddd; border-top: none; border-radius: 0 0 8px 8px; {}",
                        if panel0.is_selected.get() { "" } else { "display: none;" }
                    )
                >
                    <h3 style="margin: 0 0 0.5em 0;">{tab_labels[0]} " Settings"</h3>
                    <p style="margin: 0; color: #666;">"Configure your account settings here."</p>
                </div>
                <div
                    {..panel1.props.into_attrs()}
                    style=move || format!(
                        "padding: 1.5em; border: 1px solid #ddd; border-top: none; border-radius: 0 0 8px 8px; {}",
                        if panel1.is_selected.get() { "" } else { "display: none;" }
                    )
                >
                    <h3 style="margin: 0 0 0.5em 0;">{tab_labels[1]} " Settings"</h3>
                    <p style="margin: 0; color: #666;">"Configure your password settings here."</p>
                </div>
                <div
                    {..panel2.props.into_attrs()}
                    style=move || format!(
                        "padding: 1.5em; border: 1px solid #ddd; border-top: none; border-radius: 0 0 8px 8px; {}",
                        if panel2.is_selected.get() { "" } else { "display: none;" }
                    )
                >
                    <h3 style="margin: 0 0 0.5em 0;">{tab_labels[2]} " Settings"</h3>
                    <p style="margin: 0; color: #666;">"Configure your notifications settings here."</p>
                </div>
            </div>

            <h2 id="use_tabs" class="anchor">
                "use_tabs"
                <AnchorLink href="#use_tabs" description="Direct link to use_tabs"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseTabsReturn { props: tabs_props, id_base, select_tab, .. } = use_tabs(UseTabsInput {
                        selected_key: selected_tab.into(),
                        default_selected_key: Some("tab1".to_string()),
                        orientation: TabsOrientation::Horizontal,
                        activation_mode: TabsActivationMode::Automatic, // or Manual
                        on_selection_change: Some(Callback::new(move |key| set_selected_tab.set(Some(key)))),
                        is_disabled: false.into(),
                    });
                "#)}
            </Code>

            <h2 id="use_tab_list" class="anchor">
                "use_tab_list"
                <AnchorLink href="#use_tab_list" description="Direct link to use_tab_list"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseTabListReturn { props: tab_list_props, .. } = use_tab_list(UseTabListInput {
                        id_base: tabs.id_base.clone(),
                        orientation: TabsOrientation::Horizontal,
                        is_disabled: false.into(),
                        label: Some("Settings".to_string()),
                        on_focus_next: Some(Callback::new(move |_| { /* focus next tab */ })),
                        on_focus_previous: Some(Callback::new(move |_| { /* focus previous tab */ })),
                        on_focus_first: Some(Callback::new(move |_| { /* focus first tab */ })),
                        on_focus_last: Some(Callback::new(move |_| { /* focus last tab */ })),
                    });
                "#)}
            </Code>

            <p>"Provides:"</p>
            <ul>
                <li>"role=\"tablist\""</li>
                <li>"aria-label"</li>
                <li>"aria-orientation"</li>
                <li>"Keyboard navigation handlers"</li>
            </ul>

            <h2 id="use_tab" class="anchor">
                "use_tab"
                <AnchorLink href="#use_tab" description="Direct link to use_tab"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseTabReturn { props: tab_props, is_selected, is_focus_visible, .. } = use_tab(UseTabInput {
                        tab_key: "tab1".to_string(),
                        id_base: tabs.id_base.clone(),
                        is_selected: Signal::derive(move || selected_tab.get() == Some("tab1".to_string())),
                        is_disabled: false.into(),
                        is_focused: Signal::derive(move || focused_index.get() == 0),
                        activation_mode: TabsActivationMode::Automatic,
                        on_select: Some(Callback::new(move |_| select_tab.run("tab1".to_string()))),
                        on_focus: Some(Callback::new(move |_| set_focused_index.set(0))),
                    });
                "#)}
            </Code>

            <p>"Provides:"</p>
            <ul>
                <li>"role=\"tab\""</li>
                <li>"aria-selected"</li>
                <li>"aria-controls (links to panel)"</li>
                <li>"tabindex management"</li>
            </ul>

            <h2 id="use_tab_panel" class="anchor">
                "use_tab_panel"
                <AnchorLink href="#use_tab_panel" description="Direct link to use_tab_panel"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseTabPanelReturn { props: panel_props, is_selected, .. } = use_tab_panel(UseTabPanelInput {
                        panel_key: "tab1".to_string(),
                        id_base: tabs.id_base.clone(),
                        is_selected: Signal::derive(move || selected_tab.get() == Some("tab1".to_string())),
                    });
                "#)}
            </Code>

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><strong>"Arrow Left/Right"</strong> " - Navigate between tabs (horizontal)"</li>
                <li><strong>"Arrow Up/Down"</strong> " - Navigate between tabs (vertical)"</li>
                <li><strong>"Home"</strong> " - Focus first tab"</li>
                <li><strong>"End"</strong> " - Focus last tab"</li>
                <li><strong>"Enter/Space"</strong> " - Activate focused tab (manual mode)"</li>
            </ul>

            <h2 id="activation-modes" class="anchor">
                "Activation Modes"
                <AnchorLink href="#activation-modes" description="Direct link to activation modes"/>
            </h2>

            <ul>
                <li><code>"TabsActivationMode::Automatic"</code> " - Tab is selected immediately on focus"</li>
                <li><code>"TabsActivationMode::Manual"</code> " - Tab requires Enter/Space to select"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Horizontal and vertical orientations"</li>
                <li>"Automatic and manual activation"</li>
                <li>"Full keyboard navigation"</li>
                <li>"Disabled tabs support"</li>
                <li>"Proper ARIA associations"</li>
                <li>"Roving tabindex pattern"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Tabs Hooks", link: "#tabs" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "use_tabs", link: "#use_tabs" },
                Toc::Leaf { title: "use_tab_list", link: "#use_tab_list" },
                Toc::Leaf { title: "use_tab", link: "#use_tab" },
                Toc::Leaf { title: "use_tab_panel", link: "#use_tab_panel" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard" },
                Toc::Leaf { title: "Activation Modes", link: "#activation-modes" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
