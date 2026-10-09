use leptonic::{
    atoms::focus_scope::{FocusScope, RestoreFocusEvent},
    hooks::focus::{FocusManagerOptions, use_focus_manager_context},
};
use leptos::{portal::Portal, prelude::*, tachys::html::event::on, web_sys};
use wasm_bindgen::JsCast;

use crate::pages::{Section, prevent_focus_steal};

#[component]
pub fn PageAtomFocusScope() -> impl IntoView {
    // Signal to toggle the restore-focus scope visibility.
    let (show_restore, set_show_restore) = signal(false);
    let (show_nested_restore, set_show_nested_restore) = signal(false);
    // Restore fallback: the inner scope and its node to restore go away together.
    let fallback = RwSignal::new(false);
    let fallback_empty = RwSignal::new(false);
    // A dialog opened from a menu, rendered outside the menu's scope.
    let show_menu = RwSignal::new(false);
    let show_dialog = RwSignal::new(false);
    // Containment that changes at runtime.
    let runtime_contain = RwSignal::new(false);
    // Restoring scopes whose restoration a listener cancels.
    let show_cancelled = RwSignal::new(false);
    let show_nested_cancelled = RwSignal::new(false);
    // Tabbing out of a restoring scope that doesn't contain focus.
    let show_tab_out = RwSignal::new(false);
    let outside_focused = RwSignal::new(false);

    view! {
        <div id="test-page-atom-focus-scope">
            <h1>"FocusScope Atom Test Page"</h1>
            // The original page: every scope at once (cases load the whole page).
            <Section name="classic">
            // ---- Section 2: Auto-focus ----
            <section>
                <h2>"Auto Focus"</h2>
                <FocusScope auto_focus=true>
                    <div id="test-fs-autofocus-scope">
                        <button id="test-fs-autofocus-btn-1">"AF Button 1"</button>
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

            // ---- Restore fallback (FocusScope.test.js "does not throw when there is no focusable
            // element to restore focus to", with and without another element) ----
            <section>
                <h2>"Restore fallback"</h2>
                <FallbackScopes open=fallback with_other=true prefix="test-fs-fallback" />
                <FallbackScopes open=fallback_empty with_other=false prefix="test-fs-fallback-empty" />
            </section>

            // ---- FocusScope.test.js "tracks node to restore if the node to restore was removed in
            // another part of the tree" (keydown handlers, as upstream: keyboard modality) ----
            <section>
                <h2>"Dialog from a menu"</h2>
                <FocusScope>
                    <button id="test-fs-open-menu" on:keydown=move |_| show_menu.set(true)>
                        "Open Menu"
                    </button>
                    <Show when=move || show_menu.get()>
                        <FocusScope contain=true restore_focus=true auto_focus=true>
                            <button
                                id="test-fs-open-dialog"
                                on:keydown=move |_| {
                                    show_menu.set(false);
                                    show_dialog.set(true);
                                }
                            >
                                "Open Dialog"
                            </button>
                        </FocusScope>
                    </Show>
                </FocusScope>
                <Show when=move || show_dialog.get()>
                    <FocusScope contain=true restore_focus=true auto_focus=true>
                        <button id="test-fs-close-dialog" on:keydown=move |_| show_dialog.set(false)>
                            "Close"
                        </button>
                    </FocusScope>
                </Show>
            </section>

            // ---- Containment (FocusScope.test.js "should contain focus within the scope"), and "should
            // select all text in input when tabbing" ----
            <section>
                <h2>"Select on Tab"</h2>
                <FocusScope contain=true>
                    <input id="test-fs-select-input-1" value="Test1" />
                    <input id="test-fs-select-input-2" value="Test2" />
                    <input id="test-fs-select-input-3" value="Test3" />
                </FocusScope>
            </section>

            // ---- Containment that changes at runtime (a non-modal popover starting to contain) ----
            <section>
                <h2>"Runtime contain"</h2>
                <FocusScope contain=runtime_contain>
                    <button
                        id="test-fs-runtime-toggle"
                        on:click=move |_| runtime_contain.update(|c| *c = !*c)
                    >
                        {move || if runtime_contain.get() { "Stop containing" } else { "Contain" }}
                    </button>
                    <button id="test-fs-runtime-2">"Runtime 2"</button>
                </FocusScope>
                <button id="test-fs-runtime-after">"After runtime scope"</button>
            </section>

            // ---- FocusScope.test.js "should allow restoration to be overridden with a custom event"
            // and "should not bubble focus scope restoration event out of nested focus scopes" ----
            <section>
                <h2>"Cancelled restore"</h2>
                <div {..on(
                    RestoreFocusEvent,
                    |e: web_sys::CustomEvent| e.prevent_default(),
                )}>
                    <button id="test-fs-cancel-show" on:click=move |_| show_cancelled.set(true)>
                        "Show"
                    </button>
                    <Show when=move || show_cancelled.get()>
                        <FocusScope restore_focus=true auto_focus=true>
                            <input
                                id="test-fs-cancel-input"
                                on:keydown=move |_| show_cancelled.set(false)
                            />
                        </FocusScope>
                    </Show>
                </div>
                <div {..on(
                    RestoreFocusEvent,
                    |e: web_sys::CustomEvent| e.prevent_default(),
                )}>
                    <FocusScope>
                        <button
                            id="test-fs-nested-cancel-show"
                            on:click=move |_| show_nested_cancelled.set(true)
                        >
                            "Show nested"
                        </button>
                        <Show when=move || show_nested_cancelled.get()>
                            <FocusScope restore_focus=true auto_focus=true>
                                <input
                                    id="test-fs-nested-cancel-input"
                                    on:keydown=move |_| show_nested_cancelled.set(false)
                                />
                            </FocusScope>
                        </Show>
                    </FocusScope>
                </div>
            </section>

            // ---- FocusScope.test.js "should move focus to the element after the previously focused
            // node on Tab" / "... previous element ... on Shift+Tab" ----
            <section>
                <h2>"Tab out of a restoring scope"</h2>
                <input id="test-fs-tab-before" />
                <button id="test-fs-tab-trigger" on:click=move |_| show_tab_out.update(|s| *s = !*s)>
                    "Toggle"
                </button>
                <input id="test-fs-tab-after-trigger" />
                <Show when=move || show_tab_out.get()>
                    <FocusScope restore_focus=true auto_focus=true>
                        <input id="test-fs-tab-input-1" />
                        <input id="test-fs-tab-input-2" />
                        <input id="test-fs-tab-input-3" />
                    </FocusScope>
                </Show>
                <input id="test-fs-tab-after" />
            </section>

            // ---- Top layer (e.g. toasts): focus may move there from a containing scope ----
            <div data-leptonic-top-layer="true">
                <button id="test-fs-top-layer-1">"Top layer 1"</button>
                <button id="test-fs-top-layer-2">"Top layer 2"</button>
            </div>

            // ---- Button outside all scopes (for containment tests) ----
            <button id="test-fs-outside" on:focus=move |_| outside_focused.set(true)>
                "Outside"
            </button>
            // Whether the outside button ever had focus.
            <span id="test-fs-outside-focused">{move || outside_focused.get().to_string()}</span>
            </Section>
            <MoreScopes />
        </div>
    }
}

