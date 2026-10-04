use leptonic::{
    components::prelude::*,
    hooks::{PlacementX, PlacementY, *},
    utils::{css::em, locale::WritingDirection},
};
use leptos::{portal::Portal, prelude::*};
use leptos_element_capture::CapturedElement;

/// Positioning demo: all three hooks combined with placement controls.
#[component]
pub fn PositioningDemo() -> impl IntoView {
    let (selected_placement_x, set_selected_placement_x) = signal(PlacementX::Right);
    let (selected_placement_y, set_selected_placement_y) = signal(PlacementY::Above);

    let target_element = CapturedElement::new();

    let (is_open, set_is_open) = signal(false);

    let UseOverlayReturn {
        props: overlay_props,
        underlay_props: _,
        id,
        overlay_element: _,
    } = use_overlay(UseOverlayInput {
        is_open: is_open.into(),
        on_close: Callback::new(move |()| set_is_open.set(false)),
        is_dismissable: true,
        should_close_on_blur: false,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
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
        resolved_placement_x: _,
        resolved_placement_y: _,
    } = use_overlay_position(UseOverlayPositionInput {
        target: target_element,
        placement_y: selected_placement_y.into(),
        placement_x: selected_placement_x.into(),
        writing_direction: WritingDirection::Ltr.into(),
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
                    <RadioGroup classes="demo-radio-list">
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    selected_placement_x.get() == PlacementX::OuterLeft
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_selected_placement_x.set(PlacementX::OuterLeft);
                                    }
                                }
                            />
                            <Label>"OuterLeft"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    selected_placement_x.get() == PlacementX::Left
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_selected_placement_x.set(PlacementX::Left);
                                    }
                                }
                            />
                            <Label>"Left"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    selected_placement_x.get() == PlacementX::Center
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_selected_placement_x.set(PlacementX::Center);
                                    }
                                }
                            />
                            <Label>"Center"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    selected_placement_x.get() == PlacementX::Right
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_selected_placement_x.set(PlacementX::Right);
                                    }
                                }
                            />
                            <Label>"Right"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    selected_placement_x.get() == PlacementX::OuterRight
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_selected_placement_x.set(PlacementX::OuterRight);
                                    }
                                }
                            />
                            <Label>"OuterRight"</Label>
                        </FormControl>
                    </RadioGroup>
                </Col>
                <Col xs=6 classes="demo-option-group">
                    <strong>"Vertical"</strong>
                    <RadioGroup classes="demo-radio-list">
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    selected_placement_y.get() == PlacementY::Above
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_selected_placement_y.set(PlacementY::Above);
                                    }
                                }
                            />
                            <Label>"Above"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    selected_placement_y.get() == PlacementY::Top
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_selected_placement_y.set(PlacementY::Top);
                                    }
                                }
                            />
                            <Label>"Top"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    selected_placement_y.get() == PlacementY::Center
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_selected_placement_y.set(PlacementY::Center);
                                    }
                                }
                            />
                            <Label>"Center"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    selected_placement_y.get() == PlacementY::Bottom
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_selected_placement_y.set(PlacementY::Bottom);
                                    }
                                }
                            />
                            <Label>"Bottom"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    selected_placement_y.get() == PlacementY::Below
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_selected_placement_y.set(PlacementY::Below);
                                    }
                                }
                            />
                            <Label>"Below"</Label>
                        </FormControl>
                    </RadioGroup>
                </Col>
            </Row>
        </Grid>

        <div class="demo-positioning-stage">
            <div
                {..trigger_attrs.get_value()}
                {..btn_attrs.get_value()}
                {..target_capture}
                class="demo-positioning-target"
                style=btn_styles.get_value()
            >
                "Press me"
            </div>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..overlay_attrs.get_value()}
                    {..overlay_pos_attrs.get_value()}
                    class="demo-positioned-overlay"
                    style=overlay_pos_styles.get_value()
                >
                    {move || {
                        format!(
                            "{:?} / {:?}",
                            selected_placement_x.get(),
                            selected_placement_y.get(),
                        )
                    }}
                </div>
            </Show>
        </Portal>
    }
}
