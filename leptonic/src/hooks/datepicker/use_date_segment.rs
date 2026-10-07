// Upstream: react-aria/src/datepicker/useDateSegment.ts @ 99e6102368
use leptos::{
    attr::{
        self, Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev::{self, On, SharedEventCallback},
    prelude::*,
};
use web_sys::{FocusEvent, InputEvent, KeyboardEvent, MouseEvent, PointerEvent};

use super::{
    format::{DateFormatter, FormatOptions},
    types::{DateSegment, DateSegmentType, DateValue, Granularity, HourCycle, MaxGranularity},
    use_date_field::{DateFieldData, display_name},
};
use crate::{
    hooks::{
        IntoAttrs, PropsWithStyles, UseKeyboardInput, UseSpinButtonInput, UseSpinButtonReturn,
        form::use_label::labels, use_keyboard, use_spin_button,
    },
    utils::{
        CapturedElement, ElementCaptureAttr, EventHandler,
        aria::{AriaDisabled, AriaInvalid, AriaReadonly, AriaRequired, AriaRole},
        date_time_formatter::{DateTimeFormatOptions, DateTimeFormatter, MonthFormat},
        filter::{CollatorOptions, Filter},
        i18n::{use_direction, use_locale},
        id::use_id,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        locale::WritingDirection,
        number_formatter::NumberFormatOptions,
        number_parser::NumberParser,
        platform::device::is_ios,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - One input (C8): the field's `DateFieldData` (react-aria: a `WeakMap` keyed by the state), the
//   segment as a signal (its kind stays: a field keeps a segment per kind) and its element.
// - For editable segments and the time zone only: literals are rendered hidden from assistive
//   technology by the caller (react-aria returns `aria-hidden` for them).
//
// ## OMITTED FEATURES
// - Localized segment names (see `use_date_field`).
//
// =============================================================================

/// Input of [`use_date_segment`].
pub struct UseDateSegmentInput<V: DateValue> {
    /// The segment. Its kind stays: a field keeps a segment per kind.
    pub segment: Signal<DateSegment>,
    /// What the field gives its segments.
    pub data: DateFieldData<V>,
    /// The segment's element.
    pub element: CapturedElement,
}

/// Return value of [`use_date_segment`].
pub struct UseDateSegmentReturn {
    pub segment_props: PropsWithStyles<UseDateSegmentProps>,
}

/// Props of a date segment (a `div` or `span`, editable as text).
#[derive(Debug)]
pub struct UseDateSegmentProps {
    pub id: String,
    pub role: AriaRole,
    pub aria_valuenow: Signal<Option<String>>,
    pub aria_valuetext: Signal<Option<String>>,
    pub aria_valuemin: Signal<Option<String>>,
    pub aria_valuemax: Signal<Option<String>>,
    pub aria_label: Signal<Option<String>>,
    pub aria_labelledby: Signal<Option<String>>,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_invalid: Signal<Option<AriaInvalid>>,
    pub aria_readonly: Signal<Option<AriaReadonly>>,
    pub aria_required: Signal<Option<AriaRequired>>,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub data_placeholder: Signal<Option<&'static str>>,
    pub contenteditable: Signal<Option<&'static str>>,
    pub spellcheck: Signal<Option<&'static str>>,
    pub autocorrect: Signal<Option<&'static str>>,
    pub enterkeyhint: Signal<Option<&'static str>>,
    pub inputmode: Signal<Option<&'static str>>,
    pub tabindex: Signal<Option<i32>>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_beforeinput: EventHandler<InputEvent>,
    pub on_input: EventHandler<web_sys::Event>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub element_capture: ElementCaptureAttr,
}

pub type UseDateSegmentAttrs = (
    (
        Attr<attr::Id, String>,
        Attr<attr::Role, AriaRole>,
        Attr<attr::AriaValuenow, Signal<Option<String>>>,
        Attr<attr::AriaValuetext, Signal<Option<String>>>,
        Attr<attr::AriaValuemin, Signal<Option<String>>>,
        Attr<attr::AriaValuemax, Signal<Option<String>>>,
        Attr<attr::AriaLabel, Signal<Option<String>>>,
        Attr<attr::AriaLabelledby, Signal<Option<String>>>,
        Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    ),
    (
        Attr<attr::AriaInvalid, Signal<Option<AriaInvalid>>>,
        Attr<attr::AriaReadonly, Signal<Option<AriaReadonly>>>,
        Attr<attr::AriaRequired, Signal<Option<AriaRequired>>>,
        Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
        CustomAttr<&'static str, Signal<Option<&'static str>>>,
        Attr<attr::Contenteditable, Signal<Option<&'static str>>>,
        Attr<attr::Spellcheck, Signal<Option<&'static str>>>,
        CustomAttr<&'static str, Signal<Option<&'static str>>>,
        Attr<attr::Enterkeyhint, Signal<Option<&'static str>>>,
        Attr<attr::Inputmode, Signal<Option<&'static str>>>,
        Attr<attr::Tabindex, Signal<Option<i32>>>,
    ),
    (
        On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
        On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
        On<ev::focus, SharedEventCallback<FocusEvent>>,
        On<ev::blur, SharedEventCallback<FocusEvent>>,
        On<ev::beforeinput, SharedEventCallback<InputEvent>>,
        On<ev::input, SharedEventCallback<web_sys::Event>>,
        On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
        On<ev::mousedown, SharedEventCallback<MouseEvent>>,
        ElementCaptureAttr,
    ),
);

impl IntoAttrs for UseDateSegmentProps {
    type Attrs = UseDateSegmentAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            (
                Attr(attr::Id, self.id),
                Attr(attr::Role, self.role),
                Attr(attr::AriaValuenow, self.aria_valuenow),
                Attr(attr::AriaValuetext, self.aria_valuetext),
                Attr(attr::AriaValuemin, self.aria_valuemin),
                Attr(attr::AriaValuemax, self.aria_valuemax),
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaLabelledby, self.aria_labelledby),
                Attr(attr::AriaDescribedby, self.aria_describedby),
            ),
            (
                Attr(attr::AriaInvalid, self.aria_invalid),
                Attr(attr::AriaReadonly, self.aria_readonly),
                Attr(attr::AriaRequired, self.aria_required),
                Attr(attr::AriaDisabled, self.aria_disabled),
                custom_attribute("data-placeholder", self.data_placeholder),
                Attr(attr::Contenteditable, self.contenteditable),
                Attr(attr::Spellcheck, self.spellcheck),
                custom_attribute("autocorrect", self.autocorrect),
                Attr(attr::Enterkeyhint, self.enterkeyhint),
                Attr(attr::Inputmode, self.inputmode),
                Attr(attr::Tabindex, self.tabindex),
            ),
            (
                self.on_keydown.into_on(ev::keydown),
                self.on_keyup.into_on(ev::keyup),
                self.on_focus.into_on(ev::focus),
                self.on_blur.into_on(ev::blur),
                self.on_beforeinput.into_on(ev::beforeinput),
                self.on_input.into_on(ev::input),
                self.on_pointerdown.into_on(ev::pointerdown),
                self.on_mousedown.into_on(ev::mousedown),
                self.element_capture,
            ),
        )
    }
}

