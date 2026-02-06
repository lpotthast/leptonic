use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUseScrollWheel() -> impl IntoView {
    let (value, set_value) = signal(50.0f64);
    let (disabled, set_disabled) = signal(false);
    let (last_delta, set_last_delta) = signal((0.0f64, 0.0f64));

    let scroll_wheel = use_scroll_wheel(UseScrollWheelInput {
        disabled: disabled.into(),
        on_scroll: Some(Callback::new(move |e: ScrollEvent| {
            set_last_delta.set((e.delta_x, e.delta_y));
            // Only respond to primarily vertical scrolling to avoid
            // horizontal trackpad gestures affecting the value
            if e.delta_y.abs() > e.delta_x.abs() {
                set_value.update(|v| {
                    *v = (*v - e.delta_y * 0.1).clamp(0.0, 100.0);
                });
            }
        })),
    });

    view! {
        <Article>
            <h1 id="use_scroll_wheel" class="anchor">
                "use_scroll_wheel"
                <AnchorLink href="#use_scroll_wheel" description="Direct link to article header"/>
            </h1>

            <p>"Hook for handling scroll wheel events on an element. Useful for custom scroll behaviors, zooming, and value adjustment controls."</p>

            <Code>
                {r#"let (value, set_value) = signal(50.0f64);

let scroll_wheel = use_scroll_wheel(UseScrollWheelInput {
    disabled: Signal::derive(|| false),
    on_scroll: Some(Callback::new(move |e: ScrollEvent| {
        // e.delta_x, e.delta_y give scroll amounts
        // Positive delta_y = scroll down, negative = scroll up

        // Only respond to primarily vertical scrolling
        if e.delta_y.abs() > e.delta_x.abs() {
            set_value.update(|v| {
                *v = (*v - e.delta_y * 0.1).clamp(0.0, 100.0);
            });
        }
    })),
});

view! {
    <div {..scroll_wheel.attrs} tabindex="0">
        // Round for display only
        "Value: " { move || value.get().round() as i32 }
    </div>
}"#}
            </Code>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em; margin: 1em 0;">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disable scroll wheel handling"</Label>
            </FormControl>

            <div
                {..scroll_wheel.props.into_attrs()}
                tabindex="0"
                style="
                    border: 3px solid var(--brand-color);
                    padding: 2em;
                    border-radius: 8px;
                    background: var(--brand-color-light, rgba(230, 105, 86, 0.1));
                    text-align: center;
                    cursor: ns-resize;
                    margin: 1em 0;
                "
            >
                <p style="margin: 0 0 1em 0; font-weight: bold;">"Scroll here to adjust value"</p>
                <div style="font-size: 3em; font-weight: bold;">
                    { move || value.get().round() as i32 }
                </div>
                <p style="margin: 1em 0 0 0; font-size: 0.9em; opacity: 0.8;">
                    "Use mouse wheel or trackpad"
                </p>
            </div>

            <p>"Last scroll delta: "
                <code>{ move || format!("({:.1}, {:.1})", last_delta.get().0, last_delta.get().1) }</code>
            </p>

            <h2 id="scroll-event" class="anchor">
                "ScrollEvent"
                <AnchorLink href="#scroll-event" description="Direct link to scroll event"/>
            </h2>

            <ul>
                <li><code>"delta_x"</code> " - Horizontal scroll amount"</li>
                <li><code>"delta_y"</code> " - Vertical scroll amount (positive = down)"</li>
            </ul>

            <h2 id="use-cases" class="anchor">
                "Use Cases"
                <AnchorLink href="#use-cases" description="Direct link to use cases"/>
            </h2>

            <ul>
                <li>"Custom scrollable containers"</li>
                <li>"Zoom controls (Ctrl+scroll)"</li>
                <li>"Numeric input adjustment"</li>
                <li>"Slider fine-tuning"</li>
                <li>"Image gallery navigation"</li>
                <li>"Timeline scrubbing"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Normalized scroll delta values"</li>
                <li>"Horizontal and vertical scroll support"</li>
                <li>"Can be temporarily disabled"</li>
                <li>"Attribute spreading for easy integration"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_scroll_wheel", link: "#use_scroll_wheel" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "ScrollEvent", link: "#scroll-event" },
                Toc::Leaf { title: "Use Cases", link: "#use-cases" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
