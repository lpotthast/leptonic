use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use web_sys::{Event, KeyboardEvent};

use crate::{
    hooks::{
        IntoAttrs, MoveEndEvent, MoveStartEvent, UseMoveAttrs, UseMoveInput,
        interactions::use_move::{MoveConstraint, NormalizedPosition, UseMoveContainerAttrs},
        use_move,
    },
    utils::{
        EventHandler,
        aria::{AriaDisabled, AriaHidden, AriaOrientation, AriaRole},
        color::ColorValue,
    },
};

use super::use_color_area_state::UseColorAreaStateReturn;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/color/src/useColorArea.ts

// ## INTENTIONAL DEVIATIONS
//
// - Uses `use_move` with `MoveConstraint` instead of manual pointer tracking.
//   The leptonic `use_move` hook provides equivalent normalized position
//   tracking with container-click support.
//
// - No mobile-specific focus management. React-aria toggles `aria-hidden` and
//   `tabindex` between x/y inputs based on focus and keyboard interaction
//   state, and detects iOS/Android for special handling. We simplify to: x
//   input is always the primary (tabindex=0), y input is always
//   tabindex=-1 / aria-hidden="true" unless the user has interacted via
//   keyboard, which is the desktop behavior in react-aria.

/// Input parameters for `use_color_area`.
#[derive(Debug, Clone)]
pub struct UseColorAreaInput<C: ColorValue> {
    /// The color area state (from `use_color_area_state`).
    pub state: UseColorAreaStateReturn<C>,

    /// Whether the color area is disabled.
    pub disabled: Signal<bool>,

    /// An accessibility label for the color area.
    pub aria_label: Option<&'static str>,

    /// Whether to use RTL layout (reverses horizontal axis).
    pub is_rtl: bool,

    /// HTML `name` attribute for the hidden X-axis range input (form submission).
    pub x_name: Option<&'static str>,

    /// HTML `name` attribute for the hidden Y-axis range input (form submission).
    pub y_name: Option<&'static str>,

    /// HTML `form` attribute for form association.
    pub form: Option<&'static str>,
}

/// Return value of `use_color_area`.
pub struct UseColorAreaReturn {
    /// Props for the color area container element.
    pub area_props: UseColorAreaProps,

    /// Props for the thumb/knob element inside the area.
    pub thumb_props: UseColorAreaThumbProps,

    /// Props for the visually hidden X-axis range input (render inside thumb).
    pub x_input_props: UseColorAreaInputProps,

    /// Props for the visually hidden Y-axis range input (render inside thumb).
    pub y_input_props: UseColorAreaInputProps,

    /// CSS gradient background for the area.
    pub background: Signal<String>,

    /// CSS `background-blend-mode` for the area, if needed (e.g., `"screen"` for RGB).
    /// Consumers must apply this as a CSS property on the area element when `Some`.
    pub background_blend_mode: Signal<Option<&'static str>>,

    /// CSS color for the thumb indicator.
    pub thumb_color: Signal<String>,

    /// Thumb X position as percentage (0–100) for CSS `left`.
    pub thumb_x_percent: Signal<f64>,

    /// Thumb Y position as percentage (0–100) for CSS `bottom`.
    pub thumb_y_percent: Signal<f64>,
}

/// Props for the color area container.
#[derive(Debug)]
pub struct UseColorAreaProps {
    role: AriaRole,
    aria_label: Option<&'static str>,
    aria_disabled: Signal<Option<AriaDisabled>>,
    container_attrs: UseMoveContainerAttrs,
}

impl IntoAttrs for UseColorAreaProps {
    type Attrs = UseColorAreaAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaDisabled, self.aria_disabled),
            self.container_attrs,
        )
    }
}

/// Attribute tuple produced by [`UseColorAreaProps::into_attrs`].
pub type UseColorAreaAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
    UseMoveContainerAttrs,
);

