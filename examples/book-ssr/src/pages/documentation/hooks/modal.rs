use indoc::indoc;
use leptonic::{atoms::focus_scope::FocusScope, components::prelude::*, hooks::*, prelude::Size};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, doc_styles::*, toc::Toc};

#[component]
pub fn PageUseModalHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="modal" class="anchor">
                "Modal & Dialog Hooks"
                <AnchorLink href="#modal" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible modal dialogs with proper focus management, dismiss handling, and ARIA attributes."</p>

            <h2 id="hook-composition" class="anchor">
                "Hook Composition"
                <AnchorLink href="#hook-composition" description="Direct link to hook composition"/>
            </h2>

            <p>"The modal system is built from composable layers, each handling a specific concern:"</p>

            <ol>
                <li>
                    <strong>"State Layer"</strong>
                    " - "
                    <code>"use_modal_state"</code>
                    " or "
                    <code>"use_dialog_state"</code>
                    ": Manages open/close state and optional confirmation tracking"
                </li>
                <li>
                    <strong>"Backdrop Layer"</strong>
                    " - "
                    <code>"use_modal_backdrop"</code>
                    ": Dismiss behavior (Escape key, outside click via use_overlay) and scroll prevention"
                </li>
                <li>
                    <strong>"Aria-modal Layer"</strong>
                    " - "
                    <code>"use_modal"</code>
                    ": Sets aria-modal=\"true\" for assistive technology"
                </li>
                <li>
                    <strong>"Dialog Layer"</strong>
                    " - "
                    <code>"use_dialog"</code>
                    " (optional): ARIA labeling (aria-labelledby/describedby), Dialog vs AlertDialog role, focus on mount"
                </li>
                <li>
                    <strong>"Focus Layer"</strong>
                    " - "
                    <code>"FocusScope"</code>
                    " atom: Focus trapping and restoration"
                </li>
            </ol>

            <p>"For complete modal functionality, combine the layers you need. A basic modal uses "
               <code>"use_modal_state"</code>", "
               <code>"use_modal_backdrop"</code>", "
               <code>"use_modal"</code>", and "
               <code>"FocusScope"</code>". Add "
               <code>"use_dialog"</code>" when you need title/description ARIA associations or focus-on-mount behavior."
            </p>

            <h2 id="when-to-use" class="anchor">
                "When to Use"
                <AnchorLink href="#when-to-use" description="Direct link to when to use"/>
            </h2>

            <p>"Choose the right hook combination based on your use case:"</p>

            <table style="width: 100%; border-collapse: collapse;">
                <thead>
                    <tr style="border-bottom: 2px solid var(--brand-color);">
                        <th style="text-align: left; padding: 0.5em;">"Use Case"</th>
                        <th style="text-align: left; padding: 0.5em;">"Hook(s)"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Modal dialog (title, description, focus trap)"</td>
                        <td style="padding: 0.5em;">
                            <code>"use_modal_state"</code>" + "
                            <code>"use_modal_backdrop"</code>" + "
                            <code>"use_modal"</code>" + "
                            <code>"use_dialog"</code>" + "
                            <code>"FocusScope"</code>
                        </td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Alert dialog (requires acknowledgment)"</td>
                        <td style="padding: 0.5em;">
                            "Same as above with "
                            <code>"DialogRole::AlertDialog"</code>" + non-dismissable"
                        </td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Confirmation dialog (confirm/cancel tracking)"</td>
                        <td style="padding: 0.5em;">
                            <code>"use_dialog_state"</code>" instead of "
                            <code>"use_modal_state"</code>", rest same"
                        </td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Simple modal (no ARIA title/description)"</td>
                        <td style="padding: 0.5em;">
                            <code>"use_modal_state"</code>" + "
                            <code>"use_modal_backdrop"</code>" + "
                            <code>"use_modal"</code>" + "
                            <code>"FocusScope"</code>
                        </td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Popover positioned relative to trigger"</td>
                        <td style="padding: 0.5em;">
                            <code>"use_popover"</code>" (not modal hooks)"
                        </td>
                    </tr>
                    <tr style="border-bottom: 1px solid #ccc;">
                        <td style="padding: 0.5em;">"Non-modal overlay"</td>
                        <td style="padding: 0.5em;">
                            <code>"use_overlay"</code>" directly"
                        </td>
                    </tr>
                </tbody>
            </table>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <p>"A complete modal with scroll prevention, backdrop dismiss, Escape key handling, and focus trapping:"</p>

            <BasicModalDemo />

            <h2 id="alert-dialog-demo" class="anchor">
                "Alert Dialog Demo"
                <AnchorLink href="#alert-dialog-demo" description="Direct link to alert dialog demo"/>
            </h2>

            <p>"An alert dialog is used for important messages that require user acknowledgment. "
               "It uses "<code>"role=\"alertdialog\""</code>" and typically cannot be dismissed with Escape:"</p>

            <AlertDialogDemo />

            <h2 id="non-dismissable-demo" class="anchor">
                "Non-Dismissable Modal Demo"
                <AnchorLink href="#non-dismissable-demo" description="Direct link to non-dismissable demo"/>
            </h2>

            <p>"A modal that cannot be dismissed by clicking outside or pressing Escape. "
               "Users must complete an action (like filling a form) to close it:"</p>

            <NonDismissableModalDemo />

            <h2 id="confirmation-demo" class="anchor">
                "Confirmation Dialog Demo"
                <AnchorLink href="#confirmation-demo" description="Direct link to confirmation demo"/>
            </h2>

            <p>"A confirmation dialog using "<code>"use_dialog_state"</code>" to track whether the user confirmed or cancelled. "
               "Watch the status text below to see the result:"</p>

            <ConfirmationDialogDemo />

            <h2 id="api" class="anchor">
                "API"
                <AnchorLink href="#api" description="Direct link to API"/>
            </h2>

            <h3 id="use_modal_state" class="anchor">
                "use_modal_state"
                <AnchorLink href="#use_modal_state" description="Direct link to use_modal_state"/>
            </h3>

            <Code>
                {indoc!(r"
                    let UseModalStateReturn { is_open, set_open, open, close, toggle } =
                        use_modal_state(UseModalStateInput::default());

                    // set_open.run(true) - opens the modal
                    // set_open.run(false) - closes the modal
                    // open.run(()) - opens the modal
                    // close.run(()) - closes the modal
                    // toggle.run(()) - toggles the modal
                ")}
            </Code>

            <h3 id="use_modal_state-input" class="anchor">
                "Input"
                <AnchorLink href="#use_modal_state-input" description="Direct link to input"/>
            </h3>

            <p><code>"UseModalStateInput"</code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"default_open"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Initial open state."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_open_change"</code></TableCell>
                            <TableCell><code>"Option<Callback<bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Called whenever the open state changes."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use_modal_state-return" class="anchor">
                "Return Value"
                <AnchorLink href="#use_modal_state-return" description="Direct link to return value"/>
            </h3>

            <p><code>"UseModalStateReturn"</code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"is_open"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"Whether the modal is open."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"set_open"</code></TableCell>
                            <TableCell><code>"Callback<bool>"</code></TableCell>
                            <TableCell>"Set the open state directly."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"open"</code></TableCell>
                            <TableCell><code>"Callback<()>"</code></TableCell>
                            <TableCell>"Open the modal."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"close"</code></TableCell>
                            <TableCell><code>"Callback<()>"</code></TableCell>
                            <TableCell>"Close the modal."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"toggle"</code></TableCell>
                            <TableCell><code>"Callback<()>"</code></TableCell>
                            <TableCell>"Toggle the modal."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use_modal_backdrop" class="anchor">
                "use_modal_backdrop"
                <AnchorLink href="#use_modal_backdrop" description="Direct link to use_modal_backdrop"/>
            </h3>

            <Code>
                {indoc!(r#"
                    let UseModalBackdropReturn { modal_props, backdrop_props, id: _ } =
                        use_modal_backdrop(UseModalBackdropInput {
                            is_open: state.is_open,
                            on_close: state.close,
                            is_dismissable: true,
                            is_keyboard_dismiss_disabled: false,
                            should_close_on_interact_outside: None,
                        });

                    view! {
                        <Show when=move || is_open.get()>
                            // Backdrop - spread backdrop_props (pointerdown Firefox fix)
                            <div {..backdrop_props.into_attrs()} class="backdrop">
                                <FocusScope contain=true restore_focus=true auto_focus=true>
                                    // Modal content - spread modal_props (overlay behavior)
                                    <div {..modal_props.into_attrs()} class="modal">
                                        "Modal content"
                                    </div>
                                </FocusScope>
                            </div>
                        </Show>
                    }
                "#)}
            </Code>

            <p>"Handles (via "<code>"use_overlay"</code>" internally):"</p>
            <ul>
                <li>"Escape key dismissal (topmost overlay only, respects is_composing)"</li>
                <li>"Click outside detection (overlay stacking aware)"</li>
                <li>"Scroll prevention on body"</li>
            </ul>

            <h3 id="use_modal_backdrop-input" class="anchor">
                "Input"
                <AnchorLink href="#use_modal_backdrop-input" description="Direct link to input"/>
            </h3>

            <p><code>"UseModalBackdropInput"</code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"is_open"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether the modal is open."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_close"</code></TableCell>
                            <TableCell><code>"Callback<()>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Called when the modal should close."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_dismissable"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether clicking outside closes the modal."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_keyboard_dismiss_disabled"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Whether pressing Escape to close is disabled."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"should_close_on_interact_outside"</code></TableCell>
                            <TableCell><code>"Option<Callback<Element, bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Filter for which outside interactions should close the modal."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use_modal_backdrop-return" class="anchor">
                "Return Value"
                <AnchorLink href="#use_modal_backdrop-return" description="Direct link to return value"/>
            </h3>

            <p><code>"UseModalBackdropReturn"</code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"modal_props"</code></TableCell>
                            <TableCell><code>"UseModalBackdropModalProps"</code></TableCell>
                            <TableCell>"Props for the modal content element. Includes overlay id, element capture, keydown (Escape), focus events."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"backdrop_props"</code></TableCell>
                            <TableCell><code>"UseModalBackdropProps"</code></TableCell>
                            <TableCell>"Props for the backdrop element. Includes pointerdown handler (Firefox text-selection fix)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"id"</code></TableCell>
                            <TableCell><code>"Oco<'static, str>"</code></TableCell>
                            <TableCell>"Unique overlay ID for ARIA."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use_modal" class="anchor">
                "use_modal"
                <AnchorLink href="#use_modal" description="Direct link to use_modal"/>
            </h3>

            <Code>
                {indoc!(r#"
                    let UseModalReturn { modal_props } = use_modal(UseModalInput {
                        is_disabled: false,
                    });

                    view! {
                        <div {..modal_props.into_attrs()}>
                            "Modal content (aria-modal='true')"
                        </div>
                    }
                "#)}
            </Code>

            <p>"Sets "<code>"aria-modal=\"true\""</code>" on the element so screen readers treat outside content as hidden."</p>

            <h3 id="use_modal-input" class="anchor">
                "Input"
                <AnchorLink href="#use_modal-input" description="Direct link to input"/>
            </h3>

            <p><code>"UseModalInput"</code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"is_disabled"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"When true, aria-modal is not set."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use_modal-return" class="anchor">
                "Return Value"
                <AnchorLink href="#use_modal-return" description="Direct link to return value"/>
            </h3>

            <p><code>"UseModalReturn"</code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"modal_props"</code></TableCell>
                            <TableCell><code>"UseModalProps"</code></TableCell>
                            <TableCell>"Spread onto the modal element via " <code>"modal_props.into_attrs()"</code> ". Sets aria-modal=\"true\"."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use_dialog" class="anchor">
                "use_dialog"
                <AnchorLink href="#use_dialog" description="Direct link to use_dialog"/>
            </h3>

            <Code>
                {indoc!(r#"
                    let UseDialogReturn { dialog_props, title_props, description_props, dialog_id } =
                        use_dialog(UseDialogInput {
                            title: Some("Title".to_string()),
                            description: Some("Description".to_string()),
                            aria_label: None,
                            role: DialogRole::Dialog, // or DialogRole::AlertDialog
                        });

                    // dialog_props includes an ElementCaptureAttr that automatically captures
                    // the DOM element for focus-on-mount — no manual NodeRef wiring needed.
                    view! {
                        <div {..dialog_props.into_attrs()}>
                            <h2 id=title_props.id>"Title"</h2>
                            <p id=description_props.id>"Description"</p>
                        </div>
                    }
                "#)}
            </Code>

            <p>"Provides:"</p>
            <ul>
                <li>"role=\"dialog\" or role=\"alertdialog\""</li>
                <li>"aria-labelledby (links to title)"</li>
                <li>"aria-describedby (links to description)"</li>
                <li>"Focus on mount (focuses the dialog unless a child already has focus)"</li>
                <li>"iOS Safari VoiceOver workaround (blur/refocus after 500ms)"</li>
            </ul>

            <p>"For dismiss behavior (Escape key, outside click), use "<code>"use_modal_backdrop"</code>". "
               "For aria-modal, use "<code>"use_modal"</code>"."</p>

            <h3 id="use_dialog-input" class="anchor">
                "Input"
                <AnchorLink href="#use_dialog-input" description="Direct link to input"/>
            </h3>

            <p><code>"UseDialogInput"</code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"title"</code></TableCell>
                            <TableCell><code>"Option<String>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Title for aria-labelledby. A unique ID is generated for the title element."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"description"</code></TableCell>
                            <TableCell><code>"Option<String>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Description for aria-describedby. A unique ID is generated for the description element."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"aria_label"</code></TableCell>
                            <TableCell><code>"Option<String>"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"When set, suppresses aria-labelledby. For dialogs without a visible title."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"role"</code></TableCell>
                            <TableCell><code>"DialogRole"</code></TableCell>
                            <TableCell>"-"</TableCell>
                            <TableCell>"Dialog or AlertDialog."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use_dialog-return" class="anchor">
                "Return Value"
                <AnchorLink href="#use_dialog-return" description="Direct link to return value"/>
            </h3>

            <p><code>"UseDialogReturn"</code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"dialog_props"</code></TableCell>
                            <TableCell><code>"UseDialogProps"</code></TableCell>
                            <TableCell>"Spread onto the dialog element. Includes role, aria-labelledby, aria-describedby, tabindex, blur handler, and element capture."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"title_props"</code></TableCell>
                            <TableCell><code>"UseDialogTitleProps"</code></TableCell>
                            <TableCell>"Contains the generated " <code>"id"</code> " for the title element."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"description_props"</code></TableCell>
                            <TableCell><code>"UseDialogDescriptionProps"</code></TableCell>
                            <TableCell>"Contains the generated " <code>"id"</code> " for the description element."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"dialog_id"</code></TableCell>
                            <TableCell><code>"String"</code></TableCell>
                            <TableCell>"Unique dialog ID."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use_dialog_state" class="anchor">
                "use_dialog_state"
                <AnchorLink href="#use_dialog_state" description="Direct link to use_dialog_state"/>
            </h3>

            <Code>
                {indoc!(r"
                    let UseDialogStateReturn {
                        is_open, set_open, open, close, toggle, confirm, is_confirmed,
                    } = use_dialog_state(UseDialogStateInput::default());

                    // set_open.run(true) - opens the dialog
                    // set_open.run(false) - closes the dialog
                    // open.run(()) - opens the dialog (resets is_confirmed)
                    // close.run(()) - closes the dialog
                    // toggle.run(()) - toggles the dialog
                    // confirm.run(()) - confirms and closes the dialog
                    // Check is_confirmed.get() to determine the result after close
                ")}
            </Code>

            <h3 id="use_dialog_state-input" class="anchor">
                "Input"
                <AnchorLink href="#use_dialog_state-input" description="Direct link to input"/>
            </h3>

            <p><code>"UseDialogStateInput"</code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Default"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"default_open"</code></TableCell>
                            <TableCell><code>"bool"</code></TableCell>
                            <TableCell><code>"false"</code></TableCell>
                            <TableCell>"Initial open state."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"on_open_change"</code></TableCell>
                            <TableCell><code>"Option<Callback<bool>>"</code></TableCell>
                            <TableCell><code>"None"</code></TableCell>
                            <TableCell>"Called whenever the open state changes."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="use_dialog_state-return" class="anchor">
                "Return Value"
                <AnchorLink href="#use_dialog_state-return" description="Direct link to return value"/>
            </h3>

            <p><code>"UseDialogStateReturn"</code>" fields:"</p>

            <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"Field"</TableHeaderCell>
                            <TableHeaderCell min_width=true>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Description"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell><code>"is_open"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"Whether the dialog is open."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"set_open"</code></TableCell>
                            <TableCell><code>"Callback<bool>"</code></TableCell>
                            <TableCell>"Set the open state directly."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"open"</code></TableCell>
                            <TableCell><code>"Callback<()>"</code></TableCell>
                            <TableCell>"Open the dialog (resets is_confirmed)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"close"</code></TableCell>
                            <TableCell><code>"Callback<()>"</code></TableCell>
                            <TableCell>"Close the dialog."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"toggle"</code></TableCell>
                            <TableCell><code>"Callback<()>"</code></TableCell>
                            <TableCell>"Toggle the dialog."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"confirm"</code></TableCell>
                            <TableCell><code>"Callback<()>"</code></TableCell>
                            <TableCell>"Confirm and close the dialog."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><code>"is_confirmed"</code></TableCell>
                            <TableCell><code>"Signal<bool>"</code></TableCell>
                            <TableCell>"Whether the dialog was confirmed (vs cancelled)."</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            </TableContainer>

            <h3 id="dialog-roles" class="anchor">
                "Dialog Roles"
                <AnchorLink href="#dialog-roles" description="Direct link to dialog roles"/>
            </h3>

            <ul>
                <li><code>"DialogRole::Dialog"</code> " - Standard dialog for user interaction"</li>
                <li><code>"DialogRole::AlertDialog"</code> " - Important message requiring user response"</li>
            </ul>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to accessibility"/>
            </h2>

            <ul>
                <li><strong><code>"aria-modal=\"true\""</code></strong>
                    " — Set by "<code>"use_modal"</code>" so screen readers treat outside content as hidden."
                </li>
                <li><strong><code>"role=\"dialog\""</code>" / "<code>"role=\"alertdialog\""</code></strong>
                    " — Set by "<code>"use_dialog"</code>" based on "<code>"DialogRole"</code>"."
                </li>
                <li><strong><code>"aria-labelledby"</code>" / "<code>"aria-describedby"</code></strong>
                    " — Automatic ARIA associations via "<code>"use_dialog"</code>" title and description props."
                </li>
                <li><strong>"Keyboard dismiss"</strong>
                    " — Escape closes the topmost modal only (respects overlay stacking)."
                </li>
                <li><strong>"Focus on mount"</strong>
                    " — "<code>"use_dialog"</code>" focuses the dialog element unless a child already has focus. "
                    "Includes an iOS Safari VoiceOver workaround (blur/refocus after 500ms)."
                </li>
                <li><strong>"Focus trapping and restoration"</strong>
                    " — Via the "<code>"FocusScope"</code>" atom with "<code>"contain=true"</code>" and "<code>"restore_focus=true"</code>"."
                </li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Escape key dismissal with overlay stacking (via use_modal_backdrop → use_overlay)"</li>
                <li>"Click outside to close (via use_modal_backdrop → use_overlay)"</li>
                <li>"Proper ARIA roles and associations (via use_dialog)"</li>
                <li>"aria-modal attribute (via use_modal)"</li>
                <li>"Focus on mount with iOS Safari VoiceOver workaround (via use_dialog)"</li>
                <li>"Focus trapping (when combined with FocusScope)"</li>
                <li>"Scroll prevention on body (via use_modal_backdrop)"</li>
                <li>"Dismissable and non-dismissable modes"</li>
            </ul>

            <h2 id="related-hooks" class="anchor">
                "Related Hooks"
                <AnchorLink href="#related-hooks" description="Direct link to related hooks"/>
            </h2>

            <ul>
                <li><code>"use_overlay"</code>" — Internal dismiss/stacking logic used by "<code>"use_modal_backdrop"</code>"."</li>
                <li><code>"use_popover"</code>" — Use instead when the overlay should be anchored to a trigger element."</li>
                <li><code>"use_prevent_scroll"</code>" — Internal scroll prevention used by "<code>"use_modal_backdrop"</code>"."</li>
                <li><code>"use_interact_outside"</code>" — Internal outside-click detection used by "<code>"use_overlay"</code>"."</li>
                <li><code>"FocusScope"</code>" (atom) — Focus trapping and restoration, compose with modal hooks."</li>
            </ul>

            <h2 id="deviations" class="anchor">
                "React Aria Deviations"
                <AnchorLink href="#deviations" description="Direct link to deviations"/>
            </h2>

            <h3>"Omitted Features"</h3>

            <ul>
                <li><code>"ariaHideOutside()"</code>" — Relies on "<code>"aria-modal=\"true\""</code>" instead of setting "<code>"aria-hidden"</code>" on all sibling DOM trees."</li>
                <li><code>"useOverlayFocusContain"</code>" — Focus containment is handled by the "<code>"FocusScope"</code>" atom at the consumer level."</li>
                <li><code>"ModalProvider"</code>" / "<code>"ModalContext"</code>" — Not needed without "<code>"ariaHideOutside()"</code>"."</li>
                <li><code>"useControlledState"</code>" — Leptos "<code>"Signal<T>"</code>" is Copy, making the controlled/uncontrolled pattern unnecessary. Hooks always own their WriteSignal internally."</li>
            </ul>

            <h3>"API Differences"</h3>

            <ul>
                <li><code>"useModalOverlay"</code>" + "<code>"underlayProps"</code>" → "<code>"use_modal_backdrop"</code>" + "<code>"backdrop_props"</code>"."</li>
                <li><code>"aria-modal"</code>" is set by "<code>"use_modal"</code>", not "<code>"use_dialog"</code>" (React Aria avoids "<code>"aria-modal"</code>" due to a Safari iframe bug)."</li>
                <li><code>"use_modal"</code>" sets "<code>"aria-modal=\"true\""</code>" directly vs React Aria's "<code>"aria-hidden"</code>" on siblings."</li>
                <li><code>"description_props"</code>" returned separately (React Aria passes through "<code>"filterDOMProps"</code>")."</li>
                <li>"Element capture via "<code>"CapturedElement"</code>" / "<code>"ElementCaptureAttr"</code>" instead of React "<code>"RefObject"</code>"."</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Modal & Dialog Hooks", link: "#modal" },
                Toc::Leaf { title: "Hook Composition", link: "#hook-composition" },
                Toc::Leaf { title: "When to Use", link: "#when-to-use" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "Alert Dialog Demo", link: "#alert-dialog-demo" },
                Toc::Leaf { title: "Non-Dismissable Demo", link: "#non-dismissable-demo" },
                Toc::Leaf { title: "Confirmation Demo", link: "#confirmation-demo" },
                Toc::Leaf { title: "API", link: "#api" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "Related Hooks", link: "#related-hooks" },
                Toc::Leaf { title: "React Aria Deviations", link: "#deviations" },
            ]
        }/>
    }
}

/// Basic modal demo with scroll prevention via use_modal_backdrop.
/// Uses use_dialog for proper ARIA labeling and focus-on-mount.
#[component]
fn BasicModalDemo() -> impl IntoView {
    // State layer
    let UseModalStateReturn {
        is_open,
        set_open: _,
        open,
        close,
        toggle: _,
    } = use_modal_state(UseModalStateInput::default());

    // Backdrop layer - dismiss behavior (Escape, outside click) + scroll prevention
    let UseModalBackdropReturn {
        modal_props,
        backdrop_props,
        id: _,
    } = use_modal_backdrop(UseModalBackdropInput {
        is_open,
        on_close: close,
        is_dismissable: true,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });

    // Aria-modal layer
    let UseModalReturn {
        modal_props: aria_modal_props,
    } = use_modal(UseModalInput { is_disabled: false });

    // Dialog layer - ARIA labeling and focus on mount
    let UseDialogReturn {
        dialog_props,
        title_props,
        description_props,
        dialog_id: _,
    } = use_dialog(UseDialogInput {
        title: Some("Basic Modal".to_string()),
        description: Some("This modal has scroll prevention enabled.".to_string()),
        aria_label: None,
        role: DialogRole::Dialog,
    });

    // Convert Props to Attrs before storing in StoredValue (Props is non-Clone, Attrs is Clone)
    let title_id = StoredValue::new(title_props.id.clone());
    let description_id = StoredValue::new(description_props.id.clone());
    let modal_attrs = StoredValue::new(modal_props.into_attrs());
    let aria_modal_attrs = StoredValue::new(aria_modal_props.into_attrs());
    let dialog_attrs = StoredValue::new(dialog_props.into_attrs());
    let backdrop_attrs = StoredValue::new(backdrop_props.into_attrs());

    view! {
        <button
            on:click=move |_| open.run(())
            style=demo_button_primary()
        >
            "Open Modal"
        </button>

        <Show when=move || is_open.get()>
            // Backdrop with backdrop_props (pointerdown Firefox fix)
            <div
                {..backdrop_attrs.get_value()}
                style=modal_backdrop()
            >
                // FocusScope traps focus within the modal and restores it on close
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    // Modal - spread modal_props (overlay behavior) + aria_modal_props + dialog_props
                    <div
                        {..modal_attrs.get_value()}
                        {..aria_modal_attrs.get_value()}
                        {..dialog_attrs.get_value()}
                        style=modal_panel()
                    >
                        <h2 id=title_id.get_value() style=modal_title()>
                            "Basic Modal"
                        </h2>
                        <p id=description_id.get_value() style=modal_description()>
                            "This modal has scroll prevention enabled. Try scrolling the page - it won't work! "
                            "Press Escape or click outside to close."
                        </p>
                        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                            <button
                                on:click=move |_| close.run(())
                                style=demo_button()
                            >
                                "Cancel"
                            </button>
                            <button
                                on:click=move |_| close.run(())
                                style=modal_action_button_primary()
                            >
                                "Confirm"
                            </button>
                        </Stack>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}

/// Alert dialog demo with AlertDialog role.
/// Composes use_modal_backdrop (for dismiss behavior) + use_modal (for aria-modal)
/// + use_dialog (for ARIA semantics + focus).
#[component]
fn AlertDialogDemo() -> impl IntoView {
    let UseModalStateReturn {
        is_open,
        set_open: _,
        open,
        close,
        toggle: _,
    } = use_modal_state(UseModalStateInput::default());

    // Backdrop layer - non-dismissable for alert dialogs
    let UseModalBackdropReturn {
        modal_props,
        backdrop_props,
        id: _,
    } = use_modal_backdrop(UseModalBackdropInput {
        is_open,
        on_close: close,
        is_dismissable: false,
        is_keyboard_dismiss_disabled: true,
        should_close_on_interact_outside: None,
    });

    // Aria-modal layer
    let UseModalReturn {
        modal_props: aria_modal_props,
    } = use_modal(UseModalInput { is_disabled: false });

    // Dialog layer - ARIA labeling, alertdialog role, focus on mount
    let UseDialogReturn {
        dialog_props,
        title_props,
        description_props,
        dialog_id: _,
    } = use_dialog(UseDialogInput {
        title: Some("Delete Item".to_string()),
        description: Some("This action cannot be undone. Are you sure?".to_string()),
        aria_label: None,
        role: DialogRole::AlertDialog,
    });

    let title_id = StoredValue::new(title_props.id.clone());
    let description_id = StoredValue::new(description_props.id.clone());
    let modal_attrs = StoredValue::new(modal_props.into_attrs());
    let aria_modal_attrs = StoredValue::new(aria_modal_props.into_attrs());
    let dialog_attrs = StoredValue::new(dialog_props.into_attrs());
    let backdrop_attrs = StoredValue::new(backdrop_props.into_attrs());

    view! {
        <button
            on:click=move |_| open.run(())
            style=demo_button_danger()
        >
            "Delete Item (Alert Dialog)"
        </button>

        <Show when=move || is_open.get()>
            <div
                {..backdrop_attrs.get_value()}
                style=modal_backdrop()
            >
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    <div
                        {..modal_attrs.get_value()}
                        {..aria_modal_attrs.get_value()}
                        {..dialog_attrs.get_value()}
                        style=modal_panel()
                    >
                        <h2 id=title_id.get_value() style=modal_title_danger()>
                            "Delete Item"
                        </h2>
                        <p id=description_id.get_value() style=modal_description()>
                            "This action cannot be undone. Are you sure you want to delete this item?"
                        </p>
                        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                            <button
                                on:click=move |_| close.run(())
                                style=demo_button()
                            >
                                "Cancel"
                            </button>
                            <button
                                on:click=move |_| close.run(())
                                style=modal_action_button_danger()
                            >
                                "Delete"
                            </button>
                        </Stack>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}

/// Non-dismissable modal demo.
/// Uses use_dialog for proper ARIA labeling and focus-on-mount.
#[component]
fn NonDismissableModalDemo() -> impl IntoView {
    let UseModalStateReturn {
        is_open,
        set_open: _,
        open,
        close,
        toggle: _,
    } = use_modal_state(UseModalStateInput::default());

    // Backdrop layer - non-dismissable configuration
    let UseModalBackdropReturn {
        modal_props,
        backdrop_props,
        id: _,
    } = use_modal_backdrop(UseModalBackdropInput {
        is_open,
        on_close: close,
        is_dismissable: false,
        is_keyboard_dismiss_disabled: true,
        should_close_on_interact_outside: None,
    });

    // Aria-modal layer
    let UseModalReturn {
        modal_props: aria_modal_props,
    } = use_modal(UseModalInput { is_disabled: false });

    // Dialog layer - ARIA labeling and focus on mount
    let UseDialogReturn {
        dialog_props,
        title_props,
        description_props,
        dialog_id: _,
    } = use_dialog(UseDialogInput {
        title: Some("Non-Dismissable Modal".to_string()),
        description: Some("This modal cannot be closed by pressing Escape or clicking outside.".to_string()),
        aria_label: None,
        role: DialogRole::Dialog,
    });

    let title_id = StoredValue::new(title_props.id.clone());
    let description_id = StoredValue::new(description_props.id.clone());
    let modal_attrs = StoredValue::new(modal_props.into_attrs());
    let aria_modal_attrs = StoredValue::new(aria_modal_props.into_attrs());
    let dialog_attrs = StoredValue::new(dialog_props.into_attrs());
    let backdrop_attrs = StoredValue::new(backdrop_props.into_attrs());

    view! {
        <button
            on:click=move |_| open.run(())
            style=demo_button_secondary()
        >
            "Open Non-Dismissable Modal"
        </button>

        <Show when=move || is_open.get()>
            <div
                {..backdrop_attrs.get_value()}
                style=modal_backdrop()
            >
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    <div
                        {..modal_attrs.get_value()}
                        {..aria_modal_attrs.get_value()}
                        {..dialog_attrs.get_value()}
                        style=modal_panel()
                    >
                        <h2 id=title_id.get_value() style=modal_title()>
                            "Non-Dismissable Modal"
                        </h2>
                        <p id=description_id.get_value() style=modal_description()>
                            "This modal cannot be closed by pressing Escape or clicking outside. "
                            "You must click the button below to close it."
                        </p>
                        <button
                            on:click=move |_| close.run(())
                            style=modal_action_button_primary()
                        >
                            "I Understand"
                        </button>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}

/// Confirmation dialog demo using use_dialog_state.
/// Composes use_modal_backdrop (for dismiss behavior) + use_modal (for aria-modal)
/// + use_dialog (for ARIA semantics + focus).
#[component]
fn ConfirmationDialogDemo() -> impl IntoView {
    // Use dialog state with confirmation tracking
    let UseDialogStateReturn {
        is_open,
        set_open: _,
        open,
        close,
        toggle: _,
        confirm,
        is_confirmed,
    } = use_dialog_state(UseDialogStateInput::default());

    // Track the last action result
    let (last_result, set_last_result) = signal::<Option<bool>>(None);

    // Update last_result when dialog closes
    Effect::new(move || {
        if !is_open.get() && last_result.get().is_none() {
            // Dialog just closed, check confirmation status
            set_last_result.set(Some(is_confirmed.get()));
        }
    });

    // Backdrop layer - dismissable confirmation dialog
    let UseModalBackdropReturn {
        modal_props,
        backdrop_props,
        id: _,
    } = use_modal_backdrop(UseModalBackdropInput {
        is_open,
        on_close: close,
        is_dismissable: true,
        is_keyboard_dismiss_disabled: false,
        should_close_on_interact_outside: None,
    });

    // Aria-modal layer
    let UseModalReturn {
        modal_props: aria_modal_props,
    } = use_modal(UseModalInput { is_disabled: false });

    // Dialog layer - ARIA labeling and focus on mount
    let UseDialogReturn {
        dialog_props,
        title_props,
        description_props,
        dialog_id: _,
    } = use_dialog(UseDialogInput {
        title: Some("Confirm Action".to_string()),
        description: Some("Do you want to proceed with this action?".to_string()),
        aria_label: None,
        role: DialogRole::Dialog,
    });

    let title_id = StoredValue::new(title_props.id.clone());
    let description_id = StoredValue::new(description_props.id.clone());
    let modal_attrs = StoredValue::new(modal_props.into_attrs());
    let aria_modal_attrs = StoredValue::new(aria_modal_props.into_attrs());
    let dialog_attrs = StoredValue::new(dialog_props.into_attrs());
    let backdrop_attrs = StoredValue::new(backdrop_props.into_attrs());

    view! {
        <div style=flex_row_center()>
            <button
                on:click=move |_| {
                    set_last_result.set(None);
                    open.run(());
                }
                style=demo_button_primary()
            >
                "Open Confirmation Dialog"
            </button>

            <span style=move || {
                match last_result.get() {
                    Some(true) => state_active(),
                    _ => state_inactive(),
                }
            }>
                {move || match last_result.get() {
                    None => "No action taken yet".to_string(),
                    Some(true) => "Confirmed!".to_string(),
                    Some(false) => "Cancelled".to_string(),
                }}
            </span>
        </div>

        <Show when=move || is_open.get()>
            <div
                {..backdrop_attrs.get_value()}
                style=modal_backdrop()
            >
                <FocusScope contain=true restore_focus=true auto_focus=true>
                    <div
                        {..modal_attrs.get_value()}
                        {..aria_modal_attrs.get_value()}
                        {..dialog_attrs.get_value()}
                        style=modal_panel()
                    >
                        <h2 id=title_id.get_value() style=modal_title()>
                            "Confirm Action"
                        </h2>
                        <p id=description_id.get_value() style=modal_description()>
                            "Do you want to proceed with this action? "
                            "Click Confirm to accept or Cancel to decline."
                        </p>
                        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                            <button
                                on:click=move |_| {
                                    set_last_result.set(Some(false));
                                    close.run(());
                                }
                                style=demo_button()
                            >
                                "Cancel"
                            </button>
                            <button
                                on:click=move |_| {
                                    set_last_result.set(Some(true));
                                    confirm.run(());
                                }
                                style=modal_action_button_primary()
                            >
                                "Confirm"
                            </button>
                        </Stack>
                    </div>
                </FocusScope>
            </div>
        </Show>
    }
}
