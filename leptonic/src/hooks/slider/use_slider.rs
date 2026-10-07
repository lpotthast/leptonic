// Upstream: react-aria/src/slider/useSlider.ts @ 99e6102368
// Upstream: react-aria/src/slider/utils.ts @ 99e6102368
use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use send_wrapper::SendWrapper;
use web_sys::{MouseEvent, PointerEvent};

use crate::{
    hooks::{
        IntoAttrs, LabelElementType, Modality, MoveEndEvent, MoveEvent, MoveStartEvent,
        PropsWithStyles, UseFieldInput, UseFieldReturn, UseMoveInput,
        interactions::use_move::MoveAxis, set_modality, slider::SliderState, use_field, use_move,
    },
    utils::{
        EventAccessors, EventHandler, EventTargetExt, SlotProps,
        aria::{AriaLive, AriaRole},
        css::TouchAction,
        element_capture::{CapturedElement, ElementCaptureAttr},
        event_listeners::{Listener, listen_to},
        i18n::use_direction,
        locale::WritingDirection,
        number_value::NumberValue,
        orientation::Orientation,
        style::TouchActionProperty,
        styles::Styles,
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
//   they stay stable while the label comes and goes (ids must match between server and client);
//   the thumbs' `aria-labelledby` follows the label.
// - Track presses start on `pointerdown` only: PointerEvent is always available (CLAUDE.md), so
//   react-aria's mouse and touch fallbacks are omitted.
//
// =============================================================================

/// Input of [`use_slider`].
#[derive(Debug)]
pub struct UseSliderInput<T: NumberValue> {
    pub state: SliderState<T>,
    /// The group's id. Generated when `None`.
    pub id: Option<String>,
    /// Whether a visible label is rendered (with `label_props`).
    pub has_label: Signal<bool>,
    /// Names the slider when there is no visible label.
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Option<String>,
    /// Further elements describing every thumb (next to a rendered description).
    pub aria_describedby: Option<String>,
}

/// What the thumbs need from their slider.
#[derive(Debug, Clone)]
pub struct SliderData {
    /// The slider group's id. Thumb ids derive from it (stable, unlike react-aria's, which derive
    /// from the label's id when there is one).
    pub id: String,
    /// The id the thumbs are labelled by: the label's while it is rendered, else the group's.
    pub labelled_by: Signal<String>,
    /// What describes every thumb: the description while rendered, and `aria_describedby`.
    pub aria_describedby: Signal<Option<String>>,
}

impl SliderData {
    /// The id of thumb `index`'s input.
    pub fn thumb_id(&self, index: usize) -> String {
        format!("{}-{index}", self.id)
    }
}

/// Return value of [`use_slider`].
#[derive(Debug)]
pub struct UseSliderReturn {
    /// For the label element.
    pub label_props: UseSliderLabelProps,
    /// For a description of the slider (describes every thumb while rendered).
    pub description_props: SlotProps,
    /// For an error message. Render it only while the slider is invalid.
    pub error_message_props: SlotProps,
    /// For the element around label, track and output (`role="group"`).
    pub group_props: UseSliderGroupProps,
    /// For the track: pressing it moves the closest thumb there.
    pub track_props: PropsWithStyles<UseSliderTrackProps>,
    /// For an `<output>` showing the values.
    pub output_props: UseSliderOutputProps,
    /// For [`use_slider_thumb`](super::use_slider_thumb).
    pub data: SliderData,
    /// The track element, for the thumbs.
    pub track_element: CapturedElement,
}

/// The label's props: a click focuses the first thumb (the label has no `for`: VoiceOver on iOS
/// would announce only the label for the first thumb).
#[derive(Debug)]
pub struct UseSliderLabelProps {
    pub id: String,
    pub on_click: EventHandler<MouseEvent>,
}

pub type UseSliderLabelAttrs = (
    Attr<attr::Id, String>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
);

impl IntoAttrs for UseSliderLabelProps {
    type Attrs = UseSliderLabelAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (Attr(attr::Id, self.id), self.on_click.into_on(ev::click))
    }
}

#[derive(Debug)]
pub struct UseSliderGroupProps {
    pub id: String,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
}

pub type UseSliderGroupAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::Id, String>,
    Attr<attr::AriaLabel, MaybeProp<String>>,
    Attr<attr::AriaLabelledby, Signal<Option<String>>>,
);

impl IntoAttrs for UseSliderGroupProps {
    type Attrs = UseSliderGroupAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, AriaRole::Group),
            Attr(attr::Id, self.id),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
        )
    }
}

#[derive(Debug)]
pub struct UseSliderTrackProps {
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub element_capture: ElementCaptureAttr,
}

pub type UseSliderTrackAttrs = (
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    ElementCaptureAttr,
);

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
    pub html_for: Signal<String>,
}

pub type UseSliderOutputAttrs = (
    Attr<attr::For, Signal<String>>,
    Attr<attr::AriaLive, AriaLive>,
);

impl IntoAttrs for UseSliderOutputProps {
    type Attrs = UseSliderOutputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::For, self.html_for),
            Attr(attr::AriaLive, AriaLive::Off),
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
/// [`use_slider_thumb`](super::use_slider_thumb).
#[allow(clippy::too_many_lines)]
pub fn use_slider<T: NumberValue>(input: UseSliderInput<T>) -> UseSliderReturn {
    let UseSliderInput {
        state,
        id,
        has_label,
        aria_label,
        aria_labelledby,
        aria_describedby,
    } = input;

    let UseFieldReturn {
        label_props,
        field_props,
        description_props,
        error_message_props,
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
    let data = SliderData {
        id: field_props.id.clone(),
        labelled_by: {
            let (label_id, field_id) = (label_props.id.clone(), field_props.id.clone());
            Signal::derive(move || {
                if has_label.get() {
                    label_id.clone()
                } else {
                    field_id.clone()
                }
            })
        },
        aria_describedby: field_props.aria_describedby,
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
        if e.pointer_type() == "mouse"
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
        let Some(value) = untrack(|| state.percent_value(percent)) else {
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
        axis: Signal::derive(move || match orientation.get() {
            Orientation::Horizontal => MoveAxis::Horizontal,
            Orientation::Vertical => MoveAxis::Vertical,
        }),
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
                .unwrap_or_else(|| untrack(|| state.thumb_percent(index)) * size);
            let mut delta = if is_vertical() { e.delta_y } else { e.delta_x };
            if reversed() {
                delta = -delta;
            }
            let current = current + delta;
            position.set_value(Some(current));
            state.set_thumb_percent(index, (current / size).clamp(0.0, 1.0));
        })),
        on_move_end: Some(Callback::new(move |_: MoveEndEvent| end_drag())),
    });
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
        label_props: UseSliderLabelProps {
            id: label_props.id,
            on_click: on_label_click,
        },
        description_props,
        error_message_props,
        group_props: UseSliderGroupProps {
            id: field_props.id,
            aria_label: field_props.aria_label,
            aria_labelledby: field_props.aria_labelledby,
        },
        track_props: PropsWithStyles::new(
            UseSliderTrackProps {
                on_pointerdown: EventHandler::new(on_track_down)
                    .chain(track_move.props.on_pointerdown),
                element_capture: track.attr(),
            },
            Styles::new()
                .add_unchecked("position", "relative")
                .add(TouchActionProperty.declare(TouchAction::None)),
        ),
        output_props: UseSliderOutputProps {
            html_for: Signal::derive(move || {
                (0..state.thumb_count())
                    .map(|index| thumb_data.thumb_id(index))
                    .collect::<Vec<_>>()
                    .join(" ")
            }),
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