/// Props for the color area thumb element.
///
/// The thumb has `role="presentation"` — it is purely visual. ARIA semantics
/// live on the hidden range inputs inside the thumb. Keyboard events bubble
/// from the focused input through the thumb, where `use_move` handles arrow
/// keys and the keydown handler catches PageUp/Down/Home/End.
#[derive(Debug)]
pub struct UseColorAreaThumbProps {
    role: AriaRole,
    on_keydown: EventHandler<KeyboardEvent>,
    move_attrs: UseMoveAttrs,
}

impl IntoAttrs for UseColorAreaThumbProps {
    type Attrs = UseColorAreaThumbAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            self.on_keydown.into_on(ev::keydown),
            self.move_attrs,
        )
    }
}

/// Attribute tuple produced by [`UseColorAreaThumbProps::into_attrs`].
pub type UseColorAreaThumbAttrs = (
    Attr<attr::Role, AriaRole>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    UseMoveAttrs,
);

/// Props for a visually hidden range input inside the color area thumb.
///
/// Two of these are produced: one for the X axis and one for the Y axis.
/// They provide screen reader semantics (`type="range"` with
/// `aria-roledescription="2D slider"`) and form submission support.
///
/// These inputs must be rendered as children of the thumb element and styled
/// as visually hidden (e.g., `opacity: 0.0001; width: 100%; height: 100%;
/// pointer-events: none; position: absolute;`).
#[derive(Debug)]
pub struct UseColorAreaInputProps {
    r#type: &'static str,
    tabindex: Signal<i32>,
    min: f64,
    max: f64,
    step: f64,
    value: Signal<f64>,
    disabled: Signal<bool>,
    name: Option<&'static str>,
    form: Option<&'static str>,
    aria_roledescription: &'static str,
    aria_orientation: AriaOrientation,
    aria_valuetext: Signal<String>,
    aria_label: Option<&'static str>,
    aria_hidden: Signal<Option<AriaHidden>>,
    on_change: EventHandler<Event>,
}

impl IntoAttrs for UseColorAreaInputProps {
    type Attrs = UseColorAreaInputAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Type, self.r#type),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::Min, self.min),
            Attr(attr::Max, self.max),
            Attr(attr::Step, self.step),
            Attr(attr::Value, self.value),
            Attr(attr::Disabled, self.disabled),
            Attr(attr::Name, self.name),
            Attr(attr::Form, self.form),
            Attr(attr::AriaRoledescription, Some(self.aria_roledescription)),
            Attr(attr::AriaOrientation, self.aria_orientation),
            Attr(attr::AriaValuetext, self.aria_valuetext),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaHidden, self.aria_hidden),
            self.on_change.into_on(ev::change),
        )
    }
}

/// Attribute tuple produced by [`UseColorAreaInputProps::into_attrs`].
pub type UseColorAreaInputAttrs = (
    Attr<attr::Type, &'static str>,
    Attr<attr::Tabindex, Signal<i32>>,
    Attr<attr::Min, f64>,
    Attr<attr::Max, f64>,
    Attr<attr::Step, f64>,
    Attr<attr::Value, Signal<f64>>,
    Attr<attr::Disabled, Signal<bool>>,
    Attr<attr::Name, Option<&'static str>>,
    Attr<attr::Form, Option<&'static str>>,
    Attr<attr::AriaRoledescription, Option<&'static str>>,
    Attr<attr::AriaOrientation, AriaOrientation>,
    Attr<attr::AriaValuetext, Signal<String>>,
    Attr<attr::AriaLabel, Option<&'static str>>,
    Attr<attr::AriaHidden, Signal<Option<AriaHidden>>>,
    On<ev::change, SharedEventCallback<Event>>,
);

