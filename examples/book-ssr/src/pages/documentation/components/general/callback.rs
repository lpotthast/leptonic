use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::kit::*;

#[component]
pub fn PageCallback() -> impl IntoView {
    view! {
        <DocPage title="Callbacks">
            <p>
                "leptonic components accept functions through three prop types: Leptos\u{2019} "
                <Code inline=true>"Callback"</Code>" for events, leptonic\u{2019}s "<Code inline=true>"Out"</Code>
                " for values a component hands back to you, and "<Code inline=true>"ViewCallback"</Code>" / "
                <Code inline=true>"ViewProducer"</Code>" for functions that render views. All of them are "
                <Code inline=true>"Copy"</Code>", so a component can use them in as many places as it needs, "
                "and all of them convert from closures, so you rarely name them when you use a component."
            </p>

            <Section title="Callback">
                <p>
                    <Code inline=true>"Callback<In, Out = ()>"</Code>" (from "<Code inline=true>"leptos::prelude"</Code>
                    ") wraps a function taking one argument. Event props such as a button\u{2019}s "
                    <Code inline=true>"on_press"</Code>" use it. Pass a closure; "<Code inline=true>"#[prop(into)]"</Code>
                    " converts it."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        view! {
                            <Button on_press=move |_| set_count.update(|count| *count += 1)>"Increment"</Button>
                        }
                    "#)}
                </Code>

                <p>
                    "In your own components, declare the prop with "<Code inline=true>"#[prop(into)]"</Code>
                    " and call it with "<Code inline=true>"run"</Code>". Use a tuple as input to pass several values."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        #[component]
                        fn Counter(#[prop(into)] on_change: Callback<(u32, u32)>) -> impl IntoView {
                            // ...
                            on_change.run((old, new));
                        }
                    ")}
                </Code>
            </Section>

            <Section title="Out">
                <p>
                    <Code inline=true>"Out<T>"</Code>" (from "<Code inline=true>"leptonic::prelude"</Code>
                    ") is anything a component can write a value to. Props like "<Code inline=true>"set_value"</Code>" ("
                    <Code inline=true>"Slider"</Code>") or "<Code inline=true>"set_selected"</Code>" ("<Code inline=true>"Select"</Code>") use it, so you can pass "
                    "a signal directly instead of wrapping it in a closure."
                </p>

                <DocTable headers=&["You pass", "The component calls"]>
                    <TableRow>
                        <TableCell><Code inline=true>"WriteSignal<T>"</Code>", "<Code inline=true>"RwSignal<T>"</Code></TableCell>
                        <TableCell>"The signal\u{2019}s "<Code inline=true>"set"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"StoredValue<T>"</Code></TableCell>
                        <TableCell>"The stored value\u{2019}s "<Code inline=true>"set_value"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"A closure "<Code inline=true>"Fn(T)"</Code>", or a "<Code inline=true>"Callback<T>"</Code></TableCell>
                        <TableCell>"The function."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"A function pointer, via "<Code inline=true>"Out::new_fn"</Code></TableCell>
                        <TableCell>"The function."</TableCell>
                    </TableRow>
                </DocTable>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let (volume, set_volume) = signal(0.5);
                        view! {
                            <Slider min=0.0 max=1.0 value=volume set_value=set_volume/>
                            <Slider min=0.0 max=1.0 value=volume set_value=move |v| tracing::info!("volume: {v}")/>
                        }
                    "#)}
                </Code>

                <p>
                    "In your own components, declare the prop as "<Code inline=true>"#[prop(into)] set_value: Out<T>"</Code>
                    " and write to it with "<Code inline=true>"set_value.set(value)"</Code>". "
                    <Code inline=true>"Out"</Code>" implements "<Code inline=true>"Default"</Code>" as a no-op, so "
                    <Code inline=true>"#[prop(into, optional)]"</Code>" works too."
                </p>
            </Section>

            <Section title="ViewCallback and ViewProducer">
                <p>
                    <Code inline=true>"ViewCallback<In>"</Code>" is a "<Code inline=true>"Callback<In, AnyView>"</Code>
                    ": it renders a view for an input. The select components use it for "
                    <Code inline=true>"render_option"</Code>". "<Code inline=true>"ViewProducer"</Code>
                    " renders a view without input. Both convert from closures returning anything that implements "
                    <Code inline=true>"IntoView"</Code>"; call them with "<Code inline=true>"render"</Code>" and "
                    <Code inline=true>"produce"</Code>"."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        view! {
                            <Select
                                options=users
                                selected=selected
                                set_selected=set_selected
                                search_text_provider=move |u: User| u.name
                                render_option=move |u: User| view! { <b>{u.name}</b> }
                            />
                        }
                    ")}
                </Code>
            </Section>

            <Section title="Limitations">
                <ul>
                    <li>"Callbacks take owned values, not references."</li>
                    <li>
                        "The functions must be "<Code inline=true>"Send + Sync + 'static"</Code>
                        ", so they can only capture such values, such as signals."
                    </li>
                    <li>
                        "Passing a callback to a child that expects a slightly different signature requires a new "
                        "callback that adapts it."
                    </li>
                </ul>
            </Section>
        </DocPage>
    }
}
