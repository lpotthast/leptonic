use leptonic::{
    components::prelude::*,
    hooks::{PlacementX, PlacementY, collections::Key, *},
    utils::css::em,
};
use leptos::{portal::Portal, prelude::*};
use leptos_element_capture::CapturedElement;

/// All three overlay hooks combined: a button toggles a panel placed next to it.
#[component]
pub fn PositioningDemo() -> impl IntoView {
    let (selected_placement_x, set_selected_placement_x) = signal(PlacementX::Right);
    let (selected_placement_y, set_selected_placement_y) = signal(PlacementY::Above);

    let target_element = CapturedElement::new();

    let (is_open, set_is_open) = signal(false);

    let UseOverlayReturn {
        props: overlay_props,
        id,
        overlay_element: _,
    } = use_overlay(UseOverlayInput {
        is_open: is_open.into(),
        on_close: Callback::new(move |()| set_is_open.set(false)),
        is_dismissable: true,
        should_close_on_blur: false,
        is_keyboard_dismiss_disabled: false,
        // Presses on the trigger toggle the overlay themselves, so they must not count as "outside".
        should_close_on_interact_outside: Some(Callback::new(move |el: web_sys::Element| {
            !target_element.with_untracked(|target| {
                target.is_some_and(|target| target.contains(Some(el.as_ref())))
            })
        })),
    });
    let overlay_attrs = StoredValue::new(overlay_props.into_attrs());

    let UseOverlayTriggerReturn {
        props: trigger_props,
    } = use_overlay_trigger(UseOverlayTriggerInput {
        show: is_open.into(),
        overlay_id: id,
        overlay_type: OverlayTriggerType::Dialog,
    });
    let trigger_attrs = StoredValue::new(trigger_props.into_attrs());

    let UseOverlayPositionReturn {
        props: overlay_pos_props,
        resolved_placement_x,
        resolved_placement_y,
    } = use_overlay_position(UseOverlayPositionInput {
        target: target_element,
        placement_y: selected_placement_y.into(),
        placement_x: selected_placement_x.into(),
        offset: 0.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        max_height: None,
        is_open: is_open.into(),
    });
    let (overlay_pos_attrs, overlay_pos_styles) = overlay_pos_props.into_parts();
    let overlay_pos_attrs = StoredValue::new(overlay_pos_attrs);
    let overlay_pos_styles = StoredValue::new(overlay_pos_styles);
    let target_capture = target_element.attr();

    let UseButtonReturn {
        props: btn_props, ..
    } = use_button(UseButtonInput {
        on_press: Some(Callback::new(move |_| {
            set_is_open.set(!is_open.get_untracked());
        })),
        ..Default::default()
    });
    let (btn_attrs, btn_styles) = btn_props.into_parts();
    let btn_attrs = StoredValue::new(btn_attrs);
    let btn_styles = StoredValue::new(btn_styles);

    view! {
        <Grid gap=em(0.5) classes="demo-mb-1">
            <Row>
                <Col xs=6 classes="demo-option-group">
                    <strong>"Horizontal"</strong>
                    <RadioGroup
                        classes="demo-radio-list"
                        aria_label="Horizontal placement"
                        default_value=format!("{:?}", selected_placement_x.get_untracked())
                        on_change={move |value: Option<Key>| {
                            set_selected_placement_x.set(match value.map(|v| v.to_string()).as_deref() {
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
                        default_value=format!("{:?}", selected_placement_y.get_untracked())
                        on_change={move |value: Option<Key>| {
                            set_selected_placement_y.set(match value.map(|v| v.to_string()).as_deref() {
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

        <div class="demo-positioning-stage">
            <button
                {..trigger_attrs.get_value()}
                {..btn_attrs.get_value()}
                {..target_capture}
                class="demo-positioning-target"
                style=btn_styles.get_value()
            >
                "Press me"
            </button>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..overlay_attrs.get_value()}
                    {..overlay_pos_attrs.get_value()}
                    class="demo-positioned-overlay"
                    style=overlay_pos_styles.get_value()
                >
                    <div>
                        {move || format!("Requested: {:?} / {:?}", selected_placement_x.get(), selected_placement_y.get())}
                    </div>
                    <div>
                        {move || format!("Resolved: {:?} / {:?}", resolved_placement_x.get(), resolved_placement_y.get())}
                    </div>
                </div>
            </Show>
        </Portal>
    }
}
