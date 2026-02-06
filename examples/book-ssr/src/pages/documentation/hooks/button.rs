use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::utils::aria::{AriaExpanded, AriaHasPopup};
use leptos::html;
use leptos::prelude::*;
use leptos_use::use_window;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUseButton() -> impl IntoView {
    let el: NodeRef<html::Div> = NodeRef::new();

    let UseButtonReturn {
        props,
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
            allow_propagation: false,
            on_press: Callback::new(move |_e| {
                if let Some(window) = use_window().as_ref() {
                    let _ = window.alert_with_message("Pressed!");
                }
            }),
            on_press_up: None,
            on_press_start: None,
            on_press_end: None,
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
            on_focus: None,
            on_blur: None,
            on_focus_change: None,
        },
    });
    let attrs = props.into_attrs();

    view! {
        <Article>
            <h1 id="use_button" class="anchor">
                "use_button"
                <AnchorLink href="#use_button" description="Direct link to article header"/>
            </h1>

            <p>"Create standardized buttons from arbitrary elements."</p>

            <Code>
                {indoc!(r#"
                    let el: NodeRef<html::Div> = NodeRef::new();

                    let UseButtonReturn { props, is_hovered, is_pressed, is_focus_visible } = use_button(UseButtonInput {
                        disabled: false.into(),
                        aria_haspopup: AriaHasPopup::default().into(),
                        aria_expanded: AriaExpanded::default().into(),
                        use_press_input: UsePressInput {
                            disabled: false.into(),
                            force_prevent_default: false,
                            allow_propagation: false,
                            on_press: Callback::new(move |_e| {
                                if let Some(window) = use_window().as_ref() {
                                    let _ = window.alert_with_message("Pressed!");
                                }
                            }),
                            on_press_up: None,
                            on_press_start: None,
                            on_press_end: None,
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
                        },
                    });

                    view! {
                        <div
                            {..props.into_attrs()}
                            node_ref=el
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
                {..attrs}
                node_ref=el
                style="
                    display: inline-flex;
                    border: 0.1em solid green;
                    padding: 0.5em 1em;
                    cursor: pointer;
                "
            >
                "Press me"
            </div>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_button", link: "#use-button" },
            ]
        }/>
    }
}
