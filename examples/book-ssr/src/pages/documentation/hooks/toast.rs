use indoc::indoc;
use leptos::prelude::*;

use super::demos::toast::ToastHooksDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageToastHooks() -> impl IntoView {
    view! {
        <DocPage title="Toast Hooks">
            <p>
                "The toast hooks keep a queue of toasts and give a region and its toasts their roles, names and focus "
                "handling: "<Code inline=true>"use_toast_state"</Code>" (or "<Code inline=true>"ToastQueue::new"</Code>
                ") for the queue, "<Code inline=true>"use_toast_region"</Code>" for the region and "
                <Code inline=true>"use_toast"</Code>" for each toast. See the "
                <Link href=routes::doc::Toast.materialize()>"Toast overview"</Link>" for concept guidance and keyboard "
                "interaction."
            </p>

            <ReactAria hook="useToast"/>

            <Section title="Example">
                <p>
                    "Create the queue where the toasts are added from, e.g. at the root of your app (it is "
                    <Code inline=true>"Copy"</Code>": provide it as a context). Render the region only while there are "
                    "toasts, and a toast per visible toast inside:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{hooks::*, utils::CapturedElement};

                        #[component]
                        fn Notifications(queue: ToastQueue<String>) -> impl IntoView {
                            let element = CapturedElement::new();
                            let UseToastRegionReturn { region_props } =
                                use_toast_region(UseToastRegionInput {
                                    queue,
                                    element,
                                    aria_label: MaybeProp::default(),
                                    aria_labelledby: None,
                                });
                            view! {
                                <div {..region_props.into_attrs()} {..element.attr()}>
                                    <For
                                        each=move || queue.visible_toasts.get()
                                        key=|toast| toast.key.clone()
                                        children=move |toast| view! { <Notification toast queue/> }
                                    />
                                </div>
                            }
                        }

                        #[component]
                        fn Notification(toast: QueuedToast<String>, queue: ToastQueue<String>) -> impl IntoView {
                            let message = toast.content.clone();
                            let input = UseToastInput {
                                queue,
                                toast,
                                aria_label: MaybeProp::default(),
                                aria_labelledby: None,
                                aria_describedby: None,
                            };
                            let UseToastReturn { toast_props, content_props, title_id, close_button, .. } = use_toast(input);
                            let (close_attrs, close_styles) = use_button(close_button).props.into_parts();
                            view! {
                                <div {..toast_props.into_attrs()}>
                                    <div {..content_props.into_attrs()}>
                                        <div id=title_id>{message}</div>
                                    </div>
                                    <button {..close_attrs} style=close_styles>"\u{00d7}"</button>
                                </div>
                            }
                        }

                        let queue = use_toast_state::<String>(None);
                        queue.add("Saved.".to_owned(), ToastOptions::default());

                        view! {
                            <Show when=move || queue.visible_toasts.with(|toasts| !toasts.is_empty())>
                                <Notifications queue/>
                            </Show>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Each \u{201c}Send\u{201d} adds a toast closing after 5 seconds, unless the pointer is over them or "
                    "the focus is in them; the other button adds one that stays until closed. Two toasts show at once. "
                    "Press "<Keys keys="F6"/>" to move the focus into the toasts."
                </p>
                <Demo description="Message toasts of the hooks, with and without a timeout, counting the closed ones" source=include_str!("demos/toast.rs")>
                    <ToastHooksDemo/>
                </Demo>
            </Section>

            <Section title="use_toast_state">
                <p>
                    <Code inline=true>"use_toast_state::<T>(max_visible_toasts)"</Code>" creates a "
                    <Code inline=true>"ToastQueue<T>"</Code>" showing up to "<Code inline=true>"max_visible_toasts"</Code>
                    " toasts at once (default 1). "<Code inline=true>"ToastQueue::new(max_visible_toasts)"</Code>" does the "
                    "same with a default of all toasts. "<Code inline=true>"T"</Code>" is the content of a toast: a message, "
                    "a struct with a title and a description, or anything else that is "<Code inline=true>"Clone + Send + Sync"</Code>"."
                </p>

                <Section title="ToastQueue" id="use-toast-state-queue">
                    <p>"The queue is a "<Code inline=true>"Copy"</Code>" handle: use it from any Leptos component and event handler."</p>
                    <ApiTable kind=ApiKind::Fields of="ToastQueue">
                        <ApiRow name="visible_toasts" ty="Memo<Vec<QueuedToast<T>>>">"The toasts shown: the newest first, up to the maximum."</ApiRow>
                    </ApiTable>
                    <DocTable headers=&["Method", "Description"]>
                        <TableRow>
                            <TableCell><Code inline=true>"add(content: T, options: ToastOptions) -> String"</Code></TableCell>
                            <TableCell>"Adds a toast in front of the others and returns its key."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"close(key: &str)"</Code></TableCell>
                            <TableCell>"Closes a toast and calls its "<Code inline=true>"on_close"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"clear()"</Code></TableCell>
                            <TableCell>"Removes all toasts, without calling their "<Code inline=true>"on_close"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"pause_all()"</Code>", "<Code inline=true>"resume_all()"</Code></TableCell>
                            <TableCell>
                                "Pauses and resumes the timeouts of the visible toasts. The region calls them while it is "
                                "hovered or focused."
                            </TableCell>
                        </TableRow>
                    </DocTable>
                </Section>

                <Section title="ToastOptions" id="use-toast-state-options">
                    <p><Code inline=true>"ToastOptions"</Code>" implements "<Code inline=true>"Default"</Code>"."</p>
                    <ApiTable kind=ApiKind::Fields of="ToastOptions">
                        <ApiRow name="timeout" ty="Option<Duration>" default="None">
                            "Closes the toast after this time, counted while it is shown and not paused. Give it at least "
                            "5 seconds; toasts with actions shouldn\u{2019}t time out. "<Code inline=true>"None"</Code>
                            ": the toast stays until closed."
                        </ApiRow>
                        <ApiRow name="on_close" ty="Option<Callback<()>>" default="None">"Called when the toast closes."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="QueuedToast" id="use-toast-state-queued-toast">
                    <ApiTable kind=ApiKind::Fields of="QueuedToast">
                        <ApiRow name="key" ty="String">"Identifies the toast, e.g. to close it."</ApiRow>
                        <ApiRow name="content" ty="T">"What the toast shows."</ApiRow>
                        <ApiRow name="timeout" ty="Option<Duration>">"Its timeout, from its options."</ApiRow>
                        <ApiRow name="on_close" ty="Option<Callback<()>>">"Its close callback, from its options."</ApiRow>
                        <ApiRow name="timer" ty="Option<ToastTimer>">"The pausable timer closing it, if it has a timeout."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_toast_region">
                <p>
                    <Code inline=true>"use_toast_region"</Code>" makes the element showing the "
                    "toasts a "<Link href=routes::doc::focus::UseLandmark.materialize()>"landmark"</Link>" that "
                    <Keys keys="F6"/>" reaches. It pauses the timeouts while the region is hovered or has the focus, moves "
                    "the focus to the next toast when a focused one closes, and returns the focus to where it came from "
                    "when the last one closes."
                </p>

                <Section title="Input" id="use-toast-region-input">
                    <ApiTable kind=ApiKind::Input of="UseToastRegionInput">
                        <ApiRow name="queue" ty="ToastQueue<T>">"The toasts to show. Required."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The region element; capture it with "<Code inline=true>"{..element.attr()}"</Code>". Required."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the region. Default: \u{201c}1 notification.\u{201d}, \u{201c}2 notifications.\u{201d}, \u{2026}"
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of elements naming the region."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-toast-region-return">
                    <ApiTable kind=ApiKind::Return of="UseToastRegionReturn">
                        <ApiRow name="region_props" ty="UseToastRegionProps">
                            "Spread on the region: "<Code inline=true>"role=\"region\""</Code>", its name, "
                            <Code inline=true>"tabindex=\"-1\""</Code>", "<Code inline=true>"data-leptonic-top-layer"</Code>
                            " (it stays usable while a modal is open) and the hover and focus handlers."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_toast">
                <p>
                    <Code inline=true>"use_toast"</Code>" gives a toast its role and names, starts its timeout "
                    "once it is shown, and returns the input of its close button."
                </p>

                <Section title="Input" id="use-toast-input">
                    <p>"Pass a "<Code inline=true>"UseToastInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>

                    <ApiTable kind=ApiKind::Input of="UseToastInput">
                        <ApiRow name="queue" ty="ToastQueue<T>">"The queue the toast is in; closing removes it from there. Required."</ApiRow>
                        <ApiRow name="toast" ty="QueuedToast<T>">"The toast, from "<Code inline=true>"visible_toasts"</Code>". Required."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the toast instead of its title."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of elements naming the toast. Default: its title."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"Ids of elements describing the toast. Default: its description."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-toast-return">
                    <ApiTable kind=ApiKind::Return of="UseToastReturn">
                        <ApiRow name="toast_props" ty="UseToastProps">
                            "Spread on the toast: "<Code inline=true>"role=\"alertdialog\""</Code>", "
                            <Code inline=true>"aria-modal=\"false\""</Code>", its name and description, "
                            <Code inline=true>"tabindex=\"0\""</Code>"."
                        </ApiRow>
                        <ApiRow name="content_props" ty="UseToastContentProps">
                            "Spread on the element around the title and the description: "<Code inline=true>"role=\"alert\""</Code>
                            " and "<Code inline=true>"aria-atomic"</Code>", so screen readers announce the content when the toast appears."
                        </ApiRow>
                        <ApiRow name="title_id" ty="String">"The id to give the title, which names the toast."</ApiRow>
                        <ApiRow name="description_props" ty="SlotProps">
                            "Spread on the description: its id, referenced by "<Code inline=true>"aria-describedby"</Code>
                            " while it is rendered."
                        </ApiRow>
                        <ApiRow name="close_button" ty="UseButtonInput">
                            "The input for "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>
                            ": a button named \u{201c}Close\u{201d} that closes the toast."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Toast.materialize()>"Toast overview"</Link></li>
                <li><Link href=routes::doc::toast::Atom.materialize()>"Toast Atoms"</Link></li>
                <li><Link href=routes::doc::focus::UseLandmark.materialize()>"use_landmark"</Link></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
