use std::sync::Arc;

use leptonic::{
    atoms::{button::Button, focus_scope::FocusScope},
    components::{root::Root, theme::LeptonicTheme},
    hooks::{
        get_modality, use_focus, use_focus_manager, use_focus_ring, use_focus_visible,
        use_focus_within, use_focusable, use_has_tabbable_child, FocusManager, FocusManagerOptions,
        IntoAttrs, Modality, UseFocusInput, UseFocusManagerInput, UseFocusRingInput,
        UseFocusVisibleInput, UseFocusWithinInput, UseFocusableInput, UseHasTabbableChildInput,
    },
};
use leptos::{prelude::*, web_sys};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{components::*, path};

pub const LEPTOS_OUTPUT_NAME: &str = env!("LEPTOS_OUTPUT_NAME");

/// Prevent mousedown from stealing focus (used on control buttons in focus-manager tests).
fn prevent_focus_steal(e: web_sys::MouseEvent) {
    e.prevent_default();
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href=format!("/pkg/{LEPTOS_OUTPUT_NAME}.css") />
        <Title text="Leptonic Test App" />

        <Root default_theme=LeptonicTheme::default()>
            <Router>
                <Routes fallback=|| view! { <p>"Not Found"</p> }>
                    <Route path=path!("/") view=PageIndex />
                    <Route path=path!("/atoms/button") view=PageAtomButton />
                    <Route path=path!("/hooks/focus") view=PageHookFocus />
                    <Route path=path!("/hooks/focus-within") view=PageHookFocusWithin />
                    <Route path=path!("/hooks/focus-ring") view=PageHookFocusRing />
                    <Route path=path!("/hooks/focusable") view=PageHookFocusable />
                    <Route path=path!("/hooks/focus-manager") view=PageHookFocusManager />
                    <Route path=path!("/hooks/focus-visible") view=PageHookFocusVisible />
                    <Route path=path!("/hooks/has-tabbable-child") view=PageHookHasTabbableChild />
                    <Route path=path!("/atoms/focus-scope") view=PageAtomFocusScope />
                </Routes>
            </Router>
        </Root>
    }
}

#[component]
fn PageIndex() -> impl IntoView {
    view! {
        <div id="test-index">
            <h1>"Leptonic Test App"</h1>
            <p>"Available test pages:"</p>
            <ul>
                <li>
                    <a href="/atoms/button">"Atoms: Button"</a>
                </li>
                <li>
                    <a href="/hooks/focus">"Hooks: Focus"</a>
                </li>
                <li>
                    <a href="/hooks/focus-within">"Hooks: Focus Within"</a>
                </li>
                <li>
                    <a href="/hooks/focus-ring">"Hooks: Focus Ring"</a>
                </li>
                <li>
                    <a href="/hooks/focusable">"Hooks: Focusable"</a>
                </li>
                <li>
                    <a href="/hooks/focus-manager">"Hooks: Focus Manager"</a>
                </li>
                <li>
                    <a href="/hooks/focus-visible">"Hooks: Focus Visible"</a>
                </li>
                <li>
                    <a href="/hooks/has-tabbable-child">"Hooks: Has Tabbable Child"</a>
                </li>
                <li>
                    <a href="/atoms/focus-scope">"Atoms: Focus Scope"</a>
                </li>
            </ul>
        </div>
    }
}

#[component]
fn PageAtomButton() -> impl IntoView {
    let (basic_count, set_basic_count) = signal(0u32);
    let (disabled_count, set_disabled_count) = signal(0u32);

    view! {
        <div id="test-page-atom-button">
            <h1>"Button Atom Test Page"</h1>

            <section>
                <h2>"Basic Button"</h2>
                <Button
                    on_press=move |_| set_basic_count.update(|c| *c += 1)
                    attr:id="test-button-basic"
                >
                    "Press me"
                </Button>
                <div>"Press count: " <span id="test-button-basic-count">{basic_count}</span></div>
            </section>

            <section>
                <h2>"Disabled Button"</h2>
                <Button
                    on_press=move |_| set_disabled_count.update(|c| *c += 1)
                    disabled=Signal::from(true)
                    attr:id="test-button-disabled"
                >
                    "Disabled"
                </Button>
                <div>
                    "Press count: " <span id="test-button-disabled-count">{disabled_count}</span>
                </div>
            </section>
        </div>
    }
}

