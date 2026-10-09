use indoc::indoc;
use leptos::prelude::*;

use super::demos::overlay_trigger_state::OverlayTriggerStateDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseOverlayTriggerState() -> impl IntoView {
    view! {
        <DocPage title="use_overlay_trigger_state">
            <p>
                "The "<Code inline=true>"use_overlay_trigger_state"</Code>" hook owns whether an overlay is open: the state a "
                "trigger opens and the overlay closes. See the "
                <Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link>
                " for the other overlay building blocks."
            </p>

            <ReactAriaSource path="overlays/useOverlayTriggerState.ts" package=UpstreamPackage::ReactStately/>

            <Section title="Input">
                <ApiTable kind=ApiKind::Input of="UseOverlayTriggerStateInput">
                    <ApiRow name="default_open" ty="bool" default="false">
                        "Whether the overlay starts open. Ignored when "<Code inline=true>"value"</Code>" is bound."
                    </ApiRow>
                    <ApiRow name="value" ty="Option<ValueBinding<bool>>" default="None">
                        "The open state as app state ("<Code inline=true>"ValueBinding::from(rw_signal)"</Code>"), replacing "
                        <Code inline=true>"default_open"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_open_change" ty="Option<Callback<bool>>" default="None">
                        "Called when the overlay opens or closes. Setting the state it already has calls nothing."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <p>
                    "The hook returns an "<Code inline=true>"OverlayTriggerState"</Code>": a "<Code inline=true>"Copy"</Code>
                    " struct you pass to the hooks and atoms of the overlay."
                </p>
                <ApiTable kind=ApiKind::Fields of="OverlayTriggerState">
                    <ApiRow name="is_open" ty="Signal<bool>">"Whether the overlay is open."</ApiRow>
                    <ApiRow name="point" ty="Signal<Option<Point>>">
                        "Where a point-anchored overlay (e.g. a context menu) opened; "<Code inline=true>"None"</Code>
                        " for overlays anchored to their trigger."
                    </ApiRow>
                </ApiTable>
                <DocTable headers=&["Method", "Purpose"]>
                    <TableRow>
                        <TableCell><Code inline=true>"open(), close(), toggle()"</Code></TableCell>
                        <TableCell>"Open, close or toggle the overlay."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"set_open(is_open: bool)"</Code></TableCell>
                        <TableCell>"Sets the open state."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"set_point(point: Option<Point>)"</Code></TableCell>
                        <TableCell>"Sets the point the overlay opens at, before opening it."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::{
                            hooks::{
                                button::{UseButtonInput, UseButtonReturn, use_button},
                                overlay::{
                                    OverlayState,
                                    OverlayTriggerState,
                                    UseOverlayTriggerStateInput,
                                    use_overlay,
                                    use_overlay_trigger,
                                    use_overlay_trigger_state,
                                    use_popover,
                                },
                            },
                        };

                        let (times_opened, set_times_opened) = signal(0);

                        let state = use_overlay_trigger_state(UseOverlayTriggerStateInput {
                            on_open_change: Some(Callback::new(move |is_open: bool| {
                                if is_open {
                                    set_times_opened.update(|n| *n += 1);
                                }
                            })),
                            ..UseOverlayTriggerStateInput::default()
                        });

                        // Open it from a button, close it from the overlay (or let the overlay hooks close it).
                        let UseButtonReturn { props, .. } = use_button(UseButtonInput {
                            on_press: Some(Callback::new(move |_| state.toggle())),
                            ..UseButtonInput::default()
                        });
                    ")}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The button toggles the state and the panel closes it; "<Code inline=true>"on_open_change"</Code>
                    " counts the openings. The state alone sets no attributes and dismisses nothing: "
                    <Link href=routes::doc::overlay_behavior::UseOverlayTrigger.materialize()>"use_overlay_trigger"</Link>" and "
                    <Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link>" add that."
                </p>
                <Demo
                    description="A button and a panel sharing one open state, counting openings"
                    source=include_str!("demos/overlay_trigger_state.rs")
                >
                    <OverlayTriggerStateDemo/>
                </Demo>
            </Section>

            <Section title="App State">
                <p>
                    "To keep the open state in your app (e.g. to open a dialog from elsewhere), bind a signal with "
                    <Code inline=true>"value"</Code>", or convert it directly: "<Code inline=true>"OverlayTriggerState"</Code>
                    " implements "<Code inline=true>"From"</Code>" for "<Code inline=true>"RwSignal<bool>"</Code>", a "
                    <Code inline=true>"(ReadSignal<bool>, WriteSignal<bool>)"</Code>" pair and "
                    <Code inline=true>"ValueBinding<bool>"</Code>"."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        let is_open = RwSignal::new(false);
                        let state = OverlayTriggerState::from(is_open);

                        state.open();
                        assert!(is_open.get_untracked());
                    ")}
                </Code>
            </Section>

            <Section title="OverlayState">
                <p>
                    "The overlay hooks don\u{2019}t require an "<Code inline=true>"OverlayTriggerState"</Code>" itself, but any "
                    <Code inline=true>"OverlayState"</Code>": a "<Code inline=true>"Copy"</Code>" state that tells whether the "
                    "overlay is open ("<Code inline=true>"is_open()"</Code>", tracked), closes it ("<Code inline=true>"close()"</Code>
                    ") and may name the point it opened at ("<Code inline=true>"point()"</Code>", default: none). "
                    <Code inline=true>"OverlayTriggerState"</Code>" implements it, and so do the states of menus, selects and "
                    "combo boxes, which add their own closing logic. That is why "
                    <Link href=routes::doc::popover::Hook.materialize()>"use_popover"</Link>" accepts all of them."
                </p>
            </Section>

            <Section title="Used By">
                <p>
                    "The "<Link href=routes::doc::dialog::Atom.materialize()>"DialogTrigger"</Link>" atom creates this state and "
                    "hands it to the "<Code inline=true>"Popover"</Code>" or "<Code inline=true>"ModalBackdrop"</Code>
                    " inside it; those atoms create their own when they get "<Code inline=true>"is_open"</Code>" or "
                    <Code inline=true>"default_open"</Code>". The menu, submenu and tooltip trigger states build on it."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UseOverlayTrigger.materialize()>"use_overlay_trigger"</Link></li>
                <li><Link href=routes::doc::dialog::Atom.materialize()>"Dialog Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
