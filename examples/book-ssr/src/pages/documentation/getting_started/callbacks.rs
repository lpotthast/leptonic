use indoc::indoc;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageCallbacks() -> impl IntoView {
    view! {
        <DocPage title="Callbacks">
            <p>
                "Leptonic\u{2019}s atoms accept functions through two prop types: Leptos\u{2019} "
                <Code inline=true>"Callback"</Code>" for events and leptonic\u{2019}s "<Code inline=true>"Out"</Code>
                " for values they hand back to you. For your own Leptos components, leptonic adds "
                <Code inline=true>"ViewCallback"</Code>" / "<Code inline=true>"ViewProducer"</Code>" for functions that "
                "render views. All of them are "
                <Code inline=true>"Copy"</Code>", so a Leptos component can use them in as many places as it needs, "
                "and all of them convert from closures, so you rarely name them when you use one. Hooks take the same "
                "types in their input structs, and bind state to your app with a "
                <AnchorLink href="#valuebinding">"ValueBinding"</AnchorLink>"."
            </p>

            <Section title="Callback">
                <p>
                    <Code inline=true>"Callback<In, Out = ()>"</Code>" (from "<Code inline=true>"leptos::prelude"</Code>
                    ") wraps a function taking one argument. Event props such as a button\u{2019}s "
                    <Code inline=true>"on_press"</Code>" use it, with an event type of leptonic\u{2019}s as the argument ("
                    <Code inline=true>"PressEvent"</Code>"). Pass a closure; "<Code inline=true>"#[prop(into)]"</Code>
                    " converts it."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let (count, set_count) = signal(0);

                        view! {
                            <Button on_press=move |_| set_count.update(|count| *count += 1)>"Increment"</Button>
                        }
                    "#)}
                </Code>

                <p>
                    "In your own Leptos components, declare the prop with "<Code inline=true>"#[prop(into)]"</Code>
                    " (or "<Code inline=true>"#[prop(into, optional)]"</Code>" and an "<Code inline=true>"Option"</Code>
                    ", as leptonic does for every event) and call it with "<Code inline=true>"run"</Code>". To pass "
                    "several values, define a struct for them: unlike a tuple, its fields have names at every call site."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude::Button;
                        use leptos::prelude::*;

                        /// What `Rating` reports when the user picks a rating.
                        #[derive(Debug, Clone, Copy)]
                        pub struct RatingChange {
                            pub old: u8,
                            pub new: u8,
                        }

                        #[component]
                        pub fn Rating(#[prop(into, optional)] on_change: Option<Callback<RatingChange>>) -> impl IntoView {
                            let (rating, set_rating) = signal(0_u8);
                            let rate = move |new: u8| {
                                if let Some(on_change) = on_change {
                                    on_change.run(RatingChange { old: rating.get_untracked(), new });
                                }
                                set_rating.set(new);
                            };
                            view! { <Button on_press=move |_| rate(5)>"Rate 5 stars"</Button> }
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Out">
                <p>
                    <Code inline=true>"Out<O, S = SyncStorage>"</Code>" (from "<Code inline=true>"leptonic::prelude"</Code>
                    ") is anything an atom can write a value to. Setters of state props such as "
                    <Code inline=true>"set_value"</Code>" ("<Code inline=true>"TextField"</Code>") or "
                    <Code inline=true>"set_selected"</Code>" ("<Code inline=true>"CheckboxField"</Code>") use it, so you can pass "
                    "a signal directly instead of wrapping it in a closure:"
                </p>

                <DocTable headers=&["You pass", "Writing calls"]>
                    <TableRow>
                        <TableCell><Code inline=true>"WriteSignal<O>"</Code>", "<Code inline=true>"RwSignal<O>"</Code></TableCell>
                        <TableCell>"The signal\u{2019}s "<Code inline=true>"set"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"StoredValue<O>"</Code></TableCell>
                        <TableCell>"The stored value\u{2019}s "<Code inline=true>"set_value"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"A closure "<Code inline=true>"Fn(O)"</Code>", or a "<Code inline=true>"Callback<O>"</Code></TableCell>
                        <TableCell>"The function."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"A function pointer, via "<Code inline=true>"Out::new_fn"</Code></TableCell>
                        <TableCell>"The function."</TableCell>
                    </TableRow>
                </DocTable>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let notifications = RwSignal::new(true);

                        view! {
                            // The signal receives every new state.
                            <SwitchField is_selected=notifications set_selected=notifications><SwitchButton>"Notifications"</SwitchButton></SwitchField>
                            // A closure can do more with it.
                            <SwitchField
                                is_selected=notifications
                                set_selected=move |on: bool| {
                                    tracing::info!("notifications: {on}");
                                    notifications.set(on);
                                }
                            >
                                <SwitchButton>
                                    "Notifications (logged)"
                                </SwitchButton>
                            </SwitchField>
                        }
                    "#)}
                </Code>

                <Section title="Storage" id="out-storage">
                    <p>
                        "The second type parameter is the storage of the signals an "<Code inline=true>"Out"</Code>
                        " accepts, as in Leptos: "<Code inline=true>"SyncStorage"</Code>" for signals created with "
                        <Code inline=true>"signal"</Code>" or "<Code inline=true>"RwSignal::new"</Code>", whose values are "
                        <Code inline=true>"Send + Sync"</Code>"; "<Code inline=true>"LocalStorage"</Code>" for "
                        <Code inline=true>"signal_local"</Code>" and "<Code inline=true>"RwSignal::new_local"</Code>
                        ". Every leptonic prop uses the default. Use "<Code inline=true>"Out<T, LocalStorage>"</Code>
                        " in your own components for values that can\u{2019}t be sent between threads, such as a "
                        <Code inline=true>"web_sys::MouseEvent"</Code>": callers then pass a closure or a local signal."
                    </p>
                </Section>

                <Section title="In Your Own Components" id="out-own-components">
                    <p>
                        "Declare the prop as "<Code inline=true>"#[prop(into)] set_value: Out<T>"</Code>
                        " and write to it with "<Code inline=true>"set_value.set(value)"</Code>". "
                        <Code inline=true>"Out"</Code>" implements "<Code inline=true>"Default"</Code>" as a no-op, so "
                        <Code inline=true>"#[prop(into, optional)]"</Code>" works too. How leptonic combines "
                        <Code inline=true>"Out"</Code>" setters with value props is described in "
                        <Link href=format!("{}#state-props", routes::doc::Architecture.materialize())>"Hooks & Atoms"</Link>
                        " (\u{201c}State Props\u{201d})."
                    </p>
                </Section>
            </Section>

            <Section title="ViewCallback and ViewProducer">
                <p>
                    <Code inline=true>"ViewCallback<In>"</Code>" is a "<Code inline=true>"Callback<In, AnyView>"</Code>
                    ": it renders a view for an input. "<Code inline=true>"ViewProducer"</Code>" renders a view without "
                    "input. Both convert from closures returning anything that implements "<Code inline=true>"IntoView"</Code>
                    "; call them with "<Code inline=true>"render"</Code>" and "<Code inline=true>"produce"</Code>". Use them "
                    "in your own Leptos components for props that render part of the view, such as each item of a list:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::prelude::ViewCallback;
                        use leptos::prelude::*;

                        #[component]
                        pub fn UserList(
                            #[prop(into)] users: Signal<Vec<String>>,
                            #[prop(into)] render_user: ViewCallback<String>,
                        ) -> impl IntoView {
                            view! {
                                <ul>
                                    <For each=move || users.get() key=|user| user.clone() let:user>
                                        <li>{render_user.render(user)}</li>
                                    </For>
                                </ul>
                            }
                        }

                        view! { <UserList users=users render_user=|user: String| view! { <b>{user}</b> }/> }
                    ")}
                </Code>

                <p>
                    "Leptonic\u{2019}s atoms take children instead: a closure where an atom renders something per item or "
                    "state, e.g. "<Code inline=true>"<DateInput children=|segment| \u{2026}/>"</Code>"."
                </p>
            </Section>

            <Section title="ValueBinding">
                <p>
                    "Hooks own their state (see "<Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>
                    "). To keep that state in your app instead, bind it with a "<Code inline=true>"ValueBinding<T>"</Code>
                    " (from "<Code inline=true>"leptonic::prelude"</Code>"): a "<Code inline=true>"Signal<T>"</Code>" the "
                    "hook reads and a setter the hook calls with every change. It is the hooks\u{2019} counterpart of an "
                    "atom\u{2019}s "<Code inline=true>"x"</Code>" and "<Code inline=true>"set_x"</Code>" props, and of "
                    "Leptos\u{2019} "<Code inline=true>"bind:value"</Code>". Create it from an "<Code inline=true>"RwSignal"</Code>
                    " or a signal pair with "<Code inline=true>".into()"</Code>", or with "
                    <Code inline=true>"ValueBinding::new(signal, callback)"</Code>" for any other storage. State hooks take it "
                    "as their "<Code inline=true>"value"</Code>" (or "<Code inline=true>"selected_key"</Code>", "
                    <Code inline=true>"selection"</Code>", \u{2026}) field, which replaces the "
                    <Code inline=true>"default_*"</Code>" one."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::{hooks::*, prelude::*};

                        let notifications = RwSignal::new(true);

                        let toggle = use_toggle_state(UseToggleStateInput {
                            value: Some(notifications.into()),
                            ..UseToggleStateInput::default()
                        });

                        // `toggle.toggle()` sets `notifications`, and setting `notifications` anywhere else
                        // shows in `toggle.is_selected`.
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
