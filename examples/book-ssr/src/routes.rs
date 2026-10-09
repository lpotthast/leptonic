use leptos_routes::routes;

/// The book's routes. Pages are grouped like the navigation (`nav.rs`). Moved pages keep their old URL as a redirect,
/// in modules named `moved_*`, so that code linking an old route doesn't compile.
#[routes]
pub mod routes {
    #![allow(
        clippy::must_use_candidate,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]

    use leptos::prelude::*;
    use leptos_router::components::Redirect;

    use crate::pages::documentation::concept_layout::ConceptLayout;

    fallback!(crate::pages::err404::PageErr404);

    #[route("/")]
    mod root {
        page!(crate::pages::welcome::PageWelcome);
    }

    #[route("/doc")]
    mod doc {
        layout!(crate::pages::documentation::doc_layout::DocLayout);
        index!(|| view! { <Redirect path=doc::Overview.materialize()/> });

        // ── Getting started and guides ──────────────────────────────────

        #[route("/overview")]
        mod overview {
            page!(crate::pages::documentation::getting_started::overview::PageOverview);
        }

        #[route("/installation")]
        mod installation {
            page!(crate::pages::documentation::getting_started::installation::PageInstallation);
        }

        #[route("/changelog")]
        mod changelog {
            page!(crate::pages::documentation::getting_started::changelog::PageChangelog);
        }

        #[route("/architecture")]
        mod architecture {
            page!(crate::pages::documentation::getting_started::architecture::PageArchitecture);
        }

        #[route("/event-propagation")]
        mod event_propagation {
            page!(crate::pages::documentation::getting_started::event_propagation::PageEventPropagation);
        }

        #[route("/classes-and-styles")]
        mod classes_and_styles {
            page!(crate::pages::documentation::getting_started::classes_and_styles::PageClassesAndStyles);
        }

        #[route("/callbacks")]
        mod callbacks {
            page!(crate::pages::documentation::getting_started::callbacks::PageCallbacks);
        }

        #[route("/themes")]
        mod themes {
            page!(crate::pages::documentation::getting_started::themes::PageThemes);
        }

        #[route("/forms")]
        mod forms {
            page!(crate::pages::documentation::getting_started::forms::PageForms);
        }

        #[route("/ssr")]
        mod ssr {
            page!(crate::pages::documentation::getting_started::ssr::PageSsr);
        }

        #[route("/accessibility")]
        mod accessibility {
            page!(crate::pages::documentation::getting_started::accessibility::PageAccessibility);
        }

        #[route("/optimizing-builds")]
        mod optimizing_builds {
            page!(crate::pages::documentation::getting_started::optimizing_builds::PageOptimizingBuilds);
        }

        // The guide "Build Times & Bundle Size" became "Optimizing Compile Times & Binary Sizes".
        #[route("/build-times")]
        mod moved_build_times {
            page!(|| view! { <Redirect path=crate::routes::doc::OptimizingBuilds.materialize()/> });
        }

        // ── Concepts: buttons ───────────────────────────────────────────

        #[route("/buttons")]
        mod buttons {
            page!(crate::pages::documentation::groups::buttons::PageButtons);
        }

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

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::button::Atom.materialize())/> }
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

        // ── Concepts: fields ────────────────────────────────────────────

        #[route("/fields")]
        mod fields {
            page!(crate::pages::documentation::groups::fields::PageFields);
        }

        // The "Input" category became the "Fields" group.
        #[route("/input")]
        mod moved_input {
            page!(|| view! { <Redirect path=crate::routes::doc::Fields.materialize()/> });
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

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::checkbox::Atom.materialize())/> }
                );
            }
        }

        #[route("/field")]
        mod field {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::field::PageFieldOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::label::PageUseLabel);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::field::PageAtomField);
            }
        }

        #[route("/form")]
        mod form {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::form::PageFormOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::form::PageFormHooks);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::form::PageAtomForm);
            }
        }

        #[route("/number-field")]
        mod number_field {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::number_field::PageNumberFieldOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::number_field::PageUseNumberField);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::number_field::PageAtomNumberField);
            }

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::number_field::Atom.materialize())/> }
                );
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

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::radio::Atom.materialize())/> }
                );
            }
        }

        // The rich text editor is gone: the Content & Layout overview points to leptos-tiptap.
        #[route("/rich-text-editor")]
        mod moved_rich_text_editor {
            page!(
                || view! { <Redirect path=format!("{}#rich-content", crate::routes::doc::Layout.materialize())/> }
            );
        }

        #[route("/search-field")]
        mod search_field {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::search_field::PageSearchFieldOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::search_field::PageUseSearchField);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::search_field::PageAtomSearchField);
            }

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::search_field::Atom.materialize())/> }
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

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::slider::Atom.materialize())/> }
                );
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

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::switch::Atom.materialize())/> }
                );
            }
        }

        // The `Toggle` component became `Switch`.
        #[route("/toggle")]
        mod moved_toggle {
            index!(|| view! { <Redirect path=crate::routes::doc::Switch.materialize()/> });

            #[route("/hook")]
            mod hook {
                page!(|| view! { <Redirect path=crate::routes::doc::switch::Hook.materialize()/> });
            }

            #[route("/component")]
            mod component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::switch::Atom.materialize())/> }
                );
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

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::text_field::Atom.materialize())/> }
                );
            }

            #[route("/number-field-hook")]
            mod moved_number_field_hook {
                page!(
                    || view! { <Redirect path=crate::routes::doc::number_field::Hook.materialize()/> }
                );
            }

            #[route("/number-field-atom")]
            mod moved_number_field_atom {
                page!(
                    || view! { <Redirect path=crate::routes::doc::number_field::Atom.materialize()/> }
                );
            }
        }

        // ── Concepts: pickers ───────────────────────────────────────────

        #[route("/pickers")]
        mod pickers {
            page!(crate::pages::documentation::groups::pickers::PagePickers);
        }

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

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::select::Atom.materialize())/> }
                );
            }
        }

        // ── Concepts: collections ───────────────────────────────────────

        #[route("/collections")]
        mod collections {
            page!(crate::pages::documentation::groups::collections::PageCollections);
        }

        // The "Data Display" category became part of the "Collections" group.
        #[route("/data-display")]
        mod moved_data_display {
            page!(|| view! { <Redirect path=crate::routes::doc::Collections.materialize()/> });
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

            // The layout grid is a CSS recipe of the Content & Layout overview.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#grid-layout", crate::routes::doc::Layout.materialize())/> }
                );
            }
        }

        #[route("/grid-list")]
        mod grid_list {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::grid_list::PageGridListOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::grid_list::PageGridListHooks);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::grid_list::PageAtomGridList);
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

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::table::Atom.materialize())/> }
                );
            }
        }

        #[route("/tag-group")]
        mod tag_group {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::tag_group::PageTagGroupOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::tag::PageUseTag);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::tag_group::PageAtomTagGroup);
            }
        }

        #[route("/tree")]
        mod tree {
            page!(crate::pages::documentation::hooks::tree::PageUseTree);
        }

        // ── Concepts: date & time ───────────────────────────────────────

        #[route("/date-time")]
        mod date_time {
            index!(crate::pages::documentation::groups::date_time::PageDateTime);

            #[route("/calendar-hooks")]
            mod moved_calendar_hooks {
                page!(
                    || view! { <Redirect path=crate::routes::doc::calendar::Hook.materialize()/> }
                );
            }

            #[route("/date-field-hooks")]
            mod moved_date_field_hooks {
                page!(
                    || view! { <Redirect path=crate::routes::doc::date_field::Hook.materialize()/> }
                );
            }

            #[route("/date-picker-hooks")]
            mod moved_date_picker_hooks {
                page!(
                    || view! { <Redirect path=crate::routes::doc::date_picker::Hook.materialize()/> }
                );
            }

            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::date_picker::Atom.materialize())/> }
                );
            }
        }

        #[route("/calendar")]
        mod calendar {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::calendar::PageCalendarOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::calendar::PageCalendarHooks);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::calendar::PageAtomCalendar);
            }

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::calendar::Atom.materialize())/> }
                );
            }
        }

        #[route("/date-field")]
        mod date_field {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::date_field::PageDateFieldOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::date_field::PageDateFieldHooks);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::date_field::PageAtomDateField);
            }
        }

        #[route("/date-picker")]
        mod date_picker {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::date_picker::PageDatePickerOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::date_picker::PageDatePickerHooks);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::date_picker::PageAtomDatePicker);
            }

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::date_picker::Atom.materialize())/> }
                );
            }
        }

        #[route("/time-field")]
        mod time_field {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::time_field::PageTimeFieldOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::time_field::PageTimeFieldHooks);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::time_field::PageAtomTimeField);
            }
        }

        // ── Concepts: color ─────────────────────────────────────────────

        #[route("/color")]
        mod color {
            index!(crate::pages::documentation::groups::color::PageColor);

            #[route("/hooks")]
            mod moved_hooks {
                page!(
                    || view! { <Redirect path=crate::routes::doc::color_picker::Hook.materialize()/> }
                );
            }

            #[route("/atom")]
            mod moved_atom {
                page!(
                    || view! { <Redirect path=crate::routes::doc::color_area::Atom.materialize()/> }
                );
            }

            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::color_picker::Atom.materialize())/> }
                );
            }
        }

        #[route("/color-area")]
        mod color_area {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::color_area::PageColorAreaOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::color_area::PageUseColorArea);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::color_area::PageAtomColorArea);
            }
        }

        #[route("/color-field")]
        mod color_field {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::color_field::PageColorFieldOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::color_field::PageUseColorField);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::color_field::PageAtomColorField);
            }

            // The channel field is a section of the Color Field Hooks page.
            #[route("/channel")]
            mod moved_channel {
                page!(
                    || view! { <Redirect path=format!("{}#use-color-channel-field", crate::routes::doc::color_field::Hook.materialize())/> }
                );
            }
        }

        #[route("/color-picker")]
        mod color_picker {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::color_picker::PageColorPickerOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::color::PageUseColorHooks);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::color_picker::PageAtomColorPicker);
            }

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::color_picker::Atom.materialize())/> }
                );
            }
        }

        #[route("/color-slider")]
        mod color_slider {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::color_slider::PageColorSliderOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::color_slider::PageUseColorSlider);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::color_slider::PageAtomColorSlider);
            }
        }

        #[route("/color-swatch")]
        mod color_swatch {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::color_swatch::PageColorSwatchOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::color_swatch::PageUseColorSwatch);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::color_swatch::PageAtomColorSwatch);
            }
        }

        #[route("/color-swatch-picker")]
        mod color_swatch_picker {
            page!(
                crate::pages::documentation::atoms::color_swatch_picker::PageAtomColorSwatchPicker
            );
        }

        #[route("/color-wheel")]
        mod color_wheel {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::color_wheel::PageColorWheelOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::color_wheel::PageUseColorWheel);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::color_wheel::PageAtomColorWheel);
            }
        }

        // ── Concepts: overlays ──────────────────────────────────────────

        #[route("/overlays")]
        mod overlays {
            index!(crate::pages::documentation::groups::overlays::PageOverlays);

            #[route("/use-overlay")]
            mod moved_use_overlay {
                page!(
                    || view! { <Redirect path=crate::routes::doc::overlay_behavior::UseOverlay.materialize()/> }
                );
            }

            #[route("/dismiss-button")]
            mod moved_dismiss_button {
                page!(
                    || view! { <Redirect path=crate::routes::doc::overlay_behavior::DismissButton.materialize()/> }
                );
            }
        }

        #[route("/dialog")]
        mod dialog {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::dialog::PageDialogOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::dialog::PageUseDialog);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::dialog::PageAtomDialog);
            }
        }

        // The drawer is a section of the Modal Atoms.
        #[route("/drawer")]
        mod moved_drawer {
            page!(
                || view! { <Redirect path=format!("{}#drawer", crate::routes::doc::modal::Atom.materialize())/> }
            );
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

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::modal::Atom.materialize())/> }
                );
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

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::popover::Atom.materialize())/> }
                );
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

        // ── Concepts: navigation ────────────────────────────────────────

        #[route("/navigation")]
        mod navigation {
            page!(crate::pages::documentation::groups::navigation::PageNavigation);
        }

        #[route("/breadcrumbs")]
        mod breadcrumbs {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::breadcrumbs::PageBreadcrumbsOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::breadcrumbs::PageUseBreadcrumbs);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::breadcrumbs::PageAtomBreadcrumbs);
            }
        }

        #[route("/disclosure")]
        mod disclosure {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::disclosure::PageDisclosureOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::disclosure::PageUseDisclosure);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::disclosure::PageAtomDisclosure);
            }

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::disclosure::Atom.materialize())/> }
                );
            }
        }

        // The Collapsible concept became Disclosure.
        #[route("/collapsible")]
        mod moved_collapsible {
            index!(|| view! { <Redirect path=crate::routes::doc::Disclosure.materialize()/> });

            #[route("/hook")]
            mod hook {
                page!(
                    || view! { <Redirect path=crate::routes::doc::disclosure::Hook.materialize()/> }
                );
            }

            #[route("/atom")]
            mod atom {
                page!(
                    || view! { <Redirect path=crate::routes::doc::disclosure::Atom.materialize()/> }
                );
            }

            #[route("/component")]
            mod component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::disclosure::Atom.materialize())/> }
                );
            }
        }

        #[route("/link")]
        mod link {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::link::PageLinkOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::link::PageUseLink);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::link::PageAtomLink);
            }

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::link::Atom.materialize())/> }
                );
            }

            // The anchor link hook and atom are sections of the Hooks and Atoms tabs.
            #[route("/use-anchor-link")]
            mod moved_use_anchor_link {
                page!(
                    || view! { <Redirect path=format!("{}#use-anchor-link", crate::routes::doc::link::Hook.materialize())/> }
                );
            }

            #[route("/anchor-link-atom")]
            mod moved_anchor_link_atom {
                page!(
                    || view! { <Redirect path=format!("{}#anchorlink", crate::routes::doc::link::Atom.materialize())/> }
                );
            }

            #[route("/use-link")]
            mod moved_use_link {
                page!(|| view! { <Redirect path=crate::routes::doc::link::Hook.materialize()/> });
            }

            #[route("/link-atom")]
            mod moved_link_atom {
                page!(|| view! { <Redirect path=crate::routes::doc::link::Atom.materialize()/> });
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

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::tabs::Atom.materialize())/> }
                );
            }
        }

        // ── Concepts: status ────────────────────────────────────────────

        #[route("/status")]
        mod status {
            page!(crate::pages::documentation::groups::status::PageStatus);
        }

        // The "Feedback" category became the "Status" group.
        #[route("/feedback")]
        mod moved_feedback {
            page!(|| view! { <Redirect path=crate::routes::doc::Status.materialize()/> });
        }

        // The alert is a recipe of the Status overview.
        #[route("/alert")]
        mod moved_alert {
            page!(|| view! { <Redirect path=format!("{}#alert", crate::routes::doc::Status.materialize())/> });
        }

        #[route("/meter")]
        mod meter {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::meter::PageMeterOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::meter::PageUseMeter);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::meter::PageAtomMeter);
            }

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::meter::Atom.materialize())/> }
                );
            }
        }

        #[route("/progress-bar")]
        mod progress_bar {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::progress_bar::PageProgressBarOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::progress::PageUseProgressBar);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::progress_bar::PageAtomProgressBar);
            }

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::progress_bar::Atom.materialize())/> }
                );
            }
        }

        // The Progress concept became Progress Bar.
        #[route("/progress")]
        mod moved_progress {
            index!(|| view! { <Redirect path=crate::routes::doc::ProgressBar.materialize()/> });

            #[route("/hook")]
            mod hook {
                page!(
                    || view! { <Redirect path=crate::routes::doc::progress_bar::Hook.materialize()/> }
                );
            }

            #[route("/component")]
            mod component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::progress_bar::Atom.materialize())/> }
                );
            }
        }

        #[route("/toast")]
        mod toast {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::toast::PageToastOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::toast::PageToastHooks);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::toast::PageAtomToast);
            }

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::toast::Atom.materialize())/> }
                );
            }
        }

        // ── Concepts: content & layout ──────────────────────────────────

        #[route("/layout")]
        mod layout {
            page!(crate::pages::documentation::groups::layout::PageLayout);
        }

        // The "General" category became part of "Content & Layout".
        #[route("/general")]
        mod moved_general {
            page!(|| view! { <Redirect path=crate::routes::doc::Layout.materialize()/> });
        }

        // The app bar, cards, grid layouts, icons, sanitized HTML, skeletons, stacks and typography are recipes of the
        // Content & Layout overview.
        #[route("/app-bar")]
        mod moved_app_bar {
            page!(
                || view! { <Redirect path=format!("{}#app-bar", crate::routes::doc::Layout.materialize())/> }
            );
        }

        #[route("/card-and-tile")]
        mod moved_card_and_tile {
            page!(|| view! { <Redirect path=format!("{}#card", crate::routes::doc::Layout.materialize())/> });
        }

        // The Chip became the Tag Group concept.
        #[route("/chip")]
        mod moved_chip {
            index!(|| view! { <Redirect path=crate::routes::doc::TagGroup.materialize()/> });

            #[route("/hook")]
            mod hook {
                page!(|| view! { <Redirect path=crate::routes::doc::tag_group::Hook.materialize()/> });
            }

            #[route("/component")]
            mod component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::tag_group::Atom.materialize())/> }
                );
            }
        }

        #[route("/grid-layout")]
        mod moved_grid_layout {
            page!(
                || view! { <Redirect path=format!("{}#grid-layout", crate::routes::doc::Layout.materialize())/> }
            );
        }

        #[route("/icon")]
        mod moved_icon {
            page!(|| view! { <Redirect path=format!("{}#icons", crate::routes::doc::Layout.materialize())/> });
        }

        // A single page: the Kbd concept has only atoms.
        #[route("/kbd")]
        mod kbd {
            index!(crate::pages::documentation::atoms::kbd::PageAtomKbd);

            #[route("/atom")]
            mod moved_atom {
                page!(|| view! { <Redirect path=crate::routes::doc::Kbd.materialize()/> });
            }

            #[route("/component")]
            mod moved_component {
                page!(|| view! { <Redirect path=format!("{}#styling", crate::routes::doc::Kbd.materialize())/> });
            }
        }

        #[route("/sanitized-html")]
        mod moved_sanitized_html {
            page!(
                || view! { <Redirect path=format!("{}#rich-content", crate::routes::doc::Layout.materialize())/> }
            );
        }

        #[route("/separator")]
        mod separator {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::separator::PageSeparatorOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::separator::PageUseSeparatorHook);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::separator::PageAtomSeparator);
            }

            // The component layer is gone: the atom page's "Styling" section shows the atom theme.
            #[route("/component")]
            mod moved_component {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::separator::Atom.materialize())/> }
                );
            }
        }

        #[route("/skeleton")]
        mod moved_skeleton {
            page!(
                || view! { <Redirect path=format!("{}#skeleton", crate::routes::doc::Layout.materialize())/> }
            );
        }

        #[route("/stack")]
        mod moved_stack {
            page!(|| view! { <Redirect path=format!("{}#stack", crate::routes::doc::Layout.materialize())/> });
        }

        #[route("/toolbar")]
        mod toolbar {
            layout!(ConceptLayout);
            index!(crate::pages::documentation::concepts::toolbar::PageToolbarOverview);

            #[route("/hook")]
            mod hook {
                page!(crate::pages::documentation::hooks::toolbar::PageUseToolbar);
            }

            #[route("/atom")]
            mod atom {
                page!(crate::pages::documentation::atoms::toolbar::PageAtomToolbar);
            }
        }

        #[route("/typography")]
        mod moved_typography {
            page!(
                || view! { <Redirect path=format!("{}#typography", crate::routes::doc::Layout.materialize())/> }
            );
        }

        // ── Building blocks ─────────────────────────────────────────────

        #[route("/interactions")]
        mod interactions {
            index!(crate::pages::documentation::groups::interactions::PageInteractions);

            #[route("/use-press")]
            mod use_press {
                page!(crate::pages::documentation::hooks::press::PageUsePress);
            }

            #[route("/press-responder")]
            mod press_responder {
                page!(crate::pages::documentation::atoms::press_responder::PageAtomPressResponder);
            }

            #[route("/use-hover")]
            mod use_hover {
                page!(crate::pages::documentation::hooks::hover::PageUseHover);
            }

            #[route("/hoverable")]
            mod hoverable {
                page!(crate::pages::documentation::atoms::hoverable::PageAtomHoverable);
            }

            #[route("/use-move")]
            mod use_move {
                page!(crate::pages::documentation::hooks::r#move::PageUseMove);
            }

            #[route("/use-keyboard")]
            mod use_keyboard {
                page!(crate::pages::documentation::hooks::keyboard::PageUseKeyboard);
            }

            #[route("/use-global-shortcuts")]
            mod use_global_shortcuts {
                page!(crate::pages::documentation::hooks::global_shortcuts::PageUseGlobalShortcuts);
            }

            #[route("/use-context-menu")]
            mod use_context_menu {
                page!(crate::pages::documentation::hooks::context_menu::PageUseContextMenu);
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
            mod moved_use_prevent_scroll {
                page!(
                    || view! { <Redirect path=crate::routes::doc::overlay_behavior::UsePreventScroll.materialize()/> }
                );
            }

            #[route("/dnd")]
            mod moved_dnd {
                page!(|| view! { <Redirect path=crate::routes::doc::DragAndDrop.materialize()/> });
            }
        }

        #[route("/focus")]
        mod focus {
            index!(crate::pages::documentation::groups::focus::PageFocus);

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

            #[route("/focusable")]
            mod focusable {
                page!(crate::pages::documentation::atoms::focusable::PageAtomFocusable);
            }

            #[route("/use-focus-visible")]
            mod use_focus_visible {
                page!(crate::pages::documentation::hooks::focus_visible::PageUseFocusVisible);
            }

            #[route("/use-focus-ring")]
            mod use_focus_ring {
                page!(crate::pages::documentation::hooks::focus_ring::PageUseFocusRing);
            }

            #[route("/focus-ring")]
            mod focus_ring {
                page!(crate::pages::documentation::atoms::focus_ring::PageAtomFocusRing);
            }

            #[route("/focus-scope")]
            mod focus_scope {
                page!(crate::pages::documentation::atoms::focus_scope::PageAtomFocusScope);
            }

            #[route("/use-focus-manager")]
            mod use_focus_manager {
                page!(crate::pages::documentation::hooks::focus_manager::PageUseFocusManager);
            }

            #[route("/focus-manager-provider")]
            mod focus_manager_provider {
                page!(
                    crate::pages::documentation::atoms::focus_manager_provider::PageAtomFocusManagerProvider
                );
            }

            // The atom `FocusManager` became `FocusManagerProvider`.
            #[route("/focus-manager")]
            mod moved_focus_manager {
                page!(
                    || view! { <Redirect path=crate::routes::doc::focus::FocusManagerProvider.materialize()/> }
                );
            }

            #[route("/use-landmark")]
            mod use_landmark {
                page!(crate::pages::documentation::hooks::landmark::PageUseLandmark);
            }

            #[route("/use-has-tabbable-child")]
            mod use_has_tabbable_child {
                page!(
                    crate::pages::documentation::hooks::has_tabbable_child::PageUseHasTabbableChild
                );
            }

            #[route("/focusability")]
            mod focusability {
                page!(crate::pages::documentation::utils::focusability::PageFocusability);
            }

            #[route("/virtual-focus")]
            mod virtual_focus {
                page!(crate::pages::documentation::utils::virtual_focus::PageVirtualFocus);
            }
        }

        #[route("/overlay-behavior")]
        mod overlay_behavior {
            index!(crate::pages::documentation::groups::overlay_behavior::PageOverlayBehavior);

            #[route("/use-overlay-trigger-state")]
            mod use_overlay_trigger_state {
                page!(
                    crate::pages::documentation::hooks::overlay_trigger_state::PageUseOverlayTriggerState
                );
            }

            #[route("/use-overlay")]
            mod use_overlay {
                page!(crate::pages::documentation::hooks::overlay::PageUseOverlay);
            }

            #[route("/use-overlay-trigger")]
            mod use_overlay_trigger {
                page!(crate::pages::documentation::hooks::overlay_trigger::PageUseOverlayTrigger);
            }

            #[route("/use-overlay-position")]
            mod use_overlay_position {
                page!(crate::pages::documentation::hooks::overlay_position::PageUseOverlayPosition);
            }

            #[route("/use-close-on-scroll")]
            mod use_close_on_scroll {
                page!(crate::pages::documentation::hooks::close_on_scroll::PageUseCloseOnScroll);
            }

            #[route("/use-prevent-scroll")]
            mod use_prevent_scroll {
                page!(crate::pages::documentation::hooks::prevent_scroll::PageUsePreventScroll);
            }

            #[route("/aria-hide-outside")]
            mod aria_hide_outside {
                page!(crate::pages::documentation::utils::aria_hide_outside::PageAriaHideOutside);
            }

            #[route("/use-overlay-focus-contain")]
            mod use_overlay_focus_contain {
                page!(
                    crate::pages::documentation::hooks::overlay_focus_contain::PageUseOverlayFocusContain
                );
            }

            #[route("/dismiss-button")]
            mod dismiss_button {
                page!(crate::pages::documentation::atoms::dismiss_button::PageAtomDismissButton);
            }
        }

        #[route("/collection-state")]
        mod collection_state {
            index!(crate::pages::documentation::groups::collection_state::PageCollectionState);

            #[route("/virtualizer")]
            mod virtualizer {
                page!(crate::pages::documentation::atoms::virtualizer::PageAtomVirtualizer);
            }

            #[route("/use-virtualizer-state")]
            mod use_virtualizer_state {
                page!(crate::pages::documentation::hooks::virtualizer::PageUseVirtualizerState);
            }
        }

        // The selection hooks were merged into the collection state.
        #[route("/selection")]
        mod moved_selection {
            page!(|| view! { <Redirect path=crate::routes::doc::CollectionState.materialize()/> });
        }

        #[route("/drag-and-drop")]
        mod drag_and_drop {
            page!(crate::pages::documentation::hooks::dnd::PageUseDnd);
        }

        #[route("/animation")]
        mod animation {
            index!(crate::pages::documentation::hooks::animation::PageAnimationHooks);

            // The transition components are gone: the overview shows how to animate with CSS.
            #[route("/transitions")]
            mod moved_transitions {
                page!(|| view! { <Redirect path=crate::routes::doc::Animation.materialize()/> });
            }
        }

        #[route("/screen-readers")]
        mod screen_readers {
            #[route("/live-announcer")]
            mod live_announcer {
                page!(crate::pages::documentation::utils::live_announcer::PageLiveAnnouncer);
            }

            #[route("/use-visually-hidden")]
            mod use_visually_hidden {
                page!(crate::pages::documentation::hooks::visually_hidden::PageUseVisuallyHidden);
            }

            #[route("/visually-hidden")]
            mod visually_hidden {
                page!(crate::pages::documentation::atoms::visually_hidden::PageAtomVisuallyHidden);
            }

            #[route("/use-description")]
            mod use_description {
                page!(crate::pages::documentation::utils::use_description::PageUseDescription);
            }
        }

        #[route("/utilities")]
        mod utilities {
            #[route("/i18n-provider")]
            mod i18n_provider {
                page!(crate::pages::documentation::utils::i18n_provider::PageI18nProvider);
            }

            #[route("/number-formatter")]
            mod number_formatter {
                page!(crate::pages::documentation::utils::number_formatter::PageNumberFormatter);
            }

            #[route("/date-time-formatter")]
            mod date_time_formatter {
                page!(
                    crate::pages::documentation::utils::date_time_formatter::PageDateTimeFormatter
                );
            }

            #[route("/list-formatter")]
            mod list_formatter {
                page!(crate::pages::documentation::utils::list_formatter::PageListFormatter);
            }

            #[route("/use-localized-strings")]
            mod use_localized_strings {
                page!(crate::pages::documentation::utils::localized_strings::PageLocalizedStrings);
            }

            #[route("/collator")]
            mod collator {
                page!(crate::pages::documentation::utils::collator::PageCollator);
            }

            #[route("/scroll")]
            mod scroll {
                page!(crate::pages::documentation::utils::scroll::PageScroll);
            }

            #[route("/use-spin-button")]
            mod use_spin_button {
                page!(crate::pages::documentation::hooks::spin_button::PageUseSpinButton);
            }

            #[route("/use-clipboard")]
            mod use_clipboard {
                page!(crate::pages::documentation::hooks::clipboard::PageUseClipboard);
            }
        }

        // ── Moved pages ─────────────────────────────────────────────────

        #[route("/hooks")]
        mod moved_hooks {
            #[route("/animation")]
            mod animation {
                page!(|| view! { <Redirect path=crate::routes::doc::Animation.materialize()/> });
            }

            #[route("/use-label")]
            mod use_label {
                page!(|| view! { <Redirect path=crate::routes::doc::field::Hook.materialize()/> });
            }

            #[route("/use-breadcrumbs")]
            mod use_breadcrumbs {
                page!(
                    || view! { <Redirect path=crate::routes::doc::breadcrumbs::Hook.materialize()/> }
                );
            }

            #[route("/use-meter")]
            mod use_meter {
                page!(|| view! { <Redirect path=crate::routes::doc::meter::Hook.materialize()/> });
            }

            #[route("/use-color-area")]
            mod use_color_area {
                page!(
                    || view! { <Redirect path=crate::routes::doc::color_area::Hook.materialize()/> }
                );
            }

            #[route("/use-color-slider")]
            mod use_color_slider {
                page!(
                    || view! { <Redirect path=crate::routes::doc::color_slider::Hook.materialize()/> }
                );
            }

            #[route("/use-color-wheel")]
            mod use_color_wheel {
                page!(
                    || view! { <Redirect path=crate::routes::doc::color_wheel::Hook.materialize()/> }
                );
            }

            #[route("/use-color-field")]
            mod use_color_field {
                page!(
                    || view! { <Redirect path=crate::routes::doc::color_field::Hook.materialize()/> }
                );
            }

            #[route("/use-color-swatch")]
            mod use_color_swatch {
                page!(
                    || view! { <Redirect path=crate::routes::doc::color_swatch::Hook.materialize()/> }
                );
            }

            #[route("/use-color-channel-field")]
            mod use_color_channel_field {
                page!(
                    || view! { <Redirect path=format!("{}#use-color-channel-field", crate::routes::doc::color_field::Hook.materialize())/> }
                );
            }

            #[route("/use-spin-button")]
            mod use_spin_button {
                page!(
                    || view! { <Redirect path=crate::routes::doc::utilities::UseSpinButton.materialize()/> }
                );
            }

            #[route("/use-toolbar")]
            mod use_toolbar {
                page!(
                    || view! { <Redirect path=crate::routes::doc::toolbar::Hook.materialize()/> }
                );
            }

            #[route("/use-tree")]
            mod use_tree {
                page!(|| view! { <Redirect path=crate::routes::doc::Tree.materialize()/> });
            }

            #[route("/selection")]
            mod selection {
                page!(
                    || view! { <Redirect path=crate::routes::doc::CollectionState.materialize()/> }
                );
            }
        }

        #[route("/atoms")]
        mod moved_atoms {
            #[route("/field")]
            mod field {
                page!(|| view! { <Redirect path=crate::routes::doc::field::Atom.materialize()/> });
            }

            #[route("/form")]
            mod form {
                page!(|| view! { <Redirect path=crate::routes::doc::form::Atom.materialize()/> });
            }

            #[route("/breadcrumbs")]
            mod breadcrumbs {
                page!(
                    || view! { <Redirect path=crate::routes::doc::breadcrumbs::Atom.materialize()/> }
                );
            }

            #[route("/toolbar")]
            mod toolbar {
                page!(
                    || view! { <Redirect path=crate::routes::doc::toolbar::Atom.materialize()/> }
                );
            }
        }

        #[route("/utils")]
        mod moved_utils {
            #[route("/live-announcer")]
            mod live_announcer {
                page!(
                    || view! { <Redirect path=crate::routes::doc::screen_readers::LiveAnnouncer.materialize()/> }
                );
            }
        }

        #[route("/components")]
        mod moved_components {
            #[route("/transitions")]
            mod transitions {
                page!(
                    || view! { <Redirect path=crate::routes::doc::Animation.materialize()/> }
                );
            }

            #[route("/stack")]
            mod stack {
                page!(|| view! { <Redirect path=format!("{}#stack", crate::routes::doc::Layout.materialize())/> });
            }

            #[route("/skeleton")]
            mod skeleton {
                page!(|| view! { <Redirect path=format!("{}#skeleton", crate::routes::doc::Layout.materialize())/> });
            }

            #[route("/app-bar")]
            mod app_bar {
                page!(|| view! { <Redirect path=format!("{}#app-bar", crate::routes::doc::Layout.materialize())/> });
            }

            #[route("/drawer")]
            mod drawer {
                page!(|| view! { <Redirect path=format!("{}#drawer", crate::routes::doc::modal::Atom.materialize())/> });
            }

            #[route("/date-time")]
            mod date_time {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::date_picker::Atom.materialize())/> }
                );
            }

            #[route("/tiptap-editor")]
            mod tiptap_editor {
                page!(
                    || view! { <Redirect path=format!("{}#rich-content", crate::routes::doc::Layout.materialize())/> }
                );
            }

            #[route("/alert")]
            mod alert {
                page!(|| view! { <Redirect path=format!("{}#alert", crate::routes::doc::Status.materialize())/> });
            }

            #[route("/toast")]
            mod toast {
                page!(|| view! { <Redirect path=crate::routes::doc::Toast.materialize()/> });
            }

            #[route("/kbd")]
            mod kbd {
                page!(
                    || view! { <Redirect path=format!("{}#styling", crate::routes::doc::Kbd.materialize())/> }
                );
            }

            #[route("/typography")]
            mod typography {
                page!(|| view! { <Redirect path=format!("{}#typography", crate::routes::doc::Layout.materialize())/> });
            }

            #[route("/icon")]
            mod icon {
                page!(|| view! { <Redirect path=format!("{}#icons", crate::routes::doc::Layout.materialize())/> });
            }

            #[route("/callback")]
            mod callback {
                page!(|| view! { <Redirect path=crate::routes::doc::Callbacks.materialize()/> });
            }
        }
    }

    #[route("/not-found")]
    mod not_found {
        page!(crate::pages::err404::PageErr404);
    }
}