#[component]
fn PageHookFocus() -> impl IntoView {
    let (focus_count, set_focus_count) = signal(0u32);
    let (blur_count, set_blur_count) = signal(0u32);
    let (is_focused, set_is_focused) = signal(false);
    let (focus_change_count, set_focus_change_count) = signal(0u32);

    let focus = use_focus(UseFocusInput {
        disabled: Signal::derive(|| false),
        on_focus: Some(Callback::new(move |_| {
            set_focus_count.update(|c| *c += 1);
        })),
        on_blur: Some(Callback::new(move |_| {
            set_blur_count.update(|c| *c += 1);
        })),
        on_focus_change: Some(Callback::new(move |focused| {
            set_is_focused.set(focused);
            set_focus_change_count.update(|c| *c += 1);
        })),
    });

    let (disabled_focus_count, set_disabled_focus_count) = signal(0u32);

    let disabled_focus = use_focus(UseFocusInput {
        disabled: Signal::derive(|| true),
        on_focus: Some(Callback::new(move |_| {
            set_disabled_focus_count.update(|c| *c += 1);
        })),
        on_blur: None,
        on_focus_change: None,
    });

    view! {
        <div id="test-page-hook-focus">
            <h1>"Focus Hook Test Page"</h1>

            <section>
                <h2>"Basic Focus"</h2>
                <button id="test-focus-before">"Before"</button>

                <div id="test-focus-target" tabindex="0" {..focus.props.into_attrs()}>
                    "Focus Target"
                </div>

                <div>"Focus count: " <span id="test-focus-count">{focus_count}</span></div>
                <div>"Blur count: " <span id="test-blur-count">{blur_count}</span></div>
                <div>
                    "Is focused: "
                    <span id="test-is-focused">
                        {move || if is_focused.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>
                    "Focus change count: "
                    <span id="test-focus-change-count">{focus_change_count}</span>
                </div>
            </section>

            <section>
                <h2>"Disabled Focus"</h2>
                <div id="test-focus-disabled" tabindex="0" {..disabled_focus.props.into_attrs()}>
                    "Disabled Focus Target"
                </div>
                <div>
                    "Disabled focus count: "
                    <span id="test-disabled-focus-count">{disabled_focus_count}</span>
                </div>
            </section>

            <button id="test-focus-elsewhere">"Elsewhere"</button>

            <section>
                <h2>"Child Focus Filtering"</h2>
                {
                    let (parent_focus_count, set_parent_focus_count) = signal(0u32);
                    let (parent_blur_count, set_parent_blur_count) = signal(0u32);
                    let parent_focus = use_focus(UseFocusInput {
                        disabled: Signal::derive(|| false),
                        on_focus: Some(
                            Callback::new(move |_| {
                                set_parent_focus_count.update(|c| *c += 1);
                            }),
                        ),
                        on_blur: Some(
                            Callback::new(move |_| {
                                set_parent_blur_count.update(|c| *c += 1);
                            }),
                        ),
                        on_focus_change: None,
                    });

                    view! {
                        <div
                            id="test-focus-parent"
                            tabindex="0"
                            {..parent_focus.props.into_attrs()}
                        >
                            "Parent"
                            <button id="test-focus-child">"Child"</button>
                        </div>
                        <div>
                            "Parent focus count: "
                            <span id="test-focus-parent-focus-count">{parent_focus_count}</span>
                        </div>
                        <div>
                            "Parent blur count: "
                            <span id="test-focus-parent-blur-count">{parent_blur_count}</span>
                        </div>
                    }
                }
            </section>
        </div>
    }
}

#[component]
fn PageHookFocusWithin() -> impl IntoView {
    let (focus_within_count, set_focus_within_count) = signal(0u32);
    let (blur_within_count, set_blur_within_count) = signal(0u32);

    let focus_within = use_focus_within(UseFocusWithinInput {
        disabled: Signal::derive(|| false),
        on_focus_within: Some(Callback::new(move |_| {
            set_focus_within_count.update(|c| *c += 1);
        })),
        on_blur_within: Some(Callback::new(move |_| {
            set_blur_within_count.update(|c| *c += 1);
        })),
        on_focus_within_change: None,
    });

    let is_focus_within = focus_within.is_focus_within;

    // ---- Disabled section ----
    let (disabled_focus_count, set_disabled_focus_count) = signal(0u32);

    let disabled_fw = use_focus_within(UseFocusWithinInput {
        disabled: Signal::derive(|| true),
        on_focus_within: Some(Callback::new(move |_| {
            set_disabled_focus_count.update(|c| *c += 1);
        })),
        on_blur_within: None,
        on_focus_within_change: None,
    });

    let disabled_is_focus_within = disabled_fw.is_focus_within;

    // ---- Change callback section ----
    let (change_value, set_change_value) = signal(false);
    let (change_count, set_change_count) = signal(0u32);

    let change_fw = use_focus_within(UseFocusWithinInput {
        disabled: Signal::derive(|| false),
        on_focus_within: None,
        on_blur_within: None,
        on_focus_within_change: Some(Callback::new(move |is_within: bool| {
            set_change_value.set(is_within);
            set_change_count.update(|c| *c += 1);
        })),
    });

    view! {
        <div id="test-page-hook-focus-within">
            <h1>"Focus Within Hook Test Page"</h1>

            <section>
                <h2>"Basic Focus Within"</h2>
                <button id="test-fw-before">"Before"</button>

                <div id="test-fw-container" {..focus_within.props.into_attrs()}>
                    <input id="test-fw-input-a" type="text" placeholder="Input A" />
                    <input id="test-fw-input-b" type="text" placeholder="Input B" />
                </div>

                <div>
                    "Is focus-within: "
                    <span id="test-fw-is-focus-within">
                        {move || if is_focus_within.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>
                    "Focus-within count: "
                    <span id="test-fw-focus-within-count">{focus_within_count}</span>
                </div>
                <div>
                    "Blur-within count: "
                    <span id="test-fw-blur-within-count">{blur_within_count}</span>
                </div>

                <button id="test-fw-outside">"Outside"</button>
            </section>

            <section>
                <h2>"Disabled Focus Within"</h2>
                <div id="test-fw-disabled-container" {..disabled_fw.props.into_attrs()}>
                    <input id="test-fw-disabled-input" type="text" placeholder="Disabled Input" />
                </div>

                <div>
                    "Is focus-within: "
                    <span id="test-fw-disabled-is-focus-within">
                        {move || if disabled_is_focus_within.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>
                    "Focus count: "
                    <span id="test-fw-disabled-focus-count">{disabled_focus_count}</span>
                </div>
            </section>

            <section>
                <h2>"Change Callback"</h2>
                <div id="test-fw-change-container" {..change_fw.props.into_attrs()}>
                    <input id="test-fw-change-input" type="text" placeholder="Change Input" />
                </div>

                <div>
                    "Change value: "
                    <span id="test-fw-change-value">
                        {move || if change_value.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>"Change count: " <span id="test-fw-change-count">{change_count}</span></div>
            </section>

            <section>
                <h2>"Nested Containers"</h2>
                {
                    let nested_outer_fw = use_focus_within(UseFocusWithinInput {
                        disabled: Signal::derive(|| false),
                        on_focus_within: None,
                        on_blur_within: None,
                        on_focus_within_change: None,
                    });
                    let nested_outer_is_fw = nested_outer_fw.is_focus_within;
                    let nested_inner_fw = use_focus_within(UseFocusWithinInput {
                        disabled: Signal::derive(|| false),
                        on_focus_within: None,
                        on_blur_within: None,
                        on_focus_within_change: None,
                    });
                    let nested_inner_is_fw = nested_inner_fw.is_focus_within;

                    view! {
                        <div id="test-fw-nested-outer" {..nested_outer_fw.props.into_attrs()}>
                            <div id="test-fw-nested-inner" {..nested_inner_fw.props.into_attrs()}>
                                <input
                                    id="test-fw-nested-input"
                                    type="text"
                                    placeholder="Nested Input"
                                />
                            </div>
                        </div>
                        <div>
                            "Outer is focus-within: "
                            <span id="test-fw-nested-outer-is-focus-within">
                                {move || if nested_outer_is_fw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                        <div>
                            "Inner is focus-within: "
                            <span id="test-fw-nested-inner-is-focus-within">
                                {move || if nested_inner_is_fw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                    }
                }
            </section>
        </div>
    }
}

#[component]
fn PageHookFocusRing() -> impl IntoView {
    let focus_ring = use_focus_ring(UseFocusRingInput {
        disabled: Signal::derive(|| false),
        within: false,
        auto_focus: false,
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    let is_focus_visible = focus_ring.is_focus_visible;
    let is_focused = focus_ring.is_focused;

    let focus_ring_within = use_focus_ring(UseFocusRingInput {
        disabled: Signal::derive(|| false),
        within: true,
        auto_focus: false,
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    let within_is_focus_visible = focus_ring_within.is_focus_visible;
    let within_is_focused = focus_ring_within.is_focused;

    let disabled_focus_ring = use_focus_ring(UseFocusRingInput {
        disabled: Signal::derive(|| true),
        within: false,
        auto_focus: false,
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    let disabled_is_focus_visible = disabled_focus_ring.is_focus_visible;
    let disabled_is_focused = disabled_focus_ring.is_focused;

    view! {
        <div id="test-page-hook-focus-ring">
            <h1>"Focus Ring Hook Test Page"</h1>

            <section>
                <h2>"Basic Focus Ring"</h2>
                <button id="test-fr-before">"Before"</button>

                <div id="test-fr-target" tabindex="0" {..focus_ring.props.into_attrs()}>
                    "Focus Ring Target"
                </div>

                <div>
                    "Is focus-visible: "
                    <span id="test-fr-is-focus-visible">
                        {move || if is_focus_visible.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>
                    "Is focused: "
                    <span id="test-fr-is-focused">
                        {move || if is_focused.get() { "true" } else { "false" }}
                    </span>
                </div>

                <button id="test-fr-elsewhere">"Elsewhere"</button>
            </section>

            <section>
                <h2>"Within Mode"</h2>
                <button id="test-fr-within-before">"Within Before"</button>

                <div id="test-fr-within-container" {..focus_ring_within.props.into_attrs()}>
                    <button id="test-fr-within-child-1">"Within Child 1"</button>
                    <button id="test-fr-within-child-2">"Within Child 2"</button>
                </div>

                <div>
                    "Within is focus-visible: "
                    <span id="test-fr-within-is-focus-visible">
                        {move || if within_is_focus_visible.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>
                    "Within is focused: "
                    <span id="test-fr-within-is-focused">
                        {move || if within_is_focused.get() { "true" } else { "false" }}
                    </span>
                </div>

                <button id="test-fr-within-elsewhere">"Within Elsewhere"</button>
            </section>

            <section>
                <h2>"Disabled Focus Ring"</h2>
                <div
                    id="test-fr-disabled-target"
                    tabindex="0"
                    {..disabled_focus_ring.props.into_attrs()}
                >
                    "Disabled Focus Ring Target"
                </div>

                <div>
                    "Is focused: "
                    <span id="test-fr-disabled-is-focused">
                        {move || if disabled_is_focused.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>
                    "Is focus-visible: "
                    <span id="test-fr-disabled-is-focus-visible">
                        {move || if disabled_is_focus_visible.get() { "true" } else { "false" }}
                    </span>
                </div>
            </section>
        </div>
    }
}

#[component]
fn PageHookFocusable() -> impl IntoView {
    let (keydown_count, set_keydown_count) = signal(0u32);
    let (keyup_count, set_keyup_count) = signal(0u32);

    let normal = use_focusable(UseFocusableInput {
        disabled: Signal::derive(|| false),
        auto_focus: false,
        exclude_from_tab_order: Signal::derive(|| false),
        on_key_down: Some(Callback::new(move |_| {
            set_keydown_count.update(|c| *c += 1);
        })),
        on_key_up: Some(Callback::new(move |_| {
            set_keyup_count.update(|c| *c += 1);
        })),
        ..Default::default()
    });

    let normal_focus_handle = normal.focus_handle;

    let disabled = use_focusable(UseFocusableInput {
        disabled: Signal::derive(|| true),
        ..Default::default()
    });

    let excluded = use_focusable(UseFocusableInput {
        disabled: Signal::derive(|| false),
        exclude_from_tab_order: Signal::derive(|| true),
        ..Default::default()
    });

    let autofocus = use_focusable(UseFocusableInput {
        disabled: Signal::derive(|| false),
        auto_focus: true,
        ..Default::default()
    });

    view! {
        <div id="test-page-hook-focusable">
            <h1>"Focusable Hook Test Page"</h1>

            <section>
                <h2>"Focusable Elements"</h2>
                <div id="test-fcbl-normal" {..normal.props.into_attrs()}>
                    "Normal Focusable"
                </div>

                <div id="test-fcbl-disabled" {..disabled.props.into_attrs()}>
                    "Disabled Focusable"
                </div>

                <div id="test-fcbl-excluded" {..excluded.props.into_attrs()}>
                    "Excluded Focusable"
                </div>

                <button id="test-fcbl-tab-target">"Tab Target"</button>

                <div id="test-fcbl-autofocus" {..autofocus.props.into_attrs()}>
                    "Auto-focus Focusable"
                </div>
            </section>

            <section>
                <h2>"Keyboard Events"</h2>
                <div>
                    "Keydown count: " <span id="test-fcbl-keydown-count">{keydown_count}</span>
                </div>
                <div>"Keyup count: " <span id="test-fcbl-keyup-count">{keyup_count}</span></div>

                <button
                    id="test-fcbl-focus-btn"
                    on:mousedown=prevent_focus_steal
                    on:click=move |_| normal_focus_handle.focus()
                >
                    "Programmatic Focus"
                </button>
            </section>

            <section>
                <h2>"Dynamic Disabled"</h2>
                {
                    let (dynamic_disabled, set_dynamic_disabled) = signal(false);
                    let dynamic = use_focusable(UseFocusableInput {
                        disabled: dynamic_disabled.into(),
                        ..Default::default()
                    });

                    view! {
                        <div id="test-fcbl-dynamic" {..dynamic.props.into_attrs()}>
                            "Dynamic Focusable"
                        </div>
                        <button
                            id="test-fcbl-dynamic-toggle"
                            on:click=move |_| set_dynamic_disabled.update(|v| *v = !*v)
                        >
                            "Toggle Disabled"
                        </button>
                    }
                }
            </section>
        </div>
    }
}

#[component]
fn PageHookFocusManager() -> impl IntoView {
    let fm = use_focus_manager(UseFocusManagerInput::default());
    let focus_manager = StoredValue::new(fm.focus_manager);

    let fm_wrap = use_focus_manager(UseFocusManagerInput::default());
    let fm_wrap_mgr = StoredValue::new(fm_wrap.focus_manager);

    let fm_tabbable = use_focus_manager(UseFocusManagerInput::default());
    let fm_tabbable_mgr = StoredValue::new(fm_tabbable.focus_manager);

    let fm_accept = use_focus_manager(UseFocusManagerInput::default());
    let fm_accept_mgr = StoredValue::new(fm_accept.focus_manager);

    let fm_radio = use_focus_manager(UseFocusManagerInput::default());
    let fm_radio_mgr = StoredValue::new(fm_radio.focus_manager);

    let fm_radio_none = use_focus_manager(UseFocusManagerInput::default());
    let fm_radio_none_mgr = StoredValue::new(fm_radio_none.focus_manager);

    let fm_radio_wrap = use_focus_manager(UseFocusManagerInput::default());
    let fm_radio_wrap_mgr = StoredValue::new(fm_radio_wrap.focus_manager);

    let fm_vis = use_focus_manager(UseFocusManagerInput::default());
    let fm_vis_mgr = StoredValue::new(fm_vis.focus_manager);

    let fm_inert = use_focus_manager(UseFocusManagerInput::default());
    let fm_inert_mgr = StoredValue::new(fm_inert.focus_manager);

    let fm_outside = use_focus_manager(UseFocusManagerInput::default());
    let fm_outside_mgr = StoredValue::new(fm_outside.focus_manager);

    view! {
        <div id="test-page-hook-focus-manager">
            <h1>"Focus Manager Hook Test Page"</h1>

            <section>
                <h2>"Basic Navigation"</h2>
                <div id="test-fm-scope" {..fm.props.into_attrs()}>
                    <button id="test-fm-item-1">"Item 1"</button>
                    <button id="test-fm-item-2">"Item 2"</button>
                    <button id="test-fm-item-3">"Item 3"</button>
                </div>
                <FocusManagerControls focus_manager />
            </section>

            <section>
                <h2>"Wrap"</h2>
                <div id="test-fm-wrap-scope" {..fm_wrap.props.into_attrs()}>
                    <button id="test-fm-wrap-item-1">"Wrap Item 1"</button>
                    <button id="test-fm-wrap-item-2">"Wrap Item 2"</button>
                    <button id="test-fm-wrap-item-3">"Wrap Item 3"</button>
                </div>
                <FocusManagerWrapControls focus_manager=fm_wrap_mgr />
            </section>

            <section>
                <h2>"Tabbable Filtering"</h2>
                <div id="test-fm-tabbable-scope" {..fm_tabbable.props.into_attrs()}>
                    <button id="test-fm-tabbable-item-1">"Tabbable Item 1"</button>
                    <button id="test-fm-tabbable-item-2" tabindex="-1">
                        "Tabbable Item 2 (tabindex=-1)"
                    </button>
                    <button id="test-fm-tabbable-item-3">"Tabbable Item 3"</button>
                </div>
                <FocusManagerTabbableControls focus_manager=fm_tabbable_mgr />
            </section>

            <section>
                <h2>"Accept Filter"</h2>
                <div id="test-fm-accept-scope" {..fm_accept.props.into_attrs()}>
                    <button id="test-fm-accept-item-1">"Accept Item 1"</button>
                    <button id="test-fm-accept-item-2">"Accept Item 2"</button>
                    <button id="test-fm-accept-item-3">"Accept Item 3"</button>
                </div>
                <FocusManagerAcceptControls focus_manager=fm_accept_mgr />
            </section>

            <section>
                <h2>"Radio Group (One Checked)"</h2>
                <div id="test-fm-radio-scope" {..fm_radio.props.into_attrs()}>
                    <button id="test-fm-radio-btn-before">"Before"</button>
                    <input type="radio" name="test-radio-group" id="test-fm-radio-a" value="a" />
                    <input
                        type="radio"
                        name="test-radio-group"
                        id="test-fm-radio-b"
                        value="b"
                        checked
                    />
                    <input type="radio" name="test-radio-group" id="test-fm-radio-c" value="c" />
                    <button id="test-fm-radio-btn-after">"After"</button>
                </div>
                <FocusManagerRadioControls focus_manager=fm_radio_mgr />
            </section>

            <section>
                <h2>"Radio Group (None Checked)"</h2>
                <div id="test-fm-radio-none-scope" {..fm_radio_none.props.into_attrs()}>
                    <button id="test-fm-radio-none-btn-before">"Before"</button>
                    <input
                        type="radio"
                        name="test-radio-none-group"
                        id="test-fm-radio-none-a"
                        value="a"
                    />
                    <input
                        type="radio"
                        name="test-radio-none-group"
                        id="test-fm-radio-none-b"
                        value="b"
                    />
                    <input
                        type="radio"
                        name="test-radio-none-group"
                        id="test-fm-radio-none-c"
                        value="c"
                    />
                    <button id="test-fm-radio-none-btn-after">"After"</button>
                </div>
                <FocusManagerRadioNoneControls focus_manager=fm_radio_none_mgr />
            </section>

            <section>
                <h2>"Radio Group (Wrap)"</h2>
                <div id="test-fm-radio-wrap-scope" {..fm_radio_wrap.props.into_attrs()}>
                    <input
                        type="radio"
                        name="test-radio-wrap-group"
                        id="test-fm-radio-wrap-a"
                        value="a"
                    />
                    <input
                        type="radio"
                        name="test-radio-wrap-group"
                        id="test-fm-radio-wrap-b"
                        value="b"
                        checked
                    />
                    <input
                        type="radio"
                        name="test-radio-wrap-group"
                        id="test-fm-radio-wrap-c"
                        value="c"
                    />
                </div>
                <FocusManagerRadioWrapControls focus_manager=fm_radio_wrap_mgr />
            </section>

            <section>
                <h2>"Visibility Filtering"</h2>
                <div id="test-fm-vis-scope" {..fm_vis.props.into_attrs()}>
                    <button id="test-fm-vis-item-1">"Visible 1"</button>
                    <button id="test-fm-vis-item-2" style="display:none">
                        "Hidden display:none"
                    </button>
                    <button id="test-fm-vis-item-3" hidden>
                        "Hidden attr"
                    </button>
                    <button id="test-fm-vis-item-4" style="visibility:hidden">
                        "Hidden visibility"
                    </button>
                    <button id="test-fm-vis-item-5">"Visible 2"</button>
                </div>
                <FocusManagerVisControls focus_manager=fm_vis_mgr />
            </section>

            <section>
                <h2>"Inert Filtering"</h2>
                <div id="test-fm-inert-scope" {..fm_inert.props.into_attrs()}>
                    <button id="test-fm-inert-item-1">"Item 1"</button>
                    <div inert>
                        <button id="test-fm-inert-item-2">"Item 2 (inert parent)"</button>
                    </div>
                    <button id="test-fm-inert-item-3">"Item 3"</button>
                </div>
                <FocusManagerInertControls focus_manager=fm_inert_mgr />
            </section>

            <section>
                <h2>"Outside Scope"</h2>
                <button id="test-fm-outside-external">"External (outside scope)"</button>
                <div id="test-fm-outside-scope" {..fm_outside.props.into_attrs()}>
                    <button id="test-fm-outside-item-1">"Outside Item 1"</button>
                    <button id="test-fm-outside-item-2">"Outside Item 2"</button>
                    <button id="test-fm-outside-item-3">"Outside Item 3"</button>
                </div>
                <FocusManagerOutsideControls focus_manager=fm_outside_mgr />
            </section>
        </div>
    }
}

#[component]
fn FocusManagerControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let default_opts = || FocusManagerOptions {
        wrap: false,
        tabbable: false,
        from: None,
        accept: None,
    };

    let on_focus_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(default_opts());
        });
    };

    let on_focus_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(default_opts());
        });
    };

    let on_focus_first = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_first(default_opts());
        });
    };

    let on_focus_last = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_last(default_opts());
        });
    };

    view! {
        <div id="test-fm-controls">
            <button id="test-fm-focus-next" on:mousedown=prevent_focus_steal on:click=on_focus_next>
                "Focus Next"
            </button>
            <button id="test-fm-focus-prev" on:mousedown=prevent_focus_steal on:click=on_focus_prev>
                "Focus Previous"
            </button>
            <button
                id="test-fm-focus-first"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_first
            >
                "Focus First"
            </button>
            <button id="test-fm-focus-last" on:mousedown=prevent_focus_steal on:click=on_focus_last>
                "Focus Last"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerWrapControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let wrap_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: true,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    let wrap_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: true,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    let nowrap_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    let nowrap_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    view! {
        <div id="test-fm-wrap-controls">
            <button
                id="test-fm-wrap-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=wrap_next
            >
                "Wrap Next"
            </button>
            <button
                id="test-fm-wrap-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=wrap_prev
            >
                "Wrap Prev"
            </button>
            <button
                id="test-fm-nowrap-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=nowrap_next
            >
                "No-Wrap Next"
            </button>
            <button
                id="test-fm-nowrap-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=nowrap_prev
            >
                "No-Wrap Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerTabbableControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let tabbable_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    let tabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    let nontabbable_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    let nontabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    view! {
        <div id="test-fm-tabbable-controls">
            <button
                id="test-fm-tabbable-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_next
            >
                "Tabbable Next"
            </button>
            <button
                id="test-fm-tabbable-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_prev
            >
                "Tabbable Prev"
            </button>
            <button
                id="test-fm-nontabbable-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=nontabbable_next
            >
                "Non-Tabbable Next"
            </button>
            <button
                id="test-fm-nontabbable-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=nontabbable_prev
            >
                "Non-Tabbable Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerAcceptControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let reject_item_2: Arc<dyn Fn(&web_sys::Element) -> bool + Send + Sync> =
        Arc::new(|el: &web_sys::Element| el.id() != "test-fm-accept-item-2");

    let reject_filter = StoredValue::new(reject_item_2);

    let accept_next = move |_| {
        let filter = reject_filter.with_value(Arc::clone);
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                tabbable: false,
                from: None,
                accept: Some(filter),
            });
        });
    };

    let accept_prev = move |_| {
        let filter = reject_filter.with_value(Arc::clone);
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: false,
                from: None,
                accept: Some(filter),
            });
        });
    };

    view! {
        <div id="test-fm-accept-controls">
            <button
                id="test-fm-accept-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=accept_next
            >
                "Accept Next"
            </button>
            <button
                id="test-fm-accept-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=accept_prev
            >
                "Accept Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerRadioControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let tabbable_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    let tabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    view! {
        <div id="test-fm-radio-controls">
            <button
                id="test-fm-radio-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_next
            >
                "Radio Next"
            </button>
            <button
                id="test-fm-radio-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_prev
            >
                "Radio Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerRadioNoneControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let tabbable_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    let tabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    view! {
        <div id="test-fm-radio-none-controls">
            <button
                id="test-fm-radio-none-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_next
            >
                "Radio None Next"
            </button>
            <button
                id="test-fm-radio-none-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_prev
            >
                "Radio None Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerRadioWrapControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let tabbable_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: true,
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    let tabbable_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: true,
                tabbable: true,
                from: None,
                accept: None,
            });
        });
    };

    view! {
        <div id="test-fm-radio-wrap-controls">
            <button
                id="test-fm-radio-wrap-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_next
            >
                "Radio Wrap Next"
            </button>
            <button
                id="test-fm-radio-wrap-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=tabbable_prev
            >
                "Radio Wrap Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerVisControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let default_opts = || FocusManagerOptions {
        wrap: false,
        tabbable: false,
        from: None,
        accept: None,
    };

    let on_focus_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(default_opts());
        });
    };

    let on_focus_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(default_opts());
        });
    };

    view! {
        <div id="test-fm-vis-controls">
            <button
                id="test-fm-vis-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_next
            >
                "Vis Next"
            </button>
            <button
                id="test-fm-vis-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_prev
            >
                "Vis Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerInertControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let default_opts = || FocusManagerOptions {
        wrap: false,
        tabbable: false,
        from: None,
        accept: None,
    };

    let on_focus_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(default_opts());
        });
    };

    let on_focus_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(default_opts());
        });
    };

    view! {
        <div id="test-fm-inert-controls">
            <button
                id="test-fm-inert-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_next
            >
                "Inert Next"
            </button>
            <button
                id="test-fm-inert-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_prev
            >
                "Inert Prev"
            </button>
        </div>
    }
}

