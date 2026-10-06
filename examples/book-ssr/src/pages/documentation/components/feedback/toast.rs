use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::toast_creation::ToastCreationDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageToast() -> impl IntoView {
    view! {
        <DocPage title="Toast">
            <p>
                "A toast is a short, non-blocking notification that appears on top of your app and usually disappears "
                "after a few seconds, for example to confirm that something was saved. Leptonic renders toasts in a "
                <Code inline=true>"ToastRoot"</Code>"; you create them by pushing a "<Code inline=true>"Toast"</Code>
                " to the "<Code inline=true>"Toasts"</Code>" context."
            </p>

            <Demo
                description="Form creating toasts with a chosen variant, timeout, header and body"
                source=include_str!("demos/toast_creation.rs")
            >
                <ToastCreationDemo/>
            </Demo>

            <Section title="ToastRoot">
                <p>
                    <Code inline=true>"ToastRoot"</Code>" provides the "<Code inline=true>"Toasts"</Code>
                    " context to its children and renders the toasts after them. leptonic\u{2019}s "
                    <Code inline=true>"Root"</Code>" component already includes it, so you only need it when you "
                    "don\u{2019}t use "<Code inline=true>"Root"</Code>"."
                </p>
            </Section>

            <Section title="Toasts">
                <p>
                    "Get the context with "<Code inline=true>"expect_context::<Toasts>()"</Code>
                    ". It is "<Code inline=true>"Copy"</Code>", so you can move it into event handlers."
                </p>
                <DocTable headers=&["Member", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"push(toast: Toast)"</Code></TableCell>
                        <TableCell>"Shows a toast and schedules its removal according to its timeout."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"try_remove(id: Uuid) -> Option<Toast>"</Code></TableCell>
                        <TableCell>"Removes the toast with the given id, returning it if it was shown."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"clear()"</Code></TableCell>
                        <TableCell>"Removes all toasts."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"toasts: ReadSignal<Vec<Toast>>"</Code></TableCell>
                        <TableCell>"The toasts currently shown."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Toast">
                <ApiTable kind=ApiKind::Input of="Toast">
                    <ApiRow name="id" ty="Uuid">
                        "Identifies the toast, for example to remove it with "<Code inline=true>"try_remove"</Code>"."
                    </ApiRow>
                    <ApiRow name="created_at" ty="time::OffsetDateTime">"When the toast was created."</ApiRow>
                    <ApiRow name="variant" ty="ToastVariant">
                        <Code inline=true>"Success"</Code>", "<Code inline=true>"Info"</Code>" (the default value), "
                        <Code inline=true>"Warn"</Code>" or "
                        <Code inline=true>"Error"</Code>". Rendered as the "<Code inline=true>"data-variant"</Code>" attribute."
                    </ApiRow>
                    <ApiRow name="header" ty="ViewFn">"The header content."</ApiRow>
                    <ApiRow name="body" ty="ViewFn">"The message content."</ApiRow>
                    <ApiRow name="timeout" ty="ToastTimeout">
                        "When the toast disappears, see "<a href="#timeouts">"Timeouts"</a>"."
                    </ApiRow>
                </ApiTable>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let toasts = expect_context::<Toasts>();

                        toasts.push(Toast {
                            id: Uuid::new_v4(),
                            created_at: time::OffsetDateTime::now_utc(),
                            variant: ToastVariant::Success,
                            header: (|| "Saved").into(),
                            body: (|| "Your changes were saved.").into(),
                            timeout: ToastTimeout::DefaultDelay,
                        });
                    "#)}
                </Code>
            </Section>

            <Section title="Timeouts">
                <DocTable headers=&["ToastTimeout", "Behavior"]>
                    <TableRow>
                        <TableCell><Code inline=true>"DefaultDelay"</Code></TableCell>
                        <TableCell>"The toast disappears after 3 seconds."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"CustomDelay(time::Duration)"</Code></TableCell>
                        <TableCell>
                            "The toast disappears after the given duration. For durations above 10 seconds, it also "
                            "shows a close icon."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"None"</Code></TableCell>
                        <TableCell>"The toast stays until the user closes it with its close icon or you remove it."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt toasts to your design:"</p>
                <CssVariables prefix="--toast-" scss=theme_scss!("toasts")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::components::Alert.materialize()>"Alert"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