/// An outer scope with a restore target (and optionally another button); its button opens an
/// inner scope (restoring focus, auto-focused), whose button closes it and removes the target.
#[component]
fn FallbackScopes(open: RwSignal<bool>, with_other: bool, prefix: &'static str) -> impl IntoView {
    let target_shown = RwSignal::new(true);
    view! {
        <FocusScope>
            <div>
                <Show when=move || target_shown.get()>
                    <button id=format!("{prefix}-target") on:click=move |_| open.set(true)>
                        "Restore target"
                    </button>
                </Show>
            </div>
            {with_other.then(|| view! { <button id=format!("{prefix}-other")>"Other"</button> })}
            <Show when=move || open.get()>
                <FocusScope restore_focus=true auto_focus=true>
                    <button
                        id=format!("{prefix}-inside")
                        on:click=move |_| {
                            open.set(false);
                            target_shown.set(false);
                        }
                    >
                        "Close and remove the target"
                    </button>
                </FocusScope>
            </Show>
        </FocusScope>
    }
}

/// Sections for one case each (`goto_sections`), mirroring FocusScope.test.js.
#[component]
fn MoreScopes() -> impl IntoView {
    view! {
        // "should work with multiple focus scopes"
        <Section name="multiple">
            <FocusScope contain=true>
                <input id="test-fs-multi-1" />
                <input id="test-fs-multi-2" />
                <input style="display: none" />
                <input style="visibility: hidden" />
                <input style="visibility: collapse" />
                <input id="test-fs-multi-3" />
            </FocusScope>
            <FocusScope contain=true>
                <input id="test-fs-multi-4" />
                <input id="test-fs-multi-5" />
                <input style="display: none" />
                <input style="visibility: hidden" />
                <input style="visibility: collapse" />
                <input id="test-fs-multi-6" />
            </FocusScope>
        </Section>
        // "should skip non-tabbable elements", "should work with nested elements"
        <Section name="skip">
            <FocusScope contain=true>
                <input id="test-fs-skip-1" />
                <div></div>
                <div>
                    <input id="test-fs-skip-2" />
                </div>
                <input hidden />
                <input style="display: none" />
                <input style="visibility: hidden" />
                <input style="visibility: collapse" />
                <div tabindex="-1"></div>
                <input disabled tabindex="0" />
                <div>
                    <div>
                        <input id="test-fs-skip-3" />
                    </div>
                </div>
            </FocusScope>
        </Section>
        // "should only skip content editable which are false"
        <Section name="contenteditable">
            <FocusScope contain=true>
                <input id="test-fs-ce-1" />
                <span id="test-fs-ce-2" contenteditable="true"></span>
                <span contenteditable="false"></span>
                <span id="test-fs-ce-3" contenteditable="plaintext-only"></span>
                <input id="test-fs-ce-4" />
            </FocusScope>
        </Section>
        <RestoreAfterChildrenChange />
        <TabAfterTrigger />
        <NoRestoreTab />
        // "should navigate in and out of scope in DOM order when the nodeToRestore is the
        // document.body"
        <Section name="dom-order">
            <input id="test-fs-dom-before" />
            <FocusScope>
                <input id="test-fs-dom-in" />
            </FocusScope>
            <input id="test-fs-dom-after" />
        </Section>
        // "should do nothing if something is already focused in the scope": the browser
        // focuses the `autofocus` input when the page loads.
        <Section name="autofocused">
            <FocusScope auto_focus=true>
                <div></div>
                <input id="test-fs-af-1" />
                <input id="test-fs-af-2" autofocus />
                <input id="test-fs-af-3" />
            </FocusScope>
        </Section>
        <AutoFocusFallback />
        <FocusableFirstInScope />
        <PortalChild contain=false name="portal-child" />
        <PortalChild contain=true name="portal-child-contain" />
        <ChildScopeElsewhere />
        <CorrectScopeOnUnmount />
        <StackedDialogs contain=false portaled=false name="stacked" />
        <StackedDialogs contain=true portaled=false name="stacked-contain" />
        <StackedDialogs contain=false portaled=true name="stacked-portaled" />
        <StackedDialogs contain=true portaled=true name="stacked-contain-portaled" />
        <ShadowScopes />
    }
}

