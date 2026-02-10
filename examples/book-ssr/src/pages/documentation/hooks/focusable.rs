use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use leptonic::atoms::focus_ring::FocusRing;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::prelude::Size;
use leptos::prelude::*;

#[component]
pub fn PageUseFocusable() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (exclude_from_tab, set_exclude_from_tab) = signal(false);
    let (focus_count, set_focus_count) = signal(0);
    let (blur_count, set_blur_count) = signal(0);
    let (is_focused, set_is_focused) = signal(false);
    let (key_events, set_key_events) = signal(Vec::<String>::new());

    let UseFocusableReturn {
        props,
        focus_handle,
    } = use_focusable(UseFocusableInput {
        disabled: disabled.into(),
        auto_focus: false,
        exclude_from_tab_order: exclude_from_tab.into(),
        on_focus: Some(Callback::new(move |_| {
            set_focus_count.update(|c| *c += 1);
        })),
        on_blur: Some(Callback::new(move |_| {
            set_blur_count.update(|c| *c += 1);
        })),
        on_focus_change: Some(Callback::new(move |focused: bool| {
            set_is_focused.set(focused);
        })),
        on_key_down: Some(Callback::new(move |e: KeyboardEventWrapper| {
            set_key_events.update(|events| {
                events.push(format!("Key: {}", e.key()));
                if events.len() > 5 {
                    events.remove(0);
                }
            });
            e.continue_propagation();
        })),
        on_key_up: None,
    });
    let attrs = props.into_attrs();

    view! {
        <Article>
            <h1 id="use_focusable" class="anchor">
                "use_focusable"
                <AnchorLink href="#use_focusable" description="Direct link to article header"/>
            </h1>

            <p>"Make any element focusable with proper keyboard event handling. Combines " <code>"use_focus"</code> " and " <code>"use_keyboard"</code> " for a complete focusable element solution."</p>

            <Code>
                {indoc!(r#"
                    let UseFocusableReturn { props, focus_handle } = use_focusable(UseFocusableInput {
                        disabled: Signal::derive(|| false),
                        auto_focus: false,
                        exclude_from_tab_order: Signal::derive(|| false),
                        on_focus: Some(Callback::new(|_| { /* focused */ })),
                        on_blur: Some(Callback::new(|_| { /* blurred */ })),
                        on_focus_change: Some(Callback::new(|focused: bool| { /* focus state changed */ })),
                        on_key_down: Some(Callback::new(|e: KeyboardEventWrapper| {
                            if e.key() == "Enter" {
                                // Handle enter key
                            } else {
                                e.continue_propagation();
                            }
                        })),
                        on_key_up: None,
                    });

                    view! {
                        <div role="button" {..props.into_attrs()}>
                            "Click or Tab to focus"
                        </div>

                        // Programmatically focus the element
                        <button on:click=move |_| focus_handle.focus()>
                            "Focus"
                        </button>
                    }
                "#)}
            </Code>

            <p>"Focus the custom element below using Tab, click, or the button. The demo uses " <code>"FocusRing"</code> " (powered by " <code>"use_focus_ring"</code> ") to show a visible ring on keyboard focus:"</p>

            <div style="display: flex; gap: 1em; align-items: center;">
                <FocusRing>
                    <div
                        {..attrs}
                        role="button"
                        style="
                            display: inline-flex;
                            align-items: center;
                            justify-content: center;
                            border: 2px solid var(--brand-color);
                            padding: 1em 2em;
                            border-radius: 8px;
                            cursor: pointer;
                            background: var(--brand-color-light, rgba(230, 105, 86, 0.1));
                            outline: none;
                            transition: all 0.2s;
                        "
                    >
                        "Custom Focusable Element"
                    </div>
                </FocusRing>

                <button
                    on:click=move |_| focus_handle.focus()
                    style="
                        padding: 0.5em 1em;
                        border-radius: 4px;
                        border: 1px solid var(--brand-color);
                        background: transparent;
                        cursor: pointer;
                    "
                >
                    "Click to focus"
                </button>
            </div>

            <Stack orientation=StackOrientation::Vertical spacing=Size::Em(0.5) attr:style="margin-top: 1em;">
                <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em;">
                    <Checkbox checked=disabled set_checked=set_disabled />
                    <Label>"Disabled"</Label>
                </FormControl>

                <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em;">
                    <Checkbox checked=exclude_from_tab set_checked=set_exclude_from_tab />
                    <Label>"Exclude from tab order (tabindex=-1)"</Label>
                </FormControl>
            </Stack>

            <div style="margin-top: 1em; display: flex; gap: 2em; flex-wrap: wrap;">
                <p>"Focus count: " { move || focus_count.get() }</p>
                <p>"Blur count: " { move || blur_count.get() }</p>
                <p style:color=move || if is_focused.get() { "green" } else { "gray" }>
                    { move || if is_focused.get() { "Focused" } else { "Not focused" } }
                </p>
            </div>

            <p>"Last key events: " { move || {
                let events = key_events.get();
                if events.is_empty() {
                    "(none)".to_string()
                } else {
                    events.join(", ")
                }
            }}</p>

            <h2 id="programmatic-focus" class="anchor">
                "Programmatic Focus"
                <AnchorLink href="#programmatic-focus" description="Direct link to programmatic focus"/>
            </h2>

            <p>"The hook returns a " <code>"FocusHandle"</code> " that allows you to programmatically focus the element. The " <code>"focus()"</code> " method uses " <code>"focus_safely"</code> " which defers focus during screen reader (virtual) modality to avoid VoiceOver scroll issues during CSS transitions:"</p>

            <Code>
                {indoc!(r"
                    let UseFocusableReturn { props, focus_handle } = use_focusable(input);

                    // Focus the element from anywhere
                    focus_handle.focus();

                    // Check if element has been captured (false during SSR)
                    if focus_handle.has_element() {
                        focus_handle.focus();
                    }
                ")}
            </Code>

            <p>"The " <code>"FocusHandle"</code> " is useful for:"</p>
            <ul>
                <li>"Focusing elements in response to user actions"</li>
                <li>"Implementing keyboard navigation in custom components"</li>
                <li>"Focus restoration after dialogs close"</li>
            </ul>

            <h2 id="tab-index" class="anchor">
                "Tab Index Management"
                <AnchorLink href="#tab-index" description="Direct link to tab index"/>
            </h2>

            <p>"The hook automatically manages the tabindex attribute:"</p>
            <ul>
                <li><code>"disabled=true"</code> " → no tabindex (element not focusable)"</li>
                <li><code>"exclude_from_tab_order=true"</code> " → tabindex=\"-1\" (focusable but not via Tab)"</li>
                <li>"Default → tabindex=\"0\" (focusable via Tab in normal order)"</li>
            </ul>

            <h2 id="auto-focus" class="anchor">
                "Auto Focus"
                <AnchorLink href="#auto-focus" description="Direct link to auto focus"/>
            </h2>

            <p>"Set " <code>"auto_focus: true"</code> " to automatically focus the element when it mounts:"</p>

            <Code>
                {indoc!(r"
                    let focusable = use_focusable(UseFocusableInput {
                        auto_focus: true,  // Element will be focused on mount
                        ..Default::default()
                    });
                ")}
            </Code>

            <h2 id="related-hooks" class="anchor">
                "Related Hooks"
                <AnchorLink href="#related-hooks" description="Direct link to related hooks"/>
            </h2>

            <ul>
                <li><code>"use_focus"</code> " — Low-level focus/blur event handling. Used internally by " <code>"use_focusable"</code> "."</li>
                <li><code>"use_keyboard"</code> " — Low-level keyboard event handling. Used internally by " <code>"use_focusable"</code> "."</li>
                <li><code>"use_focus_ring"</code> " — Tracks whether a focus ring should be visible (keyboard navigation only). The " <code>"FocusRing"</code> " atom wraps this hook."</li>
                <li><code>"use_focus_visible"</code> " — Tracks the current input modality (keyboard, pointer, virtual) to decide focus visibility."</li>
            </ul>

            <h2 id="deviations" class="anchor">
                "Deviations from react-aria"
                <AnchorLink href="#deviations" description="Direct link to deviations"/>
            </h2>

            <ul>
                <li><b>"useSyntheticBlurEvent"</b> " — React-aria includes a workaround for React < 17 where blur events do not fire on disabled elements. Native DOM handles this correctly, so it is not needed."</li>
                <li><b>"Handler optimization"</b> " — React-aria returns " <code>"undefined"</code> " props when no callbacks are provided. Our " <code>"EventHandler"</code> " always attaches a listener but checks the disabled state inside. The overhead is negligible."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focusable", link: "#use_focusable" },
                Toc::Leaf { title: "Programmatic Focus", link: "#programmatic-focus" },
                Toc::Leaf { title: "Tab Index Management", link: "#tab-index" },
                Toc::Leaf { title: "Auto Focus", link: "#auto-focus" },
                Toc::Leaf { title: "Related Hooks", link: "#related-hooks" },
                Toc::Leaf { title: "Deviations from react-aria", link: "#deviations" },
            ]
        }/>
    }
}
