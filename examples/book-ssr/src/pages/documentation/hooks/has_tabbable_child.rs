use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::{
    use_has_tabbable_child, UseHasTabbableChildInput, UseHasTabbableChildReturn,
};
use leptonic::prelude::Size;
use leptos::prelude::*;

#[component]
pub fn PageUseHasTabbableChild() -> impl IntoView {
    let (show_button, set_show_button) = signal(true);
    let (show_input, set_show_input) = signal(true);

    let UseHasTabbableChildReturn {
        has_tabbable_child,
        props,
    } = use_has_tabbable_child(UseHasTabbableChildInput::default());

    view! {
        <Article>
            <h1 id="use_has_tabbable_child" class="anchor">
                "use_has_tabbable_child"
                <AnchorLink href="#use_has_tabbable_child" description="Direct link to article header"/>
            </h1>

            <p>"Hook that detects whether an element contains any tabbable child elements. Useful for focus management and accessibility."</p>

            <Code>
                {r#"let UseHasTabbableChildReturn { has_tabbable_child, props } =
    use_has_tabbable_child(UseHasTabbableChildInput::default());

view! {
    <div
        {..props.into_attrs()}
        tabindex=move || if has_tabbable_child.get() { -1 } else { 0 }
    >
        <button>"Child button"</button>
    </div>
}"#}
            </Code>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Toggle the elements below to see how the tabbable child detection changes:"</p>

            <Stack orientation=StackOrientation::Vertical spacing=Size::Em(0.5) attr:style="margin: 1em 0;">
                <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em;">
                    <Checkbox checked=show_button set_checked=set_show_button />
                    <Label>"Show button"</Label>
                </FormControl>
                <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em;">
                    <Checkbox checked=show_input set_checked=set_show_input />
                    <Label>"Show input"</Label>
                </FormControl>
            </Stack>

            <div
                {..props.into_attrs()}
                style="
                    border: 3px solid var(--brand-color);
                    padding: 1.5em;
                    border-radius: 8px;
                    margin: 1em 0;
                    min-height: 80px;
                "
            >
                <p style="margin: 0 0 1em 0; font-weight: bold;">"Container"</p>
                <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                    <Show when=move || show_button.get()>
                        <button style="padding: 0.5em 1em; border-radius: 4px; border: 1px solid #ccc; cursor: pointer;">
                            "Tabbable Button"
                        </button>
                    </Show>
                    <Show when=move || show_input.get()>
                        <input
                            type="text"
                            placeholder="Tabbable input"
                            style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px;"
                        />
                    </Show>
                </Stack>
            </div>

            <p>
                "Has tabbable child: "
                <strong style=move || if has_tabbable_child.get() { "color: green;" } else { "color: red;" }>
                    { move || if has_tabbable_child.get() { "true" } else { "false" } }
                </strong>
            </p>

            <h2 id="use-cases" class="anchor">
                "Use Cases"
                <AnchorLink href="#use-cases" description="Direct link to use cases"/>
            </h2>

            <ul>
                <li>"Focus trapping in modals - ensure there's something to focus"</li>
                <li>"Skip link visibility - show only when there's content to skip to"</li>
                <li>"Keyboard navigation - determine if container needs special handling"</li>
                <li>"Accessibility testing - validate focusable content exists"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Reactive detection of tabbable children"</li>
                <li>"Considers tabindex values"</li>
                <li>"Filters out disabled elements"</li>
                <li>"Handles dynamic content changes"</li>
                <li>"Automatic element capture via prop spreading"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_has_tabbable_child", link: "#use_has_tabbable_child" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Use Cases", link: "#use-cases" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
