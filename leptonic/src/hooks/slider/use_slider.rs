// Upstream: react-aria/src/slider/useSlider.ts @ 99e6102368
// Upstream: react-aria/src/slider/utils.ts @ 99e6102368
// Upstream: react-aria/test/slider/useSlider.test.js @ 99e6102368
// Upstream: react-aria-components/test/Slider.test.js @ 99e6102368
use leptos::{ev, prelude::*};
use leptos_element_capture::{CapturedElement, ElementCaptureAttr};
use send_wrapper::SendWrapper;
use web_sys::{MouseEvent, PointerEvent};

use crate::{
    EventHandler, Id, IntoAttrs, OnEvent, Part, PropsWithStyles, SlotProps,
    hooks::{
        focus::{Modality, set_modality},
        form::{LabelElementType, UseFieldInput, UseFieldReturn, use_field},
        interactions::{MoveEndEvent, MoveEvent, MoveStartEvent, UseMoveInput, use_move},
        slider::SliderState,
    },
    utils::{
        aria::{AriaLive, AriaRole},
        dom_ext::{EventAccessors, EventTargetExt},
        event_listeners::{Listener, listen_to},
        fraction::Fraction,
        i18n::{WritingDirection, use_direction},
        number_value::NumberValue,
        orientation::Orientation,
        part::Remount,
        pointer_type::PointerType,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
        styles::{Styles, css::TouchAction, property::TouchActionProperty},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - What the thumbs need from the slider (react-aria's `sliderData` WeakMap keyed by the state)
//   is the returned `SliderData`, passed to `use_slider_thumb`.
// - The track is captured by its props (`track_element`) instead of a ref argument.
// - The slider is a field (C14, `use_field` instead of `useLabel`): a rendered description
//   describes every thumb, and there is an error message slot.
//
// ## DIFFERENT BEHAVIOR
// - Thumb ids derive from the group's id (react-aria: the label's id while there is a label), so
//   they stay stable while the label comes and goes (ids must match between server and client).
// - The thumbs' `aria-labelledby` lists the group's labelling ids (its label, `aria_labelledby`,
//   and its own id next to an `aria_label`; else only its own id), where react-aria lists the
//   label's id, else the group's: accessible-name computation doesn't follow the group's own
//   `aria-labelledby`, so a slider named only by `aria_labelledby` would name its thumbs by the
//   group's text content.
// - Track presses start on `pointerdown` only: PointerEvent is always available (CLAUDE.md), so
//   react-aria's mouse and touch fallbacks are omitted.
//
// =============================================================================

/// Input of [`use_slider`].
#[derive(Debug)]
pub struct UseSliderInput<T: NumberValue> {
    pub state: SliderState<T>,
    /// The group's id. Generated when `None`.
    pub id: Option<Id>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    /// Names the slider when there is no visible label.
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    /// Further elements describing every thumb (next to a rendered description).
    pub aria_describedby: Signal<Option<Id>>,
    /// Elements with details about every thumb (`aria-details`).
    pub aria_details: Signal<Option<Id>>,
}

/// What the thumbs need from their slider.
#[derive(Debug, Clone)]
pub struct SliderData {
    /// The slider group's id. Thumb ids derive from it (stable, unlike react-aria's, which derive
    /// from the label's id when there is one).
    pub id: Id,
    /// The ids the thumbs are labelled by: the group's `aria-labelledby` (its label, its
    /// `aria_labelledby`, and its own id next to an `aria_label`), else the group's id.
    pub labelled_by: Signal<Id>,
    /// What describes every thumb: the description while rendered, and `aria_describedby`.
    pub aria_describedby: Signal<Option<Id>>,
    /// What details every thumb (`aria_details`).
    pub aria_details: Signal<Option<Id>>,
}

impl SliderData {
    /// The id of thumb `index`'s input.
    pub fn thumb_id(&self, index: usize) -> Id {
        Id::derived(format_args!("{}-{index}", self.id))
    }
}

/// The return value of [`use_slider`].
#[derive(Debug)]
pub struct UseSliderReturn {
    /// The slider's label element (a `<span>`, focusing the thumb when clicked).
    pub label: Part<UseSliderLabelProps>,
    /// For a description of the slider (describes every thumb while rendered).
    pub description: Part<SlotProps>,
    /// For an error message. Render it only while the slider is invalid.
    pub error_message: Part<SlotProps>,
    /// The slider's group element.
    pub group: Part<UseSliderGroupProps>,
    /// The track: pressing it moves the closest thumb there.
    pub track: Part<PropsWithStyles<UseSliderTrackProps>>,
    /// An `<output>` showing the values.
    pub output: Part<UseSliderOutputProps>,
    /// For [`use_slider_thumb`](fn@super::use_slider_thumb).
    pub data: SliderData,
    /// The track element, for the thumbs.
    pub track_element: CapturedElement,
}

/// The label's props: a click focuses the first thumb (the label has no `for`: VoiceOver on iOS
/// would announce only the label for the first thumb).
#[derive(Debug)]
pub struct UseSliderLabelProps {
    pub id: Id,
    pub on_click: EventHandler<MouseEvent>,
}

impl Remount for UseSliderLabelProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            on_click: self.on_click.clone(),
        }
    }
}

