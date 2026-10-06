// Upstream: react-aria/src/table/useTableColumnResize.ts @ 99e6102368
use std::collections::HashMap;

use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
    tachys::html::property::{Property, prop},
};
use wasm_bindgen::JsCast;
use web_sys::{Event, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use super::{
    messages, table_utils::ColumnSize, use_table::TableData,
    use_table_column_resize_state::TableColumnResizeState,
};
use crate::{
    hooks::{
        IntoAttrs, Modality, MoveEndEvent, MoveEvent, MoveStartEvent, PressEvent, PropsWithStyles,
        UseKeyboardInput, UseMoveInput, UsePressInput, collections::Key, use_interaction_modality,
        use_keyboard, use_move, use_press,
    },
    utils::{
        ElementCaptureAttr, EventAccessors, EventHandler,
        aria::AriaOrientation,
        css::TouchAction,
        element_capture::CapturedElement,
        focus::focus_safely,
        i18n::use_direction,
        id::use_id,
        keyboard_shortcut::{KeyboardShortcuts, Shortcut},
        locale::WritingDirection,
        pointer_type::PointerType,
        shadow_dom::get_active_element,
        style::TouchActionProperty,
        use_description::use_reactive_description,
        visually_hidden::visually_hidden_styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `on_resize_*` get the column sizes as a `HashMap<Key, ColumnSize>`.
// - The input's `value` is set as attribute and DOM property (React sets both).
//
// =============================================================================

/// Input of [`use_table_column_resize`].
#[derive(Debug, Clone)]
pub struct UseTableColumnResizeInput {
    pub state: TableColumnResizeState,
    /// The table (from [`use_table`](super::use_table)), for the column header's id.
    pub table: TableData,
    /// The column this resizer resizes.
    pub column: Key,
    /// The resizer's label.
    pub aria_label: String,
    /// The resizer's (visually hidden) range input.
    pub element: CapturedElement,
    /// The element that started the resizing (e.g. the column header), focused again when the
    /// resizing ends. Without one, keyboard users are told how to start resizing.
    pub trigger: Option<CapturedElement>,
    pub is_disabled: Signal<bool>,
    /// Called with the column sizes when resizing starts.
    pub on_resize_start: Option<Callback<HashMap<Key, ColumnSize>>>,
    /// Called with the column sizes whenever the column is resized.
    pub on_resize: Option<Callback<HashMap<Key, ColumnSize>>>,
    /// Called with the column sizes when resizing ends.
    pub on_resize_end: Option<Callback<HashMap<Key, ColumnSize>>>,
}

impl UseTableColumnResizeInput {
    pub fn new(
        state: TableColumnResizeState,
        table: TableData,
        column: Key,
        element: CapturedElement,
    ) -> Self {
        Self {
            state,
            table,
            column,
            aria_label: messages::RESIZER.to_owned(),
            element,
            trigger: None,
            is_disabled: Signal::stored(false),
            on_resize_start: None,
            on_resize: None,
            on_resize_end: None,
        }
    }
}

/// Output of [`use_table_column_resize`].
#[derive(Debug)]
pub struct UseTableColumnResizeReturn {
    /// Props for the resizer element (the handle users drag), containing the input.
    pub resizer_props: PropsWithStyles<UseTableColumnResizerProps>,
    /// Props for the visually hidden range input.
    pub input_props: PropsWithStyles<UseTableColumnResizeInputProps>,
    /// Whether the column is being resized.
    pub is_resizing: Signal<bool>,
    /// Whether the column is being resized with the mouse.
    pub is_mouse_resizing: Signal<bool>,
}

/// Props for the resizer element.
#[derive(Debug)]
pub struct UseTableColumnResizerProps {
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub on_dragstart: EventHandler<web_sys::DragEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseTableColumnResizerProps {
    type Attrs = UseTableColumnResizerAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            self.on_keydown.into_on(ev::keydown),
            self.on_keyup.into_on(ev::keyup),
            self.on_pointerdown.into_on(ev::pointerdown),
            self.on_pointerup.into_on(ev::pointerup),
            self.on_click.into_on(ev::click),
            self.on_mousedown.into_on(ev::mousedown),
            self.on_dragstart.into_on(ev::dragstart),
            self.on_dblclick.into_on(ev::dblclick),
            self.element_capture,
        )
    }
}

pub type UseTableColumnResizerAttrs = (
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
    On<ev::pointerup, SharedEventCallback<PointerEvent>>,
    On<ev::click, SharedEventCallback<MouseEvent>>,
    On<ev::mousedown, SharedEventCallback<MouseEvent>>,
    On<ev::dragstart, SharedEventCallback<web_sys::DragEvent>>,
    On<ev::dblclick, SharedEventCallback<MouseEvent>>,
    ElementCaptureAttr,
);

/// Props for the resizer's range input.
#[derive(Debug)]
pub struct UseTableColumnResizeInputProps {
    pub id: String,
    pub aria_label: String,
    pub aria_labelledby: String,
    pub aria_describedby: Signal<Option<String>>,
    pub aria_valuetext: Signal<String>,
    pub min: Signal<f64>,
    pub max: Signal<f64>,
    pub value: Signal<f64>,
    pub disabled: Signal<bool>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_input: EventHandler<Event>,
    pub element_capture: ElementCaptureAttr,
}

impl IntoAttrs for UseTableColumnResizeInputProps {
    type Attrs = UseTableColumnResizeInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        let value = self.value;
        (
            Attr(attr::Id, self.id),
            Attr(attr::Type, "range"),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaOrientation, AriaOrientation::Horizontal),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::AriaValuetext, self.aria_valuetext),
            Attr(attr::Min, self.min),
            Attr(attr::Max, self.max),
            Attr(attr::Value, self.value),
            prop("value", Signal::derive(move || value.get().to_string())),
            Attr(attr::Disabled, self.disabled),
            self.on_blur.into_on(ev::blur),
            self.on_input.into_on(ev::input),
            self.element_capture,
        )
    }
}

pub type UseTableColumnResizeInputAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Type, &'static str>,
    Attr<attr::AriaLabel, String>,
    Attr<attr::AriaOrientation, AriaOrientation>,
    Attr<attr::AriaLabelledby, String>,
    Attr<attr::AriaDescribedby, Signal<Option<String>>>,
    Attr<attr::AriaValuetext, Signal<String>>,
    Attr<attr::Min, Signal<f64>>,
    Attr<attr::Max, Signal<f64>>,
    Attr<attr::Value, Signal<f64>>,
    Property<&'static str, Signal<String>>,
    Attr<attr::Disabled, Signal<bool>>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    On<ev::input, SharedEventCallback<Event>>,
    ElementCaptureAttr,
);

/// Starts, performs and ends the resizing of one column.
#[derive(Debug, Clone)]
struct Resizer {
    state: TableColumnResizeState,
    column: Key,
    trigger: Option<CapturedElement>,
    on_resize_start: Option<Callback<HashMap<Key, ColumnSize>>>,
    on_resize: Option<Callback<HashMap<Key, ColumnSize>>>,
    on_resize_end: Option<Callback<HashMap<Key, ColumnSize>>>,
    is_resizing: StoredValue<bool>,
    last_size: StoredValue<Option<HashMap<Key, ColumnSize>>>,
    was_focused_on_resize_start: StoredValue<bool>,
}

impl Resizer {
    fn current_width(&self) -> f64 {
        untrack(|| self.state.column_width(&self.column))
    }

    fn is_edit_mode(&self) -> bool {
        self.state
            .table_state
            .grid
            .is_keyboard_navigation_disabled
            .get_untracked()
    }

    fn start(&self) {
        if !self.is_resizing.get_value() {
            let sizes = self
                .state
                .update_resized_columns(&self.column, self.current_width());
            self.last_size.set_value(Some(sizes.clone()));
            self.state.start_resize(self.column.clone());
            self.state
                .table_state
                .grid
                .is_keyboard_navigation_disabled
                .set(true);
            if let Some(on_resize_start) = self.on_resize_start {
                on_resize_start.run(sizes);
            }
        }
        self.is_resizing.set_value(true);
    }

    fn resize(&self, width: f64) {
        let sizes = self.state.update_resized_columns(&self.column, width);
        if let Some(on_resize) = self.on_resize {
            on_resize.run(sizes.clone());
        }
        self.last_size.set_value(Some(sizes));
    }

    fn end(&self) {
        if self.is_resizing.get_value() {
            let sizes = self.last_size.get_value().unwrap_or_else(|| {
                self.state
                    .update_resized_columns(&self.column, self.current_width())
            });
            self.state.end_resize();
            self.state
                .table_state
                .grid
                .is_keyboard_navigation_disabled
                .set(false);
            if let Some(on_resize_end) = self.on_resize_end {
                on_resize_end.run(sizes);
            }
            self.is_resizing.set_value(false);
            if !self.was_focused_on_resize_start.get_value()
                && let Some(trigger) = self.trigger.and_then(|t| t.get_untracked())
            {
                focus_safely(&trigger);
            }
        }
        self.last_size.set_value(None);
    }

    /// Ends the resizing if the column is being resized with the keyboard ("edit mode").
    /// Returns whether it did.
    fn end_in_edit_mode(&self) -> bool {
        if self.is_edit_mode() {
            self.end();
            true
        } else {
            false
        }
    }
}

