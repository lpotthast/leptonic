use leptos_routes::routes;

#[routes]
pub mod routes {
    #![allow(
        clippy::must_use_candidate,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]

    use crate::pages::documentation::concept_layout::ConceptLayout;
    use leptos::prelude::*;
    use leptos_router::components::Redirect;

    fallback!(crate::pages::err404::PageErr404);

    #[route("/")]
    mod root {
        page!(crate::pages::welcome::PageWelcome);
    }

    #[route("/doc")]
    mod doc {
        layout!(crate::pages::documentation::doc_layout::DocLayout);
        index!(|| view! { <Redirect path=doc::Overview.materialize()/> });

        #[route("/overview")]
        mod overview {
            page!(crate::pages::documentation::getting_started::overview::PageOverview);
        }

        #[route("/installation")]
        mod installation {
            page!(crate::pages::documentation::getting_started::installation::PageInstallation);
        }

        #[route("/themes")]
        mod themes {
            page!(crate::pages::documentation::getting_started::themes::PageThemes);
        }

        #[route("/changelog")]
        mod changelog {
            page!(crate::pages::documentation::getting_started::changelog::PageChangelog);
        }

        #[route("/event-propagation")]
        mod event_propagation {
            page!(crate::pages::documentation::getting_started::event_propagation::PageEventPropagation);
        }

        #[route("/architecture")]
        mod architecture {
            page!(crate::pages::documentation::getting_started::architecture::PageArchitecture);
        }

        #[route("/classes-and-styles")]
        mod classes_and_styles {
            page!(crate::pages::documentation::getting_started::classes_and_styles::PageClassesAndStyles);
        }

        #[route("/forms")]
        mod forms {
            page!(crate::pages::documentation::getting_started::forms::PageForms);
        }

        // ── Behavioral domains ───────────────────────────────────

        #[route("/interactions")]
        mod interactions {
            index!(crate::pages::documentation::domains::interactions::PageInteractions);

            #[route("/use-press")]
            mod use_press {
                page!(crate::pages::documentation::hooks::press::PageUsePress);
            }
            #[route("/press-responder")]
            mod press_responder {
                page!(crate::pages::documentation::atoms::press_responder::PageAtomPressResponder);
            }
            #[route("/hoverable")]
            mod hoverable {
                page!(crate::pages::documentation::atoms::hoverable::PageAtomHoverable);
            }
            #[route("/use-hover")]
            mod use_hover {
                page!(crate::pages::documentation::hooks::hover::PageUseHover);
            }
            #[route("/use-move")]
            mod use_move {
                page!(crate::pages::documentation::hooks::r#move::PageUseMove);
            }
            #[route("/use-keyboard")]
            mod use_keyboard {
                page!(crate::pages::documentation::hooks::keyboard::PageUseKeyboard);
            }
            #[route("/use-interact-outside")]
            mod use_interact_outside {
                page!(crate::pages::documentation::hooks::interact_outside::PageUseInteractOutside);
            }
            #[route("/use-scroll-wheel")]
            mod use_scroll_wheel {
                page!(crate::pages::documentation::hooks::scroll_wheel::PageUseScrollWheel);
            }
            #[route("/use-prevent-scroll")]
            mod use_prevent_scroll {
                page!(crate::pages::documentation::hooks::prevent_scroll::PageUsePreventScroll);
            }
            #[route("/dnd")]
            mod dnd {
                page!(crate::pages::documentation::hooks::dnd::PageUseDnd);
            }
        }

        #[route("/focus")]
        mod focus {
            index!(crate::pages::documentation::domains::focus::PageFocus);

            #[route("/use-focus")]
            mod use_focus {
                page!(crate::pages::documentation::hooks::focus::PageUseFocus);
            }
            #[route("/use-focus-within")]
            mod use_focus_within {
                page!(crate::pages::documentation::hooks::focus_within::PageUseFocusWithin);
            }
            #[route("/use-focusable")]
            mod use_focusable {
                page!(crate::pages::documentation::hooks::focusable::PageUseFocusable);
            }
            #[route("/use-focus-manager")]
            mod use_focus_manager {
                page!(crate::pages::documentation::hooks::focus_manager::PageUseFocusManager);
            }
            #[route("/use-has-tabbable-child")]
            mod use_has_tabbable_child {
                page!(
                    crate::pages::documentation::hooks::has_tabbable_child::PageUseHasTabbableChild
                );
            }
            #[route("/use-focus-ring")]
            mod use_focus_ring {
                page!(crate::pages::documentation::hooks::focus_ring::PageUseFocusRing);
            }
            #[route("/use-focus-visible")]
            mod use_focus_visible {
                page!(crate::pages::documentation::hooks::focus_visible::PageUseFocusVisible);
            }
            #[route("/focus-scope")]
            mod focus_scope {
                page!(crate::pages::documentation::atoms::focus_scope::PageAtomFocusScope);
            }
            #[route("/focus-ring")]
            mod focus_ring {
                page!(crate::pages::documentation::atoms::focus_ring::PageAtomFocusRing);
            }
            #[route("/focus-manager")]
            mod focus_manager {
                page!(crate::pages::documentation::atoms::focus_manager::PageAtomFocusManager);
            }
        }

        #[route("/overlays")]
        mod overlays {
            index!(crate::pages::documentation::domains::overlays::PageOverlays);

            #[route("/use-overlay")]
            mod use_overlay {
                page!(crate::pages::documentation::hooks::overlay::PageUseOverlay);
            }
            #[route("/dismiss-button")]
            mod dismiss_button {
                page!(crate::pages::documentation::atoms::dismiss_button::PageAtomDismissButton);
            }
        }

        #[route("/collections")]
        mod collections {
            page!(crate::pages::documentation::domains::collections::PageCollections);
        }

        // The selection hooks were merged into the collections; old links land there.
        #[route("/selection")]
        mod selection_domain {
            page!(|| view! { <Redirect path=crate::routes::doc::Collections.materialize()/> });
        }

        // ── Category overview pages ──────────────────────────────

        #[route("/input")]
        mod input_category {
            page!(crate::pages::documentation::domains::input::PageInputCategory);
        }

        #[route("/data-display")]
        mod data_display {
            page!(crate::pages::documentation::domains::data_display::PageDataDisplay);
        }

        #[route("/layout")]
        mod layout_category {
            page!(crate::pages::documentation::domains::layout::PageLayoutCategory);
        }

        #[route("/feedback")]
        mod feedback {
            page!(crate::pages::documentation::domains::feedback::PageFeedback);
        }

        #[route("/navigation")]
        mod navigation {
            page!(crate::pages::documentation::domains::navigation::PageNavigation);
        }

        #[route("/general")]
        mod general {
            page!(crate::pages::documentation::domains::general::PageGeneral);
        }

        // ── Multi-layer concepts ─────────────────────────────────

        #[route("/button")]
        mod button {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::button::PageButtonOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::button::PageUseButton);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::button::PageAtomButton);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::input::button::PageButton);
            }
        }

        #[route("/checkbox")]
        mod checkbox {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::checkbox::PageCheckboxOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::checkbox::PageUseCheckboxHook);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::checkbox::PageAtomCheckbox);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::input::checkbox::PageCheckbox);
            }
        }

        #[route("/radio")]
        mod radio {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::radio::PageRadioOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::radio::PageUseRadioHook);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::radio::PageAtomRadio);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::input::radio::PageRadio);
            }
        }

        #[route("/switch")]
        mod switch {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::switch::PageSwitchOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::switch::PageUseSwitchHook);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::switch::PageAtomSwitch);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::input::switch::PageSwitch);
            }
        }

        // The `Toggle` component became `Switch`; old links land on the Switch concept.
        #[route("/toggle")]
        mod toggle {
            index!(|| view! { <Redirect path=crate::routes::doc::Switch.materialize()/> });

            #[route("/hook")]
            mod hook {
                page!(|| view! { <Redirect path=crate::routes::doc::switch::Hook.materialize()/> });
            }
            #[route("/component")]
            mod component {
                page!(
                    || view! { <Redirect path=crate::routes::doc::switch::Component.materialize()/> }
                );
            }
        }

        #[route("/toggle-button")]
        mod toggle_button {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::toggle_button::PageToggleButtonOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::toggle_button::PageUseToggleButtonHook);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::toggle_button::PageAtomToggleButton);
            }
        }

        #[route("/color")]
        mod color {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::color::PageColorOverview);

            #[route("/hooks")]
            mod hooks {
                page!(crate::pages::documentation::hooks::color::PageUseColorHooks);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::color::PageAtomColor);
            }
            #[route("/component")]
            mod component {
                page!(
                    crate::pages::documentation::components::input::color_picker::PageColorPicker
                );
            }
        }

        #[route("/slider")]
        mod slider {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::slider::PageSliderOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::slider::PageUseSliderHook);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::slider::PageAtomSlider);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::input::slider::PageSlider);
            }
        }

        #[route("/select")]
        mod select {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::select::PageSelectOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::select::PageUseSelectHook);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::select::PageAtomSelect);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::input::select::PageSelect);
            }
        }

        #[route("/text-field")]
        mod text_field {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::text_field::PageTextFieldOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::text_field::PageUseTextField);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::text_field::PageAtomTextField);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::input::text_field::PageTextField);
            }
            #[route("/number-field-atom")]
            mod number_field_atom {
                page!(crate::pages::documentation::atoms::number_field::PageAtomNumberField);
            }
            #[route("/number-field-hook")]
            mod number_field_hook {
                page!(crate::pages::documentation::hooks::number_field::PageUseNumberField);
            }
        }

        #[route("/popover")]
        mod popover {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::popover::PagePopoverOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::popover::PageUsePopoverHook);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::popover::PageAtomPopover);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::feedback::popover::PagePopover);
            }
        }

        #[route("/modal")]
        mod modal {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::modal::PageModalOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::modal::PageUseModalHook);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::modal::PageAtomModal);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::feedback::modal::PageModal);
            }
        }

        #[route("/grid")]
        mod grid {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::grid::PageGridOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::grid::PageUseGrid);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::grid::PageAtomGrid);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::layout::grid::PageGrid);
            }
        }

        #[route("/table")]
        mod table {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::table::PageTableOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::table::PageUseTableHook);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::table::PageAtomTable);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::layout::table::PageTable);
            }
        }

        #[route("/date-time")]
        mod date_time {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::date_time::PageDateTimeOverview);

            #[route("/calendar-hooks")]
            mod calendar_hooks {
                page!(crate::pages::documentation::hooks::calendar::PageCalendarHooks);
            }
            #[route("/date-field-hooks")]
            mod date_field_hooks {
                page!(crate::pages::documentation::hooks::date_field::PageDateFieldHooks);
            }
            #[route("/date-picker-hooks")]
            mod date_picker_hooks {
                page!(crate::pages::documentation::hooks::date_picker::PageDatePickerHooks);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::input::date_time::PageDateTime);
            }
        }

        #[route("/tabs")]
        mod tabs {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::tabs::PageTabsOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::tabs::PageUseTabsHook);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::tabs::PageAtomTabs);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::layout::tab::PageTab);
            }
        }

        #[route("/separator")]
        mod separator {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::separator::PageSeparatorOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::separator::PageUseSeparatorHook);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::layout::separator::PageSeparator);
            }
        }

        #[route("/collapsible")]
        mod collapsible {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::collapsible::PageCollapsibleOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::disclosure::PageUseDisclosure);
            }
            #[route("/component")]
            mod component {
                page!(
                    crate::pages::documentation::components::layout::collapsible::PageCollapsible
                );
            }
        }

        #[route("/progress")]
        mod progress {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::progress::PageProgressOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::progress::PageUseProgressBar);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::feedback::progress::PageProgress);
            }
        }

        #[route("/chip")]
        mod chip {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::chip::PageChipOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::tag::PageUseTag);
            }
            #[route("/component")]
            mod component {
                page!(crate::pages::documentation::components::feedback::chip::PageChip);
            }
        }

        // ── Concepts with hooks only ─────────────────────────────

        #[route("/combobox")]
        mod combobox {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::combobox::PageComboboxOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::combobox::PageUseCombobox);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::combobox::PageAtomComboBox);
            }
        }

        #[route("/listbox")]
        mod listbox {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::listbox::PageListboxOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::listbox::PageUseListbox);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::listbox::PageAtomListBox);
            }
        }

        #[route("/menu")]
        mod menu {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::menu::PageMenuOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::menu::PageUseMenuHook);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::menu::PageAtomMenu);
            }
        }

        #[route("/tooltip")]
        mod tooltip {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::tooltip::PageTooltipOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::tooltip::PageUseTooltipHook);
            }
            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::tooltip::PageAtomTooltip);
            }
        }

        // ── Link concept (special: 2 hooks + 2 atoms) ───────────

        #[route("/link")]
        mod link {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::link::PageLinkOverview);

            #[route("/use-link")]
            mod use_link {
                page!(crate::pages::documentation::hooks::link::PageUseLink);
            }
            #[route("/use-anchor-link")]
            mod use_anchor_link {
                page!(crate::pages::documentation::hooks::anchor_link::PageUseAnchorLink);
            }
            #[route("/link-atom")]
            mod link_atom {
                page!(crate::pages::documentation::atoms::link::PageAtomLink);
            }
            #[route("/anchor-link-atom")]
            mod anchor_link_atom {
                page!(crate::pages::documentation::atoms::anchor_link::PageAtomAnchorLink);
            }
        }

        // ── Standalone hooks ─────────────────────────────────────

        #[route("/hooks")]
        mod hooks {
            #[route("/animation")]
            mod animation {
                page!(crate::pages::documentation::hooks::animation::PageAnimationHooks);
            }
            #[route("/use-label")]
            mod use_label {
                page!(crate::pages::documentation::hooks::label::PageUseLabel);
            }
            #[route("/use-breadcrumbs")]
            mod use_breadcrumbs {
                page!(crate::pages::documentation::hooks::breadcrumbs::PageUseBreadcrumbs);
            }
            #[route("/use-meter")]
            mod use_meter {
                page!(crate::pages::documentation::hooks::meter::PageUseMeter);
            }
            #[route("/use-color-area")]
            mod use_color_area {
                page!(crate::pages::documentation::hooks::color_area::PageUseColorArea);
            }
            #[route("/use-color-slider")]
            mod use_color_slider {
                page!(crate::pages::documentation::hooks::color_slider::PageUseColorSlider);
            }
            #[route("/use-color-wheel")]
            mod use_color_wheel {
                page!(crate::pages::documentation::hooks::color_wheel::PageUseColorWheel);
            }
            #[route("/use-color-field")]
            mod use_color_field {
                page!(crate::pages::documentation::hooks::color_field::PageUseColorField);
            }
            #[route("/use-color-swatch")]
            mod use_color_swatch {
                page!(crate::pages::documentation::hooks::color_swatch::PageUseColorSwatch);
            }
            #[route("/use-color-channel-field")]
            mod use_color_channel_field {
                page!(crate::pages::documentation::hooks::color_channel_field::PageUseColorChannelField);
            }
            #[route("/use-spin-button")]
            mod use_spin_button {
                page!(crate::pages::documentation::hooks::spin_button::PageUseSpinButton);
            }
            #[route("/use-toolbar")]
            mod use_toolbar {
                page!(crate::pages::documentation::hooks::toolbar::PageUseToolbar);
            }
            #[route("/use-tree")]
            mod use_tree {
                page!(crate::pages::documentation::hooks::tree::PageUseTree);
            }
            #[route("/selection")]
            mod selection {
                page!(|| view! { <Redirect path=crate::routes::doc::Collections.materialize()/> });
            }
        }

        // ── Standalone atoms ─────────────────────────────────────

        #[route("/atoms")]
        mod atoms {
            #[route("/field")]
            mod field {
                page!(crate::pages::documentation::atoms::field::PageAtomField);
            }
            #[route("/form")]
            mod form {
                page!(crate::pages::documentation::atoms::form::PageAtomForm);
            }
        }

        // ── Utilities ────────────────────────────────────────────

        #[route("/utils")]
        mod utils {
            #[route("/live-announcer")]
            mod live_announcer {
                page!(crate::pages::documentation::utils::live_announcer::PageLiveAnnouncer);
            }
        }

        // ── Standalone components ────────────────────────────────

        #[route("/components")]
        mod components {
            #[route("/transitions")]
            mod transitions {
                page!(
                    crate::pages::documentation::components::general::transitions::PageTransitions
                );
            }
            #[route("/stack")]
            mod stack {
                page!(crate::pages::documentation::components::layout::stack::PageStack);
            }
            #[route("/skeleton")]
            mod skeleton {
                page!(crate::pages::documentation::components::layout::skeleton::PageSkeleton);
            }
            #[route("/app-bar")]
            mod app_bar {
                page!(crate::pages::documentation::components::layout::app_bar::PageAppBar);
            }
            #[route("/drawer")]
            mod drawer {
                page!(crate::pages::documentation::components::layout::drawer::PageDrawer);
            }
            // Moved into the Date & Time concept; old links land on its component tab.
            #[route("/date-time")]
            mod date_time {
                page!(
                    || view! { <Redirect path=crate::routes::doc::date_time::Component.materialize()/> }
                );
            }
            #[route("/tiptap-editor")]
            mod tiptap_editor {
                page!(
                    crate::pages::documentation::components::input::tiptap_editor::PageTiptapEditor
                );
            }
            #[route("/alert")]
            mod alert {
                page!(crate::pages::documentation::components::feedback::alert::PageAlert);
            }
            #[route("/toast")]
            mod toast {
                page!(crate::pages::documentation::components::feedback::toast::PageToast);
            }
            #[route("/kbd")]
            mod kbd {
                page!(crate::pages::documentation::components::feedback::kbd::PageKbd);
            }
            #[route("/typography")]
            mod typography {
                page!(crate::pages::documentation::components::general::typography::PageTypography);
            }
            #[route("/icon")]
            mod icon {
                page!(crate::pages::documentation::components::general::icon::PageIcon);
            }
            #[route("/callback")]
            mod callback {
                page!(crate::pages::documentation::components::general::callback::PageCallback);
            }
        }
    }

    #[route("/not-found")]
    mod not_found {
        page!(crate::pages::err404::PageErr404);
    }
}