pub type UseSliderLabelAttrs = (FixedAttr, OnEvent<ev::click>);

impl IntoAttrs for UseSliderLabelProps {
    type Attrs = UseSliderLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (fixed_attr("id", self.id), self.on_click.into_on(ev::click))
    }
}

#[derive(Debug)]
pub struct UseSliderGroupProps {
    pub id: Id,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
}

impl Remount for UseSliderGroupProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
        }
    }
}

pub type UseSliderGroupAttrs = (
    FixedAttr,
    FixedAttr,
    SignalAttr<Option<String>>,
    SignalAttr<Option<Id>>,
);

impl IntoAttrs for UseSliderGroupProps {
    type Attrs = UseSliderGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("role", AriaRole::Group),
            fixed_attr("id", self.id),
            maybe_attr("aria-label", self.aria_label),
            signal_attr("aria-labelledby", self.aria_labelledby),
        )
    }
}

#[derive(Debug)]
pub struct UseSliderTrackProps {
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub element_capture: ElementCaptureAttr,
}

impl Remount for UseSliderTrackProps {
    fn remount(&self) -> Self {
        Self {
            on_pointerdown: self.on_pointerdown.clone(),
            element_capture: self.element_capture.clone(),
        }
    }
}

pub type UseSliderTrackAttrs = (OnEvent<ev::pointerdown>, ElementCaptureAttr);

impl IntoAttrs for UseSliderTrackProps {
    type Attrs = UseSliderTrackAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_pointerdown.into_on(ev::pointerdown),
            self.element_capture,
        )
    }
}

/// The output's props: `for` all thumbs, not announced while dragging (`aria-live="off"`).
#[derive(Debug)]
pub struct UseSliderOutputProps {
    pub html_for: Signal<Option<Id>>,
}

pub type UseSliderOutputAttrs = (SignalAttr<Option<Id>>, FixedAttr);

impl IntoAttrs for UseSliderOutputProps {
    type Attrs = UseSliderOutputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            signal_attr("for", self.html_for),
            fixed_attr("aria-live", AriaLive::Off),
        )
    }
}

/// The thumb closest to `value`: of stacked thumbs, the one after (react-aria's `onDownTrack`).
fn closest_thumb(value: f64, values: &[f64]) -> Option<usize> {
    let split = values.iter().position(|v| value - v < 0.0);
    match split {
        _ if values.is_empty() => None,
        Some(0) => Some(0),
        None => Some(values.len() - 1),
        Some(split) => {
            let (last_left, first_right) = (values[split - 1], values[split]);
            if (last_left - value).abs() < (first_right - value).abs() {
                Some(split - 1)
            } else {
                Some(split)
            }
        }
    }
}

