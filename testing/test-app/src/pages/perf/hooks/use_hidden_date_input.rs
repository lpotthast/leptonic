//! `use_hidden_date_input` on plain elements, for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_hidden_date_input.rs`): the visually hidden date input
//! (`#perf-hidden`, for a browser's autofill) of a date and time field's state
//! (`use_date_field_state`, empty, its value app state and shown in `#perf-value`; its placeholder
//! June 12, 2024, midnight: never today), mounted at first. Its controls ([`Control`]) mount and
//! unmount it, set and clear the value, switch the granularity between minutes and days (a
//! `datetime-local` or a `date` input) and disable and enable it.

use leptonic::{
    hooks::datepicker::{
        Granularity, UseDateFieldStateInput, UseHiddenDateInputInput, UseHiddenDateInputReturn,
        use_date_field_state, use_hidden_date_input,
    },
    jiff::civil::{DateTime, date},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The field's state and its hidden date input.
#[component]
fn HiddenInput(
    value: RwSignal<Option<DateTime>>,
    granularity: RwSignal<Option<Granularity>>,
    disabled: RwSignal<bool>,
) -> impl IntoView {
    let state = use_date_field_state(UseDateFieldStateInput::<DateTime> {
        value: Some(value.into()),
        placeholder_value: Signal::stored(Some(date(2024, 6, 12).at(0, 0, 0, 0))),
        granularity: granularity.into(),
        is_disabled: disabled.into(),
        ..UseDateFieldStateInput::default()
    });
    let UseHiddenDateInputReturn {
        container: container_props,
        input: input_props,
    } = use_hidden_date_input(UseHiddenDateInputInput {
        value: state.value,
        base: state.date_value,
        granularity: state.granularity,
        set_value: Callback::new(move |value| state.set_value(value)),
        auto_complete: Some("bday".to_owned()),
        name: None,
        is_disabled: state.is_disabled,
    });
    let (container_attrs, styles) = container_props.attrs();
    view! {
        <div {..container_attrs} style=styles>
            <input {..input_props.attrs()} id="perf-hidden" />
        </div>
    }
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let value = RwSignal::new(None::<DateTime>);
    let granularity = RwSignal::new(None::<Granularity>);
    let disabled = RwSignal::new(false);
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("set-value", move || {
            value.set(Some(date(2024, 6, 5).at(9, 45, 0, 0)));
        }),
        Control::action("clear-value", move || value.set(None)),
        Control::action("show-days", move || granularity.set(Some(Granularity::Day))),
        Control::action("show-minutes", move || granularity.set(None)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
    ];
    view! {
        <h1>"use_hidden_date_input performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <HiddenInput value=value granularity=granularity disabled=disabled />
        </Show>
        <p>
            "Value: "
            <span id="perf-value">
                {move || value.get().map_or_else(|| "none".to_owned(), |value| value.to_string())}
            </span>
        </p>
    }
}