/// A control that changes the page without taking focus.
#[component]
fn Control(id: &'static str, on_press: impl Fn() + 'static, children: Children) -> impl IntoView {
    view! {
        <button id=id tabindex="-1" on:mousedown=prevent_focus_steal on:click=move |_| on_press()>
            {children()}
        </button>
    }
}

/// "should restore focus to the previously focused node after children change"
#[component]
fn RestoreAfterChildrenChange() -> impl IntoView {
    let show = RwSignal::new(false);
    let show_child = RwSignal::new(false);
    view! {
        <Section name="children-change">
            <input id="test-fs-cc-outside" />
            <Control id="test-fs-cc-show" on_press=move || show.set(true)>"Show"</Control>
            <Control id="test-fs-cc-show-child" on_press=move || show_child.set(true)>
                "Show child"
            </Control>
            <Control id="test-fs-cc-hide" on_press=move || show.set(false)>"Hide"</Control>
            <Show when=move || show.get()>
                <FocusScope restore_focus=true auto_focus=true>
                    <input id="test-fs-cc-input" />
                    <Show when=move || show_child.get()>
                        <input id="test-fs-cc-dynamic" />
                    </Show>
                </FocusScope>
            </Show>
        </Section>
    }
}

/// "should skip over elements within the scope when moving focus to the next element": the
/// scope follows its trigger directly.
#[component]
fn TabAfterTrigger() -> impl IntoView {
    let show = RwSignal::new(false);
    view! {
        <Section name="after-trigger">
            <input id="test-fs-at-before" />
            <button id="test-fs-at-trigger" on:click=move |_| show.set(true)>
                "Trigger"
            </button>
            <Show when=move || show.get()>
                <FocusScope restore_focus=true auto_focus=true>
                    <input id="test-fs-at-1" />
                    <input id="test-fs-at-2" />
                    <input id="test-fs-at-3" />
                </FocusScope>
            </Show>
            <input id="test-fs-at-after" />
        </Section>
    }
}

