use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::*;
use leptos_use::use_window;

use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;

#[component]
pub fn PageUseButton() -> impl IntoView {
    let el: NodeRef<html::Div> = create_node_ref();
    let disabled = false;

    let btn = use_button(
        UseButtonInput::builder()
            .node_ref(el)
            .disabled(disabled)
            .use_press_input(
                UsePressInput::builder()
                    .disabled(disabled)
                    .on_press(Callback::new(move |_e| {
                        if let Some(window) = use_window().as_ref() {
                            let _ = window.alert_with_message("Pressed!");
                        }
                    }))
                    .build(),
            )
            .use_hover_input(UseHoverInput::builder().disabled(disabled).build())
            .use_focus_input(UseFocusInput::builder().disabled(disabled).build())
            .build(),
    );

    view! {
        <Article>
            <H1 id="use_button" class="anchor">
                "use_button"
                <AnchorLink href="#use_button" description="Direct link to article header"/>
            </H1>

            <P>"Create standardized buttons from arbitrary elements."</P>

            <Code>
                {indoc!(r#"
                    let el: NodeRef<html::Div> = create_node_ref();
                    let disabled = false;

                    let btn = use_button(
                        UseButtonInput::builder()
                            .node_ref(el)
                            .disabled(disabled)
                            .use_press_input(
                                UsePressInput::builder()
                                    .disabled(disabled)
                                    .on_press(Callback::new(move |_e| {
                                        if let Some(window) = use_window().as_ref() {
                                            let _ = window.alert_with_message("Pressed!");
                                        }
                                    }))
                                    .build(),
                            )
                            .use_hover_input(UseHoverInput::builder().disabled(disabled).build())
                            .use_focus_input(UseFocusInput::builder().disabled(disabled).build())
                            .build(),
                    );

                    view! {
                        <div
                            {..btn.props.attrs}
                            {..btn.props.handlers}
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
                {..btn.props.attrs}
                {..btn.props.handlers}
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
