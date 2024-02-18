use indoc::indoc;
use leptonic::components::prelude::*;
use leptonic::hooks::button::UseButtonInput;
use leptonic::hooks::focus::UseFocusInput;
use leptonic::hooks::prelude::*;
use leptonic::utils::aria::{AriaExpanded, AriaHasPopup};
use leptos::*;
use leptos_use::use_window;

#[component]
pub fn PageUseButton() -> impl IntoView {
    let el: NodeRef<html::Div> = create_node_ref();

    let UseButtonReturn { props } = use_button(UseButtonInput {
        node_ref: el,
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
                if let Some(window) = use_window().as_ref() {
                    let _ = window.alert_with_message("Pressed!");
                }
            }),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
        },
    });

    view! {
        <H1>"use_hover"</H1>

        <P>"Create standardized buttons from arbitrary elements."</P>

        <Code>
            {indoc!(r#"
                let el: NodeRef<html::Div> = create_node_ref();

                let UseButtonReturn { props } = use_button(UseButtonInput {
                    node_ref: el,
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
                            if let Some(window) = use_window().as_ref() {
                                let _ = window.alert_with_message("Pressed!");
                            }
                        }),
                        on_press_up: None,
                        on_press_start: None,
                        on_press_end: None,
                    },
                });

                view! {
                    <div
                        {..props.attrs}
                        node_ref=el
                        on:keydown=props.on_key_down
                        on:click=props.on_click
                        on:pointerdown=props.on_pointer_down
                        on:focus=props.on_focus
                        on:blur=props.on_blur
                        style="
                            display: inline-flex;
                            border: 0.1em solid green;
                            padding: 0.5em 1em;
                            cursor: pointer;
                        "
                    >
                        "Press me"
                    </div>
                }
            "#)}
        </Code>

        <div
            {..props.attrs}
            node_ref=el
            on:keydown=props.on_key_down
            on:click=props.on_click
            on:pointerdown=props.on_pointer_down
            on:focus=props.on_focus
            on:blur=props.on_blur
            style="
                display: inline-flex;
                border: 0.1em solid green;
                padding: 0.5em 1em;
                cursor: pointer;
            "
        >
            "Press me"
        </div>
    }
}
