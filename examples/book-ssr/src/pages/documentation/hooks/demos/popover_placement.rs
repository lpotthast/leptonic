use leptonic::utils::CapturedElement;
use leptonic::{
    atoms::prelude::FocusScope,
    components::prelude::*,
    hooks::{collections::Key, *},
    utils::{css::em, id::use_id},
};
use leptos::{portal::Portal, prelude::*};

/// The sides of the trigger the demo offers, with their labels.
const SIDES: [(&str, Side); 6] = [
    ("Top", Side::Top),
    ("Bottom", Side::Bottom),
    ("Left", Side::Left),
    ("Right", Side::Right),
    ("Start", Side::Start),
    ("End", Side::End),
];

/// The alignments along the side, with their labels.
const ALIGNMENTS: [(&str, Align); 3] = [
    ("Start", Align::Start),
    ("Center", Align::Center),
    ("End", Align::End),
];

#[component]
pub fn PlacementPopoverDemo() -> impl IntoView {
    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput::default());
    let is_open = state.is_open;
    let side = RwSignal::new(Side::Bottom);
    let align = RwSignal::new(Align::Center);
    let placement = Signal::derive(move || side.get().placement(align.get()));
    let title_id = use_id("placement-title");

    let UsePopoverReturn {
        props,
        trigger_props,
        id,
        placement: resolved_placement,
        ..
    } = use_popover(UsePopoverInput {
        placement,
        offset: Signal::stored(8.0),
        state,
        trigger: CapturedElement::new(),
        cross_offset: Signal::stored(0.0),
        container_padding: Signal::stored(12.0),
        should_flip: Signal::stored(true),
        max_height: Signal::stored(None),
        arrow_size: Signal::stored(None),
        arrow_boundary_offset: Signal::stored(0.0),
        boundary: None,
        target_rect: Signal::stored(None),
        modality: PopoverModality::Modal,
        is_keyboard_dismiss_disabled: Signal::stored(false),
        should_close_on_interact_outside: None,
        group: None,
        is_submenu: false,
    });

    let UseOverlayTriggerReturn {
        props: overlay_trigger,
    } = use_overlay_trigger(UseOverlayTriggerInput {
        show: is_open,
        overlay_id: id,
        overlay_type: OverlayTriggerType::Dialog,
    });
    let UseButtonReturn {
        props: button_props,
        ..
    } = use_button(UseButtonInput {
        on_press: Some(Callback::new(move |_| state.toggle())),
        aria_expanded: overlay_trigger.aria_expanded,
        aria_controls: overlay_trigger.aria_controls,
        ..UseButtonInput::default()
    });
    let (button_attrs, button_styles) = button_props.into_parts();
    let (popover_attrs, popover_styles) = props.into_parts();
    let popover_attrs = StoredValue::new(popover_attrs);
    let popover_styles = StoredValue::new(popover_styles);
    let title_id = StoredValue::new(title_id);

    view! {
        <Grid gap=em(0.5) classes="demo-mb-1">
            <Row>
                <Col xs=6 classes="demo-option-group">
                    <RadioGroup
                        label="Side"
                        classes="demo-radio-list"
                        default_value="Bottom"
                        on_change={move |key: Option<Key>| {
                            if let Some(new_side) = option_for(&SIDES, key) {
                                side.set(new_side);
                            }
                        }}
                    >
                        {SIDES.into_iter().map(|(label, _)| view! { <Radio value=label>{label}</Radio> }).collect_view()}
                    </RadioGroup>
                </Col>
                <Col xs=6 classes="demo-option-group">
                    <RadioGroup
                        label="Alignment"
                        classes="demo-radio-list"
                        default_value="Center"
                        on_change={move |key: Option<Key>| {
                            if let Some(new_align) = option_for(&ALIGNMENTS, key) {
                                align.set(new_align);
                            }
                        }}
                    >
                        {ALIGNMENTS.into_iter().map(|(label, _)| view! { <Radio value=label>{label}</Radio> }).collect_view()}
                    </RadioGroup>
                </Col>
            </Row>
        </Grid>

        <div class="demo-popover-stage">
            <button {..button_attrs} {..trigger_props.into_attrs()} style=button_styles class="demo-btn">
                "Open popover"
            </button>
        </div>
        <p class="demo-status">
            {move || match (is_open.get(), resolved_placement.get()) {
                (true, Some(side)) => format!("Open on the {} side.", side.as_str()),
                _ => "Closed.".to_owned(),
            }}
        </p>

        <Portal>
            <Show when=move || is_open.get()>
                <div class="demo-popover-underlay"/>
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    <div
                        {..popover_attrs.get_value()}
                        style=popover_styles.get_value()
                        class="demo-popover"
                        role="dialog"
                        aria-labelledby=title_id.get_value()
                        tabindex="-1"
                    >
                        <h2 id=title_id.get_value() class="demo-overlay-title">"Placement"</h2>
                        // Differs from the requested side when the popover had to flip.
                        <p class="demo-overlay-text">
                            {move || {
                                format!(
                                    "Opened on the {} side.",
                                    resolved_placement.get().map_or("?", PlacementAxis::as_str),
                                )
                            }}
                        </p>
                    </div>
                </FocusScope>
            </Show>
        </Portal>
    }
}

/// The option whose label is `key`.
fn option_for<T: Copy>(options: &[(&'static str, T)], key: Option<Key>) -> Option<T> {
    let key = key?;
    options
        .iter()
        .find(|(label, _)| Key::from(*label) == key)
        .map(|(_, option)| *option)
}

/// A side of the trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
    Start,
    End,
}

/// The alignment along the side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Align {
    Start,
    Center,
    End,
}

impl Side {
    /// The placement for this side and alignment (top and bottom align horizontally, the other
    /// sides vertically).
    fn placement(self, align: Align) -> Placement {
        match (self, align) {
            (Side::Top, Align::Start) => Placement::TopStart,
            (Side::Top, Align::Center) => Placement::Top,
            (Side::Top, Align::End) => Placement::TopEnd,
            (Side::Bottom, Align::Start) => Placement::BottomStart,
            (Side::Bottom, Align::Center) => Placement::Bottom,
            (Side::Bottom, Align::End) => Placement::BottomEnd,
            (Side::Left, Align::Start) => Placement::LeftTop,
            (Side::Left, Align::Center) => Placement::Left,
            (Side::Left, Align::End) => Placement::LeftBottom,
            (Side::Right, Align::Start) => Placement::RightTop,
            (Side::Right, Align::Center) => Placement::Right,
            (Side::Right, Align::End) => Placement::RightBottom,
            (Side::Start, Align::Start) => Placement::StartTop,
            (Side::Start, Align::Center) => Placement::Start,
            (Side::Start, Align::End) => Placement::StartBottom,
            (Side::End, Align::Start) => Placement::EndTop,
            (Side::End, Align::Center) => Placement::End,
            (Side::End, Align::End) => Placement::EndBottom,
        }
    }
}
