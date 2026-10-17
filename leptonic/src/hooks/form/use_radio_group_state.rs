// Upstream: react-stately/src/radio/useRadioGroupState.ts @ 99e6102368
use leptos::prelude::*;

use super::use_form_validation_state::{
    FormValidationState, UseFormValidationStateInput, ValidateFn, ValidationBehavior,
    use_form_validation_state,
};
use crate::{
    hooks::collections::Key,
    utils::{id::use_id, keyed_triggers::KeyedTriggers},
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Values are `Key`s (strings or integers), not strings: what collections and toggle button
//   groups use; they render as the radios' form values.
// - State (C4): `default_value` + `on_change`, or `value` bound to app state (a `ValueBinding`,
//   the atoms' `value` + `set_value`).
// - The validation behavior is set here (react-aria: on `useRadioGroup`), so the state and its
//   radios read it from one place.
// - The generated group name comes from `use_id` (hydration-stable). React-aria: a random
//   instance number plus a counter.
//
// ## ADDITIONS
// - Each radio subscribes to its own value (`subscribe_radio`): a change of the selection or of
//   the tab stop notifies the radios whose state changed, not every radio (react-aria re-renders
//   the whole group).
//
// =============================================================================

/// Input of [`use_radio_group_state`].
#[derive(Clone)]
pub struct UseRadioGroupStateInput {
    /// The initially selected value. Ignored when `value` is bound.
    pub default_value: Option<Key>,
    /// The selected value as app state, replacing `default_value`.
    pub value: Option<crate::ValueBinding<Option<Key>>>,
    /// Called with the selected value when it changes.
    pub on_change: Option<Callback<Option<Key>>>,
    /// The radios' `name` (for form submission). Generated when `None`.
    pub name: Option<String>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_required: Signal<bool>,
    pub is_invalid: Signal<bool>,
    pub validate: Option<ValidateFn<Option<Key>>>,
    pub validation_behavior: ValidationBehavior,
}

impl Default for UseRadioGroupStateInput {
    fn default() -> Self {
        Self {
            default_value: None,
            value: None,
            on_change: None,
            name: None,
            is_disabled: Signal::stored(false),
            is_read_only: Signal::stored(false),
            is_required: Signal::stored(false),
            is_invalid: Signal::stored(false),
            validate: None,
            validation_behavior: ValidationBehavior::default(),
        }
    }
}

impl std::fmt::Debug for UseRadioGroupStateInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseRadioGroupStateInput")
            .field("default_value", &self.default_value)
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

/// The state of a radio group: its selected value, the radio focused last, and validation.
#[derive(Clone, Copy)]
pub struct RadioGroupState {
    /// The selected value.
    pub selected_value: Signal<Option<Key>>,
    /// The radio focused last (decides the group's tab stop while nothing is selected).
    pub last_focused_value: Signal<Option<Key>>,
    pub is_disabled: Signal<bool>,
    pub is_read_only: Signal<bool>,
    pub is_required: Signal<bool>,
    /// Whether the group's displayed validation is invalid.
    pub is_invalid: Signal<bool>,
    pub validation: FormValidationState,
    pub validation_behavior: ValidationBehavior,
    name: StoredValue<String>,
    default_selected_value: StoredValue<Option<Key>>,
    set_selected: crate::ValueBinding<Option<Key>>,
    set_last_focused: WriteSignal<Option<Key>>,
    on_change: Option<Callback<Option<Key>>>,
    /// The radios, by value: notified when their selection or tab stop changes.
    radios: KeyedTriggers<Key>,
    /// Notified when every radio becomes a tab stop, or stops being one (nothing selected or
    /// focused): only the radios' tab stops change then, not their selection.
    every_radio_tab_stop: Trigger,
    /// The selected and the last focused value the radios were last notified of.
    notified: StoredValue<(Option<Key>, Option<Key>)>,
}

impl std::fmt::Debug for RadioGroupState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadioGroupState")
            .field("selected_value", &self.selected_value.get_untracked())
            .finish_non_exhaustive()
    }
}

impl RadioGroupState {
    /// The radios' `name`.
    pub fn name(&self) -> String {
        self.name.get_value()
    }

