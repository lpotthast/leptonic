use indoc::indoc;
use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageUseInteractOutside() -> impl IntoView {
    let (outside_click_count, set_outside_click_count) = signal(0);
    let (is_open, set_is_open) = signal(true);
    let (disabled, set_disabled) = signal(false);

    let interact_outside = use_interact_outside(UseInteractOutsideInput {
        disabled: disabled.into(),
        on_interact_outside_start: None,
        on_interact_outside: Some(Callback::new(move |_| {
            set_outside_click_count.update(|count| *count += 1);
            set_is_open.set(false);
        })),
    });

    let interact_outside_attrs = interact_outside.props.into_attrs();

    view! {
        <Article>
            <h1 id="use_interact_outside" class="anchor">
                "use_interact_outside"
                <AnchorLink href="#use_interact_outside" description="Direct link to article header"/>
            </h1>

            <p>"Hook for detecting interactions (clicks, touches) outside a specified element. Commonly used for closing dropdowns, modals, and popovers."</p>

            <Code>
                {indoc!(r#"
                    let (is_open, set_is_open) = signal(true);

                    let interact_outside = use_interact_outside(UseInteractOutsideInput {
                        disabled: Signal::derive(|| false),
                        on_interact_outside_start: None,
                        on_interact_outside: Some(Callback::new(move |_| {
                            set_is_open.set(false);
                        })),
                    });

                    view! {
                        <div {..interact_outside.props.into_attrs()}>
                            "Interact, e.g. click, anywhere outside this div and `on_interact_outside` will be called."
                        </div>
                    }
                "#)}
            </Code>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em; margin: 1em 0;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disable outside interaction detection"</Label>
            </FormControl>

            <Show when=move || is_open.get()>
                <div
                    {..interact_outside_attrs.clone()}
                    style="
                        border: 3px solid var(--brand-color);
                        padding: 1.5em;
                        border-radius: 8px;
                        background: var(--brand-color-light, rgba(230, 105, 86, 0.1));
                        max-width: 300px;
                        margin: 1em 0;
                    "
                >
                    <p style="margin: 0 0 0.5em 0; font-weight: bold;">"Click outside to close"</p>
                    <p style="margin: 0; font-size: 0.9em; opacity: 0.8;">
                        "This element will close when you click anywhere outside of it."
                    </p>
                </div>
            </Show>

            <Show when=move || !is_open.get()>
                <button
                    on:click=move |_| set_is_open.set(true)
                    style="padding: 0.75em 1.5em; border-radius: 8px; cursor: pointer; background: var(--brand-color); color: white; border: none;"
                >
                    "Reopen"
                </button>
            </Show>

            <p>"Outside clicks detected: " <strong>{ move || outside_click_count.get() }</strong></p>

            <h2 id="options" class="anchor">
                "Options"
                <AnchorLink href="#options" description="Direct link to options"/>
            </h2>

            <ul>
                <li><code>"disabled"</code> " - Whether to disable the detection"</li>
                <li><code>"on_interact_outside_start"</code> " - Optional callback when interaction starts outside"</li>
                <li><code>"on_interact_outside"</code> " - Callback when outside interaction completes"</li>
            </ul>

            <h2 id="use-cases" class="anchor">
                "Use Cases"
                <AnchorLink href="#use-cases" description="Direct link to use cases"/>
            </h2>

            <ul>
                <li>"Closing dropdown menus"</li>
                <li>"Dismissing modal dialogs"</li>
                <li>"Closing popovers and tooltips"</li>
                <li>"Deselecting in-place editors"</li>
                <li>"Closing context menus"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Handles both mouse and touch events"</li>
                <li>"Can be temporarily disabled"</li>
                <li>"Works with dynamic elements"</li>
                <li>"Provides event details in callback"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_interact_outside", link: "#use_interact_outside" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Options", link: "#options" },
                Toc::Leaf { title: "Use Cases", link: "#use-cases" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
