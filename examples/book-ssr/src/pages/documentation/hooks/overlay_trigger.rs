use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageUseOverlayTrigger() -> impl IntoView {
    view! {
        <DocPage title="use_overlay_trigger">
            <p>
                "The "<Code inline=true>"use_overlay_trigger"</Code>" hook sets the ARIA attributes that connect a trigger to "
                "the overlay it opens. See the "<Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link>
                " for the other overlay building blocks."
            </p>

            <ReactAriaSource path="overlays/useOverlayTrigger.ts"/>

            <Section title="Input">
                <ApiTable kind=ApiKind::Input of="UseOverlayTriggerInput">
                    <ApiRow name="show" ty="Signal<bool>">"Whether the overlay is shown. Required."</ApiRow>
                    <ApiRow name="overlay_id" ty="Oco<'static, str>">
                        "The overlay\u{2019}s id, as returned by "
                        <Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link>". Required."
                    </ApiRow>
                    <ApiRow name="overlay_type" ty="OverlayTriggerType">
                        "What the trigger opens: "<Code inline=true>"Dialog"</Code>", "<Code inline=true>"Menu"</Code>", "
                        <Code inline=true>"Listbox"</Code>", "<Code inline=true>"Tree"</Code>" or "<Code inline=true>"Grid"</Code>
                        ". Required."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseOverlayTriggerReturn">
                    <ApiRow name="props" ty="UseOverlayTriggerProps">
                        "The trigger\u{2019}s ARIA attributes, see "<AnchorLink href="#attributes">"Attributes"</AnchorLink>
                        ". Spread "<Code inline=true>"{..props.into_attrs()}"</Code>" onto the trigger."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::*;

                        let (is_open, set_is_open) = signal(false);
                        let UseOverlayReturn { props: overlay_props, id, .. } =
                            use_overlay(UseOverlayInput::new(is_open.into(), Callback::new(move |()| set_is_open.set(false))));

                        let UseOverlayTriggerReturn { props: trigger_props } = use_overlay_trigger(UseOverlayTriggerInput {
                            show: is_open.into(),
                            overlay_id: id,
                            overlay_type: OverlayTriggerType::Dialog,
                        });
                        let UseButtonReturn { props: button_props, .. } = use_button(UseButtonInput {
                            on_press: Some(Callback::new(move |_| set_is_open.update(|open| *open = !*open))),
                            ..UseButtonInput::default()
                        });
                        let (button_attrs, button_styles) = button_props.into_parts();

                        view! {
                            <button {..button_attrs} {..trigger_props.into_attrs()} style=button_styles>"Filters"</button>
                        }
                    "#)}
                </Code>
                <p>
                    "The hook only sets attributes: open the overlay from your own press handling, as above. The "
                    <Link href=format!("{}#quick-start", routes::doc::OverlayBehavior.materialize())>"Quick Start of the overview"</Link>
                    " shows a complete trigger with its overlay."
                </p>
            </Section>

            <Section title="Attributes">
                <DocTable headers=&["Attribute", "Value"]>
                    <TableRow>
                        <TableCell><Code inline=true>"aria-expanded"</Code></TableCell>
                        <TableCell>"Whether the overlay is shown."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"aria-controls"</Code></TableCell>
                        <TableCell>"The overlay\u{2019}s id, only while it is shown (a closed overlay isn\u{2019}t rendered)."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"aria-haspopup"</Code></TableCell>
                        <TableCell>
                            <Code inline=true>"true"</Code>" for "<Code inline=true>"Menu"</Code>", "<Code inline=true>"listbox"</Code>
                            " for "<Code inline=true>"Listbox"</Code>". Omitted for the other types, because screen readers "
                            "announce other values as a menu."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::OverlayBehavior.materialize()>"Overlay Behavior overview"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UseOverlay.materialize()>"use_overlay"</Link></li>
                <li><Link href=routes::doc::overlay_behavior::UseOverlayTriggerState.materialize()>"use_overlay_trigger_state"</Link></li>
                <li><Link href=format!("{}#use-menu-trigger", routes::doc::menu::Hook.materialize())>"use_menu_trigger"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