    /// The initially selected value (restored on form reset).
    pub fn default_selected_value(&self) -> Option<Key> {
        self.default_selected_value.get_value()
    }

    /// Select `value` (ignored while the group is disabled or read-only).
    pub fn set_selected_value(&self, value: Option<Key>) {
        if self.is_disabled.get_untracked() || self.is_read_only.get_untracked() {
            return;
        }
        if self.selected_value.with_untracked(|v| *v != value) {
            self.set_selected.set(value.clone());
            self.notify_radios();
            if let Some(on_change) = self.on_change {
                on_change.run(value);
            }
        }
        self.validation.commit_validation();
    }

    /// Record the radio focused last.
    pub fn set_last_focused_value(&self, value: Option<Key>) {
        if self
            .last_focused_value
            .with_untracked(|last| *last != value)
        {
            self.set_last_focused.set(value);
            self.notify_radios();
        }
    }

    /// Subscribes the radio of `value` to changes of its selection and tab stop: once per radio,
    /// until its owner is cleaned up. Its readers track the trigger and read the state untracked;
    /// readers of the tab stop also call [`Self::track_every_radio_tab_stop`].
    pub(crate) fn subscribe_radio(&self, value: Key) -> ArcTrigger {
        self.radios.subscribe(value)
    }

    /// Tracks whether every radio is a tab stop (the changes of the tab stops that the radios'
    /// own triggers don't report).
    pub(crate) fn track_every_radio_tab_stop(&self) {
        self.every_radio_tab_stop.track();
    }

    /// Whether the radio of `value` is the group's tab stop: the selected radio, else the one
    /// focused last, else every radio.
    pub(crate) fn is_tab_stop_untracked(&self, value: &Key) -> bool {
        self.selected_value.with_untracked(|selected| {
            self.last_focused_value
                .with_untracked(|last| is_tab_stop(selected.as_ref(), last.as_ref(), value))
        })
    }

    /// Tells the radios whose selection or tab stop changed since they were last told.
    fn notify_radios(&self) {
        let current = (
            self.selected_value.get_untracked(),
            self.last_focused_value.get_untracked(),
        );
        let Some((selected, last)) = self
            .notified
            .try_update_value(|notified| std::mem::replace(notified, current.clone()))
        else {
            return;
        };
        let (new_selected, new_last) = (current.0.as_ref(), current.1.as_ref());
        let (selected, last) = (selected.as_ref(), last.as_ref());
        // With nothing selected or focused, every radio is a tab stop: a change from or to that
        // changes every radio's tab stop.
        let every_radio =
            |selected: Option<&Key>, last: Option<&Key>| selected.is_none() && last.is_none();
        let every_radio_changed =
            every_radio(selected, last) != every_radio(new_selected, new_last);
        if every_radio_changed {
            self.every_radio_tab_stop.notify();
        }
        // The radios whose selection changed, and whose tab stop changed otherwise: at most the
        // previously and the newly selected and focused ones.
        let changed = |key: &Key| {
            (selected == Some(key)) != (new_selected == Some(key))
                || (!every_radio_changed
                    && is_tab_stop(selected, last, key) != is_tab_stop(new_selected, new_last, key))
        };
        let candidates = [selected, last, new_selected, new_last];
        let changed = candidates.iter().enumerate().filter_map(|(i, key)| {
            let key = (*key)?;
            (!candidates[..i].contains(&Some(key)) && changed(key)).then_some(key)
        });
        self.radios.notify(changed);
    }
}

/// Whether the radio of `value` is the tab stop of a group with `selected` and `last_focused`.
fn is_tab_stop(selected: Option<&Key>, last_focused: Option<&Key>, value: &Key) -> bool {
    match selected {
        Some(selected) => selected == value,
        None => last_focused.is_none_or(|last| last == value),
    }
}

