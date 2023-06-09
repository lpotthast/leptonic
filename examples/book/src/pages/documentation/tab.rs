use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageTab(cx: Scope) -> impl IntoView {
    let (test_reactive_label_bool, set_test_reactive_label_bool) = create_signal(cx, false);
    view! { cx,
        <H1>"Tabs"</H1>

        <P>
            <Code inline=true>"<Tabs>"</Code>" allow you to spread out your UI components into multiple pages, where only one page is shown at any given time. "
            "Every "<Code inline=true>"<Tab>"</Code>" inside represents a page with a label to select it. A user can interact with labels to bring the tab associated to it into view."
        </P>

        <Code>
            {indoc!(r#"
                <Tabs mount=Mount::Once>
                    <Tab name="tab-1" label="Tab 1">"Content of tab 1"</Tab>
                    <Tab name="tab-2" label="Tab 2">"Content of tab 2"</Tab>
                    <Tab name="tab-3" label="Tab 3">"Content of tab 3"</Tab>
                </Tabs>
            "#)}
        </Code>

        <Tabs mount=Mount::Once>
            <Tab name="tab-1" label="Tab 1">"Content of tab 1"</Tab>
            <Tab name="tab-2" label="Tab 2">"Content of tab 2"</Tab>
            <Tab name="tab-3" label="Tab 3">"Content of tab 3"</Tab>
        </Tabs>

        <H2>"Reactivity"</H2>

        <P>
            "Labels can be anything implementing Leptos's "<Code inline=true>"IntoView"</Code>" trait and are therefore as reactive as anything else."
        </P>

        <Code>
            {indoc!(r#"
                let (bool, set_bool) = create_signal(cx, false);

                view! {cx,
                    <Tabs mount=Mount::Once>
                        <Tab name="tab-1" label=view! {cx, "State: " {move || bool.get()}}>
                            <Toggle state=bool on_toggle=move |s| set_bool.set(s) />
                        </Tab>
                        <Tab name="tab-2" label="Tab 2">
                            "Content of tab 2"
                        </Tab>
                    </Tabs>
                }
            "#)}
        </Code>

        <Tabs mount=Mount::Once>
            <Tab name="tab-1" label=view! {cx, "State: " {move || test_reactive_label_bool.get()}}>
                <Toggle state=test_reactive_label_bool on_toggle=move |s| set_test_reactive_label_bool.set(s) />
            </Tab>
            <Tab name="tab-2" label="Tab 2">
                "Content of tab 2"
            </Tab>
        </Tabs>

        <H2>"Nesting"</H2>

        <P>
            "Tabs can be nested just as one would expect."
        </P>

        <Code>
            {indoc!(r#"
                <Tabs mount=Mount::Once>
                    <Tab name="outer-1" label="Outer 1">
                        <Tabs>
                            <Tab name="inner-1" label="Inner 1">
                                "This is a nested tab."
                            </Tab>
                            <Tab name="inner-2" label="Inner 2">
                                "This tah is nested as well."
                            </Tab>
                        </Tabs>
                    </Tab>
                    <Tab name="outer-2" label="Outer 2"></Tab>
                </Tabs>
            "#)}
        </Code>

        <Tabs mount=Mount::Once>
            <Tab name="outer-1" label="Outer 1">
                <Tabs>
                    <Tab name="inner-1" label="Inner 1">
                        "This is a nested tab."
                    </Tab>
                    <Tab name="inner-2" label="Inner 2">
                        "This tab is nested as well."
                    </Tab>
                </Tabs>
            </Tab>
            <Tab name="outer-2" label="Outer 2"></Tab>
        </Tabs>

        <H2>"When are tabs rendered?"</H2>

        <P>
            "You might have spotted a particular behavior in the above example. "
            "When switching to the \"Inner 2\" tab, then switching to \"Outer 2\" and back to \"Outer 1\", "
            "we still see \"Inner 2\" and not the default tab \"Inner 1\" again."
        </P>

        <P>
            "This is where the "<Code inline=true>"mount"</Code>" property comes into play. We had it set to "<Code inline=true>"Mount::Once"</Code>" in all of our examples. "
            "There are two variants to choose from:"
        </P>

        <ul>
            <li>
                <Code inline=true>"Mount::Once"</Code>
                <P style="margin-top: 0.5em;">
                    "Tab content is rendered once. Tabs are simply hidden when not shown."
                </P>
            </li>
            <li>
                <Code inline=true>"Mount::WhenShown"</Code>
                <P style="margin-top: 0.5em;">
                    "Tab content is rendered every time a tab is shown. The dom of the tab is unmounted when hidden. "
                    "This means that there is only ever one tab in the final dom, not requiring any hiding-mechanism as in the "<Code inline=true>"Mount::Once"</Code>" case."
                </P>
            </li>
        </ul>

        <Tabs mount=Mount::WhenShown>
            <Tab name="outer-1" label="Outer 1">
                <Tabs>
                    <Tab name="inner-1" label="Inner 1">
                        "This is a nested tab."
                    </Tab>
                    <Tab name="inner-2" label="Inner 2">
                        "This tab is nested as well."
                    </Tab>
                </Tabs>
            </Tab>
            <Tab name="outer-2" label="Outer 2"></Tab>
        </Tabs>

        // <H2>"Default tab"</H2>
    }
}