/// "should not handle tabbing if the focus scope does not restore focus"
#[component]
fn NoRestoreTab() -> impl IntoView {
    let show = RwSignal::new(false);
    view! {
        <Section name="no-restore">
            <input id="test-fs-nr-before" />
            <button id="test-fs-nr-trigger" on:click=move |_| show.set(true)>
                "Trigger"
            </button>
            <input id="test-fs-nr-after-trigger" />
            <Show when=move || show.get()>
                <FocusScope auto_focus=true>
                    <input id="test-fs-nr-1" />
                    <input id="test-fs-nr-2" />
                    <input id="test-fs-nr-3" />
                </FocusScope>
            </Show>
            <input id="test-fs-nr-after" />
        </Section>
    }
}

/// Auto focus without a tabbable element focuses the first focusable one (react-aria's
/// `getFirstInScope`).
#[component]
fn AutoFocusFallback() -> impl IntoView {
    let show = RwSignal::new(false);
    view! {
        <Section name="autofocus-fallback">
            <Control id="test-fs-aff-show" on_press=move || show.set(true)>"Show"</Control>
            <Show when=move || show.get()>
                <FocusScope auto_focus=true>
                    <div id="test-fs-aff-dialog" role="dialog" aria-label="Fallback" tabindex="-1">
                        "Nothing tabbable"
                    </div>
                </FocusScope>
            </Show>
        </Section>
    }
}

/// "focusable first in scope": every item moves focus on with the scope's focus manager and
/// removes itself; once none is left, focus goes to the focusable dialog.
#[component]
fn FocusableFirstInScope() -> impl IntoView {
    view! {
        <Section name="first-focusable">
            <FocusScope contain=true auto_focus=true>
                <div id="test-fs-ff-dialog" role="dialog" aria-label="Items" tabindex="-1">
                    <RemovableItem id="test-fs-ff-tabbable" tabindex=None label="Remove me!" />
                    <RemovableItem id="test-fs-ff-item-1" tabindex=Some(0) label="Remove me, too!" />
                    <RemovableItem
                        id="test-fs-ff-item-2"
                        tabindex=Some(-1)
                        label="Remove me, three!"
                    />
                </div>
            </FocusScope>
        </Section>
    }
}

#[component]
fn RemovableItem(id: &'static str, tabindex: Option<i32>, label: &'static str) -> impl IntoView {
    let shown = RwSignal::new(true);
    let manager = use_focus_manager_context();
    let button = NodeRef::<leptos::html::Button>::new();
    view! {
        <Show when=move || shown.get()>
            <button
                id=id
                node_ref=button
                tabindex=tabindex
                on:click=move |_| {
                    if let Some(manager) = manager {
                        manager.focus_next(FocusManagerOptions::default());
                    }
                    if let Some(button) = button.get_untracked() {
                        let _ = button.blur();
                    }
                    shown.set(false);
                }
            >
                {label}
            </button>
        </Show>
    }
}

/// "should not lock focus inside a focus scope with a child scope in a portal" (`contain:
/// false`) and "should lock focus inside a child focus scope with contain in a portal".
#[component]
fn PortalChild(contain: bool, name: &'static str) -> impl IntoView {
    view! {
        <Section name=name>
            <FocusScope auto_focus=true restore_focus=true contain=true>
                <input id=format!("test-fs-{name}-parent") aria-label="Parent" />
                <div>
                    <Portal>
                        <FocusScope contain=contain>
                            <input id=format!("test-fs-{name}-child") aria-label="Child" />
                        </FocusScope>
                    </Portal>
                </div>
            </FocusScope>
        </Section>
    }
}

/// "should make child FocusScopes the active scope regardless of DOM structure": a containing
/// child scope rendered elsewhere (a portal) takes focus from its containing parent.
#[component]
fn ChildScopeElsewhere() -> impl IntoView {
    let show = RwSignal::new(false);
    view! {
        <Section name="child-elsewhere">
            <input id="test-fs-ce-outside" aria-label="Outside" />
            <Control id="test-fs-ce-show" on_press=move || show.set(true)>"Show"</Control>
            <FocusScope restore_focus=true contain=true>
                <input id="test-fs-ce-input-1" aria-label="Input 1" />
                <Show when=move || show.get()>
                    <Portal>
                        <FocusScope restore_focus=true contain=true>
                            <input id="test-fs-ce-input-3" aria-label="Input 3" />
                        </FocusScope>
                    </Portal>
                </Show>
            </FocusScope>
        </Section>
    }
}

