use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::prelude::Size;
use leptos::prelude::*;

#[component]
pub fn PageUseFocusWithin() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (focus_count, set_focus_count) = signal(0);
    let (blur_count, set_blur_count) = signal(0);

    let UseFocusWithinReturn {
        props,
        is_focus_within,
    } = use_focus_within(UseFocusWithinInput {
        disabled: disabled.into(),
        on_focus_within: Some(Callback::new(move |_| {
            set_focus_count.update(|c| *c += 1);
        })),
        on_blur_within: Some(Callback::new(move |_| {
            set_blur_count.update(|c| *c += 1);
        })),
        on_focus_within_change: None,
    });

    view! {
        <Article>
            <h1 id="use_focus_within" class="anchor">
                "use_focus_within"
                <AnchorLink href="#use_focus_within" description="Direct link to article header"/>
            </h1>

            <p>"Track when focus is anywhere within an element tree. Unlike " <code>"use_focus"</code> " which only fires when the element itself receives focus, " <code>"use_focus_within"</code> " fires when focus enters or leaves the entire element tree."</p>

            <Code>
                {r#"let UseFocusWithinReturn { attrs, is_focus_within } = use_focus_within(
    UseFocusWithinInput {
        disabled: Signal::derive(|| false),
        on_focus_within: Some(Callback::new(|_| { /* focus entered */ })),
        on_blur_within: Some(Callback::new(|_| { /* focus left */ })),
        on_focus_within_change: Some(Callback::new(|is_focused| { /* ... */ })),
    }
);

view! {
    <div {..attrs}>
        <input type="text" />
        <button>"Submit"</button>
    </div>
}"#}
            </Code>

            <p>"Click on any element inside the highlighted box. Focus within will be tracked even as you move between different focusable children:"</p>

            <div
                {..props.into_attrs()}
                style=move || format!(
                    "padding: 1.5em; border-radius: 8px; margin: 1em 0; transition: all 0.2s; {}",
                    if is_focus_within.get() {
                        "border: 3px solid var(--brand-color); background: var(--brand-color-light, rgba(230, 105, 86, 0.15));"
                    } else {
                        "border: 3px solid #ccc; background: transparent;"
                    }
                )
            >
                <Stack orientation=StackOrientation::Vertical spacing=Size::Em(1.0)>
                    <p style="margin: 0;">
                        "Focus within: "
                        <strong style=move || if is_focus_within.get() { "color: var(--brand-color);" } else { "" }>
                            { move || if is_focus_within.get() { "Yes" } else { "No" } }
                        </strong>
                    </p>

                    <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(1.0)>
                        <input
                            type="text"
                            placeholder="Click me..."
                            style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px;"
                        />
                        <button style="padding: 0.5em 1em; cursor: pointer; border-radius: 4px; border: 1px solid #ccc;">
                            "Button 1"
                        </button>
                        <button style="padding: 0.5em 1em; cursor: pointer; border-radius: 4px; border: 1px solid #ccc;">
                            "Button 2"
                        </button>
                    </Stack>

                    <p style="margin: 0; font-size: 0.85em; opacity: 0.7;">
                        "Tab between elements - focus stays \"within\" the container"
                    </p>
                </Stack>
            </div>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em; margin-top: 1em;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>

            <p>"Focus within events: " { move || focus_count.get() } " | Blur within events: " { move || blur_count.get() }</p>

            <h2 id="use-cases" class="anchor">
                "Use Cases"
                <AnchorLink href="#use-cases" description="Direct link to use cases"/>
            </h2>

            <ul>
                <li>"Form groups that need visual feedback when any field is focused"</li>
                <li>"Dropdown menus that should stay open while navigating between items"</li>
                <li>"Card components that highlight when any child is focused"</li>
                <li>"Complex widgets with multiple interactive elements"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Tracks focusin/focusout events for the entire element tree"</li>
                <li>"Handles focus moving between children without triggering blur"</li>
                <li>"Ignores events bubbling through portals"</li>
                <li>"Sets up global focus listener to detect DOM removal edge cases"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focus_within", link: "#use_focus_within" },
                Toc::Leaf { title: "Use Cases", link: "#use-cases" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
