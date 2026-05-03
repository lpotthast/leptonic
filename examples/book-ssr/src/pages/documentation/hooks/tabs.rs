use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::tabs::TabsDemo;

#[component]
pub fn PageUseTabsHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="tabs" class="anchor">
                "Tabs Hooks"
                <AnchorLink href="#tabs" description="Direct link to article header"/>
            </h1>

            <p>
                "Hooks for creating accessible tabbed interfaces with keyboard navigation and ARIA support. "
                "See the "<Link href=crate::routes::doc::Tabs.materialize()>"Tabs overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useTabList.html" target=LinkTarget::_Blank>
                    "useTabList"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell source=include_str!("demos/tabs.rs")>
                <TabsDemo />
            </DemoShell>

            <h2 id="use_tabs" class="anchor">
                "use_tabs"
                <AnchorLink href="#use_tabs" description="Direct link to use_tabs"/>
            </h2>

            <Code language=Language::Rust>
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

            <Code language=Language::Rust>
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

            <Code language=Language::Rust>
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

            <Code language=Language::Rust>
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
            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Tabs.materialize()>"Tabs overview"</Link></li>
                <li><Link href=crate::routes::doc::tabs::Component.materialize()>"Tabs component"</Link></li>
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
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