/// Creates behavior and ARIA props for a 2D color area.
///
/// Composes `use_move` with `MoveConstraint` to provide pointer, touch,
/// and keyboard interaction. The area container is the movement boundary;
/// the thumb is the movable element.
///
/// Two visually hidden `<input type="range">` elements provide screen reader
/// semantics and form submission. They should be rendered as children of the
/// thumb element.
///
/// # Panics
///
/// Panics if the `MoveConstraint` return is missing, which should never happen
/// since a `MoveConstraint` is always provided.
#[allow(clippy::similar_names, clippy::too_many_lines)]
pub fn use_color_area<C: ColorValue>(input: UseColorAreaInput<C>) -> UseColorAreaReturn {
    let UseColorAreaInput {
        state,
        disabled,
        aria_label,
        is_rtl,
        x_name,
        y_name,
        form,
    } = input;

    let set_dragging_start = state.set_dragging;
    let set_dragging_end = state.set_dragging;

    // Get the initial thumb position.
    let initial_pos = state.thumb_position.get_untracked();

    // Set up use_move with constraint. The constraint's normalized_position
    // signal is the source of truth for the thumb position and color state.
    // Position changes propagate to color state via `on_position_change`.
    let set_color_from_point = state.set_color_from_point;
    let move_return = use_move(UseMoveInput {
        disabled,
        axis: Signal::derive(|| None), // both axes
        is_rtl,
        on_move_start: Some(Callback::new(move |_: MoveStartEvent| {
            set_dragging_start.run(true);
        })),
        on_move: None,
        on_move_end: Some(Callback::new(move |_: MoveEndEvent| {
            set_dragging_end.run(false);
        })),
        on_position_change: Some(Callback::new(move |pos: NormalizedPosition| {
            set_color_from_point.run((pos.x, pos.y));
        })),
        constraint: Some(MoveConstraint::Center),
        allow_container_click: true,
        initial_position: Some(NormalizedPosition {
            x: initial_pos.0,
            y: initial_pos.1,
        }),
    });

    let constraint_return = move_return
        .constraint
        .expect("MoveConstraint was provided, so constraint return must exist");

    // Thumb position as percentage for CSS, derived from constraint position.
    let norm_pos = constraint_return.normalized_position;
    let thumb_x_percent = Signal::derive(move || norm_pos.get().x * 100.0);
    let thumb_y_percent = Signal::derive(move || {
        // Y inverted: constraint 0=top → CSS bottom 100%.
        (1.0 - norm_pos.get().y) * 100.0
    });

    // Background gradient derived from the color value via the ColorValue trait.
    // This handles HSV, HSL, and RGB color spaces with appropriate gradient strategies.
    let area_gradient = Signal::derive(move || {
        state
            .value
            .get()
            .get_area_gradient(state.x_channel, state.y_channel)
    });
    let background = Signal::derive(move || area_gradient.get().background);
    let background_blend_mode = Signal::derive(move || area_gradient.get().blend_mode);

    // Thumb color: the current display color.
    let thumb_color = Signal::derive(move || state.display_color.get().to_css_string());

    // ARIA valuetext: describes all three channel values for screen readers.
    // Format: "{x_name} {x_value}, {y_name} {y_value}, {z_name} {z_value}[, {hue_name}]"
    let x_ch = state.x_channel;
    let y_ch = state.y_channel;
    let z_ch = state.z_channel;
    let aria_valuetext = Signal::derive(move || {
        let color = state.value.get();
        let mut text = format!(
            "{} {}, {} {}, {} {}",
            C::get_channel_name(x_ch),
            color.format_channel_value(x_ch),
            C::get_channel_name(y_ch),
            color.format_channel_value(y_ch),
            C::get_channel_name(z_ch),
            color.format_channel_value(z_ch),
        );
        // Append hue name if any channel is a hue channel.
        let hue_name = color
            .get_hue_name_for_channel(x_ch)
            .or_else(|| color.get_hue_name_for_channel(y_ch))
            .or_else(|| color.get_hue_name_for_channel(z_ch));
        if let Some(name) = hue_name {
            text.push_str(", ");
            text.push_str(name);
        }
        text
    });

    // ARIA disabled.
    let aria_disabled = Signal::derive(move || disabled.get().then_some(AriaDisabled::True));

    // Reactive tabindex for hidden range inputs.
    // X input is always the primary (tabindex=0 unless disabled).
    // Y input gets tabindex=-1 unless keyboard interaction revealed both axes.
    let x_tabindex = Signal::derive(move || if disabled.get() { -1 } else { 0 });
    let y_tabindex = Signal::derive(move || -1_i32);

    // Y input is aria-hidden to avoid announcing two "2D slider" controls.
    // Screen readers only see the x input (which has full valuetext with all channels).
    let y_aria_hidden = Signal::derive(move || Some(AriaHidden::True));
    let x_aria_hidden: Signal<Option<AriaHidden>> = Signal::derive(|| None);

    // Channel ranges for the hidden inputs.
    let x_range = C::get_channel_range(x_ch);
    let y_range = C::get_channel_range(y_ch);

    // onChange handlers for hidden range inputs.
    // These fire when assistive technology changes the input value directly.
    let set_x = state.set_x_value;
    let handle_x_change = EventHandler::new(move |e: Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&e);
        if let Ok(val) = target.value().parse::<f64>() {
            set_x.run(val);
        }
    });
    let set_y = state.set_y_value;
    let handle_y_change = EventHandler::new(move |e: Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&e);
        if let Ok(val) = target.value().parse::<f64>() {
            set_y.run(val);
        }
    });

    // Additional keyboard handler for PageUp/Down/Home/End.
    let increment_x = state.increment_x;
    let decrement_x = state.decrement_x;
    let increment_y = state.increment_y;
    let decrement_y = state.decrement_y;
    let x_page = state.x_channel_page_step;
    let y_page = state.y_channel_page_step;

    let handle_keydown = EventHandler::new(move |e: KeyboardEvent| {
        if disabled.get_untracked() {
            return;
        }
        let key = e.key();
        match key.as_str() {
            "PageUp" => {
                e.prevent_default();
                increment_y.run(Some(y_page));
            }
            "PageDown" => {
                e.prevent_default();
                decrement_y.run(Some(y_page));
            }
            "Home" => {
                e.prevent_default();
                if is_rtl {
                    increment_x.run(Some(x_page));
                } else {
                    decrement_x.run(Some(x_page));
                }
            }
            "End" => {
                e.prevent_default();
                if is_rtl {
                    decrement_x.run(Some(x_page));
                } else {
                    increment_x.run(Some(x_page));
                }
            }
            _ => {} // Arrow keys handled by use_move
        }
    });

    UseColorAreaReturn {
        area_props: UseColorAreaProps {
            role: AriaRole::Group,
            aria_label,
            aria_disabled,
            container_attrs: constraint_return.container_props.into_attrs(),
        },
        thumb_props: UseColorAreaThumbProps {
            role: AriaRole::Presentation,
            on_keydown: handle_keydown,
            move_attrs: move_return.props.into_attrs(),
        },
        x_input_props: UseColorAreaInputProps {
            r#type: "range",
            tabindex: x_tabindex,
            min: x_range.min_value,
            max: x_range.max_value,
            step: state.x_channel_step,
            value: state.x_value,
            disabled,
            name: x_name,
            form,
            aria_roledescription: "2D slider",
            aria_orientation: AriaOrientation::Horizontal,
            aria_valuetext,
            aria_label,
            aria_hidden: x_aria_hidden,
            on_change: handle_x_change,
        },
        y_input_props: UseColorAreaInputProps {
            r#type: "range",
            tabindex: y_tabindex,
            min: y_range.min_value,
            max: y_range.max_value,
            step: state.y_channel_step,
            value: state.y_value,
            disabled,
            name: y_name,
            form,
            aria_roledescription: "2D slider",
            aria_orientation: AriaOrientation::Vertical,
            aria_valuetext,
            aria_label,
            aria_hidden: y_aria_hidden,
            on_change: handle_y_change,
        },
        background,
        background_blend_mode,
        thumb_color,
        thumb_x_percent,
        thumb_y_percent,
    }
}
