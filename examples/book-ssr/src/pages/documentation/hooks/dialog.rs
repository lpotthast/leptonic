use indoc::indoc;
use leptos::prelude::*;

use super::demos::modal_alert::AlertDialogDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseDialog() -> impl IntoView {
    view! {
        <DocPage title="use_dialog">
            <p>
                "The "<Code inline=true>"use_dialog"</Code>" hook gives the content of an overlay dialog semantics: its role, "
                "its accessible name and focus when it opens. See the "<Link href=routes::doc::Dialog.materialize()>"Dialog overview"</Link>
                " for concept guidance."
            </p>

            <ReactAriaSource path="dialog/useDialog.ts"/>

            <Section title="Input">
                <ApiTable kind=ApiKind::Input of="UseDialogInput">
                    <ApiRow name="role" ty="DialogRole" default="Dialog">
                        <Code inline=true>"DialogRole::AlertDialog"</Code>" for an urgent message that needs a response."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "Names a dialog without a title element; the title then doesn\u{2019}t name it."
                    </ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">
                        "Ids of the elements naming the dialog, instead of its title element."
                    </ApiRow>
                    <ApiRow name="aria_describedby" ty="Option<String>" default="None">
                        "Ids of the elements describing the dialog. Without them, an alert dialog is described by its "
                        "content element ("<Code inline=true>"content_props"</Code>")."
                    </ApiRow>
                    <ApiRow name="is_entering" ty="Signal<bool>" default="false">
                        "While "<Code inline=true>"true"</Code>" (an entry animation runs), the dialog isn\u{2019}t focused yet."
                    </ApiRow>
                    <ApiRow name="fallback_aria_labelledby" ty="Signal<Option<String>>" default="None">
                        "Names the dialog when it has neither a title, nor "<Code inline=true>"aria_label"</Code>" or "
                        <Code inline=true>"aria_labelledby"</Code>": the "
                        <Link href=routes::doc::dialog::Atom.materialize()>"Dialog"</Link>" atom passes the id of its "
                        <Code inline=true>"DialogTrigger"</Code>"\u{2019}s button."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseDialogReturn">
                    <ApiRow name="dialog_props" ty="UseDialogProps">
                        "Spread "<Code inline=true>"{..dialog_props.into_attrs()}"</Code>" onto the dialog element: its id, "
                        "role, labelling, "<Code inline=true>"tabindex=\"-1\""</Code>", a focus handler and the element "
                        "capture used to focus it."
                    </ApiRow>
                    <ApiRow name="title_props" ty="SlotProps">
                        "Spread onto the title element (a heading): while it is rendered, it names the dialog."
                    </ApiRow>
                    <ApiRow name="content_props" ty="SlotProps">
                        "Spread onto the content element: while it is rendered, it describes an alert dialog."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::dialog::{DialogRole, UseDialogInput, UseDialogReturn, use_dialog};

                        let UseDialogReturn { dialog_props, title_props, content_props } =
                            use_dialog(UseDialogInput {
                                role: DialogRole::AlertDialog,
                                ..UseDialogInput::default()
                            });

                        view! {
                            // Named by the heading, described by the paragraph.
                            <section {..dialog_props.into_attrs()}>
                                <h2 {..title_props.into_attrs()}>"Delete file?"</h2>
                                <p {..content_props.into_attrs()}>"It is deleted permanently."</p>
                                <button>"Delete"</button>
                            </section>
                        }
                    "#)}
                </Code>
                <p>
                    "The hook doesn\u{2019}t open or close the dialog: put it in an overlay, such as a modal built with the "
                    <Link href=routes::doc::modal::Hook.materialize()>"Modal Hooks"</Link>" or a popover built with "
                    <Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link>"."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "An alert dialog in a modal built with the "<Link href=routes::doc::modal::Hook.materialize()>"Modal Hooks"</Link>
                    ". It is named by its title and described by its message, so screen readers read both when it opens. "
                    <Keys keys="Escape"/>" cancels; a click outside does nothing, so the user has to choose."
                </p>

                <Demo description="Alert dialog confirming a deletion, with Cancel and Delete buttons" source=include_str!("demos/modal_alert.rs")>
                    <AlertDialogDemo/>
                </Demo>
            </Section>

            <Section title="Naming">
                <p>
                    "Every dialog needs an accessible name. Spread "<Code inline=true>"title_props"</Code>" onto its heading: "
                    "while the heading is rendered, the dialog\u{2019}s "<Code inline=true>"aria-labelledby"</Code>" points "
                    "to it. A dialog without a visible title gets "<Code inline=true>"aria_label"</Code>" instead; "
                    <Code inline=true>"aria_labelledby"</Code>" points to other elements. In debug builds, a dialog "
                    "without a name logs a warning once it is rendered."
                </p>
                <p>
                    "An alert dialog is also described by the element you spread "<Code inline=true>"content_props"</Code>
                    " onto ("<Code inline=true>"aria-describedby"</Code>"). A regular dialog is described only through "
                    <Code inline=true>"aria_describedby"</Code>"."
                </p>
            </Section>

            <Section title="Focus">
                <p>
                    "When the dialog is rendered, it focuses itself unless focus is already inside it; with "
                    <Code inline=true>"is_entering"</Code>", it waits until the entry animation finished. Half a second later, "
                    "if the dialog still has focus, it is blurred and focused again, so that VoiceOver on iOS announces it."
                </p>
                <p>
                    "Keeping focus inside and returning it when the dialog closes is the job of the overlay around it: a "
                    <Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link>" with "
                    <Code inline=true>"contain=true"</Code>" and "<Code inline=true>"restore_focus=true"</Code>
                    ". A dialog inside a non-modal "<Link href=routes::doc::popover::Atom.materialize()>"Popover"</Link>
                    " atom makes the popover contain focus."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Dialog.materialize()>"Dialog overview"</Link></li>
                <li><Link href=routes::doc::dialog::Atom.materialize()>"Dialog Atoms"</Link></li>
                <li><Link href=routes::doc::modal::Hook.materialize()>"Modal Hooks"</Link></li>
                <li><Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link></li>
                <li><Link href=routes::doc::focus::FocusScope.materialize()>"FocusScope"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
