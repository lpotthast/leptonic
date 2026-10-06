use leptonic::hooks::{
    IntoAttrs, UseButtonInput, UseSpinButtonInput, UseSpinButtonReturn, use_button, use_spin_button,
};
use leptos::prelude::*;

/// A spin button over `value` (1 to 5) whose callbacks log their name to `log`, as the mocks of
/// react-aria's `useSpinButton.test.js`. Without `pages`, PageUp/PageDown have no callbacks of
/// their own.
#[component]
fn TestSpinButton(
    label: &'static str,
    value: RwSignal<f64>,
    log: RwSignal<String>,
    #[prop(optional)] pages: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_read_only: bool,
    #[prop(optional)] text_value: Option<Signal<Option<String>>>,
) -> impl IntoView {
    let logged = move |name: &'static str, change: f64| {
        Some(Callback::new(move |()| {
            log.set(name.to_owned());
            value.update(|v| *v += change);
        }))
    };
    let UseSpinButtonReturn {
        props,
        increment_button,
        ..
    } = use_spin_button(UseSpinButtonInput {
        value: Signal::derive(move || Some(value.get())),
        text_value: text_value.unwrap_or_default(),
        min_value: Signal::stored(Some(1.0)),
        max_value: Signal::stored(Some(5.0)),
        is_disabled: Signal::stored(is_disabled),
        is_read_only: Signal::stored(is_read_only),
        on_increment: logged("increment", 1.0),
        on_increment_page: if pages {
            logged("increment_page", 2.0)
        } else {
            None
        },
        on_decrement: logged("decrement", -1.0),
        on_decrement_page: if pages {
            logged("decrement_page", -2.0)
        } else {
            None
        },
        on_decrement_to_min: logged("decrement_to_min", 0.0),
        on_increment_to_max: logged("increment_to_max", 0.0),
        ..UseSpinButtonInput::default()
    });
    // Its increment stepper, as returned.
    let (up_attrs, up_styles) = use_button(UseButtonInput {
        aria_label: format!("{label} up").into(),
        ..increment_button
    })
    .props
    .into_parts();
    view! {
        <div {..props.into_attrs()} tabindex="0" aria-label=label>{move || value.get()}</div>
        <button {..up_attrs} style=up_styles>"+"</button>
    }
}

/// A spin button without a value.
#[component]
fn EmptySpinButton() -> impl IntoView {
    let UseSpinButtonReturn { props, .. } = use_spin_button(UseSpinButtonInput::default());
    view! { <div {..props.into_attrs()} tabindex="0" aria-label="Empty"></div> }
}

#[component]
pub fn PageHookSpinButton() -> impl IntoView {
    let log = RwSignal::new(String::new());
    let temperature = RwSignal::new(-5.0);

    view! {
        <h1>"use_spin_button"</h1>
        <TestSpinButton label="Pages" value=RwSignal::new(2.0) log pages=true />
        <TestSpinButton label="No pages" value=RwSignal::new(2.0) log />
        <TestSpinButton label="Disabled" value=RwSignal::new(2.0) log is_disabled=true />
        <TestSpinButton label="Read only" value=RwSignal::new(2.0) log is_read_only=true />
        <TestSpinButton
            label="Temperature"
            value=temperature
            log
            text_value=Signal::derive(move || Some(format!("{} \u{b0}C", temperature.get())))
        />
        <EmptySpinButton />
        <div>"Last call: " <span id="test-sb-log">{log}</span></div>
    }
}
