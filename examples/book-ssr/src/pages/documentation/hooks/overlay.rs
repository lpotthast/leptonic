use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::state::overlay::OverlayTriggerState;
use leptonic::utils::aria::AriaHasPopup;
use leptonic::utils::locale::WritingDirection;
use leptos::*;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUseOverlay() -> impl IntoView {
    let (selected_placement_x, set_selected_placement_x) = create_signal(PlacementX::Right);
    let (selected_placement_y, set_selected_placement_y) = create_signal(PlacementY::Above);

    let trigger_el: NodeRef<html::Div> = create_node_ref();
    let overlay_el: NodeRef<html::Div> = create_node_ref();

    let (overlay_content, set_overlay_content) = create_signal(String::from("overlay"));

    let disabled = false;

    let UseOverlayReturn {
        props: overlay_props,
        id,
        state,
        set_state,
    } = use_overlay(UseOverlayInput::builder().disabled(disabled).build());

    let UseButtonReturn { props: btn_props, press_responder } = use_button(
        UseButtonInput::builder()
            .node_ref(trigger_el)
            .disabled(disabled)
            .use_press_input(
                UsePressInput::builder()
                    .disabled(disabled)
                    .build(),
            )
            .use_hover_input(UseHoverInput::builder().disabled(disabled).build())
            .use_focus_input(UseFocusInput::builder().disabled(disabled).build())
            .build(),
    );

    let trigger_state = OverlayTriggerState {
        show: state,
        set_show: set_state,
    };

    let UseOverlayTriggerReturn {
        props: trigger_props,
    } = use_overlay_trigger(
        trigger_state,
        Some(press_responder),
        UseOverlayTriggerInput::builder()
            .overlay_id(id)
            .overlay_type(AriaHasPopup::Menu)
            .build(),
    );

    let UseOverlayPositionReturn {
        props: overlay_pos_props,
    } = use_overlay_position(
        UseOverlayPositionInput::builder()
            .overlay_ref(overlay_el)
            .target_ref(trigger_el)
            .container_ref(Option::<NodeRef<leptos::html::Custom>>::None)
            .placement_y(selected_placement_y)
            .placement_x(selected_placement_x)
            .writing_direction(WritingDirection::Ltr)
            .build(),
    );

    view! {
        <Article>
            <H1 id="use_overlay" class="anchor">
                "use_overlay"
                <AnchorLink href="#use_overlay" description="Direct link to article header"/>
            </H1>

            <P>"Create overlays."</P>

            <Code>
                {indoc!(r#"
                    ...
                "#)}
            </Code>

            <Grid gap=leptonic::Size::Em(0.5) style="margin-bottom: 1em;">
                <Row>
                    <Col xs=6 style="
                        border: 0.1em solid lightgrey;
                        border-radius: 0.25em;
                        padding: 0.5em;
                    ">
                        <RadioGroup style="display: flex; flex-direction: column; gap: 0.2em; width: 100%;">
                            <FormControl style="display: flex; flex-direction: row; align-items: center; width: 100%;">
                                <Radio checked=move || {selected_placement_x.get() == PlacementX::OuterLeft} set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::OuterLeft)} }/>
                                <Label style="margin-left: 0.25em; width: 100%; height: 100%;">"OuterLeft"</Label>
                            </FormControl>
                            <FormControl style="display: flex; flex-direction: row; align-items: center; width: 100%;">
                                <Radio checked=move || {selected_placement_x.get() == PlacementX::Left} set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::Left)} }/>
                                <Label style="margin-left: 0.25em; width: 100%; height: 100%;">"Left"</Label>
                            </FormControl>
                            <FormControl style="display: flex; flex-direction: row; align-items: center; width: 100%;">
                                <Radio checked=move || {selected_placement_x.get() == PlacementX::Center} set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::Center)} }/>
                                <Label style="margin-left: 0.25em; width: 100%; height: 100%;">"Center"</Label>
                            </FormControl>
                            <FormControl style="display: flex; flex-direction: row; align-items: center; width: 100%;">
                                <Radio checked=move || {selected_placement_x.get() == PlacementX::Right} set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::Right)} }/>
                                <Label style="margin-left: 0.25em; width: 100%; height: 100%;">"Right"</Label>
                            </FormControl>
                            <FormControl style="display: flex; flex-direction: row; align-items: center; width: 100%;">
                                <Radio checked=move || {selected_placement_x.get() == PlacementX::OuterRight} set_checked=move |checked| { if checked { set_selected_placement_x.set(PlacementX::OuterRight)} }/>
                                <Label style="margin-left: 0.25em; width: 100%; height: 100%;">"OuterRight"</Label>
                            </FormControl>
                        </RadioGroup>
                    </Col>

                    <Col xs=6 style="
                        border: 0.1em solid lightgrey;
                        border-radius: 0.25em;
                        padding: 0.5em;
                    ">
                        <RadioGroup style="display: flex; flex-direction: column; gap: 0.2em; width: 100%;">
                            <FormControl style="display: flex; flex-direction: row; align-items: center; width: 100%;">
                                <Radio checked=move || {selected_placement_y.get() == PlacementY::Above} set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Above)} }/>
                                <Label style="margin-left: 0.25em; width: 100%; height: 100%;">"Above"</Label>
                            </FormControl>
                            <FormControl style="display: flex; flex-direction: row; align-items: center; width: 100%;">
                                <Radio checked=move || {selected_placement_y.get() == PlacementY::Top} set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Top)} }/>
                                <Label style="margin-left: 0.25em; width: 100%; height: 100%;">"Top"</Label>
                            </FormControl>
                            <FormControl style="display: flex; flex-direction: row; align-items: center; width: 100%;">
                                <Radio checked=move || {selected_placement_y.get() == PlacementY::Center} set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Center)} }/>
                                <Label style="margin-left: 0.25em; width: 100%; height: 100%;">"Center"</Label>
                            </FormControl>
                            <FormControl style="display: flex; flex-direction: row; align-items: center; width: 100%;">
                                <Radio checked=move || {selected_placement_y.get() == PlacementY::Bottom} set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Bottom)} }/>
                                <Label style="margin-left: 0.25em; width: 100%; height: 100%;">"Bottom"</Label>
                            </FormControl>
                            <FormControl style="display: flex; flex-direction: row; align-items: center; width: 100%;">
                                <Radio checked=move || {selected_placement_y.get() == PlacementY::Below} set_checked=move |checked| { if checked { set_selected_placement_y.set(PlacementY::Below)} }/>
                                <Label style="margin-left: 0.25em; width: 100%; height: 100%;">"Below"</Label>
                            </FormControl>
                        </RadioGroup>
                    </Col>
                </Row>
            </Grid>

            <FormControl>
                <Label>"Content"</Label>
                <TextInput get=overlay_content set=set_overlay_content/>
            </FormControl>

            <div style="display: flex; width: 100%; height: 20em; justify-content: center; align-items: center;">
                <div
                    {..trigger_props.attrs}
                    {..btn_props.attrs}
                    {..btn_props.handlers}
                    node_ref=trigger_el
                    style="
                        display: inline-flex;
                        border: 0.1em solid green;
                        padding: 0.5em;
                        cursor: pointer;
                        width: 7em;
                        height: 7em;
                        justify-content: center;
                        align-items: center;
                    "
                >
                    "Press me"
                </div>
            </div>

            <Portal>
                {
                    // TODO: Merge attributes?
                    let overlay_props_attrs = overlay_props.attrs.clone();
                    let overlay_pos_props_attrs = overlay_pos_props.attrs.clone();
                    view! {
                        <Show when=move || state.get()>
                            <style>
                                ".my-overlay {
                                    background-color: #0009;
                                    color: white;
                                    padding: 1em;
                                    border-radius: 0.25em;
                                }"
                            </style>
                            <div
                                {..overlay_props_attrs.clone()}
                                {..overlay_pos_props_attrs.clone()}
                                node_ref=overlay_el
                                class="my-overlay"
                            >
                                { move || overlay_content.get() }
                            </div>
                        </Show>
                    }
                }
            </Portal>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_overlay", link: "#use-overlay" },
            ]
        }/>
    }
}
