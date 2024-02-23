use indoc::indoc;
use leptonic::components::prelude::*;
use leptonic::hooks::button::UseButtonInput;
use leptonic::hooks::overlay::{PlacementX, PlacementY};
use leptonic::hooks::prelude::*;
use leptonic::utils::aria::{AriaExpanded, AriaHasPopup};
use leptonic::utils::locale::WritingDirection;
use leptos::*;

#[component]
pub fn PageOverlayButton() -> impl IntoView {
    let (selected_placement_x, set_selected_placement_x) = create_signal(PlacementX::Right);
    let (selected_placement_y, set_selected_placement_y) = create_signal(PlacementY::Above);
    
    let trigger_el: NodeRef<html::Div> = create_node_ref();
    let overlay_el: NodeRef<html::Div> = create_node_ref();

    let (overlay_content, set_overlay_content) = create_signal(String::from("overlay"));

    let UseOverlayReturn {
        props: overlay_props,
        id,
        state,
        set_state,
    } = use_overlay(UseOverlayInput {
        disabled: false.into(),
    });

    let UseOverlayTriggerReturn {
        props: trigger_props,
    } = use_overlay_trigger(UseOverlayTriggerInput {
        show: state.into(),
        overlay_id: id,
        overlay_type: AriaHasPopup::Menu,
    });

    let UseOverlayPositionReturn {
        props: overlay_pos_props,
    } = use_overlay_position(UseOverlayPositionInput {
        overlay_ref: overlay_el,
        target_ref: trigger_el,
        placement_y: selected_placement_y.into(),
        placement_x: selected_placement_x.into(),
        writing_direction: WritingDirection::Ltr.into(),
    });

    let UseButtonReturn { props: btn_props } = use_button(UseButtonInput {
        node_ref: trigger_el,
        disabled: false.into(),
        aria_haspopup: AriaHasPopup::default().into(),
        aria_expanded: AriaExpanded::default().into(),

        use_focus_input: UseFocusInput {
            disabled: false.into(),
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },

        use_press_input: UsePressInput {
            disabled: false.into(),
            on_press: Callback::new(move |_e| {
                set_state.set(!state.get_untracked());
            }),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
        },
    });
    view! {
        <H1>"use_overlay"</H1>

        <P>"Create overlays."</P>

        <Code>
            {indoc!(r#"
                ...
            "#)}
        </Code>

        <Select
            options=vec![PlacementX::OuterLeft, PlacementX::Left, PlacementX::Center, PlacementX::Right, PlacementX::OuterRight]
            search_text_provider=move |o| format!("{o:?}")
            render_option=move |o| format!("{o:?}")
            selected=selected_placement_x
            set_selected=move |v| set_selected_placement_x.set(v)
        />

        <Select
            options=vec![PlacementY::Above, PlacementY::Top, PlacementY::Center, PlacementY::Bottom, PlacementY::Below]
            search_text_provider=move |o| format!("{o:?}")
            render_option=move |o| format!("{o:?}")
            selected=selected_placement_y
            set_selected=move |v| set_selected_placement_y.set(v)
        />

        <TextInput get=overlay_content set=set_overlay_content/>

        <div style="display: flex; width: 100%; height: 20em; justify-content: center; align-items: center;">
            <div
                {..trigger_props.attrs}
                {..btn_props.attrs}
                node_ref=trigger_el
                on:keydown=btn_props.on_key_down
                on:click=btn_props.on_click
                on:pointerdown=btn_props.on_pointer_down
                on:focus=btn_props.on_focus
                on:blur=btn_props.on_blur
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
    }
}