/// The day period names (AM, PM) of a locale: as typed by the first letter.
fn day_periods(locale: &crate::utils::i18n::Locale) -> [String; 2] {
    let formatter = DateFormatter::new(
        locale,
        &FormatOptions {
            granularity: Granularity::Hour,
            max_granularity: MaxGranularity::Hour,
            time_zone: None,
            hide_time_zone: true,
            hour_cycle: Some(HourCycle::H12),
            show_era: false,
            should_force_leading_zeros: false,
        },
    );
    [0, 12].map(|hour| {
        formatter
            .format_to_parts(&jiff::civil::date(2001, 1, 1).at(hour, 0, 0, 0))
            .into_iter()
            .find(|(kind, _)| *kind == Some(DateSegmentType::DayPeriod))
            .map_or_else(
                || {
                    if hour == 0 {
                        "AM".to_owned()
                    } else {
                        "PM".to_owned()
                    }
                },
                |(_, text)| text,
            )
    })
}

/// The era names (BC, AD), without a common prefix, as typed by the first letter.
fn eras(locale: &crate::utils::i18n::Locale) -> [String; 2] {
    let formatter = DateFormatter::new(
        locale,
        &FormatOptions {
            granularity: Granularity::Day,
            max_granularity: MaxGranularity::Year,
            time_zone: None,
            hide_time_zone: true,
            hour_cycle: None,
            show_era: true,
            should_force_leading_zeros: false,
        },
    );
    let mut names = [jiff::civil::date(0, 1, 1), jiff::civil::date(1, 1, 1)].map(|date| {
        formatter
            .format_to_parts(&date)
            .into_iter()
            .find(|(kind, _)| *kind == Some(DateSegmentType::Era))
            .map(|(_, text)| text)
            .unwrap_or_default()
    });
    let prefix = names[0]
        .chars()
        .zip(names[1].chars())
        .take_while(|(a, b)| a == b)
        .count();
    if prefix > 0 {
        for name in &mut names {
            *name = name.chars().skip(prefix).collect();
        }
    }
    names
}

