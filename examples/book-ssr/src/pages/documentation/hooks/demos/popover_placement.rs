use leptonic::{
    components::prelude::*,
    hooks::{PlacementX, PlacementY, *},
    utils::css::em,
    utils::{classes::Classes, locale::WritingDirection},
};
use leptos::{portal::Portal, prelude::*};

/// Placement demo showing different positions
#[component]
pub fn PlacementPopoverDemo() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (placement_x, set_placement_x) = signal(PlacementX::Center);
    let (placement_y, set_placement_y) = signal(PlacementY::Below);

    let UsePopoverReturn {
        props,
        trigger_props,
        underlay_props,
        id: _,
        resolved_placement_x: _,
        resolved_placement_y: _,
    } = use_popover(UsePopoverInput {
        is_open: is_open.into(),
        on_close: Callback::new(move |()| set_is_open.set(false)),
        placement_x: placement_x.into(),
        placement_y: placement_y.into(),
        writing_direction: Signal::derive(|| WritingDirection::Ltr),
        offset: 0.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        is_non_modal: false,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });

    let trigger_attrs = StoredValue::new(trigger_props.into_attrs());
    let (popover_props, popover_styles) = props.into_parts();
    let popover_props = StoredValue::new(popover_props);
    let popover_styles = StoredValue::new(popover_styles);
    let underlay_props = StoredValue::new(underlay_props.into_attrs());

    view! {
        <Grid gap=em(0.5) classes="demo-mb-1">
            <Row>
                <Col xs=6 classes="demo-option-group">
                    <strong>"Horizontal"</strong>
                    <RadioGroup classes="demo-radio-list">
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    placement_x.get() == PlacementX::OuterLeft
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_placement_x.set(PlacementX::OuterLeft);
                                    }
                                }
                            />
                            <Label>"OuterLeft"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    placement_x.get() == PlacementX::Left
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_placement_x.set(PlacementX::Left);
                                    }
                                }
                            />
                            <Label>"Left"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    placement_x.get() == PlacementX::Center
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_placement_x.set(PlacementX::Center);
                                    }
                                }
                            />
                            <Label>"Center"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    placement_x.get() == PlacementX::Right
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_placement_x.set(PlacementX::Right);
                                    }
                                }
                            />
                            <Label>"Right"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    placement_x.get() == PlacementX::OuterRight
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_placement_x.set(PlacementX::OuterRight);
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
                                    placement_y.get() == PlacementY::Above
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_placement_y.set(PlacementY::Above);
                                    }
                                }
                            />
                            <Label>"Above"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || placement_y.get() == PlacementY::Top)
                                set_checked=move |checked| {
                                    if checked {
                                        set_placement_y.set(PlacementY::Top);
                                    }
                                }
                            />
                            <Label>"Top"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    placement_y.get() == PlacementY::Center
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_placement_y.set(PlacementY::Center);
                                    }
                                }
                            />
                            <Label>"Center"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    placement_y.get() == PlacementY::Bottom
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_placement_y.set(PlacementY::Bottom);
                                    }
                                }
                            />
                            <Label>"Bottom"</Label>
                        </FormControl>
                        <FormControl classes="demo-form-row">
                            <Radio
                                checked=Signal::derive(move || {
                                    placement_y.get() == PlacementY::Below
                                })
                                set_checked=move |checked| {
                                    if checked {
                                        set_placement_y.set(PlacementY::Below);
                                    }
                                }
                            />
                            <Label>"Below"</Label>
                        </FormControl>
                    </RadioGroup>
                </Col>
            </Row>
        </Grid>

        <div class="demo-popover-stage">
            <button
                {..trigger_attrs.get_value()}
                on:click=move |_| set_is_open.set(!is_open.get())
                class=Classes::from("demo-btn-primary")
            >
                {move || if is_open.get() { "Close" } else { "Open Popover" }}
            </button>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                <div {..underlay_props.get_value()} class="demo-popover-underlay" />
                <div
                    {..popover_props.get_value()}
                    class="demo-popover-panel"
                    style=popover_styles.get_value()
                >
                    <p class="demo-muted-text">
                        {move || {
                            format!("Placement: {:?} / {:?}", placement_x.get(), placement_y.get())
                        }}
                    </p>
                </div>
            </Show>
        </Portal>
    }
}
