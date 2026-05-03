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
                                checked=Signal::derive(move || placement_x.get() == PlacementX::OuterLeft)
                                set_checked=move |checked| { if checked { set_placement_x.set(PlacementX::OuterLeft)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"OuterLeft"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_x.get() == PlacementX::Left)
                                set_checked=move |checked| { if checked { set_placement_x.set(PlacementX::Left)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Left"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_x.get() == PlacementX::Center)
                                set_checked=move |checked| { if checked { set_placement_x.set(PlacementX::Center)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Center"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_x.get() == PlacementX::Right)
                                set_checked=move |checked| { if checked { set_placement_x.set(PlacementX::Right)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Right"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_x.get() == PlacementX::OuterRight)
                                set_checked=move |checked| { if checked { set_placement_x.set(PlacementX::OuterRight)} }
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
                                checked=Signal::derive(move || placement_y.get() == PlacementY::Above)
                                set_checked=move |checked| { if checked { set_placement_y.set(PlacementY::Above)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Above"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_y.get() == PlacementY::Top)
                                set_checked=move |checked| { if checked { set_placement_y.set(PlacementY::Top)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Top"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_y.get() == PlacementY::Center)
                                set_checked=move |checked| { if checked { set_placement_y.set(PlacementY::Center)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Center"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_y.get() == PlacementY::Bottom)
                                set_checked=move |checked| { if checked { set_placement_y.set(PlacementY::Bottom)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Bottom"</Label>
                        </FormControl>
                        <FormControl attr:style="display: flex; flex-direction: row; align-items: center;">
                            <Radio
                                checked=Signal::derive(move || placement_y.get() == PlacementY::Below)
                                set_checked=move |checked| { if checked { set_placement_y.set(PlacementY::Below)} }
                            />
                            <Label attr:style="margin-left: 0.25em;">"Below"</Label>
                        </FormControl>
                    </RadioGroup>
                </Col>
            </Row>
        </Grid>

        <div style="display: flex; justify-content: center; padding: 4em;">
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
                <div
                    {..underlay_props.get_value()}
                    style="position: fixed; inset: 0; z-index: 999;"
                />
                <div
                    {..popover_props.get_value()}
                    style=popover_styles.get_value()
                        .add("background", "white")
                        .add("border", "1px solid #ccc")
                        .add("border-radius", "8px")
                        .add("padding", "1em")
                        .add("box-shadow", "0 4px 12px rgba(0,0,0,0.15)")
                        .add("z-index", "1000")
                >
                    <p style="margin: 0; color: #666;">
                        {move || format!("Placement: {:?} / {:?}", placement_x.get(), placement_y.get())}
                    </p>
                </div>
            </Show>
        </Portal>
    }
}
