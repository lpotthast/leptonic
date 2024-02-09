use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::*;

#[component]
pub fn PageCallback() -> impl IntoView {
    view! {
        <H1>"Callbacks"</H1>

        <Code>
            {indoc!(r#"
                #[prop(into)] set_value: Callback<u32>,
                #[prop(into, optional)] maybe_render: Option<Callback<(Scope, String), View>>

                ...

                set_value.call(42);

                if let Some(render) = maybe_render {
                    let _view = render.call(("foo".to_owned()));
                }
            "#)}
        </Code>

        <P>
            "Easy, isn't it?"
        </P>

        <P>
            "Callbacks can sometimes be tricky in Leptos. When realized with generics, they are easy to use as a component-consumer and fast, "
            "but hard(er) to write as an author and, because of their generic nature, increase WASM binary size."
        </P>

        <P>
            "If the property of a component specifying a callback should be "<Code inline=true>"Option"</Code>"al, the generic approach is of no help, "
            "as Rust will have problems inferring unspecified, and not otherwise explicitly nameable, types "
            "for the properties left out when instantiating such a component."
        </P>

        <P>
            "An additional burden with the generic approach: "<Code inline=true>"Copy"</Code>". "
            "A simple generic "<Code inline=true>"Fn"</Code>" prop can only be moved once inside your component when not specified with an additional "
            <Code inline=true>"Copy"</Code>" bound. "
            "But doing so will limit your users ability to write compiling closures for the callback, "
            "as only closures not taking ownership of non-copyable-values, implicitly making the closure copy, can be used."
        </P>

        <P>
            "If you, as a component author, need to move a callback into more places, you can put it inside a call to "
            <Code inline=true>"store_value"</Code>" to get an easily copyable and therefore moveable value. "
            "Using a generic "<Code inline=true>"Fn"</Code>" in the non-optional case or a "<Code inline=true>"Box<dyn Fn>"</Code>" in "
            "case the prop is optional and always storing it boxed in a "<Code inline=true>"StoredValue"</Code>" is good solution. "
            "Arguments could be made that "
        </P>

        <ul>
            <li>"(a) components are not often recreated, as we always want to reactively updated their content instead"</li>
            <li>"(b) some callbacks aren't called often, as they are only used to propagate user input and users are incredibly slow compared to machines."</li>
        </ul>

        <P>
            "These make both the performance hit of creating a Box<dyn Fn> and calling it later somewhat irrelevant. "
            "The penalty of managing an extra StoredValue through Leptos should not be overlooked though."
        </P>

        <P>
            "Because creating boxed closures manually and storing them in is not ergonomic, "
            "Leptonic provides an easy to use "<Code inline=true>"Callback"</Code>" type, which is defined as"
        </P>

        <Code>
            "pub struct Callback<T: 'static, R: 'static = ()>(leptos::StoredValue<Box<dyn Fn(T) -> R>>);"
        </Code>

        <P>
            "This callback type is both Clone and Copy and can be used at multiple locations in a component "
            "without requiring an explicitly Clone/Copy closure to be provided by the user of the component."
        </P>

        <P>
            "Some Leptonic components use this type for some of their callbacks when the generic approach was not ergonomic. "
            "You can also use the Callback type in your own components props. For example:"
        </P>

        <Code>
            {indoc!(r"
                #[prop(into, optional)] set_value: Option<Callback<String>>
            ")}
        </Code>

        <P>
            "As seen in the definition. Two generic arguments are available. Use a tuple for T if the callback should receive multiple values. "
            "Provide the second type, otherwise defaulting to (), when the callback should return a value. "
        </P>

        <Code>
            {indoc!(r"
                #[prop(into, optional)] render: Option<Callback<(Scope, MyType), View>>
            ")}
        </Code>

        <P>"Create it when instantiating your component using the "<Code inline=true>"create_callback"</Code>" convenience function."</P>

        <Code>
            {indoc!(r"
                view! {
                    <MyComponent set_value=create_callback(move |v| {}) />
                }
            ")}
        </Code>

        <P>
            "Calling a callback in a component is as simple as this:"
        </P>

        <Code>
            {indoc!(r#"
                set_value.call("foo".to_owned())
            "#)}
        </Code>

        <P>"Passing the callback further down to children is no problem."</P>

        <P>"There are two drawbacks though:"</P>
        <ul>
            <li>"Callbacks cannot take (non-static) references, only owned values."</li>
            <li>"If a child requires a callback of slightly changed signature, you have to pay for creating an intermediate callback.."</li>
        </ul>
    }
}