/// Behavior and accessibility of a segment of a date field (react-aria's `useDateSegment`): a
/// spin button (arrows, Page Up/Down, Home/End) editable as text. Typing digits fills it (moving
/// on once no further digit fits), letters choose the day period and era, Backspace deletes a
/// digit; the selection stays collapsed (Android Chrome's composition would break the DOM).
#[allow(clippy::too_many_lines)]
pub fn use_date_segment<V: DateValue>(input: UseDateSegmentInput<V>) -> UseDateSegmentReturn {
    let UseDateSegmentInput {
        segment,
        data,
        element,
    } = input;
    let state = data.state;
    let kind = segment.with_untracked(|segment| segment.kind);
    let locale = use_locale();
    let direction = use_direction();
    let entered_keys = StoredValue::new(String::new());

    // "6 – June" for numeric months, "1 PM" for hours.
    let text_value = Signal::derive(move || {
        let segment = segment.get();
        if segment.is_placeholder {
            return String::new();
        }
        match kind {
            DateSegmentType::Month => {
                let month = DateTimeFormatter::new(
                    &locale.get(),
                    DateTimeFormatOptions {
                        month: Some(MonthFormat::Long),
                        ..DateTimeFormatOptions::default()
                    },
                )
                .format_date(state.date_value.get().date());
                if month == segment.text {
                    month
                } else {
                    format!("{} – {month}", segment.text)
                }
            }
            DateSegmentType::Hour => {
                let options = state.format_options();
                DateFormatter::new(
                    &locale.get(),
                    &FormatOptions {
                        granularity: Granularity::Hour,
                        max_granularity: MaxGranularity::Hour,
                        time_zone: None,
                        hide_time_zone: true,
                        ..options
                    },
                )
                .format(&state.date_value.get())
            }
            _ => segment.text,
        }
    });

    let reset_keys = move || entered_keys.set_value(String::new());
    let UseSpinButtonReturn { props: spin, .. } = use_spin_button(UseSpinButtonInput {
        value: Signal::derive(move || segment.get().value.map(f64::from)),
        text_value: Signal::derive(move || Some(text_value.get())),
        min_value: Signal::derive(move || segment.get().min_value.map(f64::from)),
        max_value: Signal::derive(move || segment.get().max_value.map(f64::from)),
        is_disabled: state.is_disabled,
        is_read_only: Signal::derive(move || {
            state.is_read_only.get() || !segment.get().is_editable
        }),
        is_required: state.is_required,
        on_increment: Some(Callback::new(move |()| {
            reset_keys();
            state.increment(kind);
        })),
        on_decrement: Some(Callback::new(move |()| {
            reset_keys();
            state.decrement(kind);
        })),
        on_increment_page: Some(Callback::new(move |()| {
            reset_keys();
            state.increment_page(kind);
        })),
        on_decrement_page: Some(Callback::new(move |()| {
            reset_keys();
            state.decrement_page(kind);
        })),
        on_increment_to_max: Some(Callback::new(move |()| {
            reset_keys();
            state.increment_to_max(kind);
        })),
        on_decrement_to_min: Some(Callback::new(move |()| {
            reset_keys();
            state.decrement_to_min(kind);
        })),
    });

    let parser = Memo::new_with_compare(
        move |_| {
            NumberParser::new(
                &locale.get(),
                &NumberFormatOptions {
                    maximum_fraction_digits: Some(0),
                    ..NumberFormatOptions::default()
                },
            )
        },
        |_, _| true,
    );
    let is_partial_number = move |text: &str| {
        parser.with_untracked(|parser| parser.is_valid_partial_number::<i64>(text, None, None))
    };
    let parse = move |text: &str| parser.with_untracked(|parser| parser.parse::<i64>(text));

    let backspace = move || {
        let segment = segment.get_untracked();
        if segment.text == segment.placeholder {
            data.focus_previous();
        }
        if is_partial_number(&segment.text)
            && !state.is_read_only.get_untracked()
            && !segment.is_placeholder
        {
            let mut text = segment.text.clone();
            text.pop();
            let parsed = parse(&text);
            match parsed
                .and_then(|parsed| i32::try_from(parsed).ok())
                .filter(|parsed| *parsed != 0)
            {
                Some(parsed) if !text.is_empty() => {
                    state.set_segment(kind, parsed);
                    entered_keys.set_value(text);
                }
                _ => {
                    state.clear_segment(kind);
                    entered_keys.set_value(String::new());
                }
            }
        } else if matches!(kind, DateSegmentType::DayPeriod | DateSegmentType::Era) {
            state.clear_segment(kind);
        }
    };
    let keyboard = use_keyboard(UseKeyboardInput {
        shortcuts: Some(
            KeyboardShortcuts::new()
                .on(Shortcut::key("Backspace"), move |_: &KeyboardEvent| {
                    backspace();
                })
                .on(Shortcut::key("Delete"), move |_: &KeyboardEvent| {
                    backspace();
                })
                // Firefox fires no `selectstart` for Ctrl/Cmd+A.
                .on(Shortcut::key("a").primary(), |_: &KeyboardEvent| {}),
        ),
        allow_repeats: true,
        ..UseKeyboardInput::default()
    });

    let filter = Memo::new_with_compare(
        move |_| Filter::new(&locale.get(), &CollatorOptions::default()),
        |_, _| true,
    );
    let starts_with =
        move |name: &str, key: &str| filter.with_untracked(|filter| filter.starts_with(name, key));
    let day_periods = Memo::new(move |_| day_periods(&locale.get()));
    let eras = Memo::new(move |_| {
        if kind == DateSegmentType::Era {
            eras(&locale.get())
        } else {
            Default::default()
        }
    });

    let on_input = move |key: &str| {
        if state.is_disabled.get_untracked() || state.is_read_only.get_untracked() {
            return;
        }
        let text = format!("{}{key}", entered_keys.get_value());
        match kind {
            DateSegmentType::DayPeriod => {
                let [am, pm] = day_periods.get_untracked();
                if starts_with(&am, key) {
                    state.set_segment(DateSegmentType::DayPeriod, 0);
                } else if starts_with(&pm, key) {
                    state.set_segment(DateSegmentType::DayPeriod, 1);
                } else {
                    return;
                }
                data.focus_next();
            }
            DateSegmentType::Era => {
                if let Some(index) = eras
                    .get_untracked()
                    .iter()
                    .position(|era| starts_with(era, key))
                {
                    state.set_segment(DateSegmentType::Era, i32::try_from(index).unwrap_or(1));
                    data.focus_next();
                }
            }
            DateSegmentType::Day
            | DateSegmentType::Hour
            | DateSegmentType::Minute
            | DateSegmentType::Second
            | DateSegmentType::Month
            | DateSegmentType::Year => {
                if !is_partial_number(&text) {
                    return;
                }
                let Some(number) = parse(&text) else {
                    return;
                };
                let max = segment
                    .with_untracked(|segment| segment.max_value)
                    .map(i64::from);
                // A number beyond the maximum starts over with the typed digit.
                let value = if max.is_some_and(|max| number > max) {
                    parse(key).unwrap_or(number)
                } else {
                    number
                };
                state.set_segment(kind, i32::try_from(value).unwrap_or(0));
                // Moves on once no further digit fits.
                let is_full = max.is_some_and(|max| {
                    number * 10 > max || text.chars().count() >= max.to_string().len()
                });
                if is_full {
                    entered_keys.set_value(String::new());
                    data.focus_next();
                } else {
                    entered_keys.set_value(text);
                }
            }
            DateSegmentType::Literal | DateSegmentType::TimeZoneName => {}
        }
    };

    let collapse_selection = move || {
        #[cfg(not(feature = "ssr"))]
        if let Some(element) = element.get_untracked()
            && let Some(window) = leptos_use::use_window().as_ref()
            && let Ok(Some(selection)) = window.get_selection()
        {
            let _ = selection.collapse(Some(&**element));
        }
    };

    // While a segment has the focus, a selection inside it stays collapsed.
    #[cfg(not(feature = "ssr"))]
    {
        let handle = leptos_use::use_event_listener(
            leptos_use::use_document(),
            ev::selectionchange,
            move |_| {
                let Some(segment_element) = element.get_untracked() else {
                    return;
                };
                let Some(window) = leptos_use::use_window().as_ref().cloned() else {
                    return;
                };
                let Ok(Some(selection)) = window.get_selection() else {
                    return;
                };
                let is_inside = selection.anchor_node().is_some_and(|anchor| {
                    let segment: &web_sys::Element = &segment_element;
                    crate::utils::shadow_dom::node_contains(segment.as_ref(), &anchor)
                });
                let is_active = segment_element
                    .owner_document()
                    .as_ref()
                    .and_then(crate::utils::shadow_dom::get_active_element)
                    .is_some_and(|active| active == *segment_element);
                if is_inside && is_active {
                    let _ = selection.collapse(Some(&**segment_element));
                }
            },
        );
        on_cleanup(handle);
    }

    // A removed focused segment hands the focus to the previous one (else the next). react-aria
    // checks the active element before the DOM removal (a layout effect's cleanup); Leptos
    // removes the element first, so whether it had the focus is tracked: set on focus, cleared
    // on blur once the element turns out to still be there (a removal blurs it, too). The state
    // is kept outside the reactive graph, which may be disposed by then.
    #[cfg(not(feature = "ssr"))]
    let had_focus = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    #[cfg(not(feature = "ssr"))]
    let track_focus = {
        use std::sync::atomic::Ordering;
        let on_focus = std::sync::Arc::clone(&had_focus);
        let on_blur = std::sync::Arc::clone(&had_focus);
        (
            EventHandler::new(move |_: FocusEvent| on_focus.store(true, Ordering::Relaxed)),
            EventHandler::new(move |e: FocusEvent| {
                let had_focus = std::sync::Arc::clone(&on_blur);
                let target = send_wrapper::SendWrapper::new(e.target());
                queue_microtask(move || {
                    let connected = target
                        .as_ref()
                        .and_then(|target| wasm_bindgen::JsCast::dyn_ref::<web_sys::Node>(target))
                        .is_some_and(web_sys::Node::is_connected);
                    if connected {
                        had_focus.store(false, Ordering::Relaxed);
                    }
                });
            }),
        )
    };
    #[cfg(feature = "ssr")]
    let track_focus = (EventHandler::default(), EventHandler::default());
    #[cfg(not(feature = "ssr"))]
    {
        let had_focus = std::sync::Arc::clone(&had_focus);
        on_cleanup(move || {
            if !had_focus.load(std::sync::atomic::Ordering::Relaxed) {
                return;
            }
            // Unless the focus went elsewhere meanwhile.
            let active = leptos_use::use_document()
                .as_ref()
                .and_then(crate::utils::shadow_dom::get_active_element);
            let is_lost =
                active.is_none_or(|active| active.tag_name().eq_ignore_ascii_case("body"));
            if is_lost && !data.focus_previous() {
                data.focus_next();
            }
        });
    }

    let composition = StoredValue::new(None::<String>);
    let on_beforeinput = EventHandler::new(move |e: InputEvent| {
        let Some(segment_element) = element.get_untracked() else {
            return;
        };
        e.prevent_default();
        match e.input_type().as_str() {
            "deleteContentBackward" | "deleteContentForward" => {
                if is_partial_number(&segment.get_untracked().text)
                    && !state.is_read_only.get_untracked()
                {
                    backspace();
                }
            }
            "insertCompositionText" => {
                // It can't be canceled: restored on `input`.
                let text = segment_element.text_content();
                composition.set_value(text.clone());
                // Safari stays composing otherwise.
                segment_element.set_text_content(text.as_deref());
            }
            _ => {
                if let Some(data) = e.data() {
                    on_input(&data);
                }
            }
        }
    });
    let on_input_event = EventHandler::new(move |e: web_sys::Event| {
        let Some(e) = wasm_bindgen::JsCast::dyn_ref::<InputEvent>(&e) else {
            return;
        };
        if e.input_type() == "insertCompositionText" {
            if let Some(segment_element) = element.get_untracked() {
                segment_element.set_text_content(composition.get_value().as_deref());
            }
            // Android types letters as compositions; also Pinyin on iOS.
            if let Some(data) = e.data() {
                let [am, pm] = day_periods.get_untracked();
                if starts_with(&am, &data) || starts_with(&pm, &data) {
                    on_input(&data);
                }
            }
        }
    });

    let on_focus = EventHandler::new(move |_: FocusEvent| {
        reset_keys();
        #[cfg(not(feature = "ssr"))]
        if let Some(segment_element) = element.get_untracked() {
            use crate::utils::scroll::{
                ScrollIntoViewportOpts, get_scroll_parent, scroll_into_viewport,
            };
            scroll_into_viewport(
                Some(&segment_element),
                &ScrollIntoViewportOpts {
                    containing_element: Some(get_scroll_parent(&segment_element, false)),
                },
            );
        }
        // Collapsed, or Chrome fires no input events.
        collapse_selection();
    });

    // Spin buttons can't be focused with VoiceOver on iOS.
    let as_textbox = is_ios() || kind == DateSegmentType::TimeZoneName;
    let unless_textbox = move |signal: Signal<Option<String>>| {
        Signal::derive(move || if as_textbox { None } else { signal.get() })
    };
    let aria_valuetext = spin.aria_valuetext;

    // Only the first segment is described (unless invalid): read once, not on every segment.
    let is_first = Signal::derive(move || {
        state.segments.with(|segments| {
            segments
                .iter()
                .find(|segment| segment.is_editable)
                .map(|segment| segment.kind)
        }) == Some(kind)
    });
    let describedby = data.aria_describedby;
    let aria_describedby = Signal::derive(move || {
        if is_first.get() || state.is_invalid.get() {
            describedby.get()
        } else {
            None
        }
    });

    let id = use_id("date-segment");
    // The field's label after the segment's name (VoiceOver on iOS doesn't announce groups).
    let field_label = data.aria_label;
    let field_labelledby = data.aria_labelledby;
    let label_id = id.clone();
    let label = Signal::derive(move || {
        let labelledby = field_labelledby.get();
        let name = display_name(kind);
        let label = format!(
            "{name}{}{}",
            field_label
                .get()
                .map(|label| format!(", {label}"))
                .unwrap_or_default(),
            if labelledby.is_some() { ", " } else { "" },
        );
        labels(&label_id, Some(label), labelledby)
    });
    let is_editable = Signal::derive(move || {
        !state.is_disabled.get() && !state.is_read_only.get() && segment.get().is_editable
    });
    let editable_flag =
        move |value: &'static str| Signal::derive(move || is_editable.get().then_some(value));
    fn flag<T: From<bool> + Send + Sync + 'static>(signal: Signal<bool>) -> Signal<Option<T>> {
        Signal::derive(move || signal.get().then(|| T::from(true)))
    }

    // Placeholders and values in left-to-right order in right-to-left locales (a left-to-right
    // embedding), following the locale.
    let is_rtl = move || direction.get() == WritingDirection::Rtl;
    let is_numeric = !matches!(
        kind,
        DateSegmentType::DayPeriod | DateSegmentType::Era | DateSegmentType::TimeZoneName
    );
    let styles = Styles::new()
        .add_unchecked("caret-color", "transparent")
        .add_optional_unchecked("unicode-bidi", move || is_rtl().then_some("embed"))
        .add_optional_unchecked("direction", move || {
            (is_rtl() && is_numeric).then_some("ltr")
        });

    UseDateSegmentReturn {
        segment_props: PropsWithStyles::new(
            UseDateSegmentProps {
                id,
                role: if as_textbox {
                    AriaRole::Textbox
                } else {
                    spin.role
                },
                aria_valuenow: unless_textbox(spin.aria_valuenow),
                aria_valuetext: Signal::derive(move || (!as_textbox).then(|| aria_valuetext.get())),
                aria_valuemin: unless_textbox(spin.aria_valuemin),
                aria_valuemax: unless_textbox(spin.aria_valuemax),
                aria_label: Signal::derive(move || label.get().0),
                aria_labelledby: Signal::derive(move || label.get().1),
                aria_describedby,
                aria_invalid: flag(state.is_invalid),
                aria_readonly: Signal::derive(move || {
                    (state.is_read_only.get() || !segment.get().is_editable)
                        .then_some(AriaReadonly::True)
                }),
                aria_required: flag(state.is_required),
                aria_disabled: flag(state.is_disabled),
                data_placeholder: Signal::derive(move || {
                    segment.get().is_placeholder.then_some("true")
                }),
                contenteditable: editable_flag("true"),
                spellcheck: editable_flag("false"),
                autocorrect: editable_flag("off"),
                enterkeyhint: editable_flag("next"),
                inputmode: Signal::derive(move || {
                    (is_editable.get()
                        && !matches!(kind, DateSegmentType::DayPeriod | DateSegmentType::Era))
                    .then_some("numeric")
                }),
                tabindex: Signal::derive(move || (!state.is_disabled.get()).then_some(0)),
                on_keydown: spin.on_keydown.chain(keyboard.props.on_keydown),
                on_keyup: spin.on_keyup.chain(keyboard.props.on_keyup),
                on_focus: spin.on_focus.chain(on_focus).chain(track_focus.0),
                on_blur: spin.on_blur.chain(track_focus.1),
                on_beforeinput,
                on_input: on_input_event,
                // The field's group mustn't handle presses on segments; the browser focuses them.
                on_pointerdown: EventHandler::new(|e: PointerEvent| e.stop_propagation()),
                on_mousedown: EventHandler::new(|e: MouseEvent| e.stop_propagation()),
                element_capture: element.attr(),
            },
            styles,
        ),
    }
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;
    use crate::utils::i18n::Locale;

    fn locale(tag: &str) -> Locale {
        tag.parse().expect("a locale")
    }

    /// The day period names typed by their first letter (react-aria's `useDateSegment`:
    /// `amPmFormatter`).
    #[test]
    fn names_the_day_periods_of_the_locale() {
        assert_that!(day_periods(&locale("en-US"))).is_equal_to(["AM".to_owned(), "PM".to_owned()]);
        assert_that!(day_periods(&locale("ja-JP")))
            .is_equal_to(["午前".to_owned(), "午後".to_owned()]);
        // German has no day period in its 24-hour times, but names them in 12-hour ones.
        let [am, pm] = day_periods(&locale("de-DE"));
        assert_that!(am.as_str()).is_equal_to("AM");
        assert_that!(pm.as_str()).is_equal_to("PM");
    }

    /// The era names without their common prefix, so that the first letter tells them apart
    /// (react-aria's `useDateSegment`: `eras`).
    #[test]
    fn names_the_eras_without_a_common_prefix() {
        assert_that!(eras(&locale("en-US"))).is_equal_to(["BC".to_owned(), "AD".to_owned()]);
        // "v. Chr." and "n. Chr.": distinct first letters already.
        assert_that!(eras(&locale("de-DE")))
            .is_equal_to(["v. Chr.".to_owned(), "n. Chr.".to_owned()]);
    }
}
