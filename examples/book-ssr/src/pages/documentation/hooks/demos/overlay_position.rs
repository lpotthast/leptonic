use leptonic::{
    atoms::{
        button::Button,
        field::Label,
        focus_scope::FocusScope,
        radio::{RadioButton, RadioField, RadioGroup},
    },
    hooks::*,
};
use leptos::{portal::Portal, prelude::*};
use leptos_element_capture::CapturedElement;

/// `use_overlay_position` with `use_overlay` and `use_overlay_trigger`: a button toggles a panel placed next to it.
#[component]
pub fn PositioningDemo() -> impl IntoView {
    // The radio groups select the enums themselves (`selection_value!` below).
    let side = RwSignal::new(Some(Side::Top));
    let align = RwSignal::new(Some(Align::End));
    let placement = Signal::derive(move || {
        side.get()
            .unwrap_or(Side::Top)
            .placement(align.get().unwrap_or(Align::End))
    });

    let target = CapturedElement::new();
    let (is_open, set_is_open) = signal(false);

    let UseOverlayReturn {
        props: overlay_props,
        id,
        ..
    } = use_overlay(UseOverlayInput {
        is_dismissable: Signal::stored(true),
        // The trigger toggles the overlay itself, so presses on it must not count as "outside".
        should_close_on_interact_outside: Some(InteractOutsideFilter::new(
            move |el: &web_sys::Element| {
                !target.with_untracked(|target| {
                    target.is_some_and(|target| target.contains(Some(el.as_ref())))
                })
            },
        )),
        is_open: is_open.into(),
        on_close: Callback::new(move |()| set_is_open.set(false)),
        should_close_on_blur: Signal::stored(false),
        is_keyboard_dismiss_disabled: Signal::stored(false),
        group: None,
    });
    let overlay_attrs = StoredValue::new(overlay_props.into_attrs());

    let UseOverlayTriggerReturn {
        props: trigger_props,
    } = use_overlay_trigger(UseOverlayTriggerInput {
        is_open: is_open.into(),
        overlay_id: id,
        overlay_type: OverlayTriggerType::Dialog,
    });

    let UseOverlayPositionReturn {
        props: position_props,
        placement: opened_on,
        ..
    } = use_overlay_position(UseOverlayPositionInput {
        placement,
        offset: Signal::stored(8.0),
        target,
        is_open: is_open.into(),
        container_padding: Signal::stored(12.0),
        cross_offset: Signal::stored(0.0),
        should_flip: Signal::stored(true),
        boundary: None,
        max_height: Signal::stored(None),
        arrow_size: Signal::stored(None),
        arrow_boundary_offset: Signal::stored(0.0),
        should_update_position: Signal::stored(true),
        target_rect: Signal::stored(None),
        scroll: None,
        on_close: None,
    });
    let (position_attrs, position_styles) = position_props.into_parts();
    let position_attrs = StoredValue::new(position_attrs);
    let position_styles = StoredValue::new(position_styles);

    let UseButtonReturn {
        props: button_props,
        ..
    } = use_button(UseButtonInput {
        on_press: Some(Callback::new(move |_| {
            set_is_open.update(|open| *open = !*open);
        })),
        ..UseButtonInput::default()
    });
    let (button_attrs, button_styles) = button_props.into_parts();

    view! {
        <div class="demo-option-groups">
            <RadioGroup value=side set_value=side classes=["demo-choice-group", "demo-option-group"]>
                <Label classes="demo-choice-group-label">"Side"</Label>
                <div class="demo-choice-group-items">
                    {Side::ALL
                        .into_iter()
                        .map(|side| view! {
                            <RadioField value=side>
                                <RadioButton classes="demo-radio">
                                    <span class="demo-radio-circle" aria-hidden="true"></span>
                                    {format!("{side:?}")}
                                </RadioButton>
                            </RadioField>
                        })
                        .collect_view()}
                </div>
            </RadioGroup>
            <RadioGroup value=align set_value=align classes=["demo-choice-group", "demo-option-group"]>
                <Label classes="demo-choice-group-label">"Alignment"</Label>
                <div class="demo-choice-group-items">
                    {Align::ALL
                        .into_iter()
                        .map(|align| view! {
                            <RadioField value=align>
                                <RadioButton classes="demo-radio">
                                    <span class="demo-radio-circle" aria-hidden="true"></span>
                                    {format!("{align:?}")}
                                </RadioButton>
                            </RadioField>
                        })
                        .collect_view()}
                </div>
            </RadioGroup>
        </div>

        <div class="demo-positioning-stage">
            <button
                {..button_attrs}
                {..trigger_props.into_attrs()}
                {..target.attr()}
                class="demo-positioning-target"
                style=button_styles
            >
                "Show placement"
            </button>
        </div>

        <p class="demo-status">
            {move || match (is_open.get(), opened_on.get()) {
                (true, Some(side)) => format!("Requested {:?}, opened on the {} side.", placement.get(), side.as_str()),
                _ => format!("Requested {:?}. The overlay is closed.", placement.get()),
            }}
        </p>

        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..overlay_attrs.get_value()}
                    {..position_attrs.get_value()}
                    style=position_styles.get_value()
                    role="dialog"
                    aria-label="Placement"
                    class="demo-popover"
                >
                    // Moves focus into the overlay, so that Escape reaches its key handler, and back to the trigger.
                    <FocusScope restore_focus=true auto_focus=true>
                        <p class="demo-overlay-text">{move || format!("Placed {:?} of the button.", placement.get())}</p>
                        <Button on_press=move |_| set_is_open.set(false) classes="demo-btn">"Close"</Button>
                    </FocusScope>
                </div>
            </Show>
        </Portal>
    }
}

/// The side of the trigger the overlay goes to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
    Start,
    End,
}

impl Side {
    const ALL: [Self; 6] = [
        Self::Top,
        Self::Bottom,
        Self::Left,
        Self::Right,
        Self::Start,
        Self::End,
    ];

    /// The placement for this side and alignment (top and bottom align horizontally, the other sides vertically).
    fn placement(self, align: Align) -> Placement {
        match (self, align) {
            (Self::Top, Align::Start) => Placement::TopStart,
            (Self::Top, Align::Center) => Placement::Top,
            (Self::Top, Align::End) => Placement::TopEnd,
            (Self::Bottom, Align::Start) => Placement::BottomStart,
            (Self::Bottom, Align::Center) => Placement::Bottom,
            (Self::Bottom, Align::End) => Placement::BottomEnd,
            (Self::Left, Align::Start) => Placement::LeftTop,
            (Self::Left, Align::Center) => Placement::Left,
            (Self::Left, Align::End) => Placement::LeftBottom,
            (Self::Right, Align::Start) => Placement::RightTop,
            (Self::Right, Align::Center) => Placement::Right,
            (Self::Right, Align::End) => Placement::RightBottom,
            (Self::Start, Align::Start) => Placement::StartTop,
            (Self::Start, Align::Center) => Placement::Start,
            (Self::Start, Align::End) => Placement::StartBottom,
            (Self::End, Align::Start) => Placement::EndTop,
            (Self::End, Align::Center) => Placement::End,
            (Self::End, Align::End) => Placement::EndBottom,
        }
    }
}

leptonic::selection_value!(Side {
    Top = "top",
    Bottom = "bottom",
    Left = "left",
    Right = "right",
    Start = "start",
    End = "end",
});

/// The alignment along the side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Align {
    Start,
    Center,
    End,
}

impl Align {
    const ALL: [Self; 3] = [Self::Start, Self::Center, Self::End];
}

leptonic::selection_value!(Align {
    Start = "start",
    Center = "center",
    End = "end",
});