/// Creates the state of a radio group.
pub fn use_radio_group_state(input: UseRadioGroupStateInput) -> RadioGroupState {
    crate::metrics::subject!();
    let UseRadioGroupStateInput {
        default_value,
        value,
        on_change,
        name,
        is_disabled,
        is_read_only,
        is_required,
        is_invalid,
        validate,
        validation_behavior,
    } = input;
    let name = name.unwrap_or_else(|| use_id("radio-group").to_string());
    let is_bound = value.is_some();
    let set_selected =
        value.unwrap_or_else(|| crate::ValueBinding::from(RwSignal::new(default_value)));
    let selected_value = set_selected.value;
    let default_value = selected_value.get_untracked();
    let (last_focused_value, set_last_focused) = signal(None);
    let validation = use_form_validation_state(UseFormValidationStateInput {
        builtin_validation: Signal::default(),
        is_invalid,
        value: selected_value,
        validate,
        validation_behavior,
        names: vec![name.clone()],
    });
    let state = RadioGroupState {
        selected_value,
        last_focused_value: last_focused_value.into(),
        is_disabled,
        is_read_only,
        is_required,
        is_invalid: validation.is_invalid,
        validation,
        validation_behavior,
        name: StoredValue::new(name),
        default_selected_value: StoredValue::new(default_value.clone()),
        set_selected,
        set_last_focused,
        on_change,
        radios: KeyedTriggers::new(),
        every_radio_tab_stop: Trigger::new(),
        notified: StoredValue::new((default_value, None)),
    };
    // A bound value the app changes (not through the state): tell the radios right away.
    if is_bound {
        ImmediateEffect::new_scoped(move || {
            selected_value.track();
            state.notify_radios();
        });
    }
    state
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::testing::{count_allocations, flush_effects, with_owner};

    #[test]
    fn a_bound_value_is_read_and_written() {
        with_owner(|| {
            let app = RwSignal::new(Some(Key::from("a")));
            let state = use_radio_group_state(UseRadioGroupStateInput {
                value: Some(crate::ValueBinding::from(app)),
                ..UseRadioGroupStateInput::default()
            });
            assert_that!(state.default_selected_value()).is_equal_to(Some(Key::from("a")));
            state.set_selected_value(Some(Key::from("b")));
            assert_that!(app.get_untracked()).is_equal_to(Some(Key::from("b")));
            app.set(Some(Key::from("c")));
            assert_that!(state.selected_value.get_untracked()).is_equal_to(Some(Key::from("c")));
        });
    }

    #[test]
    fn selects_values_unless_read_only_or_disabled() {
        with_owner(|| {
            let changes = RwSignal::new(Vec::new());
            let is_read_only = RwSignal::new(false);
            let state = use_radio_group_state(UseRadioGroupStateInput {
                default_value: Some(Key::from("a")),
                on_change: Some(Callback::new(move |v| changes.update(|c| c.push(v)))),
                is_read_only: is_read_only.into(),
                ..UseRadioGroupStateInput::default()
            });
            state.set_selected_value(Some(Key::from("a")));
            state.set_selected_value(Some(Key::from("b")));
            is_read_only.set(true);
            state.set_selected_value(Some(Key::from("c")));
            assert_that!(state.selected_value.get_untracked()).is_equal_to(Some(Key::from("b")));
            assert_that!(changes.get_untracked()).is_equal_to(vec![Some(Key::from("b"))]);
            assert_that!(state.default_selected_value()).is_equal_to(Some(Key::from("a")));
        });
    }

    /// Subscribes radios `0..4` and counts how often each one's readers rerun.
    fn radios(state: &RadioGroupState) -> impl Fn() -> Vec<usize> {
        let state = *state;
        let radios: Vec<(Memo<(bool, bool)>, RwSignal<usize>)> = (0..4)
            .map(|value| {
                let key = Key::from(value);
                let changed = state.subscribe_radio(key.clone());
                let runs = RwSignal::new(0);
                let memo = Memo::new(move |_| {
                    changed.track();
                    state.track_every_radio_tab_stop();
                    runs.update_untracked(|runs| *runs += 1);
                    let selected = state
                        .selected_value
                        .with_untracked(|selected| selected.as_ref() == Some(&key));
                    (selected, state.is_tab_stop_untracked(&key))
                });
                memo.get_untracked();
                (memo, runs)
            })
            .collect();
        move || {
            radios
                .iter()
                .map(|(memo, runs)| {
                    memo.get_untracked();
                    runs.get_untracked() - 1
                })
                .collect()
        }
    }

    #[test]
    fn notifies_the_radios_whose_state_changed() {
        with_owner(|| {
            let state = use_radio_group_state(UseRadioGroupStateInput {
                default_value: Some(Key::from(0)),
                ..UseRadioGroupStateInput::default()
            });
            let reruns = radios(&state);
            state.set_selected_value(Some(Key::from(2)));
            assert_that!(reruns()).is_equal_to(vec![1, 0, 1, 0]);
            // While a radio is selected, it is the tab stop: focus changes nothing.
            state.set_last_focused_value(Some(Key::from(3)));
            assert_that!(reruns()).is_equal_to(vec![1, 0, 1, 0]);
            // Nothing selected: the radio focused last is the tab stop.
            state.set_selected_value(None);
            assert_that!(reruns()).is_equal_to(vec![1, 0, 2, 1]);
            state.set_last_focused_value(Some(Key::from(1)));
            assert_that!(reruns()).is_equal_to(vec![1, 1, 2, 2]);
            // Nothing selected or focused: every radio is a tab stop.
            state.set_last_focused_value(None);
            assert_that!(reruns()).is_equal_to(vec![2, 2, 3, 3]);
        });
    }

    #[test]
    fn a_change_of_every_tab_stop_leaves_the_selection_readers_alone() {
        with_owner(|| {
            let state = use_radio_group_state(UseRadioGroupStateInput::default());
            let changed = state.subscribe_radio(Key::from(0));
            let selection_runs = RwSignal::new(0);
            let is_selected = Memo::new(move |_| {
                changed.track();
                selection_runs.update_untracked(|runs| *runs += 1);
                state
                    .selected_value
                    .with_untracked(|selected| selected.as_ref() == Some(&Key::from(0)))
            });
            is_selected.get_untracked();
            // Every radio was a tab stop; now only radio 1 is.
            state.set_last_focused_value(Some(Key::from(1)));
            is_selected.get_untracked();
            assert_that!(selection_runs.get_untracked()).is_equal_to(1);
            state.set_selected_value(Some(Key::from(0)));
            assert_that!(is_selected.get_untracked()).is_true();
            assert_that!(selection_runs.get_untracked()).is_equal_to(2);
        });
    }

    #[test]
    fn notifies_the_radios_of_app_changes() {
        with_owner(|| {
            let app = RwSignal::new(Some(Key::from(0)));
            let state = use_radio_group_state(UseRadioGroupStateInput {
                value: Some(crate::ValueBinding::from(app)),
                ..UseRadioGroupStateInput::default()
            });
            let reruns = radios(&state);
            app.set(Some(Key::from(3)));
            assert_that!(reruns()).is_equal_to(vec![1, 0, 0, 1]);
            state.set_selected_value(Some(Key::from(1)));
            assert_that!(reruns()).is_equal_to(vec![1, 1, 0, 2]);
        });
    }

    /// The allocations of selecting the last of `n` radios (each with a render effect reading its
    /// selection and tab stop, as a view does).
    fn selection_cost(n: i32) -> usize {
        with_owner(|| {
            let state = use_radio_group_state(UseRadioGroupStateInput {
                default_value: Some(Key::from(0)),
                ..UseRadioGroupStateInput::default()
            });
            let effects: Vec<RenderEffect<()>> = (0..n)
                .map(|value| {
                    let key = Key::from(value);
                    let changed = state.subscribe_radio(key.clone());
                    RenderEffect::new(move |_| {
                        changed.track();
                        state.track_every_radio_tab_stop();
                        state.is_tab_stop_untracked(&key);
                    })
                })
                .collect();
            flush_effects();
            let ((), allocations) = count_allocations(|| {
                state.set_selected_value(Some(Key::from(n - 1)));
                flush_effects();
            });
            drop(effects);
            allocations
        })
    }

    #[test]
    fn a_selection_costs_the_same_in_any_group_size() {
        assert_that!(selection_cost(1_000)).is_equal_to(selection_cost(10));
    }

    #[test]
    fn named_explicitly_or_generated() {
        with_owner(|| {
            let named = use_radio_group_state(UseRadioGroupStateInput {
                name: Some("plan".to_owned()),
                ..UseRadioGroupStateInput::default()
            });
            assert_that!(named.name()).is_equal_to("plan".to_owned());
            let generated = use_radio_group_state(UseRadioGroupStateInput::default());
            assert_that!(generated.name().is_empty()).is_false();
        });
    }
}
