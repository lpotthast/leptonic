use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;
use leptos_icons::BsIcon;

#[component]
pub fn PageToggle(cx: Scope) -> impl IntoView {
    let (state, set_state) = create_signal(cx, false);

    view! { cx,
        <H1>"Toggle"</H1>

        <P>"A toggle is a representation of a boolean value."</P>

        <Code>
            {indoc!(r#"
                let (state, set_state) = create_signal(cx, false);

                view! {cx,
                    <Toggle state=state set_state=set_state/>
                }
            "#)}
        </Code>

        <Toggle state=state set_state=set_state/>

        <H2>"Icons"</H2>

        <P>"A toggle can be configured with a pair of icons. One icon being rendered in the off position, the other being rendered in the on position."</P>

        <Code>
            {indoc!(r#"
                let (state, set_state) = create_signal(cx, false);

                view! {cx,
                    <Toggle state=state set_state=set_state icons=ToggleIcons {
                        on: BsIcon::BsFolderFill.into(),
                        off: BsIcon::BsFolder.into(),
                    }/>
                }
            "#)}
        </Code>

        <Toggle state=state set_state=set_state icons=ToggleIcons {
            on: BsIcon::BsFolderFill.into(),
            off: BsIcon::BsFolder.into(),
        }/>

        <H2>"Variations"</H2>

        <P>"The toggle comes in two variants: Sliding and Stationary. Sliding toggles are the default and the ones we have used so far."</P>
        <P>"Stationary toggles are not animated and only consist of a single circle."</P>

        <Toggle state=state set_state=set_state variant=ToggleVariant::Stationary icons=ToggleIcons {
            on: BsIcon::BsFolderFill.into(),
            off: BsIcon::BsFolder.into(),
        }/>
    }
}
