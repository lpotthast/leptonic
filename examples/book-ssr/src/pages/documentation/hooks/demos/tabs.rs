use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn TabsDemo() -> impl IntoView {
    let (selected_tab, set_selected_tab) = signal(Some("account".to_string()));
    let (focused_tab_index, set_focused_tab_index) = signal(0usize);

    let tab_labels = ["Account", "Password", "Notifications"];

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

    let tab_list = use_tab_list(UseTabListInput {
        id_base: id_base.clone(),
        orientation,
        is_disabled: false.into(),
        label: Some("Settings".to_string()),
        on_focus_next: Some(Callback::new(move |()| {
            set_focused_tab_index.update(|i| *i = (*i + 1) % 3);
        })),
        on_focus_previous: Some(Callback::new(move |()| {
            set_focused_tab_index.update(|i| *i = if *i == 0 { 2 } else { *i - 1 });
        })),
        on_focus_first: Some(Callback::new(move |()| {
            set_focused_tab_index.set(0);
        })),
        on_focus_last: Some(Callback::new(move |()| {
            set_focused_tab_index.set(2);
        })),
    });

    let id_base_for_tab0 = id_base.clone();
    let tab_account = use_tab(UseTabInput {
        tab_key: "account".to_string(),
        id_base: id_base_for_tab0,
        is_selected: Signal::derive(move || selected_tab.get() == Some("account".to_string())),
        is_disabled: false.into(),
        is_focused: Signal::derive(move || focused_tab_index.get() == 0),
        activation_mode,
        on_select: Some(Callback::new(move |()| {
            select_tab.run("account".to_string());
        })),
        on_focus: Some(Callback::new(move |()| {
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
        on_select: Some(Callback::new(move |()| {
            select_tab.run("password".to_string());
        })),
        on_focus: Some(Callback::new(move |()| {
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
        on_select: Some(Callback::new(move |()| {
            select_tab.run("notifications".to_string());
        })),
        on_focus: Some(Callback::new(move |()| {
            set_focused_tab_index.set(2);
        })),
    });

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
        <div {..tabs.props.into_attrs()}>
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
    }
}
