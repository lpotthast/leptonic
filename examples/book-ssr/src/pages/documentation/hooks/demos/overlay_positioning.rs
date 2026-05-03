use leptonic::{
    components::prelude::*,
    hooks::{PlacementX, PlacementY, *},
    utils::css::em,
    utils::{
        aria::{AriaExpanded, AriaHasPopup},
        locale::WritingDirection,
    },
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
        props: btn_props,
        is_hovered: _,
        is_pressed: _,
        is_focus_visible: _,
    } = use_button(UseButtonInput {
        disabled: false.into(),
        aria_haspopup: AriaHasPopup::default().into(),
        aria_expanded: AriaExpanded::default().into(),
        use_press_input: UsePressInput {
            disabled: false.into(),
            force_prevent_default: false,
            force_propagation: false,
            allow_text_selection_on_press: false,
            should_cancel_on_pointer_exit: false,
            prevent_focus_on_press: false,
            force_is_pressed: None,
            on_press: Callback::new(move |_e| {
                set_is_open.set(!is_open.get_untracked());
            }),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
            on_press_change: None,
            on_double_press: None,
            on_long_press_start: None,
            on_long_press: None,
            on_long_press_end: None,
            long_press_threshold: None,
            long_press_accessibility_description: None,
        },
        use_hover_input: UseHoverInput {
            disabled: false.into(),
            on_hover_start: None,
            on_hover_end: None,
            on_hover_change: None,
        },
        use_focus_ring_input: UseFocusRingInput {
            disabled: false.into(),
            within: false,
            auto_focus: false,
            is_text_input: false,
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },
    });
    let (btn_attrs, btn_styles) = btn_props.into_parts();
    let btn_attrs = StoredValue::new(btn_attrs);
    let btn_styles = StoredValue::new(btn_styles);

    view! {
        <Grid gap=em(0.5) attr:style="margin-bottom: 1em;">
            <Row>
                <Col xs=6 attr:style="
                    border: 0.1em solid lightgrey;
                    border-radius: 0.25em;
                    padding: 0.5em;
                ">
                    <strong>"Horizontal"</strong>
                    <RadioGroup attr:style="display: flex; flex-direction: column; gap: 0.2em; width: 100%; margin-top: 0.5em;">
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_x.get() == PlacementX::OuterLeft)
                                set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::OuterLeft)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"OuterLeft"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_x.get() == PlacementX::Left)
                                set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::Left)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Left"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_x.get() == PlacementX::Center)
                                set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::Center)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Center"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_x.get() == PlacementX::Right)
                                set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::Right)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Right"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_x.get() == PlacementX::OuterRight)
                                set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::OuterRight)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"OuterRight"</Label>
                        </FormControl>
                    </RadioGroup>
                </Col>
                <Col xs=6 attr:style="
                    border: 0.1em solid lightgrey;
                    border-radius: 0.25em;
                    padding: 0.5em;
                ">
                    <strong>"Vertical"</strong>
                    <RadioGroup attr:style="display: flex; flex-direction: column; gap: 0.2em; width: 100%; margin-top: 0.5em;">
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_y.get() == PlacementY::Above)
                                set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Above)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Above"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_y.get() == PlacementY::Top)
                                set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Top)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Top"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_y.get() == PlacementY::Center)
                                set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Center)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Center"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_y.get() == PlacementY::Bottom)
                                set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Bottom)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Bottom"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || selected_placement_y.get() == PlacementY::Below)
                                set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Below)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Below"</Label>
                        </FormControl>
                    </RadioGroup>
                </Col>
            </Row>
        </Grid>

        <div style="display: flex; width: 100%; height: 20em; justify-content: center; align-items: center;">
            <div
                {..trigger_attrs.get_value()}
                {..btn_attrs.get_value()}
                {..target_capture}
                style=btn_styles.get_value()
                    .add("display", "inline-flex")
                    .add("border", "0.1em solid green")
                    .add("padding", "0.5em")
                    .add("cursor", "pointer")
                    .add("width", "7em")
                    .add("height", "7em")
                    .add("justify-content", "center")
                    .add("align-items", "center")
            >
                "Press me"
            </div>
        </div>

        <Portal>
            <Show when=move || is_open.get()>
                <div
                    {..overlay_attrs.get_value()}
                    {..overlay_pos_attrs.get_value()}
                    style=overlay_pos_styles.get_value()
                        .add("background-color", "#0009")
                        .add("color", "white")
                        .add("padding", "1em")
                        .add("border-radius", "0.25em")
                >
                    {move || format!("{:?} / {:?}", selected_placement_x.get(), selected_placement_y.get())}
                </div>
            </Show>
        </Portal>
    }
}