#[component]
fn FocusManagerOutsideControls(focus_manager: StoredValue<FocusManager>) -> impl IntoView {
    let on_focus_next = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_next(FocusManagerOptions {
                wrap: false,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    let on_focus_prev = move |_| {
        focus_manager.with_value(|fm: &FocusManager| {
            fm.focus_previous(FocusManagerOptions {
                wrap: false,
                tabbable: false,
                from: None,
                accept: None,
            });
        });
    };

    view! {
        <div id="test-fm-outside-controls">
            <button
                id="test-fm-outside-focus-next"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_next
            >
                "Outside Next"
            </button>
            <button
                id="test-fm-outside-focus-prev"
                on:mousedown=prevent_focus_steal
                on:click=on_focus_prev
            >
                "Outside Prev"
            </button>
        </div>
    }
}

#[component]
fn PageHookFocusVisible() -> impl IntoView {
    let fv = use_focus_visible(UseFocusVisibleInput::default());
    let focus_should_be_visible = fv.focus_should_be_visible;
    let modality = fv.modality;

    // Stored (global) modality — read on keyup to capture silent updates.
    let (stored_modality, set_stored_modality) = signal(get_modality());

    view! {
        <div
            id="test-page-hook-focus-visible"
            on:keyup=move |_| {
                set_stored_modality.set(get_modality());
            }
            on:pointerdown=move |_| {
                set_stored_modality.set(get_modality());
            }
        >
            <h1>"Focus Visible Hook Test Page"</h1>

            <button id="test-fv-before">"Before"</button>

            <div id="test-fv-target" tabindex="0">
                "Focus Target"
            </div>

            <input type="text" id="test-fv-text-input" placeholder="Text input" />

            <button id="test-fv-after">"After"</button>

            <div>
                "Focus visible: "
                <span id="test-fv-visible">
                    {move || if focus_should_be_visible.get() { "true" } else { "false" }}
                </span>
            </div>
            <div>
                "Modality: "
                <span id="test-fv-modality">
                    {move || match modality.get() {
                        Modality::Unknown => "Unknown",
                        Modality::Pointer => "Pointer",
                        Modality::Keyboard => "Keyboard",
                        Modality::Virtual => "Virtual",
                    }}
                </span>
            </div>
            <div>
                "Stored modality: "
                <span id="test-fv-stored-modality">
                    {move || match stored_modality.get() {
                        Modality::Unknown => "Unknown",
                        Modality::Pointer => "Pointer",
                        Modality::Keyboard => "Keyboard",
                        Modality::Virtual => "Virtual",
                    }}
                </span>
            </div>
        </div>
    }
}

#[component]
fn PageHookHasTabbableChild() -> impl IntoView {
    let (show_child, set_show_child) = signal(true);

    // Section 1: Dynamic toggle
    let htc = use_has_tabbable_child(UseHasTabbableChildInput::default());
    let has_tabbable_child = htc.has_tabbable_child;

    // Section 2: No tabbable children
    let htc_none = use_has_tabbable_child(UseHasTabbableChildInput::default());
    let has_tabbable_none = htc_none.has_tabbable_child;

    // Section 3: Disabled hook
    let htc_disabled = use_has_tabbable_child(UseHasTabbableChildInput {
        disabled: Signal::derive(|| true),
    });
    let has_tabbable_disabled = htc_disabled.has_tabbable_child;

    // Section 4: Deeply nested tabbable child
    let htc_nested = use_has_tabbable_child(UseHasTabbableChildInput::default());
    let has_tabbable_nested = htc_nested.has_tabbable_child;

    // Section 5: Attribute mutation (disabled toggle on child button)
    let (child_disabled, set_child_disabled) = signal(false);
    let htc_attr = use_has_tabbable_child(UseHasTabbableChildInput::default());
    let has_tabbable_attr = htc_attr.has_tabbable_child;

    view! {
        <div id="test-page-hook-has-tabbable-child">
            <h1>"Has Tabbable Child Hook Test Page"</h1>

            <section>
                <h2>"Dynamic Toggle"</h2>
                <div id="test-htc-container" {..htc.props.into_attrs()}>
                    <Show when=move || show_child.get()>
                        <button id="test-htc-child-btn">"Child Button"</button>
                    </Show>
                </div>
                <div>
                    "Has tabbable child: "
                    <span id="test-htc-result">
                        {move || if has_tabbable_child.get() { "true" } else { "false" }}
                    </span>
                </div>
                <button id="test-htc-toggle" on:click=move |_| set_show_child.update(|v| *v = !*v)>
                    "Toggle"
                </button>
            </section>

            <section>
                <h2>"No Tabbable Children"</h2>
                <div id="test-htc-none-container" {..htc_none.props.into_attrs()}>
                    <span>"Just a span"</span>
                    <div tabindex="-1">"Focusable but not tabbable"</div>
                </div>
                <div>
                    "Has tabbable child: "
                    <span id="test-htc-none-result">
                        {move || if has_tabbable_none.get() { "true" } else { "false" }}
                    </span>
                </div>
            </section>

            <section>
                <h2>"Disabled Hook"</h2>
                <div id="test-htc-disabled-container" {..htc_disabled.props.into_attrs()}>
                    <button>"A button child"</button>
                </div>
                <div>
                    "Has tabbable child: "
                    <span id="test-htc-disabled-result">
                        {move || if has_tabbable_disabled.get() { "true" } else { "false" }}
                    </span>
                </div>
            </section>

            <section>
                <h2>"Deeply Nested"</h2>
                <div id="test-htc-nested-container" {..htc_nested.props.into_attrs()}>
                    <div>
                        <div>
                            <button>"Deeply Nested Button"</button>
                        </div>
                    </div>
                </div>
                <div>
                    "Has tabbable child: "
                    <span id="test-htc-nested-result">
                        {move || if has_tabbable_nested.get() { "true" } else { "false" }}
                    </span>
                </div>
            </section>

            <section>
                <h2>"Attribute Mutation"</h2>
                <div id="test-htc-attr-container" {..htc_attr.props.into_attrs()}>
                    <button disabled=move || child_disabled.get()>"Attr Button"</button>
                </div>
                <div>
                    "Has tabbable child: "
                    <span id="test-htc-attr-result">
                        {move || if has_tabbable_attr.get() { "true" } else { "false" }}
                    </span>
                </div>
                <button
                    id="test-htc-attr-toggle"
                    on:click=move |_| set_child_disabled.update(|v| *v = !*v)
                >
                    "Toggle Disabled"
                </button>
            </section>
        </div>
    }
}

#[component]
fn PageAtomFocusScope() -> impl IntoView {
    // Signal to toggle the restore-focus scope visibility.
    let (show_restore, set_show_restore) = signal(false);
    let (show_nested_restore, set_show_nested_restore) = signal(false);

    view! {
        <div id="test-page-atom-focus-scope">
            <h1>"FocusScope Atom Test Page"</h1>

            // ---- Section 1: Basic containment ----
            <section>
                <h2>"Containment"</h2>
                <FocusScope contain=true>
                    <div id="test-fs-contain-scope">
                        <button id="test-fs-contain-btn-1">"Button 1"</button>
                        <button id="test-fs-contain-btn-2">"Button 2"</button>
                        <button id="test-fs-contain-btn-3">"Button 3"</button>
                    </div>
                </FocusScope>
            </section>

            // ---- Section 2: Auto-focus ----
            <section>
                <h2>"Auto Focus"</h2>
                <FocusScope auto_focus=true>
                    <div id="test-fs-autofocus-scope">
                        <button id="test-fs-autofocus-btn-1">"AF Button 1"</button>
                        <button id="test-fs-autofocus-btn-2">"AF Button 2"</button>
                    </div>
                </FocusScope>
            </section>

            // ---- Section 3: Restore focus ----
            <section>
                <h2>"Restore Focus"</h2>
                <button
                    id="test-fs-restore-toggle"
                    on:click=move |_| set_show_restore.update(|v| *v = !*v)
                >
                    "Toggle Scope"
                </button>
                <Show when=move || show_restore.get()>
                    <FocusScope auto_focus=true restore_focus=true>
                        <div id="test-fs-restore-scope">
                            <button id="test-fs-restore-btn">"Restore Button"</button>
                        </div>
                    </FocusScope>
                </Show>
            </section>

            // ---- Section 4: Nested scopes ----
            <section>
                <h2>"Nested Scopes"</h2>
                <FocusScope contain=true>
                    <div id="test-fs-nested-outer">
                        <button id="test-fs-nested-outer-btn">"Outer Button"</button>
                        <FocusScope contain=true>
                            <div id="test-fs-nested-inner">
                                <button id="test-fs-nested-inner-btn-1">"Inner 1"</button>
                                <button id="test-fs-nested-inner-btn-2">"Inner 2"</button>
                            </div>
                        </FocusScope>
                    </div>
                </FocusScope>
            </section>

            // ---- Section 5: Nested restore ----
            <section>
                <h2>"Nested Restore"</h2>
                <button
                    id="test-fs-nested-restore-trigger"
                    on:click=move |_| set_show_nested_restore.update(|v| *v = !*v)
                >
                    "Toggle Nested Restore"
                </button>
                <Show when=move || show_nested_restore.get()>
                    <FocusScope restore_focus=true auto_focus=true>
                        <div id="test-fs-nested-restore-outer">
                            <button id="test-fs-nested-restore-outer-btn">"Outer Btn"</button>
                            <FocusScope restore_focus=true auto_focus=true>
                                <div id="test-fs-nested-restore-inner">
                                    <button id="test-fs-nested-restore-inner-btn">
                                        "Inner Btn"
                                    </button>
                                </div>
                            </FocusScope>
                        </div>
                    </FocusScope>
                </Show>
            </section>

            // ---- Button outside all scopes (for containment tests) ----
            <button id="test-fs-outside">"Outside"</button>
        </div>
    }
}
