use leptonic::{
    IntoAttrs,
    hooks::focus::{UseFocusInput, use_focus},
};
use leptos::{html, portal::Portal, prelude::*, web_sys};

#[component]
pub fn PageHookFocus() -> impl IntoView {
    let (focus_count, set_focus_count) = signal(0u32);
    let (blur_count, set_blur_count) = signal(0u32);
    let (is_focused, set_is_focused) = signal(false);
    let (focus_change_count, set_focus_change_count) = signal(0u32);

    let focus = use_focus(UseFocusInput {
        is_disabled: Signal::derive(|| false),
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
        is_disabled: Signal::derive(|| true),
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
                        is_disabled: Signal::derive(|| false),
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

            <section>
                <h2>"Disabled While Focused"</h2>
                {
                    // useFocus.test.js, "should fire onBlur when a focused element is disabled".
                    let disabled = RwSignal::new(false);
                    let blur_count = RwSignal::new(0u32);
                    let focus = use_focus(UseFocusInput {
                        is_disabled: Signal::derive(|| false),
                        on_focus: None,
                        on_blur: Some(Callback::new(move |_| blur_count.update(|c| *c += 1))),
                        on_focus_change: None,
                    });
                    view! {
                        <button
                            id="test-focus-disable-me"
                            disabled=move || disabled.get()
                            on:click=move |_| disabled.set(true)
                            {..focus.props.into_attrs()}
                        >
                            "Disable me"
                        </button>
                        <div>
                            "Blur count: " <span id="test-focus-disable-me-blur-count">{blur_count}</span>
                        </div>
                    }
                }
            </section>
            <ShadowFocus />
        </div>
    }
}

/// useFocus.test.js "useFocus with Shadow DOM": `use_focus` on elements inside a shadow root (one
/// disabled), with their calls shown in the light DOM.
#[component]
fn ShadowFocus() -> impl IntoView {
    let host = NodeRef::<html::Div>::new();
    let log = RwSignal::new(Vec::<&'static str>::new());
    let target = NodeRef::<html::Div>::new();
    let disabled_target = NodeRef::<html::Div>::new();
    let focus = |element: NodeRef<html::Div>| {
        move |e: web_sys::MouseEvent| {
            e.prevent_default();
            if let Some(element) = element.get_untracked() {
                let _ = element.focus();
            }
        }
    };
    view! {
        <section>
            <h2>"Shadow DOM"</h2>
            <div node_ref=host></div>
            <Show when=move || host.get().is_some()>
                <Portal mount=web_sys::Element::from(host.get_untracked().expect("mounted")) use_shadow=true>
                    <ShadowFocusTargets log target disabled_target />
                </Portal>
            </Show>
            <button id="test-focus-shadow-focus" on:mousedown=focus(target)>
                "Focus the shadow target"
            </button>
            <button id="test-focus-shadow-focus-disabled" on:mousedown=focus(disabled_target)>
                "Focus the disabled shadow target"
            </button>
            <div>"Shadow log: " <span id="test-focus-shadow-log">{move || log.get().join(", ")}</span></div>
        </section>
    }
}

#[component]
fn ShadowFocusTargets(
    log: RwSignal<Vec<&'static str>>,
    target: NodeRef<html::Div>,
    disabled_target: NodeRef<html::Div>,
) -> impl IntoView {
    let entry = move |name: &'static str| move |_| log.update(|log| log.push(name));
    let focus = use_focus(UseFocusInput {
        is_disabled: Signal::stored(false),
        on_focus: Some(Callback::new(entry("focus"))),
        on_blur: Some(Callback::new(entry("blur"))),
        on_focus_change: Some(Callback::new(move |focused: bool| {
            log.update(|log| {
                log.push(if focused {
                    "change true"
                } else {
                    "change false"
                })
            });
        })),
    });
    let disabled = use_focus(UseFocusInput {
        is_disabled: Signal::stored(true),
        on_focus: Some(Callback::new(entry("disabled focus"))),
        on_blur: Some(Callback::new(entry("disabled blur"))),
        on_focus_change: None,
    });
    view! {
        <div id="test-focus-shadow-target" tabindex="-1" {..focus.props.into_attrs()} node_ref=target>
            "Shadow target"
        </div>
        <div
            id="test-focus-shadow-disabled"
            tabindex="-1"
            {..disabled.props.into_attrs()}
            node_ref=disabled_target
        >
            "Disabled shadow target"
        </div>
    }
}