/// A slider: a group of thumbs on a track, named by a label. Pressing the track moves the
/// closest thumb there (and drags it). Render each thumb with
/// [`use_slider_thumb`](fn@super::use_slider_thumb).
pub fn use_slider<T: NumberValue>(input: UseSliderInput<T>) -> UseSliderReturn {
    crate::metrics::subject!();
    let UseSliderInput {
        state,
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
        aria_details,
    } = input;

    let UseFieldReturn {
        label: label_props,
        field: field_props,
        description: description_props,
        error_message: error_message_props,
        ..
    } = use_field(UseFieldInput {
        id,
        has_label,
        label_element_type: LabelElementType::Span,
        aria_label,
        aria_labelledby,
        aria_describedby,
        ..UseFieldInput::default()
    });
    let label_props = label_props.into_props();
    let field_props = field_props.into_props();
    let data = SliderData {
        id: field_props.id.clone(),
        labelled_by: {
            let field_id = field_props.id.clone();
            let labelled_by = field_props.aria_labelledby;
            // A thumb must reference the actual labels: accessible-name computation does not
            // follow an aria-labelledby chain through the slider group. A memo: every thumb
            // reads it.
            Memo::new(move |_| labelled_by.get().unwrap_or_else(|| field_id.clone())).into()
        },
        aria_describedby: field_props.aria_describedby,
        aria_details,
    };

    let direction = use_direction();
    let track = CapturedElement::new();
    let orientation = state.orientation;
    // The thumb the track press moves, and its position in pixels along the track: dragging adds
    // the deltas here, so movements smaller than a step aren't lost when the value snaps.
    let dragged_thumb = StoredValue::new(None::<usize>);
    let position = StoredValue::new(None::<f64>);
    let release_listeners: StoredValue<Option<SendWrapper<Vec<Listener>>>> = StoredValue::new(None);

    let is_vertical = move || orientation.get_untracked() == Orientation::Vertical;
    let reversed = move || is_vertical() || direction.get_untracked() == WritingDirection::Rtl;
    let end_drag = move || {
        if let Some(index) = dragged_thumb.get_value() {
            state.set_thumb_dragging(index, false);
            dragged_thumb.set_value(None);
        }
        release_listeners.set_value(None);
    };

    let on_track_down = move |e: PointerEvent| {
        if PointerType::of(&e) == PointerType::Mouse
            && (e.button() != 0 || e.alt_key() || e.ctrl_key() || e.meta_key())
        {
            return;
        }
        let Some(track_el) = track.get_untracked() else {
            return;
        };
        let thumbs = state.values.get_untracked();
        if state.is_disabled.get_untracked()
            || (0..thumbs.len()).any(|index| untrack(|| state.is_thumb_dragging(index)))
        {
            return;
        }
        let rect = track_el.get_bounding_client_rect();
        let (size, offset) = if is_vertical() {
            (rect.height(), e.client_y() - rect.top())
        } else {
            (rect.width(), e.client_x() - rect.left())
        };
        let mut percent = offset / size;
        if reversed() {
            percent = 1.0 - percent;
        }
        let Some(value) = untrack(|| state.percent_value(Fraction::new(percent))) else {
            return;
        };
        let values: Vec<f64> = thumbs.iter().map(|v| v.to_f64()).collect();
        let Some(closest) = closest_thumb(value.to_f64(), &values) else {
            return;
        };
        if !state.is_thumb_editable(closest) {
            dragged_thumb.set_value(None);
            return;
        }
        // Don't move focus away.
        e.prevent_default();
        dragged_thumb.set_value(Some(closest));
        state.set_focused_thumb(Some(closest));
        state.set_thumb_dragging(closest, true);
        state.set_thumb_value(closest, value);

        // A press without movement ends on the pointer's release.
        let pointer_id = e.pointer_id();
        if let Some(document) = e.expect_current_target().get_owner_document() {
            let on_up = listen_to(&document, ev::pointerup, false, move |e: PointerEvent| {
                if e.pointer_id() == pointer_id {
                    end_drag();
                }
            });
            release_listeners.set_value(Some(SendWrapper::new(vec![on_up])));
        }
    };

    let track_move = use_move(UseMoveInput {
        is_disabled: state.is_disabled,
        on_move_start: Some(Callback::new(move |_: MoveStartEvent| {
            position.set_value(None);
        })),
        on_move: Some(Callback::new(move |e: MoveEvent| {
            let (Some(index), Some(track_el)) = (dragged_thumb.get_value(), track.get_untracked())
            else {
                return;
            };
            let rect = track_el.get_bounding_client_rect();
            let size = if is_vertical() {
                rect.height()
            } else {
                rect.width()
            };
            let current = position
                .get_value()
                .unwrap_or_else(|| untrack(|| state.thumb_percent(index)).get() * size);
            let mut delta = if is_vertical() { e.delta_y } else { e.delta_x };
            if reversed() {
                delta = -delta;
            }
            let current = current + delta;
            position.set_value(Some(current));
            state.set_thumb_percent(index, Fraction::new(current / size));
        })),
        on_move_end: Some(Callback::new(move |_: MoveEndEvent| end_drag())),
    });
    let track_move_props = track_move.target.into_props();
    on_cleanup(move || {
        release_listeners.try_update_value(Option::take);
    });

    // A click on the label focuses the first thumb (Safari doesn't focus range inputs through
    // labels) and shows its focus ring.
    let first_thumb = data.thumb_id(0);
    let on_label_click = EventHandler::new(move |_: MouseEvent| {
        if let Some(thumb) = leptos_use::use_document()
            .as_ref()
            .and_then(|document| document.get_element_by_id(&first_thumb))
            .and_then(|el| wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(el).ok())
        {
            let _ = thumb.focus();
            set_modality(Modality::Keyboard);
        }
    });

    let thumb_data = data.clone();
    UseSliderReturn {
        label: Part::stored(UseSliderLabelProps {
            id: label_props.id,
            on_click: on_label_click,
        }),
        description: description_props,
        error_message: error_message_props,
        group: Part::stored(UseSliderGroupProps {
            id: field_props.id,
            aria_label: field_props.aria_label,
            aria_labelledby: field_props.aria_labelledby,
        }),
        track: {
            let on_pointerdown =
                EventHandler::new(on_track_down).chain(track_move_props.on_pointerdown);
            let element_capture = track.attr();
            Part::new(move || {
                PropsWithStyles::new(
                    UseSliderTrackProps {
                        on_pointerdown: on_pointerdown.clone(),
                        element_capture: element_capture.clone(),
                    },
                    Styles::new()
                        .add_unchecked("position", "relative")
                        .add(TouchActionProperty.declare(TouchAction::None)),
                )
            })
        },
        output: {
            let html_for = Signal::derive(move || {
                let mut ids = crate::utils::id::IdListBuilder::default();
                for index in 0..state.thumb_count() {
                    ids.push(&thumb_data.thumb_id(index));
                }
                ids.finish()
            });
            Part::new(move || UseSliderOutputProps { html_for })
        },
        data,
        track_element: track,
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::closest_thumb;

    #[test]
    fn closest_thumb_prefers_the_later_of_stacked_thumbs() {
        assert_that!(closest_thumb(5.0, &[10.0, 20.0])).is_equal_to(Some(0));
        assert_that!(closest_thumb(30.0, &[10.0, 20.0])).is_equal_to(Some(1));
        assert_that!(closest_thumb(14.0, &[10.0, 20.0])).is_equal_to(Some(0));
        assert_that!(closest_thumb(15.0, &[10.0, 20.0])).is_equal_to(Some(1));
        assert_that!(closest_thumb(10.0, &[10.0, 10.0])).is_equal_to(Some(1));
        assert_that!(closest_thumb(1.0, &[])).is_none();
    }
}
