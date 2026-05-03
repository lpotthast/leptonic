use leptonic::{
    components::prelude::*,
    hooks::*,
    utils::{classes::Classes, css::em},
};
use leptos::{prelude::*, web_sys};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

#[component]
pub fn FocusManagerBasicDemo() -> impl IntoView {
    let (wrap, set_wrap) = signal(true);
    let (tabbable_only, set_tabbable_only) = signal(false);

    let last_focused: StoredValue<Option<SendWrapper<web_sys::Element>>> = StoredValue::new(None);

    let UseFocusManagerReturn {
        focus_manager,
        props,
    } = use_focus_manager(UseFocusManagerInput::default());

    let build_opts = {
        let last = last_focused;
        move || {
            let from = last.with_value(|el| el.as_ref().map(|sw| sw.clone().take()));
            FocusManagerOptions {
                from,
                wrap: wrap.get_untracked(),
                tabbable: tabbable_only.get_untracked(),
                ..Default::default()
            }
        }
    };

    let store_result = move |result: Option<web_sys::Element>| {
        if let Some(el) = result {
            last_focused.set_value(Some(SendWrapper::new(el)));
        }
    };

    view! {
        <Stack orientation=StackOrientation::Horizontal spacing=em(0.5) attr:style="margin-bottom: 1em;">
            <button
                class=Classes::from("demo-btn")
                on:click={
                    let fm = focus_manager.clone();
                    let opts = build_opts;
                    move |_| store_result(fm.focus_first(opts()))
                }
            >
                "Focus First"
            </button>
            <button
                class=Classes::from("demo-btn")
                on:click={
                    let fm = focus_manager.clone();
                    let opts = build_opts;
                    move |_| store_result(fm.focus_previous(opts()))
                }
            >
                "Focus Previous"
            </button>
            <button
                class=Classes::from("demo-btn")
                on:click={
                    let fm = focus_manager.clone();
                    let opts = build_opts;
                    move |_| store_result(fm.focus_next(opts()))
                }
            >
                "Focus Next"
            </button>
            <button
                class=Classes::from("demo-btn")
                on:click={
                    let fm = focus_manager.clone();
                    let opts = build_opts;
                    move |_| store_result(fm.focus_last(opts()))
                }
            >
                "Focus Last"
            </button>
        </Stack>

        <Stack orientation=StackOrientation::Vertical spacing=em(0.5) attr:style="margin-bottom: 1em;">
            <FormControl classes="demo-form-row">
                <Checkbox checked=wrap set_checked=set_wrap />
                <Label>"Wrap around"</Label>
            </FormControl>
            <FormControl classes="demo-form-row">
                <Checkbox checked=tabbable_only set_checked=set_tabbable_only />
                <Label>"Tabbable only (tabindex >= 0)"</Label>
            </FormControl>
        </Stack>

        <style>
            ".focus-scope-demo button:focus, .focus-scope-demo input:focus { outline: 3px solid var(--brand-color, #e66956); outline-offset: 2px; }"
        </style>

        <div
            {..props.into_attrs()}
            class=Classes::from(["focus-scope-demo", "demo-container-active"])
            on:focusin=move |ev| {
                if let Some(target) = ev.target()
                    && let Some(el) = target.dyn_ref::<web_sys::Element>() {
                        last_focused.set_value(Some(SendWrapper::new(el.clone())));
                    }
            }
        >
            <p class=Classes::from("demo-container-title")>"Focus Scope Container"</p>
            <Stack orientation=StackOrientation::Horizontal spacing=em(0.5)>
                <button class=Classes::from("demo-btn-solid")>
                    "Button 1"
                </button>
                <button class=Classes::from("demo-btn-solid")>
                    "Button 2"
                </button>
                <input
                    type="text"
                    placeholder="Input field"
                    class=Classes::from("demo-input-solid")
                />
                <button class=Classes::from("demo-btn-solid")>
                    "Button 3"
                </button>
            </Stack>
        </div>
    }
}
