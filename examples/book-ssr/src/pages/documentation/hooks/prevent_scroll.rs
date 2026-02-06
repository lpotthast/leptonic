use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUsePreventScroll() -> impl IntoView {
    let (prevent_scroll, set_prevent_scroll) = signal(false);

    let _scroll_prevention = use_prevent_scroll(UsePreventScrollInput {
        disabled: Signal::derive(move || !prevent_scroll.get()),
    });

    view! {
        <Article>
            <h1 id="use_prevent_scroll" class="anchor">
                "use_prevent_scroll"
                <AnchorLink href="#use_prevent_scroll" description="Direct link to article header"/>
            </h1>

            <p>"Prevent the page from scrolling. Commonly used when displaying modals, overlays, or other UI elements that should prevent background scrolling."</p>

            <Code>
                {r#"let (is_modal_open, set_is_modal_open) = signal(false);

// Scroll is prevented when disabled=false (i.e., when modal is open)
use_prevent_scroll(UsePreventScrollInput {
    disabled: Signal::derive(move || !is_modal_open.get()),
});

view! {
    <button on:click=move |_| set_is_modal_open.set(true)>
        "Open Modal"
    </button>
}"#}
            </Code>

            <p>"Toggle scroll prevention to see the effect. When enabled, you won't be able to scroll the page:"</p>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em; margin: 1em 0;">
                <Checkbox checked=prevent_scroll set_checked=set_prevent_scroll />
                <Label>"Prevent scroll"</Label>
            </FormControl>

            <div style="
                padding: 1em;
                border-radius: 8px;
                background: var(--brand-color-light, rgba(230, 105, 86, 0.1));
                border: 2px solid var(--brand-color);
                margin: 1em 0;
            ">
                <Show
                    when=move || prevent_scroll.get()
                    fallback=|| view! {
                        <p style="margin: 0;">"Scroll prevention is " <strong>"disabled"</strong> ". You can scroll the page normally."</p>
                    }
                >
                    <p style="margin: 0;">"Scroll prevention is " <strong>"enabled"</strong> ". Try scrolling - the page won't scroll!"</p>
                </Show>
            </div>

            <h2 id="how-it-works" class="anchor">
                "How it works"
                <AnchorLink href="#how-it-works" description="Direct link to how it works"/>
            </h2>

            <p>"The hook adds " <code>"overflow: hidden"</code> " to the document root element and adjusts padding to compensate for the scrollbar width, preventing layout shift."</p>

            <h2 id="reference-counting" class="anchor">
                "Reference Counting"
                <AnchorLink href="#reference-counting" description="Direct link to reference counting"/>
            </h2>

            <p>"Multiple components can call " <code>"use_prevent_scroll"</code> " simultaneously. The hook uses reference counting - scroll prevention is only removed when all components have cleaned up or disabled their prevention."</p>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Prevents page scrolling by setting overflow: hidden on document root"</li>
                <li>"Compensates for scrollbar width to prevent layout shift"</li>
                <li>"Reference counted - supports multiple simultaneous users"</li>
                <li>"Automatically cleans up on component unmount"</li>
            </ul>

            <h2 id="what-still-scrolls" class="anchor">
                "What still scrolls"
                <AnchorLink href="#what-still-scrolls" description="Direct link to what still scrolls"/>
            </h2>

            <p>"This hook only prevents scrolling on the main document/page. Nested scrollable containers (elements with their own " <code>"overflow: auto"</code> " or " <code>"overflow: scroll"</code> ") will still be scrollable. This is typically the desired behavior for modals with scrollable content."</p>

            <h2 id="ios-safari" class="anchor">
                "iOS Safari limitation"
                <AnchorLink href="#ios-safari" description="Direct link to iOS Safari limitation"/>
            </h2>

            <p>"This hook does not fully prevent scrolling on iOS Safari. React Aria implements sophisticated touch event handling and focus interception for iOS, but this is complex to port to Rust/WASM:"</p>

            <ul>
                <li>"Requires intercepting touch events and tracking scrollable parents"</li>
                <li>"Requires overriding " <code>"HTMLElement.prototype.focus"</code> " to call " <code>"focus({preventScroll: true})"</code></li>
                <li>"Requires custom scroll-into-view logic for keyboard navigation"</li>
            </ul>

            <p>"For iOS Safari support, consider using CSS " <code>"touch-action: none"</code> " on the overlay/modal container as a workaround."</p>

            // Add some content to make the page scrollable
            <div style="margin-top: 2em;">
                <h3>"Sample content to enable scrolling"</h3>
                <p>"This content is here to make the page scrollable so you can test the scroll prevention."</p>
                {(0..10).map(|i| view! {
                    <p>"Paragraph " {i + 1} " - Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."</p>
                }).collect::<Vec<_>>()}
            </div>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_prevent_scroll", link: "#use_prevent_scroll" },
                Toc::Leaf { title: "How it works", link: "#how-it-works" },
                Toc::Leaf { title: "Reference Counting", link: "#reference-counting" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "What still scrolls", link: "#what-still-scrolls" },
                Toc::Leaf { title: "iOS Safari limitation", link: "#ios-safari" },
            ]
        }/>
    }
}
