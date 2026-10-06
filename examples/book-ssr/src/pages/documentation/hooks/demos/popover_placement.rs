use leptonic::{
    components::prelude::*,
    hooks::{PlacementX, PlacementY, collections::Key, *},
    utils::css::em,
};
use leptos::{portal::Portal, prelude::*};

#[component]
pub fn PlacementPopoverDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (placement_x, set_placement_x) = signal(PlacementX::Center);
    let (placement_y, set_placement_y) = signal(PlacementY::Below);

    let UsePopoverReturn {
        props,
        trigger_props,
        id,
        resolved_placement_x,
        resolved_placement_y,
        ..
    } = use_popover(UsePopoverInput {
        placement_x: placement_x.into(),
        placement_y: placement_y.into(),
        offset: 0.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        modality: PopoverModality::Modal,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
        ..UsePopoverInput::new(OverlayTriggerState::from((is_open, set_is_open)))
    });

    // The trigger: a button toggling the popover, announcing it with `aria-haspopup`, `aria-expanded` and
    // `aria-controls`.
    let UseOverlayTriggerReturn {
        props: overlay_trigger,
    } = use_overlay_trigger(UseOverlayTriggerInput {
        show: is_open.into(),
        overlay_id: id,
        overlay_type: OverlayTriggerType::Dialog,
    });
    let UseButtonReturn {
        props: button_props,
        ..
    } = use_button(UseButtonInput {
        on_press: Some(Callback::new(move |_| {
            set_is_open.update(|open| *open = !*open);
        })),
        aria_haspopup: Signal::stored(overlay_trigger.aria_haspopup),
        aria_expanded: overlay_trigger.aria_expanded,
        aria_controls: overlay_trigger.aria_controls,
        ..UseButtonInput::default()
    });
    let (button_attrs, button_styles) = button_props.into_parts();
    let (popover_props, popover_styles) = props.into_parts();
    let popover_props = StoredValue::new(popover_props);
    let popover_styles = StoredValue::new(popover_styles);

    view! {
        <Grid gap=em(0.5) classes="demo-mb-1">
            <Row>
                <Col xs=6 classes="demo-option-group">
                    <strong>"Horizontal"</strong>
                    <RadioGroup
                        classes="demo-radio-list"
                        aria_label="Horizontal placement"
                        default_value=format!("{:?}", placement_x.get_untracked())
                        on_change={move |value: Option<Key>| {
                            set_placement_x.set(match value.map(|v| v.to_string()).as_deref() {
                                Some("OuterLeft") => PlacementX::OuterLeft,
                                Some("Left") => PlacementX::Left,
                                Some("Center") => PlacementX::Center,
                                Some("Right") => PlacementX::Right,
                                Some("OuterRight") => PlacementX::OuterRight,
                                _ => return,
                            });
                        }}
                    >
                        <Radio value="OuterLeft" classes="demo-form-row">"OuterLeft"</Radio>
                        <Radio value="Left" classes="demo-form-row">"Left"</Radio>
                        <Radio value="Center" classes="demo-form-row">"Center"</Radio>
                        <Radio value="Right" classes="demo-form-row">"Right"</Radio>
                        <Radio value="OuterRight" classes="demo-form-row">"OuterRight"</Radio>
                    </RadioGroup>
                </Col>
                <Col xs=6 classes="demo-option-group">
                    <strong>"Vertical"</strong>
                    <RadioGroup
                        classes="demo-radio-list"
                        aria_label="Vertical placement"
                        default_value=format!("{:?}", placement_y.get_untracked())
                        on_change={move |value: Option<Key>| {
                            set_placement_y.set(match value.map(|v| v.to_string()).as_deref() {
                                Some("Above") => PlacementY::Above,
                                Some("Top") => PlacementY::Top,
                                Some("Center") => PlacementY::Center,
                                Some("Bottom") => PlacementY::Bottom,
                                Some("Below") => PlacementY::Below,
                                _ => return,
                            });
                        }}
                    >
                        <Radio value="Above" classes="demo-form-row">"Above"</Radio>
                        <Radio value="Top" classes="demo-form-row">"Top"</Radio>
                        <Radio value="Center" classes="demo-form-row">"Center"</Radio>
                        <Radio value="Bottom" classes="demo-form-row">"Bottom"</Radio>
                        <Radio value="Below" classes="demo-form-row">"Below"</Radio>
                    </RadioGroup>
                </Col>
            </Row>
        </Grid>

        <div class="demo-popover-stage">
            <button {..button_attrs} {..trigger_props.into_attrs()} style=button_styles class="demo-btn-primary">
                {move || if is_open.get() { "Close" } else { "Open Popover" }}
            </button>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                <div class="demo-popover-underlay" />
                <div
                    {..popover_props.get_value()}
                    class="demo-overlays-popover"
                    style=popover_styles.get_value()
                >
                    <p class="demo-overlays-text">
                        {move || {
                            format!("Requested: {:?} / {:?}", placement_x.get(), placement_y.get())
                        }}
                    </p>
                    // Differs from the requested placement when the popover had to flip.
                    <p class="demo-overlays-text">
                        {move || {
                            format!(
                                "Resolved: {:?} / {:?}",
                                resolved_placement_x.get(),
                                resolved_placement_y.get(),
                            )
                        }}
                    </p>
                </div>
            </Show>
        </Portal>
    }
}