fn has_touch_events() -> bool {
    leptos_use::use_window()
        .as_ref()
        .is_some_and(|window| js_sys::Reflect::has(window, &"ontouchstart".into()).unwrap_or(false))
}

/// Provides the behavior and accessibility of a table column resizer: a handle resizing its
/// column with the mouse, touch and (after pressing Enter on its focused range input) the
/// arrow keys.
///
/// Render the resizer element inside the column header, and the input inside the resizer.
#[allow(clippy::too_many_lines)]
pub fn use_table_column_resize(input: UseTableColumnResizeInput) -> UseTableColumnResizeReturn {
    let UseTableColumnResizeInput {
        state,
        table,
        column,
        aria_label,
        element,
        trigger,
        is_disabled,
        on_resize_start,
        on_resize,
        on_resize_end,
    } = input;
    let id = use_id("table-column-resizer");
    let resizer = Resizer {
        state,
        column: column.clone(),
        trigger,
        on_resize_start,
        on_resize,
        on_resize_end,
        is_resizing: StoredValue::new(false),
        last_size: StoredValue::new(None),
        was_focused_on_resize_start: StoredValue::new(false),
    };
    let is_resizing = {
        let column = column.clone();
        Signal::derive(move || state.resizing_column.with(|r| r.as_ref() == Some(&column)))
    };
    let (is_mouse_resizing, set_mouse_resizing) = signal(false);
    let direction = use_direction();

    let focus_input = move || {
        if let Some(input) = element.get_untracked() {
            focus_safely(&input);
        }
    };

    // Keyboard: Enter starts and ends resizing; Escape, Space and Tab end it.
    let shortcuts = {
        let r = resizer.clone();
        let enter = resizer.clone();
        let space = resizer.clone();
        let tab = resizer.clone();
        KeyboardShortcuts::new()
            .on(Shortcut::key("Escape"), move |_| r.end_in_edit_mode())
            .on(Shortcut::key("Enter"), move |_| {
                if enter.is_edit_mode() {
                    enter.end();
                } else {
                    enter.start();
                }
            })
            .on(Shortcut::key(" "), move |_| space.end_in_edit_mode())
            .on(Shortcut::key("Tab"), move |_| tab.end_in_edit_mode())
    };
    let keyboard = use_keyboard(UseKeyboardInput {
        shortcuts: Some(shortcuts),
        ..UseKeyboardInput::default()
    })
    .props;

    // Moving the resizer (pointer drags, and the arrow keys in edit mode).
    let column_resize_width = StoredValue::new(0.0_f64);
    let move_props = {
        let start = resizer.clone();
        let moving = resizer.clone();
        let end = resizer.clone();
        use_move(UseMoveInput {
            is_disabled: Signal::stored(false),
            axis: Signal::stored(None),
            on_move_start: Some(Callback::new(move |e: MoveStartEvent| {
                column_resize_width.set_value(start.current_width());
                if e.pointer_type == PointerType::Mouse {
                    set_mouse_resizing.set(true);
                }
                start.start();
            })),
            on_move: Some(Callback::new(move |e: MoveEvent| {
                let mut delta_x = e.delta_x;
                if direction.get_untracked() == WritingDirection::Rtl {
                    delta_x *= -1.0;
                }
                if e.pointer_type == PointerType::Keyboard {
                    if e.delta_y != 0.0 && delta_x == 0.0 {
                        delta_x = -e.delta_y;
                    }
                    delta_x *= 10.0;
                }
                if delta_x != 0.0 {
                    column_resize_width.update_value(|w| *w += delta_x);
                    moving.resize(column_resize_width.get_value());
                }
            })),
            on_move_end: Some(Callback::new(move |e: MoveEndEvent| {
                column_resize_width.set_value(0.0);
                set_mouse_resizing.set(false);
                if e.pointer_type == PointerType::Mouse
                    || (e.pointer_type == PointerType::Touch
                        && end.was_focused_on_resize_start.get_value())
                {
                    end.end();
                }
            })),
            on_position_change: None,
            constraint: None,
            allow_container_click: false,
            initial_position: None,
        })
        .props
    };
    // The arrow keys only resize in edit mode.
    let move_keydown = {
        let r = resizer.clone();
        let on_keydown = move_props.on_keydown;
        EventHandler::new(move |e: KeyboardEvent| {
            if r.is_edit_mode() {
                on_keydown.call(e);
            }
        })
    };

    // Pressing the resizer: focus the input and start resizing.
    let press = {
        let start = resizer.clone();
        let press = resizer.clone();
        use_press(UsePressInput {
            prevent_focus_on_press: true,
            on_press_start: Some(Callback::new(move |e: PressEvent| {
                let m = e.modifiers;
                if m.ctrl_key
                    || m.alt_key
                    || m.meta_key
                    || m.shift_key
                    || e.pointer_type == PointerType::Keyboard
                {
                    return;
                }
                let is_virtual = e.pointer_type == PointerType::Virtual;
                if is_virtual && state.resizing_column.get_untracked().is_some() {
                    start.end();
                    return;
                }
                focus_input();
                if !is_virtual {
                    start.start();
                }
            })),
            on_press: Callback::new(move |e: PressEvent| {
                let ends = (e.pointer_type == PointerType::Touch
                    && press.was_focused_on_resize_start.get_value())
                    || e.pointer_type == PointerType::Mouse;
                if ends && state.resizing_column.get_untracked().is_some() {
                    press.end();
                }
            }),
            ..UsePressInput::default()
        })
    };
    let (press_props, press_styles) = press.props.into_inner();

    // Resizing started elsewhere (e.g. from a column menu): focus the input.
    let prev_resizing_column = StoredValue::new(None::<Key>);
    {
        let resizer = resizer.clone();
        let column = column.clone();
        Effect::new(move |_| {
            let resizing_column = state.resizing_column.get();
            untrack(|| {
                if prev_resizing_column.get_value() != resizing_column
                    && resizing_column.as_ref() == Some(&column)
                {
                    let active = leptos_use::use_document()
                        .as_ref()
                        .and_then(get_active_element);
                    let input = element.get_untracked();
                    resizer
                        .was_focused_on_resize_start
                        .set_value(active.is_some() && active.as_ref() == input.as_deref());
                    resizer.start();
                    // Twice: VoiceOver moves focus back shortly after.
                    let timeouts = [0, 400].map(|ms| {
                        set_timeout_with_handle(focus_input, std::time::Duration::from_millis(ms))
                            .ok()
                    });
                    on_cleanup(move || {
                        for timeout in timeouts.into_iter().flatten() {
                            timeout.clear();
                        }
                    });
                } else {
                    prev_resizing_column.set_value(resizing_column);
                }
            });
        });
    }

    // Screen readers change the input's value: resize in steps of 10 pixels.
    let on_input = {
        let r = resizer.clone();
        EventHandler::new(move |e: Event| {
            let current = r.current_width();
            let Some(value) = e
                .expect_target()
                .dyn_ref::<web_sys::HtmlInputElement>()
                .and_then(|input| input.value().parse::<f64>().ok())
            else {
                return;
            };
            r.resize(if value > current {
                current + 10.0
            } else {
                current - 10.0
            });
        })
    };
    let on_blur = {
        let r = resizer.clone();
        EventHandler::new(move |_: FocusEvent| r.end())
    };

    // Keyboard users without a trigger are told how to start resizing.
    let modality = use_interaction_modality();
    let has_trigger = trigger.is_some();
    let description = Signal::derive(move || {
        let modality = match modality.get() {
            Some(Modality::Virtual) if has_touch_events() => None,
            modality => modality,
        };
        let describes = !has_trigger
            && matches!(modality, Some(Modality::Keyboard | Modality::Virtual))
            && !is_resizing.get();
        describes.then(|| messages::RESIZER_DESCRIPTION.to_owned())
    });
    let aria_describedby = use_reactive_description(description);

    let width_of = move |f: fn(&TableColumnResizeState, &Key) -> f64| {
        let column = column.clone();
        Signal::derive(move || f(&state, &column).floor())
    };
    let value = width_of(TableColumnResizeState::column_width);
    let min = width_of(TableColumnResizeState::column_min_width);
    let max = width_of(TableColumnResizeState::column_max_width);

    UseTableColumnResizeReturn {
        resizer_props: PropsWithStyles::new(
            UseTableColumnResizerProps {
                on_keydown: keyboard
                    .on_keydown
                    .chain(move_keydown)
                    .chain(press_props.on_keydown),
                on_keyup: keyboard.on_keyup,
                on_pointerdown: move_props.on_pointerdown.chain(press_props.on_pointerdown),
                on_pointerup: press_props.on_pointerup,
                on_click: press_props.on_click,
                on_mousedown: press_props.on_mousedown,
                on_dragstart: press_props.on_dragstart,
                on_dblclick: press_props.on_dblclick,
                element_capture: move_props.element_capture,
            },
            press_styles.add(TouchActionProperty.declare(TouchAction::None)),
        ),
        input_props: PropsWithStyles::new(
            UseTableColumnResizeInputProps {
                aria_labelledby: format!("{id} {}", table.column_header_id(&resizer.column)),
                id,
                aria_label,
                aria_describedby,
                aria_valuetext: Signal::derive(move || messages::column_size(value.get())),
                min,
                max,
                value,
                disabled: is_disabled,
                on_blur,
                on_input,
                element_capture: element.attr(),
            },
            visually_hidden_styles(),
        ),
        is_resizing,
        is_mouse_resizing: is_mouse_resizing.into(),
    }
}
