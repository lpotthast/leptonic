use indoc::indoc;
use leptonic::hooks::link::LinkTarget;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageAccessibility() -> impl IntoView {
    view! {
        <DocPage title="Accessibility">
            <p>
                "Leptonic\u{2019}s hooks are ports of "
                <Link href="https://react-spectrum.adobe.com/react-aria/" target=LinkTarget::Blank>"react-aria"</Link>
                "\u{2019}s, which implement the "
                <Link href="https://www.w3.org/WAI/ARIA/apg/patterns/" target=LinkTarget::Blank>"WAI-ARIA Authoring Practices"</Link>
                ". Building on them gives your app the hard "
                "parts of accessibility: semantics, keyboard interaction and focus management. This guide lists what "
                "leptonic takes care of and what remains for your app."
            </p>

            <Section title="What Each Layer Does">
                <DocTable headers=&["Layer", "Accessibility it provides"]>
                    <TableRow>
                        <TableCell><b>"Hook"</b></TableCell>
                        <TableCell>
                            "The ARIA pattern: roles, "<Code inline=true>"aria-*"</Code>" attributes and the ids linking "
                            "elements, keyboard interaction, focus handling, announcements. You render the elements and spread "
                            "the hook\u{2019}s attributes onto them."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><b>"Atom"</b></TableCell>
                        <TableCell>
                            "The hook on the right semantic element (a "<Code inline=true>"<button>"</Code>", an "
                            <Code inline=true>"<input>"</Code>" behind a custom checkbox), plus "<Code inline=true>"data-*"</Code>
                            " attributes for every state, so your CSS shows focus, selection or disabled state without "
                            "tracking it."
                        </TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "Whichever layer you use, the behavior is the same; see "
                    <Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link>". The looks are yours: "
                    "your styles (or the optional "<Link href=format!("{}#the-atom-theme", routes::doc::Themes.materialize())>"atom theme"</Link>
                    ") show focus, states and contrast."
                </p>
            </Section>

            <Section title="ARIA Patterns">
                <p>
                    "Each concept implements its WAI-ARIA pattern: a menu button opens a "<Code inline=true>"menu"</Code>
                    " with "<Code inline=true>"menuitem"</Code>"s, tabs are a "<Code inline=true>"tablist"</Code>" of "
                    <Code inline=true>"tab"</Code>"s controlling "<Code inline=true>"tabpanel"</Code>"s, a combobox links "
                    "its input to its "<Code inline=true>"listbox"</Code>" and the active option. Roles and ARIA attributes "
                    "are typed ("<Code inline=true>"AriaRole"</Code>", "<Code inline=true>"AriaExpanded"</Code>", \u{2026}), "
                    "so they can\u{2019}t be misspelled. Native elements keep their implicit semantics: a "
                    <Code inline=true>"<button>"</Code>" gets no "<Code inline=true>"role=\"button\""</Code>", a disabled one "
                    "the "<Code inline=true>"disabled"</Code>" attribute instead of "<Code inline=true>"aria-disabled"</Code>"."
                </p>
                <p>
                    "Every concept overview names its pattern in its \u{201c}Accessibility\u{201d} section, e.g. the "
                    <Link href=routes::doc::Button.materialize()>"Button"</Link>" and "
                    <Link href=routes::doc::Menu.materialize()>"Menu"</Link>" overviews."
                </p>
            </Section>

            <Section title="Keyboard and Pointer Interaction">
                <ul>
                    <li>
                        "Every concept is operable with the keyboard alone. Composite concepts (lists, grids, tabs, "
                        "toolbars, radio groups) are one tab stop, and arrow keys move within them. In lists and menus, typing the "
                        "first letters of an item moves to it."
                    </li>
                    <li>
                        <Link href=routes::doc::interactions::UsePress.materialize()><Code inline=true>"use_press"</Code></Link>
                        " treats mouse, touch, pen, keyboard and the virtual clicks of screen readers alike, so a button "
                        "behaves the same however it is pressed. "
                        <Link href=routes::doc::interactions::UseHover.materialize()><Code inline=true>"use_hover"</Code></Link>
                        " ignores the emulated hover of touch screens."
                    </li>
                    <li>
                        "In a right-to-left locale, arrow keys follow the reading direction: "<Keys keys="ArrowLeft"/>
                        " moves to the next item of a horizontal toolbar or tab list and raises a horizontal slider."
                    </li>
                </ul>
            </Section>

            <Section title="Focus Management">
                <ul>
                    <li>
                        <Link href=routes::doc::focus::FocusScope.materialize()><Code inline=true>"FocusScope"</Code></Link>
                        " keeps focus inside modal dialogs, moves it into an overlay when it opens and back to the trigger "
                        "when it closes."
                    </li>
                    <li>
                        "While a modal or a modal popover is open, the rest of the page is "<Code inline=true>"inert"</Code>
                        ": hidden from screen readers and not interactive. An open combobox list hides the page from screen "
                        "readers with "<Code inline=true>"aria-hidden"</Code>" and keeps it usable with the pointer. Page "
                        "scrolling is locked while a modal overlay is open ("
                        <Link href=routes::doc::overlay_behavior::UsePreventScroll.materialize()><Code inline=true>"use_prevent_scroll"</Code></Link>
                        ")."
                    </li>
                    <li>
                        "Popovers render a visually hidden "
                        <Link href=routes::doc::overlay_behavior::DismissButton.materialize()><Code inline=true>"DismissButton"</Code></Link>
                        ", so screen reader users on touch devices, who have no "<Keys keys="Escape"/>" key, can close them."
                    </li>
                </ul>
            </Section>

            <Section title="Focus Rings">
                <p>
                    "A focus ring should show when the user navigates with the keyboard, not after every click. "
                    <Link href=routes::doc::focus::UseFocusVisible.materialize()><Code inline=true>"use_focus_visible"</Code></Link>
                    " tracks how the user interacts, and atoms set "<Code inline=true>"data-focus-visible"</Code>
                    " on the focused element only for keyboard focus. Draw the ring from that attribute; if your styles "
                    "remove the browser\u{2019}s own "<Code inline=true>":focus-visible"</Code>" outline, every interactive "
                    "element needs a ring of yours:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        *:focus-visible { outline: none; }
                        [data-focus-visible] { outline: 3px solid var(--focus); outline-offset: 2px; }
                    ")}
                </Code>
                <p>
                    "Without the browser\u{2019}s outline, elements you build without leptonic have no focus ring. Mark them with "
                    <Link href=routes::doc::focus::FocusRing.materialize()><Code inline=true>"FocusRing"</Code></Link>
                    " (or "<Link href=routes::doc::focus::UseFocusRing.materialize()><Code inline=true>"use_focus_ring"</Code></Link>
                    "), or give them a "<Code inline=true>":focus-visible"</Code>" style of your own."
                </p>
            </Section>

            <Section title="Screen Reader Announcements">
                <p>
                    "Some changes have no element that screen readers would read: the new value of a spin button, the "
                    "progress of a keyboard drag, the month a calendar switched to. Leptonic announces them through the "
                    <Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"live announcer"</Link>
                    ", which you can use for your app\u{2019}s own messages too (\u{201c}Saved\u{201d}, \u{201c}3 results\u{201d}). "
                    "Content only screen readers need, such as the context of a repeated \u{201c}Download\u{201d} button, goes "
                    "into a "
                    <Link href=routes::doc::screen_readers::VisuallyHidden.materialize()><Code inline=true>"VisuallyHidden"</Code></Link>
                    " element."
                </p>
            </Section>

            <Section title="Internationalization">
                <p>
                    "Hooks read the locale and writing direction from the nearest "
                    <Link href=routes::doc::utilities::I18nProvider.materialize()><Code inline=true>"I18nProvider"</Code></Link>
                    " (\u{201c}en-US\u{201d}, left-to-right, without one). The direction swaps arrow keys and mirrors sliders "
                    "and overlay placements at the start or end; the locale formats numbers and compares text. The texts "
                    "leptonic adds itself (such as the \u{201c}Dismiss\u{201d} label of "<Code inline=true>"DismissButton"</Code>
                    ") follow the locale too, in 34 languages, with leptonic\u{2019}s default "<Code inline=true>"intl-strings"</Code>
                    " feature; where a concept takes an "<Code inline=true>"aria_label"</Code>", you can pass your own."
                </p>
            </Section>

            <Section title="What Your App Has to Do">
                <ul>
                    <li>
                        <b>"Name every control."</b>" Give fields a visible label and icon-only buttons an "
                        <Code inline=true>"aria-label"</Code>". Leptonic can\u{2019}t invent names; in debug builds, "
                        "fields without a label or "<Code inline=true>"aria_label"</Code>" log a warning."
                    </li>
                    <li>
                        <b>"Structure the page."</b>" Use headings in order, landmarks ("<Code inline=true>"<main>"</Code>", "
                        <Code inline=true>"<nav>"</Code>"; with "<Link href=routes::doc::focus::UseLandmark.materialize()>"use_landmark"</Link>
                        ", "<Keys keys="F6"/>" moves between them), a meaningful page title, and text alternatives for images."
                    </li>
                    <li>
                        <b>"Set the document language and direction."</b>" Render "<Code inline=true>"lang"</Code>" and "
                        <Code inline=true>"dir"</Code>" on "<Code inline=true>"<html>"</Code>"; "<Code inline=true>"I18nProvider"</Code>
                        " informs leptonic\u{2019}s hooks, not the browser."
                    </li>
                    <li>
                        <b>"Keep contrast."</b>" When you style atoms or change theme variables (see "
                        <Link href=routes::doc::Themes.materialize()>"Themes"</Link>"), check that text, borders of controls "
                        "and focus rings contrast with their background, in light and dark theme."
                    </li>
                    <li>
                        <b>"Don\u{2019}t rely on color alone."</b>" Pair colors with text or icons, e.g. show an error message "
                        "next to a red border (see "<Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link>")."
                    </li>
                    <li>
                        <b>"Announce your own changes."</b>" Results loaded in the background, a saved form, a failed "
                        "request: tell screen reader users with the live announcer."
                    </li>
                    <li>
                        <b>"Test."</b>" Use your app with the keyboard alone and with a screen reader (VoiceOver, NVDA, "
                        "TalkBack, \u{2026})."
                    </li>
                </ul>
            </Section>

            <Section title="Keyboard Interaction in This Book">
                <p>
                    "Concept overviews list the keyboard interaction of all their layers in their \u{201c}Accessibility\u{201d} "
                    "section; hook pages repeat it in a \u{201c}Keyboard\u{201d} section, with details such as right-to-left "
                    "behavior. Keys are written as key caps: "<Keys keys="Shift + Tab"/>" means pressing both together, "
                    "alternatives are separated by a slash."
                </p>
                <KeyboardTable>
                    <KeyRow keys="Tab / Shift + Tab">"Moves focus to the next or previous element."</KeyRow>
                    <KeyRow keys="Enter / Space">"Activates the focused button."</KeyRow>
                </KeyboardTable>
                <p>"Every demo in this book is keyboard accessible: try them without a mouse."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Architecture.materialize()>"Hooks & Atoms"</Link></li>
                <li><Link href=routes::doc::Focus.materialize()>"Focus"</Link></li>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions"</Link></li>
                <li><Link href=routes::doc::screen_readers::LiveAnnouncer.materialize()>"Live Announcer"</Link></li>
                <li><Link href=routes::doc::screen_readers::VisuallyHidden.materialize()>"VisuallyHidden"</Link></li>
                <li><Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link></li>
                <li><Link href=routes::doc::Ssr.materialize()>"Server-Side Rendering"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