/// "should restore to the correct scope on unmount": nested containing scopes shown one by one.
#[component]
fn CorrectScopeOnUnmount() -> impl IntoView {
    let show = [
        RwSignal::new(false),
        RwSignal::new(false),
        RwSignal::new(false),
    ];
    view! {
        <Section name="correct-scope">
            <input id="test-fs-cs-outside" aria-label="Outside" />
            <Control id="test-fs-cs-show-1" on_press=move || show[0].set(true)>"Show 1"</Control>
            <Control id="test-fs-cs-show-2" on_press=move || show[1].set(true)>"Show 2"</Control>
            <Control id="test-fs-cs-show-3" on_press=move || show[2].set(true)>"Show 3"</Control>
            <Control
                id="test-fs-cs-only-1"
                on_press=move || {
                    show[1].set(false);
                    show[2].set(false);
                }
            >
                "Only 1"
            </Control>
            <FocusScope auto_focus=true restore_focus=true contain=true>
                <input id="test-fs-cs-parent" aria-label="Parent" />
                <Show when=move || show[0].get()>
                    <FocusScope contain=true>
                        <input id="test-fs-cs-child-1" aria-label="Child 1" />
                        <Show when=move || show[1].get()>
                            <FocusScope contain=true>
                                <input id="test-fs-cs-child-2" aria-label="Child 2" />
                                <Show when=move || show[2].get()>
                                    <FocusScope contain=true>
                                        <input id="test-fs-cs-child-3" aria-label="Child 3" />
                                    </FocusScope>
                                </Show>
                            </FocusScope>
                        </Show>
                    </FocusScope>
                </Show>
            </FocusScope>
        </Section>
    }
}

/// FocusScope.test.js "contain=$contain, isPortaled=$isPortaled should restore focus to previous
/// nodeToRestore when the nodeToRestore for the unmounting scope in no longer in the DOM" (the
/// story `FocusScope.stories.tsx` `Example`): dialogs opening dialogs, each restoring focus.
#[component]
fn StackedDialogs(contain: bool, portaled: bool, name: &'static str) -> impl IntoView {
    let open = RwSignal::new(false);
    let root = NodeRef::<leptos::html::Div>::new();
    view! {
        <Section name=name>
            <div id=format!("test-fs-{name}")>
                <input aria-label="input before" />
                <button type="button" on:click=move |_| open.set(true)>
                    "Open dialog"
                </button>
                <input aria-label="input after" />
                <Show when=move || open.get()>
                    {move || {
                        nested_dialog(
                            contain,
                            portaled.then_some(root),
                            Callback::new(move |()| open.set(false)),
                        )
                    }}
                </Show>
                <div node_ref=root></div>
            </div>
        </Section>
    }
}

/// One dialog of [`StackedDialogs`]: three inputs, a button opening the next dialog (inside it, or
/// in `portal`) and one closing it. Erased, as it renders itself.
fn nested_dialog(
    contain: bool,
    portal: Option<NodeRef<leptos::html::Div>>,
    on_close: Callback<()>,
) -> AnyView {
    let open = RwSignal::new(false);
    let dialog = move || {
        view! {
            <FocusScope contain=contain restore_focus=true auto_focus=true>
                <div role="dialog" aria-label="Dialog">
                    <input aria-label="Dialog input" />
                    <input aria-label="Dialog input" />
                    <input aria-label="Dialog input" />
                    <button type="button" on:click=move |_| open.set(true)>
                        "Open dialog"
                    </button>
                    <button type="button" on:click=move |_| on_close.run(())>
                        "close"
                    </button>
                    <Show when=move || open.get()>
                        {move || nested_dialog(contain, portal, Callback::new(move |()| open.set(false)))}
                    </Show>
                </div>
            </FocusScope>
        }
    };
    match portal.and_then(|root| root.get_untracked()) {
        Some(root) => {
            view! { <Portal mount=web_sys::Element::from(root)>{dialog}</Portal> }.into_any()
        }
        None => dialog().into_any(),
    }
}

