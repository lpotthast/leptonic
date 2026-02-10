use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::atoms::prelude::FocusScope;
use leptonic::components::prelude::*;
use leptonic::hooks::{
    use_focus_manager, FocusManagerOptions, UseFocusManagerInput, UseFocusManagerReturn,
};
use leptonic::prelude::Size;
use leptos::prelude::*;
use leptos::web_sys;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

#[component]
pub fn PageUseFocusManager() -> impl IntoView {
    let (wrap, set_wrap) = signal(true);

    // Track the last focused element within the scope.
    // This is needed because when control buttons are clicked, they become
    // document.activeElement, which is outside the scope.
    // Using ElementMaybeSignal pattern with SendWrapper for Send+Sync compatibility.
    let last_focused: StoredValue<Option<SendWrapper<web_sys::Element>>> = StoredValue::new(None);

    let UseFocusManagerReturn {
        focus_manager,
        props,
    } = use_focus_manager(UseFocusManagerInput::default());

    view! {
        <Article>
            <h1 id="use_focus_manager" class="anchor">
                "use_focus_manager"
                <AnchorLink href="#use_focus_manager" description="Direct link to article header"/>
            </h1>

            <p>"Programmatically navigate focus within a container. Provides methods to move focus to next, previous, first, or last focusable element."</p>

            <Code>
                {indoc!(r#"
                    let UseFocusManagerReturn { focus_manager, props } =
                        use_focus_manager(UseFocusManagerInput::default());

                    view! {
                        <div {..props.into_attrs()}>
                            <button>"First"</button>
                            <button>"Second"</button>
                            <button on:click=move |_| {
                                focus_manager.focus_next(FocusManagerOptions::default());
                            }>"Next"</button>
                        </div>
                    }
                "#)}
            </Code>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"Use the control buttons to move focus within the scope container:"</p>

            <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5) attr:style="margin-bottom: 1em;">
                <button
                    style="padding: 0.5em 1em; cursor: pointer; border-radius: 4px; border: 1px solid #ccc;"
                    on:click={
                        let focus_manager = focus_manager.clone();
                        move |_| {
                            let from = last_focused.with_value(|el| el.as_ref().map(|sw| sw.clone().take()));
                            let result = focus_manager.focus_first(FocusManagerOptions {
                                from,
                                wrap: wrap.get(),
                                ..Default::default()
                            });
                            if let Some(el) = result {
                                last_focused.set_value(Some(SendWrapper::new(el)));
                            }
                        }
                    }
                >
                    "Focus First"
                </button>
                <button
                    style="padding: 0.5em 1em; cursor: pointer; border-radius: 4px; border: 1px solid #ccc;"
                    on:click={
                        let focus_manager = focus_manager.clone();
                        move |_| {
                            let from = last_focused.with_value(|el| el.as_ref().map(|sw| sw.clone().take()));
                            let result = focus_manager.focus_previous(FocusManagerOptions {
                                from,
                                wrap: wrap.get(),
                                ..Default::default()
                            });
                            if let Some(el) = result {
                                last_focused.set_value(Some(SendWrapper::new(el)));
                            }
                        }
                    }
                >
                    "Focus Previous"
                </button>
                <button
                    style="padding: 0.5em 1em; cursor: pointer; border-radius: 4px; border: 1px solid #ccc;"
                    on:click={
                        let focus_manager = focus_manager.clone();
                        move |_| {
                            let from = last_focused.with_value(|el| el.as_ref().map(|sw| sw.clone().take()));
                            let result = focus_manager.focus_next(FocusManagerOptions {
                                from,
                                wrap: wrap.get(),
                                ..Default::default()
                            });
                            if let Some(el) = result {
                                last_focused.set_value(Some(SendWrapper::new(el)));
                            }
                        }
                    }
                >
                    "Focus Next"
                </button>
                <button
                    style="padding: 0.5em 1em; cursor: pointer; border-radius: 4px; border: 1px solid #ccc;"
                    on:click={
                        let focus_manager = focus_manager.clone();
                        move |_| {
                            let from = last_focused.with_value(|el| el.as_ref().map(|sw| sw.clone().take()));
                            let result = focus_manager.focus_last(FocusManagerOptions {
                                from,
                                wrap: wrap.get(),
                                ..Default::default()
                            });
                            if let Some(el) = result {
                                last_focused.set_value(Some(SendWrapper::new(el)));
                            }
                        }
                    }
                >
                    "Focus Last"
                </button>
            </Stack>

            <FormControl attr:style="flex-direction: row; align-items: center; gap: 0.5em; margin-bottom: 1em;">
                <Checkbox checked=wrap set_checked=set_wrap />
                <Label>"Wrap around"</Label>
            </FormControl>

            <style>
                ".focus-scope-demo button:focus, .focus-scope-demo input:focus {
                    outline: 3px solid var(--brand-color, #e66956);
                    outline-offset: 2px;
                }"
            </style>

            <div
                {..props.into_attrs()}
                class="focus-scope-demo"
                style="
                    border: 3px solid var(--brand-color);
                    padding: 1.5em;
                    border-radius: 8px;
                    background: var(--brand-color-light, rgba(230, 105, 86, 0.1));
                "
                on:focusin=move |ev| {
                    if let Some(target) = ev.target() {
                        if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                            last_focused.set_value(Some(SendWrapper::new(el.clone())));
                        }
                    }
                }
            >
                <p style="margin: 0 0 1em 0; font-weight: bold;">"Focus Scope Container"</p>
                <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                    <button style="padding: 0.75em 1.5em; border-radius: 4px; border: 2px solid #333; cursor: pointer; background: white;">
                        "Button 1"
                    </button>
                    <button style="padding: 0.75em 1.5em; border-radius: 4px; border: 2px solid #333; cursor: pointer; background: white;">
                        "Button 2"
                    </button>
                    <input
                        type="text"
                        placeholder="Input field"
                        style="padding: 0.75em; border: 2px solid #333; border-radius: 4px; width: 150px; background: white;"
                    />
                    <button style="padding: 0.75em 1.5em; border-radius: 4px; border: 2px solid #333; cursor: pointer; background: white;">
                        "Button 3"
                    </button>
                </Stack>
            </div>

            <h2 id="focus-trapping" class="anchor">
                "Focus Trapping with FocusScope"
                <AnchorLink href="#focus-trapping" description="Direct link to focus trapping"/>
            </h2>

            <p>"The " <code>"use_focus_manager"</code> " hook provides " <em>"programmatic"</em> " focus control only. It does not trap focus or intercept Tab key presses. For focus trapping (preventing Tab from leaving the container), use the " <code>"FocusScope"</code> " component which combines focus management with keyboard event handling."</p>

            <Code>
                {indoc!(r#"
                    <FocusScope contain=true auto_focus=true restore_focus=true>
                        <button>"First"</button>
                        <input type="text" placeholder="Middle" />
                        <button>"Last"</button>
                    </FocusScope>
                "#)}
            </Code>

            <p>"Try tabbing through the container below. Focus will wrap from the last element back to the first, and vice versa with Shift+Tab:"</p>

            <style>
                ".focus-trap-demo button:focus, .focus-trap-demo input:focus {
                    outline: 3px solid #4a9eff;
                    outline-offset: 2px;
                }"
            </style>

            <FocusScope contain=true>
                <div
                    class="focus-trap-demo"
                    style="
                        border: 3px solid #4a9eff;
                        padding: 1.5em;
                        border-radius: 8px;
                        background: rgba(74, 158, 255, 0.1);
                    "
                >
                    <p style="margin: 0 0 1em 0; font-weight: bold;">"Focus Trap Container (Tab cycles within)"</p>
                    <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                        <button style="padding: 0.75em 1.5em; border-radius: 4px; border: 2px solid #333; cursor: pointer; background: white;">
                            "Trapped 1"
                        </button>
                        <button style="padding: 0.75em 1.5em; border-radius: 4px; border: 2px solid #333; cursor: pointer; background: white;">
                            "Trapped 2"
                        </button>
                        <input
                            type="text"
                            placeholder="Trapped input"
                            style="padding: 0.75em; border: 2px solid #333; border-radius: 4px; width: 150px; background: white;"
                        />
                        <button style="padding: 0.75em 1.5em; border-radius: 4px; border: 2px solid #333; cursor: pointer; background: white;">
                            "Trapped 3"
                        </button>
                    </Stack>
                </div>
            </FocusScope>

            <h3>"FocusScope Props"</h3>

            <ul>
                <li><code>"contain"</code> " - When true, Tab/Shift+Tab navigation wraps within the scope"</li>
                <li><code>"auto_focus"</code> " - When true, focuses the first focusable element on mount"</li>
                <li><code>"restore_focus"</code> " - When true, restores focus to the previously focused element when unmounted"</li>
            </ul>

            <h2 id="options" class="anchor">
                "FocusManagerOptions"
                <AnchorLink href="#options" description="Direct link to options"/>
            </h2>

            <ul>
                <li><code>"from"</code> " - Element to start navigation from (defaults to document.activeElement)"</li>
                <li><code>"wrap"</code> " - Whether to wrap around when reaching the end/beginning"</li>
                <li><code>"tabbable"</code> " - Only consider elements with tabindex >= 0"</li>
                <li><code>"accept"</code> " - Custom filter function for acceptable elements"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Navigate to next/previous/first/last focusable element"</li>
                <li>"Respects tabindex order (positive tabindex elements come first)"</li>
                <li>"Filters out hidden and disabled elements"</li>
                <li>"Optional wrap-around behavior"</li>
                <li>"Custom element filtering via accept function"</li>
                <li>"Automatic element capture via prop spreading"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_focus_manager", link: "#use_focus_manager" },
                Toc::Leaf { title: "Interactive Demo", link: "#demo" },
                Toc::Leaf { title: "Focus Trapping", link: "#focus-trapping" },
                Toc::Leaf { title: "FocusManagerOptions", link: "#options" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
