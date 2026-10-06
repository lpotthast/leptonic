use leptonic::{
    components::prelude::*,
    hooks::*,
    utils::{classes::Classes, css::em},
};
use leptos::{prelude::*, web_sys};
use send_wrapper::SendWrapper;

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
        <Stack orientation=StackOrientation::Horizontal spacing=em(0.5) classes="demo-mb-1">
            <Button on_press={
                let fm = focus_manager.clone();
                move |_| store_result(fm.focus_first(build_opts()))
            }>
                "Focus First"
            </Button>
            <Button on_press={
                let fm = focus_manager.clone();
                move |_| store_result(fm.focus_previous(build_opts()))
            }>
                "Focus Previous"
            </Button>
            <Button on_press={
                let fm = focus_manager.clone();
                move |_| store_result(fm.focus_next(build_opts()))
            }>
                "Focus Next"
            </Button>
            <Button on_press={
                let fm = focus_manager.clone();
                move |_| store_result(fm.focus_last(build_opts()))
            }>
                "Focus Last"
            </Button>
        </Stack>

        <Stack orientation=StackOrientation::Vertical spacing=em(0.5) classes="demo-mb-1">
            <Checkbox state=(wrap, set_wrap) classes="demo-form-row">"Wrap around"</Checkbox>
            <Checkbox state=(tabbable_only, set_tabbable_only) classes="demo-form-row">"Tabbable only (tabindex >= 0)"</Checkbox>
        </Stack>

        <div
            {..props.into_attrs()}
            class=Classes::from("demo-focus-scope")
            on:focusin=move |ev| {
                last_focused.set_value(Some(SendWrapper::new(event_target::<web_sys::Element>(&ev))));
            }
        >
            <p class=Classes::from("demo-container-title")>"Managed container"</p>
            <div class=Classes::from("demo-focus-row")>
                <button class=Classes::from("demo-focus-item")>"Button 1"</button>
                <button class=Classes::from("demo-focus-item")>"Button 2"</button>
                <div tabindex="-1" class=Classes::from("demo-focus-item")>"tabindex=-1"</div>
                <input type="text" placeholder="Input field" class=Classes::from("demo-focus-item")/>
                <button class=Classes::from("demo-focus-item")>"Button 3"</button>
            </div>
        </div>
    }
}