/// FocusScope.test.js "FocusScope with Shadow DOM": scopes inside shadow roots (`Portal` with
/// `use_shadow`). The deepest focused element's id (`composedPath()[0]`) is shown in the light
/// DOM, as the browser tests can't look into shadow roots.
#[component]
fn ShadowScopes() -> impl IntoView {
    let focused = RwSignal::new(String::new());
    let host = NodeRef::<leptos::html::Div>::new();
    let nested_host = NodeRef::<leptos::html::Div>::new();
    let restore_host = NodeRef::<leptos::html::Div>::new();
    let first = NodeRef::<leptos::html::Input>::new();
    let nested_first = NodeRef::<leptos::html::Input>::new();
    let restore_first = NodeRef::<leptos::html::Input>::new();
    let show_restore = RwSignal::new(true);
    // The window sees focus entering/leaving a shadow root. Moving within one root does not
    // reach it (target and relatedTarget retarget to the same host), so children also record focus.
    let record_focus = move |e: web_sys::FocusEvent| {
        let id = e
            .composed_path()
            .get(0)
            .dyn_into::<web_sys::Element>()
            .map(|element| element.id())
            .unwrap_or_default();
        focused.set(id);
    };
    let listener = window_event_listener(leptos::ev::focusin, record_focus);
    on_cleanup(move || listener.remove());
    let focus = |input: NodeRef<leptos::html::Input>| {
        move || {
            if let Some(input) = input.get_untracked() {
                let _ = input.focus();
            }
        }
    };
    view! {
        <Section name="shadow">
            "Focused: " <span id="test-fs-shadow-focused">{focused}</span>
            <Control id="test-fs-shadow-focus-1" on_press=focus(first)>"Focus input 1"</Control>
            <Control id="test-fs-shadow-focus-nested" on_press=focus(nested_first)>
                "Focus the nested input 1"
            </Control>
            <Control id="test-fs-shadow-focus-restore" on_press=focus(restore_first)>
                "Focus the restoring input 1"
            </Control>
            <Control id="test-fs-shadow-unmount" on_press=move || show_restore.set(false)>
                "Unmount the restoring scope"
            </Control>
            // "should contain focus within the shadow DOM scope", "should autofocus and lock tab
            // navigation inside shadow DOM"
            <div node_ref=host></div>
            <Show when=move || host.get().is_some()>
                <Portal mount=web_sys::Element::from(host.get_untracked().expect("mounted")) use_shadow=true>
                    <FocusScope contain=true>
                        <input id="test-fs-shadow-1" node_ref=first on:focus=record_focus />
                        <input id="test-fs-shadow-2" on:focus=record_focus />
                        <button id="test-fs-shadow-button" on:focus=record_focus>"Button"</button>
                    </FocusScope>
                </Portal>
            </Show>
            // "should manage focus within nested shadow DOMs"
            <div node_ref=nested_host></div>
            <Show when=move || nested_host.get().is_some()>
                <Portal mount=web_sys::Element::from(nested_host.get_untracked().expect("mounted")) use_shadow=true>
                    <NestedShadow first=nested_first on_focus=Callback::new(record_focus) />
                </Portal>
            </Show>
            // "should restore focus to the element outside shadow DOM on unmount, with FocusScope
            // outside as well"
            <FocusScope restore_focus=true>
                <input id="test-fs-shadow-outside" />
            </FocusScope>
            <div node_ref=restore_host></div>
            <Show when=move || restore_host.get().is_some() && show_restore.get()>
                <Portal mount=web_sys::Element::from(restore_host.get_untracked().expect("mounted")) use_shadow=true>
                    <FocusScope restore_focus=true>
                        <input id="test-fs-shadow-restore-1" node_ref=restore_first on:focus=record_focus />
                        <input id="test-fs-shadow-restore-2" on:focus=record_focus />
                    </FocusScope>
                </Portal>
            </Show>
        </Section>
    }
}

/// A shadow root inside a shadow root, with a containing scope.
#[component]
fn NestedShadow(
    first: NodeRef<leptos::html::Input>,
    on_focus: Callback<web_sys::FocusEvent>,
) -> impl IntoView {
    let inner_host = NodeRef::<leptos::html::Div>::new();
    view! {
        <div node_ref=inner_host></div>
        <Show when=move || inner_host.get().is_some()>
            <Portal mount=web_sys::Element::from(inner_host.get_untracked().expect("mounted")) use_shadow=true>
                <FocusScope contain=true>
                    <input id="test-fs-nested-shadow-1" node_ref=first on:focus=move |event| on_focus.run(event) />
                    <input id="test-fs-nested-shadow-2" on:focus=move |event| on_focus.run(event) />
                </FocusScope>
            </Portal>
        </Show>
    }
}
