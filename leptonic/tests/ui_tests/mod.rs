pub mod test_aria_hide_outside;
pub mod test_breadcrumbs;
pub mod test_button;
pub mod test_calendar;
pub mod test_checkbox;
pub mod test_clipboard;
pub mod test_clipboard_write;
pub mod test_color_area;
pub mod test_color_field;
pub mod test_color_picker;
pub mod test_color_slider;
pub mod test_color_swatch;
pub mod test_color_wheel;
pub mod test_combobox;
pub mod test_combobox_forms;
pub mod test_context_menu;
pub mod test_context_menu_atoms;
pub mod test_date_field;
pub mod test_date_picker;
pub mod test_dialog;
pub mod test_disclosure;
pub mod test_dismiss_button;
pub mod test_dnd;
pub mod test_dnd_collection;
pub mod test_dnd_draggable_collection;
pub mod test_dnd_native;
pub mod test_focus;
pub mod test_focus_manager;
pub mod test_focus_ring;
pub mod test_focus_safely;
pub mod test_focus_scope;
pub mod test_focus_visible;
pub mod test_focus_within;
pub mod test_focusable;
pub mod test_focusable_atoms;
pub mod test_forms;
pub mod test_global_shortcuts;
pub mod test_grid;
pub mod test_grid_list;
pub mod test_grid_list_cases;
pub mod test_grid_list_features;
pub mod test_has_tabbable_child;
pub mod test_helpers;
pub mod test_hover;
pub mod test_hydration_ids;
pub mod test_interact_outside;
pub mod test_keyboard;
pub mod test_label_slots;
pub mod test_landmark;
pub mod test_link;
pub mod test_listbox;
pub mod test_listbox_features;
pub mod test_listbox_selection;
pub mod test_live_announcer;
pub mod test_localized_atoms;
pub mod test_long_press;
pub mod test_menu;
pub mod test_menu_atoms;
pub mod test_menu_trigger;
pub mod test_move;
pub mod test_number_field;
pub mod test_number_field_atoms;
pub mod test_overlay;
pub mod test_overlay_position;
pub mod test_overlay_state;
pub mod test_popover;
pub mod test_press;
pub mod test_pressable;
pub mod test_progress_bar;
pub mod test_radio_group;
pub mod test_scroll;
pub mod test_scroll_wheel;
pub mod test_search_field;
pub mod test_select;
pub mod test_select_behavior;
pub mod test_select_forms;
pub mod test_separator;
pub mod test_server_panics;
pub mod test_slider;
pub mod test_spin_button;
pub mod test_submenu;
pub mod test_switch;
pub mod test_table;
pub mod test_table_navigation;
pub mod test_table_resizing;
pub mod test_table_selection;
pub mod test_table_tree;
pub mod test_tabs;
pub mod test_tag_group;
pub mod test_tag_group_atoms;
pub mod test_text_field;
pub mod test_text_field_atoms;
pub mod test_theme;
pub mod test_toast;
pub mod test_toggle_button;
pub mod test_toolbar;
pub mod test_tooltip;
pub mod test_tree;
pub mod test_use_button;
pub mod test_virtual_list;
pub mod test_virtualizer;
pub mod test_visually_hidden;

use browser_test::{BrowserTests, InvalidEnvVar, Parallelism, TestFilter, TestGroup};

use crate::harness::fixture_test;

/// All registrations and logical groups live here, independently of implementation modules.
/// Group members share the suite's parallelism limit. Known issues are a separate opt-in suite.
pub fn all(parallelism: Parallelism) -> Result<BrowserTests<str>, InvalidEnvVar> {
    let known_issues = std::env::var("BROWSER_TEST_KNOWN_ISSUES").is_ok_and(|value| value == "1");
    let tests = if known_issues {
        known_issues_tests(parallelism)
    } else {
        regular_tests(parallelism)
    };
    let after = BrowserTests::sequential().named("after all").run_always();
    let after = if known_issues {
        after
    } else {
        after.with(fixture_test(test_server_panics::ServerPanicTests {}))
    };
    Ok(BrowserTests::sequential()
        .with_group(tests.filter(&TestFilter::from_env()?))
        .with_group(after))
}

fn known_issues_tests(parallelism: Parallelism) -> BrowserTests<str> {
    BrowserTests::parallel(parallelism).with_test_group(TestGroup::new("virtual_list").with(
        fixture_test(test_virtual_list::RebuiltComponentSpreadDropsTheOldHandlers {}),
    ))
}

fn regular_tests(parallelism: Parallelism) -> BrowserTests<str> {
    let tests = BrowserTests::parallel(parallelism)
        .with_test_group(
            TestGroup::new("helpers")
                .with(fixture_test(test_helpers::LookupContract {}))
                .with(fixture_test(test_helpers::AccessibilityContract {}))
                .with(fixture_test(test_helpers::ReplacementContract {}))
                .with(fixture_test(test_helpers::StabilityContract {}))
                .with(fixture_test(test_helpers::RecordingContract {}))
                .with(fixture_test(test_helpers::DiagnosticsContract {}))
                .with(fixture_test(test_helpers::EventContract {})),
        )
        .with_test_group(
            TestGroup::new("table")
                .with(fixture_test(test_table_resizing::InitialWidths {}))
                .with(fixture_test(test_table_resizing::ResizingEachColumn {}))
                .with(fixture_test(
                    test_table_resizing::CannotResizeBelowTheMinWidth {},
                ))
                .with(fixture_test(
                    test_table_resizing::ResizingTheFirstColumnPreservesFrRatios {},
                ))
                .with(fixture_test(
                    test_table_resizing::ResizingTheLastColumnLocksTheColumnsBeforeIt {},
                ))
                .with(fixture_test(
                    test_table_resizing::OnResizeStartReportsTheSizes {},
                ))
                .with(fixture_test(
                    test_table_resizing::OnResizeEndWithoutMoving {},
                ))
                .with(fixture_test(test_table_resizing::KeyboardResizing {}))
                .with(fixture_test(
                    test_table_resizing::ExitingKeyboardResizing {},
                ))
                .with(fixture_test(test_table::AriaStructure {}))
                .with(fixture_test(test_table::ColumnGroups {}))
                .with(fixture_test(test_table::NavigationIntoTheColumnHeaders {}))
                .with(fixture_test(test_table::Sorting {}))
                .with(fixture_test(test_table::SortableColumnsAreDescribed {}))
                .with(fixture_test(test_table::HoverOnTheTableHeader {}))
                .with(fixture_test(test_table::SelectAll {}))
                .with(fixture_test(test_table::DisabledRows {}))
                .with(fixture_test(test_table::TypeAhead {}))
                .with(fixture_test(test_table::RemovingTheFocusedRow {}))
                .with(fixture_test(test_table::Localized {}))
                .with(fixture_test(
                    test_table_navigation::TabFromACellFocusesItsFirstTabbableChild {},
                ))
                .with(fixture_test(
                    test_table_navigation::TabFromACellWithoutChildrenExitsTheTable {},
                ))
                .with(fixture_test(
                    test_table_navigation::ShiftTabFromAChildReturnsToTheCell {},
                ))
                .with(fixture_test(
                    test_table_navigation::KeysInATextInputStayThere {},
                ))
                .with(fixture_test(test_table_navigation::ClickingAChildOrARow {}))
                .with(fixture_test(
                    test_table_navigation::ChildFocusModeInTabNavigation {},
                ))
                .with(fixture_test(
                    test_table_navigation::ArrowNavigationThroughCellChildren {},
                ))
                .with(fixture_test(
                    test_table_navigation::ArrowNavigationWithCellFocusMode {},
                ))
                .with(fixture_test(test_table_navigation::RightToLeft {}))
                .with(fixture_test(
                    test_table_navigation::PageUpReachesTheColumnHeaders {},
                ))
                .with(fixture_test(test_table_navigation::ColumnSpans {}))
                .with(fixture_test(test_table_navigation::AnEmptyTable {}))
                .with(fixture_test(
                    test_table_navigation::EnterOnAButtonThatIsNotTheFirstChild {},
                ))
                .with(fixture_test(
                    test_table_selection::ReplaceSelectionWithTheMouse {},
                ))
                .with(fixture_test(
                    test_table_selection::ReplaceSelectionInSingleMode {},
                ))
                .with(fixture_test(
                    test_table_selection::ReplaceSelectionWithTheKeyboard {},
                ))
                .with(fixture_test(test_table_selection::EscapeWithoutClearing {}))
                .with(fixture_test(test_table_selection::SelectOnPressDownOrUp {}))
                .with(fixture_test(test_table_selection::RowActions {}))
                .with(fixture_test(test_table_selection::ChangingColumns {}))
                .with(fixture_test(
                    test_table_selection::TheSelectionColumnFollowsTheSelectionMode {},
                ))
                .with(fixture_test(
                    test_table_selection::RemovingTheFocusedColumnHeader {},
                ))
                .with(fixture_test(
                    test_table_selection::SelectAllShortcutInSingleMode {},
                ))
                .with(fixture_test(test_table_selection::HoverAndFocusStates {}))
                .with(fixture_test(test_table_tree::RendersATreegrid {}))
                .with(fixture_test(test_table_tree::ExpandsARowWithTheMouse {}))
                .with(fixture_test(test_table_tree::ExpandsARowWithTheKeyboard {}))
                .with(fixture_test(
                    test_table_tree::ExpandsARowWithTheKeyboardRtl {},
                ))
                .with(fixture_test(test_table_tree::DefaultExpandedKeys {}))
                .with(fixture_test(test_table_tree::ControlledExpandedKeys {}))
                .with(fixture_test(
                    test_table_tree::KeyboardNavigationOfFlattenedRows {},
                ))
                .with(fixture_test(test_table_tree::KeyboardNavigationOfCells {}))
                .with(fixture_test(test_table_tree::Selection {}))
                .with(fixture_test(
                    test_table_tree::TypeAheadSearchesTheRowsShown {},
                ))
                .with(fixture_test(
                    test_table_tree::ArrowLeftMovesFromAChildRowToItsParent {},
                ))
                .with(fixture_test(
                    test_table_tree::CollapsingFromOutsideMovesFocusToTheParent {},
                ))
                .with(fixture_test(test_table_tree::LeafRowsAreNeverExpanded {})),
        )
        .with_test_group(
            TestGroup::new("button")
                .with(fixture_test(test_button::PressesAndProps {}))
                .with(fixture_test(test_button::StateAttributes {}))
                .with(fixture_test(test_button::Pending {}))
                .with(fixture_test(test_button::PendingFormSubmission {}))
                .with(fixture_test(test_button::PendingLabelled {}))
                .with(fixture_test(test_button::PendingTrigger {})),
        )
        .with_test_group(
            TestGroup::new("focus")
                .with(fixture_test(test_focus::BasicFocus {}))
                .with(fixture_test(test_focus::TabFocus {}))
                .with(fixture_test(test_focus::FocusChangeCount {}))
                .with(fixture_test(test_focus::ChildFocusDoesNotTriggerParent {}))
                .with(fixture_test(test_focus::BlurWhenDisabledWhileFocused {}))
                .with(fixture_test(test_focus::ShadowDomFocusEvents {}))
                .with(fixture_test(test_focus::ShadowDomDisabled {}))
                .with(fixture_test(test_focus_within::BasicFocusWithin {}))
                .with(fixture_test(test_focus_within::Disabled {}))
                .with(fixture_test(test_focus_within::ChangeCallback {}))
                .with(fixture_test(test_focus_within::TabIntoAndOutOfContainer {}))
                .with(fixture_test(test_focus_within::NestedFocusWithin {}))
                .with(fixture_test(
                    test_focus_within::FocusOutsideAfterAHiddenBlur {},
                ))
                .with(fixture_test(test_focus_within::RemovalOfTheFocusedChild {}))
                .with(fixture_test(
                    test_focus_within::DisablingTheFocusedElement {},
                ))
                .with(fixture_test(test_focus_ring::BasicClickFocus {}))
                .with(fixture_test(test_focus_ring::BasicTabFocus {}))
                .with(fixture_test(test_focus_ring::WithinClickFocus {}))
                .with(fixture_test(test_focus_ring::WithinTabFocus {}))
                .with(fixture_test(test_focus_ring::ModalitySwitch {}))
                .with(fixture_test(test_focus_ring::ArrowKeyKeyboardModality {}))
                .with(fixture_test(test_focus_ring::DisabledFocusRing {}))
                .with(fixture_test(test_focus_ring::AtomTextInput {}))
                .with(fixture_test(test_focus_safely::Connected {}))
                .with(fixture_test(test_focus_safely::NoLongerConnected {}))
                .with(fixture_test(test_focus_safely::Svg {}))
                .with(fixture_test(test_focus_safely::ConnectedInShadowDom {}))
                .with(fixture_test(
                    test_focus_safely::NoLongerConnectedInShadowDom {},
                ))
                .with(fixture_test(test_focusable_atoms::FocusableChild {}))
                .with(fixture_test(test_focusable_atoms::DisabledAndExcluded {}))
                .with(fixture_test(
                    test_focusable_atoms::FocusableTooltipTrigger {},
                ))
                .with(fixture_test(test_focusable_atoms::MergedProps {}))
                .with(fixture_test(
                    test_focusable_atoms::PressableTooltipTrigger {},
                ))
                .with(fixture_test(test_focusable_atoms::AutoFocus {}))
                .with(fixture_test(
                    test_focusable_atoms::TooltipFollowsDisabled {},
                ))
                .with(fixture_test(test_focusable::TabindexAttributes {}))
                .with(fixture_test(test_focusable::KeyboardEvents {}))
                .with(fixture_test(test_focusable::TabSkip {}))
                .with(fixture_test(test_focusable::FocusHandle {}))
                .with(fixture_test(test_focusable::DynamicDisabledTransition {}))
                .with(fixture_test(test_focus_manager::BasicNavigation {}))
                .with(fixture_test(test_focus_manager::WrapNext {}))
                .with(fixture_test(test_focus_manager::WrapPrev {}))
                .with(fixture_test(test_focus_manager::NowrapBoundaryNext {}))
                .with(fixture_test(test_focus_manager::NowrapBoundaryPrev {}))
                .with(fixture_test(test_focus_manager::TabbableSkip {}))
                .with(fixture_test(test_focus_manager::NontabbableInclude {}))
                .with(fixture_test(test_focus_manager::AcceptFilter {}))
                .with(fixture_test(test_focus_manager::RadioGroupChecked {}))
                .with(fixture_test(test_focus_manager::RadioGroupNoneChecked {}))
                .with(fixture_test(test_focus_manager::RadioGroupWrapNext {}))
                .with(fixture_test(test_focus_manager::RadioGroupWrapPrev {}))
                .with(fixture_test(test_focus_manager::HiddenElementsSkipped {}))
                .with(fixture_test(test_focus_manager::InertElementsSkipped {}))
                .with(fixture_test(
                    test_focus_manager::FocusNextFromOutsideScope {},
                ))
                .with(fixture_test(
                    test_focus_manager::FocusPreviousFromOutsideScope {},
                ))
                .with(fixture_test(test_focus_manager::ScopeManager {}))
                .with(fixture_test(test_focus_manager::FromAContainer {}))
                .with(fixture_test(
                    test_focus_visible::ClickSetsPointerModality {},
                ))
                .with(fixture_test(test_focus_visible::TabSetsKeyboardModality {}))
                .with(fixture_test(
                    test_focus_visible::ArrowKeySetsKeyboardModality {},
                ))
                .with(fixture_test(
                    test_focus_visible::TypingOnNonTextInputSetsKeyboardModality {},
                ))
                .with(fixture_test(
                    test_focus_visible::TypingInTextInputDoesNotShowFocus {},
                ))
                .with(fixture_test(
                    test_focus_visible::TypingInTextInputSilentlyUpdatesStoredModality {},
                ))
                .with(fixture_test(
                    test_focus_visible::BeforeunloadKeepsTracking {},
                ))
                .with(fixture_test(test_focus_visible::OtherWindowTracking {}))
                .with(fixture_test(test_focus_visible::OtherWindowBeforeunload {}))
                .with(fixture_test(test_focus_visible::OtherWindowTeardown {}))
                .with(fixture_test(
                    test_focus_visible::OtherWindowPointerThenEscape {},
                ))
                .with(fixture_test(test_focus_visible::PointerAfterKeyboard {}))
                .with(fixture_test(
                    test_focus_visible::EscapeSetsKeyboardModality {},
                ))
                .with(fixture_test(
                    test_focus_visible::EnterSetsKeyboardModality {},
                ))
                .with(fixture_test(
                    test_focus_visible::SpaceSetsKeyboardModality {},
                ))
                .with(fixture_test(
                    test_focus_visible::FocusWithoutAPrecedingEventIsVirtual {},
                ))
                .with(fixture_test(
                    test_focus_visible::ProgrammaticFocusKeepsTheModality {},
                ))
                .with(fixture_test(
                    test_focus_visible::WindowRefocusKeepsTheModality {},
                ))
                .with(fixture_test(
                    test_focus_visible::SafariWindowRefocusKeepsTheModality {},
                ))
                .with(fixture_test(
                    test_focus_visible::FocusMovedAfterInvalidShowsFocus {},
                ))
                .with(fixture_test(test_has_tabbable_child::WithTabbableChild {}))
                .with(fixture_test(
                    test_has_tabbable_child::ChildRemovedAndReAdded {},
                ))
                .with(fixture_test(test_has_tabbable_child::NoTabbableChildren {}))
                .with(fixture_test(
                    test_has_tabbable_child::DeeplyNestedTabbableChild {},
                ))
                .with(fixture_test(
                    test_has_tabbable_child::ChildDisabledAttributeChange {},
                ))
                .with(fixture_test(test_focus_scope::AutoFocus {}))
                .with(fixture_test(test_focus_scope::TabWrapping {}))
                .with(fixture_test(test_focus_scope::ShiftTabWrapping {}))
                .with(fixture_test(test_focus_scope::FocusRestoration {}))
                .with(fixture_test(test_focus_scope::NestedScopes {}))
                .with(fixture_test(test_focus_scope::ContainmentBlocksEscape {}))
                .with(fixture_test(test_focus_scope::OuterToInnerNavigation {}))
                .with(fixture_test(
                    test_focus_scope::NestedRestoreFocusesOutermost {},
                ))
                .with(fixture_test(test_focus_scope::RestoreFallback {}))
                .with(fixture_test(
                    test_focus_scope::RestoreFallbackWithoutTabbables {},
                ))
                .with(fixture_test(test_focus_scope::DialogFromMenu {}))
                .with(fixture_test(test_focus_scope::RestoreOnBlur {}))
                .with(fixture_test(test_focus_scope::SelectOnTab {}))
                .with(fixture_test(
                    test_focus_scope::TabOutsideTheScopeIsNative {},
                ))
                .with(fixture_test(test_focus_scope::RuntimeContain {}))
                .with(fixture_test(test_focus_scope::CancelledRestore {}))
                .with(fixture_test(
                    test_focus_scope::RestoreEventStaysInNestedScopes {},
                ))
                .with(fixture_test(test_focus_scope::TabOutOfRestoringScope {}))
                .with(fixture_test(test_focus_scope::MultipleFocusScopes {}))
                .with(fixture_test(test_focus_scope::SkipsNonTabbableElements {}))
                .with(fixture_test(
                    test_focus_scope::SkipsOnlyNonEditableContent {},
                ))
                .with(fixture_test(test_focus_scope::ModifierTabDoesNothing {}))
                .with(fixture_test(
                    test_focus_scope::RestoreAfterChildrenChange {},
                ))
                .with(fixture_test(
                    test_focus_scope::TabSkipsTheScopeAfterItsTrigger {},
                ))
                .with(fixture_test(
                    test_focus_scope::NoTabHandlingWithoutRestore {},
                ))
                .with(fixture_test(
                    test_focus_scope::DomOrderWithoutNodeToRestore {},
                ))
                .with(fixture_test(test_focus_scope::AutoFocusKeepsFocusInside {}))
                .with(fixture_test(
                    test_focus_scope::AutoFocusFallsBackToFocusable {},
                ))
                .with(fixture_test(
                    test_focus_scope::FocusFallsBackToTheFirstFocusable {},
                ))
                .with(fixture_test(
                    test_focus_scope::PortalChildScopeWithoutContain {},
                ))
                .with(fixture_test(
                    test_focus_scope::PortalChildScopeWithContain {},
                ))
                .with(fixture_test(
                    test_focus_scope::ChildScopeActiveRegardlessOfDom {},
                ))
                .with(fixture_test(
                    test_focus_scope::RestoresToTheCorrectScopeOnUnmount {},
                ))
                .with(fixture_test(test_focus_scope::StackedDialogsInline {}))
                .with(fixture_test(
                    test_focus_scope::StackedDialogsInlineContaining {},
                ))
                .with(fixture_test(test_focus_scope::StackedDialogsPortaled {}))
                .with(fixture_test(
                    test_focus_scope::StackedDialogsPortaledContaining {},
                ))
                .with(fixture_test(test_focus_scope::ShadowDomContainment {}))
                .with(fixture_test(test_focus_scope::ShadowDomLockBackwards {}))
                .with(fixture_test(test_focus_scope::NestedShadowDom {}))
                .with(fixture_test(
                    test_focus_scope::ShadowDomUnmountKeepsOutsideFocus {},
                )),
        )
        .with_test_group(
            TestGroup::new("scroll")
                .with(fixture_test(test_scroll::ScrollParents {}))
                .with(fixture_test(test_scroll::ScrollIntoView {})),
        )
        .with_test_group(
            TestGroup::new("scroll_wheel")
                .with(fixture_test(test_scroll_wheel::PixelDeltas {}))
                .with(fixture_test(test_scroll_wheel::LineDeltasInPixels {}))
                .with(fixture_test(test_scroll_wheel::ControlWheelZooms {})),
        )
        .with_test_group(
            TestGroup::new("hover")
                .with(fixture_test(test_hover::TargetIsTheHookedElement {}))
                .with(fixture_test(test_hover::NoHoverByTouch {}))
                .with(fixture_test(test_hover::HoverEndsWhenDisabled {}))
                .with(fixture_test(
                    test_hover::HoverEndsWhenTheElementIsRemoved {},
                ))
                .with(fixture_test(test_hover::HoverableAtom {}))
                .with(fixture_test(
                    test_hover::IgnoresEmulatedMouseEventsAfterTouch {},
                ))
                .with(fixture_test(test_hover::MouseHoversAgainAfterATouch {})),
        )
        .with_test_group(
            TestGroup::new("move")
                .with(fixture_test(test_move::RespondsToPointerEvents {}))
                .with(fixture_test(test_move::EndsWithPointercancel {}))
                .with(fixture_test(test_move::IgnoresRightClicksAndTaps {}))
                .with(fixture_test(test_move::IgnoresAdditionalPointers {}))
                .with(fixture_test(test_move::DoesntBubbleToAMovableParent {}))
                .with(fixture_test(test_move::RespondsToKeys {})),
        )
        .with_test_group(
            TestGroup::new("keyboard")
                .with(fixture_test(test_keyboard::HandlesKeyboardEvents {}))
                .with(fixture_test(test_keyboard::Disabled {}))
                .with(fixture_test(test_keyboard::ContinuePropagation {}))
                .with(fixture_test(test_keyboard::ShortcutChainedWithHandlers {}))
                .with(fixture_test(test_keyboard::UnhandledShortcutContinues {}))
                .with(fixture_test(
                    test_keyboard::PreventDefaultAndPropagationControlled {},
                ))
                .with(fixture_test(test_keyboard::StopIfAShortcutOfTheKeyStops {}))
                .with(fixture_test(
                    test_keyboard::ContinueIfAllShortcutsOfTheKeyContinue {},
                ))
                .with(fixture_test(test_keyboard::StopIfAnyShortcutStops {}))
                .with(fixture_test(
                    test_keyboard::ContinueIfAllShortcutsContinue {},
                ))
                .with(fixture_test(test_keyboard::UnhandledKeyBubbles {}))
                .with(fixture_test(test_keyboard::IgnoresRepeatsByDefault {}))
                .with(fixture_test(test_keyboard::HandlesRepeatsWhenAllowed {}))
                .with(fixture_test(test_keyboard::IgnoresComposingByDefault {}))
                .with(fixture_test(test_keyboard::HandlesComposingWhenAllowed {}))
                .with(fixture_test(test_keyboard::NoShortcutsOnKeyup {})),
        )
        .with_test_group(
            TestGroup::new("color_area")
                .with(fixture_test(test_color_area::InputProps {}))
                .with(fixture_test(
                    test_color_area::BothInputsReachableOnPhones {},
                ))
                .with(fixture_test(test_color_area::Keyboard {}))
                .with(fixture_test(test_color_area::KeyboardSteps {}))
                .with(fixture_test(test_color_area::PressAndDragThumb {}))
                .with(fixture_test(test_color_area::Disabled {}))
                .with(fixture_test(test_color_area::Labelling {}))
                .with(fixture_test(test_color_area::Forms {}))
                .with(fixture_test(test_color_area::Hsv {}))
                .with(fixture_test(test_color_area::Gradients {}))
                .with(fixture_test(test_color_area::RightToLeft {}))
                .with(fixture_test(test_color_area::InputEvent {}))
                .with(fixture_test(test_color_area::ThumbWithoutAlpha {}))
                .with(fixture_test(test_color_area::MountedAgain {}))
                .with(fixture_test(test_color_area::WhiteByDefault {}))
                .with(fixture_test(test_color_area::ChannelOrderInValueTexts {}))
                .with(fixture_test(test_color_area::Focusable {}))
                .with(fixture_test(test_color_area::PressAndDragArea {}))
                .with(fixture_test(test_color_area::ThumbPressFocuses {}))
                .with(fixture_test(test_color_area::DisabledDrag {}))
                .with(fixture_test(test_color_area::RepeatedPageKeys {}))
                .with(fixture_test(test_color_area::ThumbFocusRing {}))
                .with(fixture_test(test_color_area::ThumbHover {}))
                .with(fixture_test(test_color_area::ThumbDragging {}))
                .with(fixture_test(test_color_area::ClassesAttributesAndForm {}))
                .with(fixture_test(test_color_area::RightToLeftKeys {})),
        )
        .with_test_group(
            TestGroup::new("color_field")
                .with(fixture_test(test_color_field::Defaults {}))
                .with(fixture_test(test_color_field::UncontrolledState {}))
                .with(fixture_test(test_color_field::InvalidCharacters {}))
                .with(fixture_test(test_color_field::Stepping {}))
                .with(fixture_test(test_color_field::MouseWheel {}))
                .with(fixture_test(test_color_field::Flags {}))
                .with(fixture_test(test_color_field::FormReset {}))
                .with(fixture_test(test_color_field::Channel {})),
        )
        .with_test_group(
            TestGroup::new("color_picker")
                .with(fixture_test(test_color_picker::SharedColor {}))
                .with(fixture_test(test_color_picker::Alpha {})),
        )
        .with_test_group(
            TestGroup::new("color_slider")
                .with(fixture_test(test_color_slider::InputProps {}))
                .with(fixture_test(test_color_slider::HueValueTextAndLabel {}))
                .with(fixture_test(test_color_slider::Keyboard {}))
                .with(fixture_test(test_color_slider::TrackClick {}))
                .with(fixture_test(test_color_slider::Disabled {}))
                .with(fixture_test(test_color_slider::Forms {}))
                .with(fixture_test(test_color_slider::DefaultLabel {}))
                .with(fixture_test(test_color_slider::DragThumb {}))
                .with(fixture_test(test_color_slider::DragThumbVertical {}))
                .with(fixture_test(test_color_slider::DragTrackVertical {}))
                .with(fixture_test(test_color_slider::MountedAgain {}))
                .with(fixture_test(
                    test_color_slider::ValueTextNamesTheDisplayColor {},
                ))
                .with(fixture_test(test_color_slider::Output {}))
                .with(fixture_test(test_color_slider::AriaLabel {}))
                .with(fixture_test(test_color_slider::AriaLabelledby {}))
                .with(fixture_test(test_color_slider::DisabledDrag {}))
                .with(fixture_test(test_color_slider::Focusable {}))
                .with(fixture_test(test_color_slider::DragTrack {}))
                .with(fixture_test(test_color_slider::ThumbFocusRing {}))
                .with(fixture_test(test_color_slider::ThumbHover {}))
                .with(fixture_test(test_color_slider::ThumbDragging {}))
                .with(fixture_test(test_color_slider::ClassesAttributesAndForm {}))
                .with(fixture_test(test_color_slider::TrackGradients {})),
        )
        .with_test_group(
            TestGroup::new("color_swatch")
                .with(fixture_test(test_color_swatch::Swatches {}))
                .with(fixture_test(test_color_swatch::PickerDefaultValue {}))
                .with(fixture_test(test_color_swatch::PickerKeyboard {}))
                .with(fixture_test(test_color_swatch::PickerDisabledItems {}))
                .with(fixture_test(test_color_swatch::SwatchInItem {})),
        )
        .with_test_group(
            TestGroup::new("color_wheel")
                .with(fixture_test(test_color_wheel::InputProps {}))
                .with(fixture_test(test_color_wheel::Keyboard {}))
                .with(fixture_test(test_color_wheel::RingPress {}))
                .with(fixture_test(test_color_wheel::Disabled {}))
                .with(fixture_test(test_color_wheel::Forms {}))
                .with(fixture_test(test_color_wheel::DragThumb {}))
                .with(fixture_test(test_color_wheel::InputEvent {}))
                .with(fixture_test(test_color_wheel::RgbColors {}))
                .with(fixture_test(test_color_wheel::MountedAgain {}))
                .with(fixture_test(test_color_wheel::Focusable {}))
                .with(fixture_test(test_color_wheel::RepeatedPageKeys {}))
                .with(fixture_test(test_color_wheel::RingDrag {}))
                .with(fixture_test(test_color_wheel::PressInsideTheRing {}))
                .with(fixture_test(test_color_wheel::DisabledDrag {}))
                .with(fixture_test(test_color_wheel::ThumbFocusRing {}))
                .with(fixture_test(test_color_wheel::ThumbHover {}))
                .with(fixture_test(test_color_wheel::ThumbDragging {}))
                .with(fixture_test(test_color_wheel::Labelledby {}))
                .with(fixture_test(test_color_wheel::ThumbWithoutAlpha {}))
                .with(fixture_test(test_color_wheel::ClassesAttributesAndForm {}))
                .with(fixture_test(test_color_wheel::RadiiChange {})),
        )
        .with_test_group(
            TestGroup::new("label_slots")
                .with(fixture_test(test_label_slots::NoLabels {}))
                .with(fixture_test(test_label_slots::LabelsAdded {}))
                .with(fixture_test(test_label_slots::LabelsRemoved {})),
        )
        .with_test_group(
            TestGroup::new("context_menu")
                .with(fixture_test(
                    test_context_menu::RightClickRequestsTheMenu {},
                ))
                .with(fixture_test(
                    test_context_menu::WithoutAHandlerNothingHappens {},
                ))
                .with(fixture_test(test_context_menu::CtrlEnterIsMacOnly {}))
                .with(fixture_test(test_context_menu::CtrlEnterOnMac {}))
                .with(fixture_test(
                    test_context_menu::CtrlEnterWithAContextmenuEventFiresOnce {},
                ))
                .with(fixture_test(test_context_menu::EnterWithoutCtrlOnMac {}))
                .with(fixture_test(test_context_menu::LongPressOnIos {}))
                .with(fixture_test(test_context_menu::CancelledLongPressOnIos {}))
                .with(fixture_test(
                    test_context_menu::LongPressOnAndroidRequestsOnce {},
                ))
                .with(fixture_test(
                    test_context_menu_atoms::RightClickOpensAtThePointer {},
                ))
                .with(fixture_test(
                    test_context_menu_atoms::EscapeReturnsFocusToTheRow {},
                ))
                .with(fixture_test(test_context_menu_atoms::ShiftF10OpensIt {})),
        )
        .with_test_group(
            TestGroup::new("interact_outside")
                .with(fixture_test(test_interact_outside::PointerEvents {}))
                .with(fixture_test(test_interact_outside::LeftButtonOnly {}))
                .with(fixture_test(
                    test_interact_outside::PointerUpWithoutPointerDown {},
                ))
                .with(fixture_test(test_interact_outside::Disabled {})),
        )
        .with_test_group(
            TestGroup::new("long_press")
                .with(fixture_test(test_long_press::LongPress {}))
                .with(fixture_test(test_long_press::CancelledWhenReleasedEarly {}))
                .with(fixture_test(test_long_press::CancelsOtherPressEvents {}))
                .with(fixture_test(
                    test_long_press::KeepsPressEventsWhenReleasedEarly {},
                ))
                .with(fixture_test(test_long_press::CustomThreshold {}))
                .with(fixture_test(test_long_press::AccessibilityDescription {}))
                .with(fixture_test(
                    test_long_press::PreventsContextMenuDuringTouch {},
                ))
                .with(fixture_test(test_long_press::NoLongPressByKeyboard {}))
                .with(fixture_test(
                    test_long_press::PreventsTheClickAfterALongPress {},
                ))
                .with(fixture_test(test_long_press::DraggingOutAndBackIn {})),
        )
        .with_test_group(
            TestGroup::new("press")
                .with(fixture_test(test_press::MouseClickFiresAllEventsInOrder {}))
                .with(fixture_test(
                    test_press::TextSelectionDisabledWhilePressed {},
                ))
                .with(fixture_test(test_press::EnterPresses {}))
                .with(fixture_test(test_press::SpacePresses {}))
                .with(fixture_test(test_press::ReleasingOutsideDoesNotPress {}))
                .with(fixture_test(test_press::DisabledElementIgnoresPresses {}))
                .with(fixture_test(
                    test_press::BecomingDisabledCancelsActivePress {},
                ))
                .with(fixture_test(test_press::EnterOnCheckboxSubmitsForm {}))
                .with(fixture_test(
                    test_press::PreventFocusOnPressKeepsTheFocus {},
                ))
                .with(fixture_test(test_press::NestedPressStopsByDefault {}))
                .with(fixture_test(
                    test_press::NestedPressPropagatesWhenContinued {},
                ))
                .with(fixture_test(
                    test_press::KeyboardPressEndsWhenKeyUpIsStopped {},
                ))
                .with(fixture_test(test_press::ADragInsideCancelsThePress {}))
                .with(fixture_test(
                    test_press::AChildStoppingTheClickCancelsThePress {},
                ))
                .with(fixture_test(
                    test_press::FocusMovingBeforeKeyUpEndsWithoutPress {},
                ))
                .with(fixture_test(test_press::RepeatingKeyDownsAreIgnored {}))
                .with(fixture_test(test_press::DraggingOutAndBackIn {}))
                .with(fixture_test(test_press::CancelOnPointerExit {}))
                .with(fixture_test(test_press::PointerCancelCancelsThePress {}))
                .with(fixture_test(test_press::SpaceOnALinkWithButtonRole {}))
                .with(fixture_test(test_press::DoublePress {}))
                .with(fixture_test(test_press::DoublePressWithHover {}))
                .with(fixture_test(test_press::TouchPressWithoutAClick {}))
                .with(fixture_test(test_press::OnlyThePrimaryButtonPresses {}))
                .with(fixture_test(test_press::PointerModifierKeys {}))
                .with(fixture_test(test_press::KeyboardModifierKeys {}))
                .with(fixture_test(test_press::MouseCoordinates {}))
                .with(fixture_test(test_press::TouchCoordinates {}))
                .with(fixture_test(test_press::KeyboardCoordinates {}))
                .with(fixture_test(test_press::CancelCoordinates {}))
                .with(fixture_test(test_press::ClickFocusesTheElement {}))
                .with(fixture_test(test_press::VirtualPointerEventsAreIgnored {}))
                .with(fixture_test(test_press::ZeroSizedPointersPressOnAndroid {}))
                .with(fixture_test(test_press::TalkbackDoubleTap {}))
                .with(fixture_test(test_press::PressureZeroPressesElsewhere {}))
                .with(fixture_test(test_press::RealPointerAfterAVirtualOne {}))
                .with(fixture_test(test_press::VirtualClickDuringAPointerPress {}))
                .with(fixture_test(
                    test_press::TypingInATextInputPressesNothing {},
                ))
                .with(fixture_test(test_press::TwoPressHooksOpenALinkOnce {}))
                .with(fixture_test(test_press::ClicksDuringPressUpAreIgnored {}))
                .with(fixture_test(
                    test_press::IosPressStartDisablesPageSelection {},
                ))
                .with(fixture_test(
                    test_press::PressStartLeavesPageSelectionElsewhere {},
                ))
                .with(fixture_test(
                    test_press::IosPressEndRestoresPageSelection {},
                ))
                .with(fixture_test(
                    test_press::IosQuickSecondPressKeepsPageSelectionDisabled {},
                ))
                .with(fixture_test(test_press::IosUnmountRestoresPageSelection {}))
                .with(fixture_test(
                    test_press::TwoPressesRestoreTheirOwnSelection {},
                ))
                .with(fixture_test(test_press::StyleChangesDuringAPressStay {}))
                .with(fixture_test(test_press::UserSelectSetDuringAPressStays {}))
                .with(fixture_test(test_press::SpaceOnACheckbox {}))
                .with(fixture_test(test_press::EnterOnALink {}))
                .with(fixture_test(test_press::EnterOnALinkRole {}))
                .with(fixture_test(test_press::PressPropagationContinue {}))
                .with(fixture_test(test_press::VirtualClick {}))
                .with(fixture_test(test_press::RemovedWhilePressed {}))
                .with(fixture_test(test_press::MetaReleaseEndsHeldKeyPresses {})),
        )
        .with_test_group(
            TestGroup::new("use_button")
                .with(fixture_test(
                    test_use_button::AttributesDependOnElementType {},
                ))
                .with(fixture_test(
                    test_use_button::NativeAndCustomElementsPress {},
                ))
                .with(fixture_test(test_use_button::TabOrder {}))
                .with(fixture_test(test_use_button::DisabledButtons {}))
                .with(fixture_test(test_use_button::FormSubmission {}))
                .with(fixture_test(test_use_button::HoverAndFocusVisible {})),
        )
        .with_test_group(
            TestGroup::new("menu_trigger")
                .with(fixture_test(test_menu_trigger::AriaAttributes {}))
                .with(fixture_test(test_menu_trigger::MousePressOpensOnce {}))
                .with(fixture_test(
                    test_menu_trigger::ArrowDownOpensOnTheFirstItem {},
                ))
                .with(fixture_test(
                    test_menu_trigger::ArrowUpOpensOnTheLastItem {},
                ))
                .with(fixture_test(test_menu_trigger::EnterOpensOnTheFirstItem {}))
                .with(fixture_test(test_menu_trigger::SpaceOpensOnTheFirstItem {}))
                .with(fixture_test(
                    test_menu_trigger::DisabledTriggerDoesntOpen {},
                ))
                .with(fixture_test(
                    test_menu_trigger::LongPressOpensOnTheFirstItem {},
                )),
        )
        .with_test_group(
            TestGroup::new("number_field")
                .with(fixture_test(test_number_field::StepperButtons {}))
                .with(fixture_test(test_number_field::IphoneInput {}))
                .with(fixture_test(
                    test_number_field::ClickStepsOnceAndFocusesInput {},
                ))
                .with(fixture_test(
                    test_number_field::HoldingSpinsUntilTheLimit {},
                ))
                .with(fixture_test(test_number_field::Keyboard {}))
                .with(fixture_test(test_number_field::SteppersAreNotTabStops {}))
                .with(fixture_test(test_number_field::EnterCommitsAndSubmits {}))
                .with(fixture_test(test_number_field_atoms::ProvidesSlots {}))
                .with(fixture_test(
                    test_number_field_atoms::HoverAndFocusVisibleState {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::SteppersShowFocusVisible {},
                ))
                .with(fixture_test(test_number_field_atoms::ReadOnlyState {}))
                .with(fixture_test(test_number_field_atoms::FormValue {}))
                .with(fixture_test(test_number_field_atoms::ValidationErrors {}))
                .with(fixture_test(test_number_field_atoms::FormReset {}))
                .with(fixture_test(
                    test_number_field_atoms::SteppersCommitValidation {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::OnlyCommitsOnBlurIfTheValueChanged {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::NativeValidateFunction {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::NativeServerValidation {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::CustomNativeErrorMessage {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::AriaValidateFunction {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::AriaServerValidation {},
                ))
                .with(fixture_test(test_number_field_atoms::ArrowKeys {}))
                .with(fixture_test(
                    test_number_field_atoms::ProgrammaticClicksOnSteppers {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::DeletingTheFirstDigitBeforeAGroupSeparator {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::TypingAndEnterCommit {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::NoGroupingCharactersWithoutGrouping {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::NoGroupingCharactersInGerman {},
                ))
                .with(fixture_test(test_number_field_atoms::ScrollWheel {}))
                .with(fixture_test(test_number_field_atoms::PastingIntoAFormat {}))
                .with(fixture_test(
                    test_number_field_atoms::RejectedValuesKeepTheText {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::ServerErrorsSurviveAnUnchangedBlur {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::ValidateCommitBehavior {},
                ))
                .with(fixture_test(
                    test_number_field_atoms::ValidateCommitBehaviorAndEnterSubmit {},
                ))
                .with(fixture_test(test_number_field_atoms::TypedValues {})),
        )
        .with_test_group(
            TestGroup::new("live_announcer")
                .with(fixture_test(test_live_announcer::Announcements {}))
                .with(fixture_test(test_live_announcer::Clear {}))
                .with(fixture_test(test_live_announcer::Timeout {})),
        )
        .with_test_group(
            TestGroup::new("listbox")
                .with(fixture_test(test_listbox::AriaStructure {}))
                .with(fixture_test(
                    test_listbox::KeyboardNavigationSkipsDisabledItems {},
                ))
                .with(fixture_test(test_listbox::Selection {}))
                .with(fixture_test(test_listbox::TabInAndOut {}))
                .with(fixture_test(test_listbox::TypeAhead {}))
                .with(fixture_test(test_listbox::SelectAllAndClear {}))
                .with(fixture_test(test_listbox::ShiftArrowExtendsSelection {}))
                .with(fixture_test(
                    test_listbox_features::SectionsAndSeparators {},
                ))
                .with(fixture_test(
                    test_listbox_features::ArrowKeysCrossSections {},
                ))
                .with(fixture_test(test_listbox_features::Hover {}))
                .with(fixture_test(
                    test_listbox_features::ReplaceSelectionByPress {},
                ))
                .with(fixture_test(
                    test_listbox_features::ReplaceSelectionByKeyboard {},
                ))
                .with(fixture_test(
                    test_listbox_features::ActionsWithoutSelection {},
                ))
                .with(fixture_test(test_listbox_features::Links {}))
                .with(fixture_test(
                    test_listbox_features::LinksWithSingleSelection {},
                ))
                .with(fixture_test(test_listbox_features::ArrowKeysPerLayout {}))
                .with(fixture_test(test_listbox_features::PageDownAndUp {}))
                .with(fixture_test(
                    test_listbox_features::DisabledSelectionBehavior {},
                ))
                .with(fixture_test(test_listbox_features::EmptyState {}))
                .with(fixture_test(
                    test_listbox_features::RemovingTheFocusedOption {},
                ))
                .with(fixture_test(
                    test_listbox_features::LabelsFollowTheCollection {},
                ))
                .with(fixture_test(test_listbox_features::RenamedItemsUpdate {}))
                .with(fixture_test(test_listbox_selection::SingleSelection {}))
                .with(fixture_test(
                    test_listbox_selection::FixedSingleSelection {},
                ))
                .with(fixture_test(
                    test_listbox_selection::SelectAllDoesNothingWithSingleSelection {},
                ))
                .with(fixture_test(
                    test_listbox_selection::MultipleDefaultSelection {},
                ))
                .with(fixture_test(
                    test_listbox_selection::FixedMultipleSelection {},
                ))
                .with(fixture_test(
                    test_listbox_selection::EscapeKeyBehaviorNone {},
                ))
                .with(fixture_test(
                    test_listbox_selection::ReplaceSelectionOnEntryAndHomeEnd {},
                ))
                .with(fixture_test(
                    test_listbox_selection::EnteringFocusesTheSelection {},
                ))
                .with(fixture_test(
                    test_listbox_selection::SpaceReplacesAfterMovingFocusOnly {},
                ))
                .with(fixture_test(
                    test_listbox_selection::PressingTheSelectedOptionKeepsIt {},
                ))
                .with(fixture_test(
                    test_listbox_selection::ReplaceBehaviorWithSingleSelection {},
                ))
                .with(fixture_test(
                    test_listbox_selection::TouchAndScreenReaderPressesToggle {},
                ))
                .with(fixture_test(
                    test_listbox_selection::LongPressSelectsNextToAnAction {},
                ))
                .with(fixture_test(
                    test_listbox_selection::LinksWithMultipleSelection {},
                ))
                .with(fixture_test(
                    test_listbox_selection::LinksWithReplaceSelection {},
                ))
                .with(fixture_test(
                    test_listbox_selection::LabelAndDescriptionSlots {},
                ))
                .with(fixture_test(
                    test_listbox_selection::OptionsFollowTheCollection {},
                ))
                .with(fixture_test(
                    test_listbox_selection::GridSelectionAndEdges {},
                ))
                .with(fixture_test(
                    test_listbox_selection::TypeAheadSpacesTimeoutAndWrap {},
                ))
                .with(fixture_test(
                    test_listbox_selection::ClassesAndAttributes {},
                ))
                .with(fixture_test(
                    test_listbox_selection::AutoFocusWithoutSelection {},
                ))
                .with(fixture_test(
                    test_listbox_selection::AutoFocusFirstAndLast {},
                ))
                .with(fixture_test(
                    test_listbox_selection::AutoFocusKeepsTheSelection {},
                )),
        )
        .with_test_group(
            TestGroup::new("select")
                .with(fixture_test(
                    test_select::PartsWithoutParentWarnAndRenderNothing {},
                ))
                .with(fixture_test(test_select::InitialState {}))
                .with(fixture_test(
                    test_select::OpeningFocusesTheSelectedOption {},
                ))
                .with(fixture_test(test_select::EscapeClosesAndRestoresFocus {}))
                .with(fixture_test(
                    test_select::EscapeAfterOpeningWithTheKeyboard {},
                ))
                .with(fixture_test(test_select::BoundSelect {}))
                .with(fixture_test(test_select::SelectingAnOption {}))
                .with(fixture_test(test_select::TriggerKeyboard {}))
                .with(fixture_test(test_select::Labelling {}))
                .with(fixture_test(test_select::FormReset {}))
                .with(fixture_test(
                    test_select_forms::TriggerHoverAndPlaceholder {},
                ))
                .with(fixture_test(test_select_forms::MultipleSelection {}))
                .with(fixture_test(test_select_forms::OpenStateBoundToAppState {}))
                .with(fixture_test(test_select_forms::NativeValidation {}))
                .with(fixture_test(test_select_forms::RequiredBlocksSubmission {}))
                .with(fixture_test(test_select_forms::Disabled {}))
                .with(fixture_test(test_select_forms::Autofill {}))
                .with(fixture_test(test_select_forms::NoItems {}))
                .with(fixture_test(test_select_forms::EmptyState {}))
                .with(fixture_test(test_select_forms::ManyItemsValidation {}))
                .with(fixture_test(
                    test_select_forms::ManyItemsSelectionAndReset {},
                ))
                .with(fixture_test(
                    test_select_forms::ValueOutsideTheOptionsIsSubmitted {},
                ))
                .with(fixture_test(
                    test_select_forms::ValueOutsideTheOptionsShowsThePlaceholder {},
                ))
                .with(fixture_test(
                    test_select_behavior::PopoverContentIsntPartOfTheSelect {},
                ))
                .with(fixture_test(test_select_behavior::LabelledByAriaLabel {}))
                .with(fixture_test(
                    test_select_behavior::LabelledByAriaLabelledby {},
                ))
                .with(fixture_test(test_select_behavior::LabelledByBoth {}))
                .with(fixture_test(
                    test_select_behavior::DescribedAndPressedWhileOpen {},
                ))
                .with(fixture_test(test_select_behavior::OpensOnPointerDown {}))
                .with(fixture_test(test_select_behavior::OpensByKeys {}))
                .with(fixture_test(
                    test_select_behavior::ClosesByTriggerAndDismissButton {},
                ))
                .with(fixture_test(
                    test_select_behavior::TabKeepsThePopoverOpen {},
                ))
                .with(fixture_test(
                    test_select_behavior::ShouldCloseOnSelectOverridesTheMode {},
                ))
                .with(fixture_test(
                    test_select_behavior::SpaceSelectsInThePopover {},
                ))
                .with(fixture_test(test_select_behavior::HoverFocusesOptions {}))
                .with(fixture_test(test_select_behavior::TypeAheadInThePopover {}))
                .with(fixture_test(
                    test_select_behavior::PressingTheSelectedOptionKeepsIt {},
                ))
                .with(fixture_test(test_select_behavior::TriggerArrowKeys {}))
                .with(fixture_test(test_select_behavior::FixedValues {}))
                .with(fixture_test(test_select_behavior::FocusChanges {}))
                .with(fixture_test(test_select_behavior::HiddenSelectMarkup {}))
                .with(fixture_test(
                    test_select_behavior::NativeValidateFunction {},
                ))
                .with(fixture_test(test_select_behavior::AriaValidateFunction {}))
                .with(fixture_test(test_select_behavior::ServerValidation {}))
                .with(fixture_test(test_select_behavior::DefaultOpen {}))
                .with(fixture_test(test_select_behavior::FixedOpenState {}))
                .with(fixture_test(
                    test_select_behavior::SectionsAndComplexOptions {},
                ))
                .with(fixture_test(
                    test_select_behavior::OpeningScrollsToTheSelectedOption {},
                )),
        )
        .with_test_group(
            TestGroup::new("menu")
                .with(fixture_test(test_menu::ClosedTrigger {}))
                .with(fixture_test(test_menu::OpenMenuAriaStructure {}))
                .with(fixture_test(
                    test_menu::KeyboardOpeningFocusesFirstOrLastItem {},
                ))
                .with(fixture_test(
                    test_menu::KeyboardNavigationSkipsDisabledItems {},
                ))
                .with(fixture_test(
                    test_menu::KeyboardActivationClosesAndRestoresFocus {},
                ))
                .with(fixture_test(test_menu::TypeAhead {}))
                .with(fixture_test(test_menu::ClickingAnItem {}))
                .with(fixture_test(test_menu::MouseOpeningFocusesTheMenu {}))
                .with(fixture_test(test_menu::TypeAheadSkipsDisabledItems {}))
                .with(fixture_test(test_menu::SelectionMenu {})),
        )
        .with_test_group(
            TestGroup::new("menu_atoms")
                .with(fixture_test(test_menu_atoms::MenuTrigger {}))
                .with(fixture_test(test_menu_atoms::KeyboardOpening {}))
                .with(fixture_test(test_menu_atoms::SelectionMenu {}))
                .with(fixture_test(test_menu_atoms::LongPressTrigger {}))
                .with(fixture_test(test_menu_atoms::SectionSelection {}))
                .with(fixture_test(test_menu_atoms::CloseOnSelect {}))
                .with(fixture_test(test_menu_atoms::ItemHover {}))
                .with(fixture_test(test_menu_atoms::DisabledItemIsntHovered {}))
                .with(fixture_test(test_menu_atoms::ItemFocusRing {}))
                .with(fixture_test(test_menu_atoms::ItemPressState {}))
                .with(fixture_test(test_menu_atoms::ItemDisabledState {}))
                .with(fixture_test(test_menu_atoms::EmptyState {}))
                .with(fixture_test(test_menu_atoms::ArrowKeysStopAtTheEnds {}))
                .with(fixture_test(test_menu_atoms::SelectionCantBecomeEmpty {}))
                .with(fixture_test(
                    test_menu_atoms::SectionWithoutHeadingIsLabelled {},
                ))
                .with(fixture_test(
                    test_menu_atoms::PressDragReleaseActivatesAnItem {},
                ))
                .with(fixture_test(test_menu_atoms::TabKeepsFocusInTheOpenMenu {})),
        )
        .with_test_group(
            TestGroup::new("grid")
                .with(fixture_test(test_grid::AriaStructure {}))
                .with(fixture_test(test_grid::RowFocusCellFocus {}))
                .with(fixture_test(test_grid::RowFocusChildFocus {}))
                .with(fixture_test(test_grid::CellFocusChildFocus {}))
                .with(fixture_test(test_grid::CellFocusCellFocus {}))
                .with(fixture_test(test_grid::RestoresTheLastFocusedChild {}))
                .with(fixture_test(test_grid::FocusingAChildFromOutsideKeepsIt {}))
                .with(fixture_test(test_grid::TwoDimensionalNavigation {}))
                .with(fixture_test(test_grid::RowSelection {}))
                .with(fixture_test(test_grid::CellFocusModeSelectsRows {}))
                .with(fixture_test(test_grid::CellActions {})),
        )
        .with_test_group(
            TestGroup::new("grid_list")
                .with(fixture_test(test_grid_list::AriaStructure {}))
                .with(fixture_test(
                    test_grid_list::TabIntoTheListFocusesTheFirstRow {},
                ))
                .with(fixture_test(
                    test_grid_list::KeyboardNavigationSkipsDisabledRows {},
                ))
                .with(fixture_test(
                    test_grid_list::TabOutAndBackRestoresTheFocusedRow {},
                ))
                .with(fixture_test(test_grid_list::SelectAllAndClear {}))
                .with(fixture_test(test_grid_list::SpaceTogglesSelection {}))
                .with(fixture_test(test_grid_list::ClickSelection {}))
                .with(fixture_test(test_grid_list::DisabledRowIsMarked {}))
                .with(fixture_test(test_grid_list::SelectAllSkipsDisabledRow {}))
                .with(fixture_test(test_grid_list::ShiftArrowExtendsSelection {}))
                .with(fixture_test(test_grid_list::SelectionAnnouncements {}))
                .with(fixture_test(
                    test_grid_list_features::ArrowsCycleThroughChildrenAndRow {},
                ))
                .with(fixture_test(
                    test_grid_list_features::ArrowsAreMirroredRightToLeft {},
                ))
                .with(fixture_test(
                    test_grid_list_features::TabWalksTheChildren {},
                ))
                .with(fixture_test(
                    test_grid_list_features::ArrowsMoveBetweenChildrenOfRows {},
                ))
                .with(fixture_test(
                    test_grid_list_features::TextInputKeepsItsKeys {},
                ))
                .with(fixture_test(
                    test_grid_list_features::HoverOnRowsWithAnAction {},
                ))
                .with(fixture_test(
                    test_grid_list_features::ActionsWithoutSelection {},
                ))
                .with(fixture_test(
                    test_grid_list_features::ReplaceSelectionBehavior {},
                ))
                .with(fixture_test(test_grid_list_features::LinksOpenOnPress {}))
                .with(fixture_test(
                    test_grid_list_features::SectionsAndDescriptions {},
                ))
                .with(fixture_test(
                    test_grid_list_features::ClickingATabbableChildInTabNavigation {},
                ))
                .with(fixture_test(test_grid_list_cases::AutoFocus {}))
                .with(fixture_test(
                    test_grid_list_cases::AutoFocusFirstSelectsIt {},
                ))
                .with(fixture_test(
                    test_grid_list_cases::AutoFocusLastSelectsIt {},
                ))
                .with(fixture_test(
                    test_grid_list_cases::AutoFocusWithoutSelection {},
                ))
                .with(fixture_test(
                    test_grid_list_cases::AutoFocusKeepsAnAllSelection {},
                ))
                .with(fixture_test(test_grid_list_cases::FocusRing {}))
                .with(fixture_test(test_grid_list_cases::PressState {}))
                .with(fixture_test(
                    test_grid_list_cases::NoPressStateWhenNotInteractive {},
                ))
                .with(fixture_test(
                    test_grid_list_cases::EscapeKeepsTheSelection {},
                ))
                .with(fixture_test(test_grid_list_cases::EmptyState {}))
                .with(fixture_test(test_grid_list_cases::GridLayout {}))
                .with(fixture_test(test_grid_list_cases::Sections {}))
                .with(fixture_test(
                    test_grid_list_cases::SelectsOnPressDownByDefault {},
                ))
                .with(fixture_test(test_grid_list_cases::SelectsOnPressUp {}))
                .with(fixture_test(
                    test_grid_list_cases::ClickingATabbableChildInArrowNavigation {},
                )),
        )
        .with_test_group(
            TestGroup::new("tabs")
                .with(fixture_test(test_tabs::AriaStructure {}))
                .with(fixture_test(test_tabs::SelectionByPress {}))
                .with(fixture_test(test_tabs::KeyboardNavigation {}))
                .with(fixture_test(test_tabs::DisabledTab {}))
                .with(fixture_test(test_tabs::DisabledFirstTab {}))
                .with(fixture_test(test_tabs::AllTabsDisabled {}))
                .with(fixture_test(test_tabs::Vertical {}))
                .with(fixture_test(test_tabs::ManualActivation {}))
                .with(fixture_test(test_tabs::RtlVertical {}))
                .with(fixture_test(test_tabs::DataAttributes {}))
                .with(fixture_test(test_tabs::ForceMount {}))
                .with(fixture_test(test_tabs::TabIsDisabled {}))
                .with(fixture_test(test_tabs::Controlled {}))
                .with(fixture_test(test_tabs::Dynamic {}))
                .with(fixture_test(test_tabs::Nested {}))
                .with(fixture_test(test_tabs::TabPanels {}))
                .with(fixture_test(test_tabs::DisabledTabs {}))
                .with(fixture_test(test_tabs::DefaultSelectedKey {}))
                .with(fixture_test(test_tabs::HomeAndEndSkipDisabledTabs {}))
                .with(fixture_test(test_tabs::PanelAriaLabel {}))
                .with(fixture_test(test_tabs::RtlHorizontal {}))
                .with(fixture_test(test_tabs::ManualActivationWithSpace {}))
                .with(fixture_test(test_tabs::PanelTabStop {}))
                .with(fixture_test(test_tabs::TooltipOnATab {}))
                .with(fixture_test(test_tabs::RovingTabindexFollowsTheApp {}))
                .with(fixture_test(test_tabs::RootStateAndDefaultClasses {})),
        )
        .with_test_group(
            TestGroup::new("calendar")
                .with(fixture_test(test_calendar::Structure {}))
                .with(fixture_test(test_calendar::SelectionByPress {}))
                .with(fixture_test(test_calendar::KeyboardArrows {}))
                .with(fixture_test(test_calendar::KeyboardPages {}))
                .with(fixture_test(test_calendar::KeyboardHomeEnd {}))
                .with(fixture_test(test_calendar::KeyboardEnterSelects {}))
                .with(fixture_test(test_calendar::KeyboardPagingFromASixthRow {}))
                .with(fixture_test(test_calendar::PreviousNextButtons {}))
                .with(fixture_test(test_calendar::MinMax {}))
                .with(fixture_test(test_calendar::Unavailable {}))
                .with(fixture_test(test_calendar::Disabled {}))
                .with(fixture_test(test_calendar::ReadOnly {}))
                .with(fixture_test(test_calendar::Invalid {}))
                .with(fixture_test(test_calendar::TwoMonths {}))
                .with(fixture_test(test_calendar::WeekView {}))
                .with(fixture_test(test_calendar::DayView {}))
                .with(fixture_test(test_calendar::FirstDayOfWeek {}))
                .with(fixture_test(test_calendar::RangeByPress {}))
                .with(fixture_test(test_calendar::RangeByKeyboard {}))
                .with(fixture_test(test_calendar::RangeByDragging {}))
                .with(fixture_test(test_calendar::RangeUnavailable {}))
                .with(fixture_test(test_calendar::LabelledByAnotherElement {}))
                .with(fixture_test(test_calendar::RightToLeft {}))
                .with(fixture_test(
                    test_calendar::SettingTheFocusedDateKeepsTheFocus {},
                ))
                .with(fixture_test(test_calendar::ControlledRangeCleared {}))
                .with(fixture_test(
                    test_calendar::UnavailableDatesDependingOnTheAnchor {},
                ))
                .with(fixture_test(test_calendar::RangeByTouchTaps {}))
                .with(fixture_test(test_calendar::RangeByTouchDragging {}))
                .with(fixture_test(test_calendar::RangeKeptWhenATouchScrolls {}))
                .with(fixture_test(test_calendar::TodayInTheBrowsersTimeZone {}))
                .with(fixture_test(test_calendar::PageBehaviorSingle {}))
                .with(fixture_test(test_calendar::TwoWeeks {}))
                .with(fixture_test(test_calendar::WeeksInMonth {}))
                .with(fixture_test(test_calendar::HeldArrowKeys {}))
                .with(fixture_test(test_calendar::ChangingTheVisibleDuration {}))
                .with(fixture_test(test_calendar::MonthAndYearPickers {}))
                .with(fixture_test(test_calendar::Announcements {}))
                .with(fixture_test(test_calendar::CommitSelectOnRelease {}))
                .with(fixture_test(test_calendar::CommitSelectOnBlur {}))
                .with(fixture_test(test_calendar::CommitClearOnRelease {}))
                .with(fixture_test(test_calendar::CommitClearOnBlur {}))
                .with(fixture_test(test_calendar::CommitResetOnRelease {}))
                .with(fixture_test(test_calendar::CommitResetOnBlur {}))
                .with(fixture_test(test_calendar::DayViewLeftRightArrows {}))
                .with(fixture_test(test_calendar::DayViewUpDownArrows {}))
                .with(fixture_test(test_calendar::DayViewPageKeys {}))
                .with(fixture_test(test_calendar::DayViewShiftPageKeys {}))
                .with(fixture_test(test_calendar::DayViewHomeEnd {}))
                .with(fixture_test(test_calendar::WeekViewLeftRightArrows {}))
                .with(fixture_test(test_calendar::WeekViewUpDownArrows {}))
                .with(fixture_test(test_calendar::WeekViewPageKeys {}))
                .with(fixture_test(test_calendar::WeekViewShiftPageKeys {}))
                .with(fixture_test(test_calendar::WeekViewHomeEnd {}))
                .with(fixture_test(test_calendar::TwoWeeksLeftRightArrows {}))
                .with(fixture_test(test_calendar::TwoWeeksUpDownArrows {}))
                .with(fixture_test(test_calendar::TwoWeeksPageKeys {}))
                .with(fixture_test(test_calendar::TwoWeeksShiftPageKeys {}))
                .with(fixture_test(test_calendar::TwoWeeksHomeEnd {}))
                .with(fixture_test(test_calendar::PaginationByTheVisibleMonths {}))
                .with(fixture_test(test_calendar::PaginationByTheVisibleWeeks {}))
                .with(fixture_test(test_calendar::PaginationByTheVisibleDays {}))
                .with(fixture_test(test_calendar::FirstDayOfWeekSaturday {}))
                .with(fixture_test(test_calendar::FirstDayOfWeekInFrench {}))
                .with(fixture_test(test_calendar::FirstDayOfWeekNeedingSixRows {}))
                .with(fixture_test(test_calendar::FirstDayOfWeekOfTheLocale {}))
                .with(fixture_test(test_calendar::LabelledByTheMonthByDefault {}))
                .with(fixture_test(test_calendar::LabelledOnlyByAnotherElement {}))
                .with(fixture_test(test_calendar::CustomId {}))
                .with(fixture_test(test_calendar::LabelledWithSeveralMonths {}))
                .with(fixture_test(test_calendar::DescribedAndDetailed {}))
                .with(fixture_test(test_calendar::DefaultClasses {}))
                .with(fixture_test(test_calendar::CustomClassesAndAttributes {}))
                .with(fixture_test(test_calendar::CellHover {}))
                .with(fixture_test(test_calendar::CellFocusRing {}))
                .with(fixture_test(test_calendar::CellPressState {}))
                .with(fixture_test(test_calendar::WeekdayStyle {}))
                .with(fixture_test(
                    test_calendar::ClearingTheValueThroughTheState {},
                ))
                .with(fixture_test(
                    test_calendar::ShowsTheCurrentMonthByDefault {},
                ))
                .with(fixture_test(
                    test_calendar::RangeShowsTheCurrentMonthByDefault {},
                ))
                .with(fixture_test(test_calendar::PressOutsideTheLimits {}))
                .with(fixture_test(test_calendar::LimitsDisableTheButtons {}))
                .with(fixture_test(
                    test_calendar::ButtonsDisabledWhileFocusedMoveTheFocus {},
                ))
                .with(fixture_test(
                    test_calendar::RangeButtonsDisabledWhileFocusedMoveTheFocus {},
                ))
                .with(fixture_test(test_calendar::KeysStopAtTheLimits {}))
                .with(fixture_test(test_calendar::MaximumDate {}))
                .with(fixture_test(test_calendar::EraOfDatesBeforeChrist {}))
                .with(fixture_test(test_calendar::RangeEraOfDatesBeforeChrist {}))
                .with(fixture_test(
                    test_calendar::AnnouncementsOfDatesBeforeChrist {},
                ))
                .with(fixture_test(
                    test_calendar::RangeAnnouncementsOfDatesBeforeChrist {},
                ))
                .with(fixture_test(test_calendar::CalendarAnnouncements {}))
                .with(fixture_test(test_calendar::AlignmentOfTheInitialValue {}))
                .with(fixture_test(
                    test_calendar::RangeAlignmentOfTheInitialValue {},
                ))
                .with(fixture_test(
                    test_calendar::SeveralMonthsPlaceTheSelectedDate {},
                ))
                .with(fixture_test(
                    test_calendar::SeveralMonthsPlaceTheSelectedRange {},
                ))
                .with(fixture_test(
                    test_calendar::ThreeMonthsPagingByTheButtons {},
                ))
                .with(fixture_test(
                    test_calendar::RangeThreeMonthsPagingByTheButtons {},
                ))
                .with(fixture_test(
                    test_calendar::ThreeMonthsPagingByTheKeyboard {},
                ))
                .with(fixture_test(
                    test_calendar::ThreeMonthsHomeEndAndPageKeys {},
                ))
                .with(fixture_test(test_calendar::UnavailableIntervals {}))
                .with(fixture_test(test_calendar::RangeUnavailableIntervals {}))
                .with(fixture_test(test_calendar::DefaultFocusedValue {}))
                .with(fixture_test(test_calendar::ControlledFocusedValue {}))
                .with(fixture_test(
                    test_calendar::DefaultFocusedValueConstrained {},
                ))
                .with(fixture_test(
                    test_calendar::ControlledFocusedValueConstrained {},
                ))
                .with(fixture_test(test_calendar::AutoFocusToday {}))
                .with(fixture_test(test_calendar::AutoFocusTheSelectedDate {}))
                .with(fixture_test(
                    test_calendar::RangeAutoFocusTheFirstSelectedDate {},
                ))
                .with(fixture_test(test_calendar::ControlledSelection {}))
                .with(fixture_test(test_calendar::ReadOnlyKeyboardSelection {}))
                .with(fixture_test(test_calendar::ValidSelectionHasNoError {}))
                .with(fixture_test(
                    test_calendar::UnavailableSelectionIsInvalid {},
                ))
                .with(fixture_test(test_calendar::RangeLabels {}))
                .with(fixture_test(test_calendar::RangeAcrossMonths {}))
                .with(fixture_test(test_calendar::RangeSelectionPrompts {}))
                .with(fixture_test(
                    test_calendar::RangeKeyboardSelectionControlled {},
                ))
                .with(fixture_test(
                    test_calendar::RangePressSelectionControlled {},
                ))
                .with(fixture_test(test_calendar::RangeReadOnlyKeyboard {}))
                .with(fixture_test(test_calendar::RangeReadOnlyPointer {}))
                .with(fixture_test(test_calendar::RangeDisabled {}))
                .with(fixture_test(test_calendar::RangePressOutsideTheLimits {}))
                .with(fixture_test(
                    test_calendar::RangeEscapeCancelsAPressedRange {},
                ))
                .with(fixture_test(test_calendar::RangeDraggingTheStart {}))
                .with(fixture_test(test_calendar::RangeDragReleasedOutside {}))
                .with(fixture_test(
                    test_calendar::RangeCommittedByAnOutsidePress {},
                ))
                .with(fixture_test(test_calendar::RangePagingDoesNotCommit {}))
                .with(fixture_test(
                    test_calendar::RangePressOnTheStartStartsANewRange {},
                ))
                .with(fixture_test(
                    test_calendar::RangePressOnTheEndStartsANewRange {},
                ))
                .with(fixture_test(
                    test_calendar::RangePressInTheMiddleStartsANewRange {},
                ))
                .with(fixture_test(
                    test_calendar::RangeInvalidEndNotDraggableByTouch {},
                ))
                .with(fixture_test(
                    test_calendar::RangeKeptWhenATouchOnADisabledDateScrolls {},
                ))
                .with(fixture_test(
                    test_calendar::RangeKeptWhenATouchOnAWeekdayScrolls {},
                ))
                .with(fixture_test(
                    test_calendar::RangeKeptWhenATouchOnTheHeadingScrolls {},
                ))
                .with(fixture_test(
                    test_calendar::RangeUnreachableDatesAndButtons {},
                ))
                .with(fixture_test(
                    test_calendar::RangePreviousButtonDisabledByUnavailableDates {},
                ))
                .with(fixture_test(
                    test_calendar::RangeNextButtonDisabledByUnavailableDates {},
                ))
                .with(fixture_test(
                    test_calendar::RangeUnavailableDatesAfterPaging {},
                ))
                .with(fixture_test(
                    test_calendar::RangeStartedAtTheEndOfAvailableDates {},
                ))
                .with(fixture_test(test_calendar::RangeNonContiguous {}))
                .with(fixture_test(
                    test_calendar::RangeBlurSelectsTheNearestAvailableDate {},
                ))
                .with(fixture_test(test_calendar::RangeInvalid {}))
                .with(fixture_test(test_calendar::RangeValidHasNoError {}))
                .with(fixture_test(
                    test_calendar::RangeUnavailableSelectionIsInvalid {},
                ))
                .with(fixture_test(test_calendar::RangeStartAndEndStates {}))
                .with(fixture_test(test_calendar::CommitClearWithoutARange {})),
        )
        .with_test_group(
            TestGroup::new("date_field")
                .with(fixture_test(test_date_field::Structure {}))
                .with(fixture_test(test_date_field::SegmentsAreTextboxesOnIos {}))
                .with(fixture_test(test_date_field::Typing {}))
                .with(fixture_test(test_date_field::ArrowsAndBackspace {}))
                .with(fixture_test(
                    test_date_field::InvalidDateCommittedWhenLeft {},
                ))
                .with(fixture_test(test_date_field::FormReset {}))
                .with(fixture_test(test_date_field::MinValidation {}))
                .with(fixture_test(test_date_field::DisabledAndReadOnly {}))
                .with(fixture_test(test_date_field::DateAndTime {}))
                .with(fixture_test(test_date_field::Zoned {}))
                .with(fixture_test(test_date_field::TimeField {}))
                .with(fixture_test(test_date_field::DatePickerStructure {}))
                .with(fixture_test(
                    test_date_field::DatePickerSelectsInItsCalendar {},
                ))
                .with(fixture_test(
                    test_date_field::DatePickerOpensByAltArrowDown {},
                ))
                .with(fixture_test(
                    test_date_field::DatePickerEscapeKeepsTheValue {},
                ))
                .with(fixture_test(
                    test_date_field::DatePickerFieldEditsTheValue {},
                ))
                .with(fixture_test(
                    test_date_field::DatePickerEditDoesNotComeBack {},
                ))
                .with(fixture_test(test_date_field::DateRangePicker {}))
                .with(fixture_test(test_date_field::GroupStates {}))
                .with(fixture_test(test_date_field::MonthArrowKeys {}))
                .with(fixture_test(test_date_field::MonthWraps {}))
                .with(fixture_test(test_date_field::MonthPageKeys {}))
                .with(fixture_test(test_date_field::MonthHomeEnd {}))
                .with(fixture_test(test_date_field::DayArrowKeys {}))
                .with(fixture_test(test_date_field::DayWraps {}))
                .with(fixture_test(test_date_field::DayPageKeys {}))
                .with(fixture_test(test_date_field::DayHomeEnd {}))
                .with(fixture_test(test_date_field::YearArrowKeys {}))
                .with(fixture_test(test_date_field::YearPageKeys {}))
                .with(fixture_test(test_date_field::HourArrowKeys {}))
                .with(fixture_test(test_date_field::HourWrapsIn12HourTime {}))
                .with(fixture_test(test_date_field::HourWrapsIn24HourTime {}))
                .with(fixture_test(test_date_field::HourPageKeys {}))
                .with(fixture_test(test_date_field::HourHomeEndIn12HourTime {}))
                .with(fixture_test(test_date_field::HourHomeEndIn24HourTime {}))
                .with(fixture_test(test_date_field::MinuteKeys {}))
                .with(fixture_test(test_date_field::SecondKeys {}))
                .with(fixture_test(test_date_field::DayPeriodArrowKeys {}))
                .with(fixture_test(test_date_field::EraShowsAndHides {}))
                .with(fixture_test(test_date_field::EraOfDatesBeforeChrist {}))
                .with(fixture_test(
                    test_date_field::ArrowsConstrainAnInvalidDateOnBlur {},
                ))
                .with(fixture_test(test_date_field::TypingIntoTheMonth {}))
                .with(fixture_test(test_date_field::TypingIntoTheDay {}))
                .with(fixture_test(test_date_field::TypingIntoTheYear {}))
                .with(fixture_test(
                    test_date_field::TypingIntoTheHourIn12HourTime {},
                ))
                .with(fixture_test(
                    test_date_field::TypingIntoTheHourIn24HourTime {},
                ))
                .with(fixture_test(test_date_field::TypingIntoTheMinute {}))
                .with(fixture_test(test_date_field::TypingIntoTheSecond {}))
                .with(fixture_test(test_date_field::TypingIntoTheDayPeriod {}))
                .with(fixture_test(test_date_field::TypingArabicDigits {}))
                .with(fixture_test(
                    test_date_field::TypingASkippedTimeConstrainsOnBlur {},
                ))
                .with(fixture_test(test_date_field::BackspaceInTheMonth {}))
                .with(fixture_test(test_date_field::BackspaceInTheDay {}))
                .with(fixture_test(test_date_field::BackspaceInTheYear {}))
                .with(fixture_test(
                    test_date_field::BackspaceInTheHourIn12HourTime {},
                ))
                .with(fixture_test(
                    test_date_field::BackspaceInTheHourIn24HourTime {},
                ))
                .with(fixture_test(test_date_field::BackspaceInTheDayPeriod {}))
                .with(fixture_test(
                    test_date_field::BackspaceInTheMinuteAndSecond {},
                ))
                .with(fixture_test(test_date_field::BackspaceWithArabicDigits {}))
                .with(fixture_test(
                    test_date_field::ClearingEverySegmentEmptiesTheValue {},
                ))
                .with(fixture_test(test_date_field::SpinButtonValues {}))
                .with(fixture_test(
                    test_date_field::ClearingTheHourKeepsTheDayPeriod {},
                ))
                .with(fixture_test(test_date_field::HourThroughTheFallBack {}))
                .with(fixture_test(
                    test_date_field::HourThroughTheFallBackWithoutMinutes {},
                ))
                .with(fixture_test(
                    test_date_field::TimeFieldThroughTheFallBackFromThePlaceholder {},
                ))
                .with(fixture_test(
                    test_date_field::TimeFieldThroughTheFallBack {},
                ))
                .with(fixture_test(test_date_field::TimeFieldKeepsItsTimeZone {}))
                .with(fixture_test(test_date_field::TimeBoundsOnTheValuesDay {}))
                .with(fixture_test(
                    test_date_field::PressingTheFieldFocusesASegment {},
                ))
                .with(fixture_test(test_date_field::AutoFocus {}))
                .with(fixture_test(test_date_field::FocusChanges {}))
                .with(fixture_test(test_date_field::LabelledByAriaLabel {}))
                .with(fixture_test(test_date_field::LabelledByAnotherElement {}))
                .with(fixture_test(test_date_field::HelpTextWithAValue {}))
                .with(fixture_test(test_date_field::ErrorMessage {}))
                .with(fixture_test(test_date_field::NoErrorMessageWhileValid {}))
                .with(fixture_test(test_date_field::UnavailableDate {}))
                .with(fixture_test(test_date_field::HoverState {}))
                .with(fixture_test(test_date_field::DisabledState {}))
                .with(fixture_test(test_date_field::ReadOnlyState {}))
                .with(fixture_test(test_date_field::RequiredState {}))
                .with(fixture_test(test_date_field::RequiredValidation {}))
                .with(fixture_test(test_date_field::ResetClearsTheValidation {}))
                .with(fixture_test(
                    test_date_field::ValidationCommitsOnBlurOnlyAfterAChange {},
                ))
                .with(fixture_test(test_date_field::NativeMinAndMax {}))
                .with(fixture_test(test_date_field::NativeValidate {}))
                .with(fixture_test(test_date_field::NativeServerValidation {}))
                .with(fixture_test(test_date_field::CustomNativeMessage {}))
                .with(fixture_test(test_date_field::AriaMinAndMax {}))
                .with(fixture_test(test_date_field::AriaValidate {}))
                .with(fixture_test(test_date_field::AriaServerValidation {}))
                .with(fixture_test(test_date_field::TimeFieldNativeMinAndMax {}))
                .with(fixture_test(
                    test_date_field::TimeFieldDescriptionAndReset {},
                ))
                .with(fixture_test(test_date_field::HiddenDateInputContainer {}))
                .with(fixture_test(test_date_field::RemovingTheFocusedField {})),
        )
        .with_test_group(
            TestGroup::new("date_picker")
                .with(fixture_test(test_date_picker::CloseOnSelect {}))
                .with(fixture_test(test_date_picker::DisabledPicker {}))
                .with(fixture_test(test_date_picker::ProgrammaticValue {}))
                .with(fixture_test(test_date_picker::RequiredPicker {}))
                .with(fixture_test(test_date_picker::RequiredTimeField {}))
                .with(fixture_test(test_date_picker::RangePlaceholderTimes {}))
                .with(fixture_test(test_date_picker::EnterDoesNothing {}))
                .with(fixture_test(test_date_picker::HeldKeys {}))
                .with(fixture_test(test_date_picker::DeletingAPartialField {}))
                .with(fixture_test(test_date_picker::Autofill {}))
                .with(fixture_test(test_date_picker::SelectionWhileElsewhere {}))
                .with(fixture_test(test_date_picker::GermanOrder {}))
                .with(fixture_test(test_date_picker::TwelveHourClocks {}))
                .with(fixture_test(test_date_picker::RightToLeft {}))
                .with(fixture_test(test_date_picker::SwitchingToRightToLeft {}))
                .with(fixture_test(test_date_picker::Slots {}))
                .with(fixture_test(test_date_picker::SlotsDescriptionAfter {}))
                .with(fixture_test(test_date_picker::RangeSlots {}))
                .with(fixture_test(
                    test_date_picker::DataAttributesOnTheOuterElement {},
                ))
                .with(fixture_test(test_date_picker::RangePressedWhileOpen {}))
                .with(fixture_test(test_date_picker::InvalidState {}))
                .with(fixture_test(test_date_picker::RequiredState {}))
                .with(fixture_test(test_date_picker::FormValue {}))
                .with(fixture_test(test_date_picker::RangeValidationErrors {}))
                .with(fixture_test(test_date_picker::RangeCloseOnSelect {}))
                .with(fixture_test(test_date_picker::RangeDisabled {}))
                .with(fixture_test(test_date_picker::ClearContexts {}))
                .with(fixture_test(test_date_picker::RangeClearContexts {}))
                .with(fixture_test(test_date_picker::SpecifiedDate {}))
                .with(fixture_test(test_date_picker::GranularitySecond {}))
                .with(fixture_test(test_date_picker::DefaultStructure {}))
                .with(fixture_test(test_date_picker::ReadOnly {}))
                .with(fixture_test(
                    test_date_picker::RequiredAndInvalidSegments {},
                ))
                .with(fixture_test(test_date_picker::ReadOnlyTimeZone {}))
                .with(fixture_test(
                    test_date_picker::PlaceholderFocusedInTheCalendar {},
                ))
                .with(fixture_test(
                    test_date_picker::SelectedDateFocusedOverThePlaceholder {},
                ))
                .with(fixture_test(test_date_picker::ForcedLeadingZeros {}))
                .with(fixture_test(test_date_picker::ButtonControlsTheDialog {}))
                .with(fixture_test(test_date_picker::RangeOpensByAltArrowDown {}))
                .with(fixture_test(test_date_picker::ArrowKeysBetweenSegments {}))
                .with(fixture_test(test_date_picker::PressOnALiteral {}))
                .with(fixture_test(test_date_picker::AutoFocus {}))
                .with(fixture_test(test_date_picker::RangeAutoFocus {}))
                .with(fixture_test(
                    test_date_picker::FocusChangeWithinThePicker {},
                ))
                .with(fixture_test(test_date_picker::FocusChangeWhenLeaving {}))
                .with(fixture_test(test_date_picker::FocusChangeWhenOpening {}))
                .with(fixture_test(test_date_picker::FocusChangeAfterClosing {}))
                .with(fixture_test(test_date_picker::TimeFieldInThePopover {}))
                .with(fixture_test(
                    test_date_picker::DeletingInThePopoversTimeField {},
                ))
                .with(fixture_test(
                    test_date_picker::ChangeOnceDateAndTimeAreSelected {},
                ))
                .with(fixture_test(
                    test_date_picker::ClosingConfirmsThePlaceholderTime {},
                ))
                .with(fixture_test(
                    test_date_picker::ClosingWithoutADateCommitsNothing {},
                ))
                .with(fixture_test(
                    test_date_picker::ClosingKeepsAValidDateTime {},
                ))
                .with(fixture_test(
                    test_date_picker::ClearingTheValueClearsDateAndTime {},
                ))
                .with(fixture_test(
                    test_date_picker::RangeTimeFieldsInThePopover {},
                ))
                .with(fixture_test(
                    test_date_picker::RangeChangeOnceDatesAndTimesAreSelected {},
                ))
                .with(fixture_test(
                    test_date_picker::RangeClosingWithoutDatesCommitsNothing {},
                ))
                .with(fixture_test(test_date_picker::RangeClearingTheValue {}))
                .with(fixture_test(test_date_picker::Labelling {}))
                .with(fixture_test(test_date_picker::LabellingWithAriaLabel {}))
                .with(fixture_test(
                    test_date_picker::LabellingWithAriaLabelledby {},
                ))
                .with(fixture_test(test_date_picker::HelpText {}))
                .with(fixture_test(test_date_picker::ErrorMessage {}))
                .with(fixture_test(test_date_picker::EraForBcDates {}))
                .with(fixture_test(
                    test_date_picker::MouseDownFocusesTheFirstSegment {},
                ))
                .with(fixture_test(
                    test_date_picker::MouseDownFocusesTheFirstUnfilledSegment {},
                ))
                .with(fixture_test(
                    test_date_picker::MouseDownFocusesTheLastSegment {},
                ))
                .with(fixture_test(test_date_picker::RangeMouseDownOnEachField {}))
                .with(fixture_test(test_date_picker::RangeMouseDownOnTheDash {}))
                .with(fixture_test(
                    test_date_picker::RemovingTheEraFocusesThePreviousSegment {},
                ))
                .with(fixture_test(test_date_picker::BelowTheMinimum {}))
                .with(fixture_test(test_date_picker::AboveTheMaximum {}))
                .with(fixture_test(test_date_picker::RangeLimits {}))
                .with(fixture_test(test_date_picker::ZoneKeptWhenCleared {}))
                .with(fixture_test(test_date_picker::FormReset {}))
                .with(fixture_test(test_date_picker::NativeMinAndMax {}))
                .with(fixture_test(test_date_picker::NativeValidate {}))
                .with(fixture_test(test_date_picker::NativeServerErrors {}))
                .with(fixture_test(test_date_picker::NativeCustomMessage {}))
                .with(fixture_test(test_date_picker::NativeErrorClearedOnReset {}))
                .with(fixture_test(
                    test_date_picker::NativeErrorUpdatedByTheCalendar {},
                ))
                .with(fixture_test(test_date_picker::AriaMinAndMax {}))
                .with(fixture_test(test_date_picker::AriaValidate {}))
                .with(fixture_test(test_date_picker::AriaServerErrors {}))
                .with(fixture_test(test_date_picker::RangeDescriptionWithTimes {})),
        )
        .with_test_group(
            TestGroup::new("slider")
                .with(fixture_test(test_slider::LabelledGroup {}))
                .with(fixture_test(test_slider::Fill {}))
                .with(fixture_test(test_slider::Keyboard {}))
                .with(fixture_test(test_slider::TrackClick {}))
                .with(fixture_test(test_slider::DraggingState {}))
                .with(fixture_test(test_slider::TwoThumbs {}))
                .with(fixture_test(test_slider::Orientation {}))
                .with(fixture_test(test_slider::DisabledState {}))
                .with(fixture_test(test_slider::Tooltips {}))
                .with(fixture_test(test_slider::ClosestThumbByClick {}))
                .with(fixture_test(test_slider::ClosestThumbByDrag {}))
                .with(fixture_test(test_slider::StackedThumbsBefore {}))
                .with(fixture_test(test_slider::StackedThumbsAfter {}))
                .with(fixture_test(test_slider::ManyStackedThumbsBefore {}))
                .with(fixture_test(test_slider::ManyStackedThumbsAfter {}))
                .with(fixture_test(test_slider::DisabledTrack {}))
                .with(fixture_test(test_slider::VerticalDrag {}))
                .with(fixture_test(test_slider::RightToLeft {}))
                .with(fixture_test(test_slider::Keys {}))
                .with(fixture_test(test_slider::KeysVertical {}))
                .with(fixture_test(test_slider::RepeatedPageKeys {}))
                .with(fixture_test(test_slider::InputEvent {}))
                .with(fixture_test(test_slider::DisabledThumb {}))
                .with(fixture_test(test_slider::FormProp {}))
                .with(fixture_test(test_slider::ThumbLabels {}))
                .with(fixture_test(test_slider::Attributes {}))
                .with(fixture_test(test_slider::ThreeThumbs {}))
                .with(fixture_test(test_slider::ControlledThumbs {}))
                .with(fixture_test(test_slider::RestrictedValues {}))
                .with(fixture_test(test_slider::MissingValue {})),
        )
        .with_test_group(
            TestGroup::new("dnd")
                .with(fixture_test(test_dnd::BasicDragAndDrop {}))
                .with(fixture_test(test_dnd::EscapeCancels {}))
                .with(fixture_test(test_dnd::ReorderAList {}))
                .with(fixture_test(test_dnd::NativeBasicDragAndDrop {}))
                .with(fixture_test(test_dnd::TabForwardSkipsNonDropTargets {}))
                .with(fixture_test(test_dnd::TabBackwardSkipsNonDropTargets {}))
                .with(fixture_test(test_dnd::PrefersAnAncestorDropTarget {}))
                .with(fixture_test(test_dnd::EnterOnTheDragSourceCancels {}))
                .with(fixture_test(test_dnd::IgnoresDropTargetsInHiddenTrees {}))
                .with(fixture_test(test_dnd::ARemovedDropTarget {}))
                .with(fixture_test(test_dnd::ADropTargetHiddenDuringTheDrag {}))
                .with(fixture_test(
                    test_dnd::AnAddedDropTargetKeepsTheCurrentTarget {},
                ))
                .with(fixture_test(test_dnd::AHiddenDragSourceIsSkipped {}))
                .with(fixture_test(test_dnd::EscapeWithAHiddenDragSource {}))
                .with(fixture_test(test_dnd::DisabledDrag {}))
                .with(fixture_test(test_dnd::DisabledDrop {}))
                .with(fixture_test(test_dnd::DropOperationOverride {}))
                .with(fixture_test(test_dnd::AllowedDropOperations {}))
                .with(fixture_test(test_dnd::CanceledTargetsAreHidden {}))
                .with(fixture_test(test_dnd::AltEnterActivates {}))
                .with(fixture_test(test_dnd::NativeDisabledDrag {}))
                .with(fixture_test(test_dnd::NativeDisabledDrop {}))
                .with(fixture_test(test_dnd::NavigatingWithFocusEventsOnly {}))
                .with(fixture_test(test_dnd::HidesEverythingButDropTargets {}))
                .with(fixture_test(test_dnd::ClickingTheDragSourceCancels {}))
                .with(fixture_test(test_dnd::RestoresFocusFromNonDropTargets {}))
                .with(fixture_test(test_dnd::IgnoresClicksNotFromScreenReaders {}))
                .with(fixture_test(
                    test_dnd::TalkbackClickOnTheDragSourceCancels {},
                ))
                .with(fixture_test(
                    test_dnd::TalkbackDoubleTapOnADropTargetDrops {},
                ))
                .with(fixture_test(test_dnd::ScreenReaderAnAddedDropTarget {}))
                .with(fixture_test(test_dnd::ScreenReaderAnAddedNonDropTarget {}))
                .with(fixture_test(test_dnd::ScreenReaderARemovedDropTarget {}))
                .with(fixture_test(test_dnd::ScreenReaderAHiddenDropTarget {})),
        )
        .with_test_group(
            TestGroup::new("dnd_draggable_collection")
                .with(fixture_test(
                    test_dnd_draggable_collection::NativeASingleItem {},
                ))
                .with(fixture_test(
                    test_dnd_draggable_collection::NativeSeveralSelectedItems {},
                ))
                .with(fixture_test(
                    test_dnd_draggable_collection::NativeOnlyTheDraggedItem {},
                ))
                .with(fixture_test(
                    test_dnd_draggable_collection::KeyboardASingleItem {},
                ))
                .with(fixture_test(
                    test_dnd_draggable_collection::KeyboardSeveralSelectedItems {},
                ))
                .with(fixture_test(
                    test_dnd_draggable_collection::KeyboardOnlyTheCurrentItem {},
                ))
                .with(fixture_test(
                    test_dnd_draggable_collection::KeyboardAListBoxWithoutDragButtons {},
                ))
                .with(fixture_test(
                    test_dnd_draggable_collection::KeyboardRowActions {},
                ))
                .with(fixture_test(
                    test_dnd_draggable_collection::ScreenReaderASingleItem {},
                ))
                .with(fixture_test(
                    test_dnd_draggable_collection::ScreenReaderSeveralSelectedItems {},
                ))
                .with(fixture_test(
                    test_dnd_draggable_collection::ScreenReaderOnlyTheClickedItem {},
                ))
                .with(fixture_test(
                    test_dnd_draggable_collection::ScreenReaderAListBoxWithoutDragButtons {},
                ))
                .with(fixture_test(
                    test_dnd_draggable_collection::ScreenReaderRowActions {},
                )),
        )
        .with_test_group(
            TestGroup::new("dnd_native")
                .with(fixture_test(test_dnd_native::DragMovesOnlyWhenMoving {}))
                .with(fixture_test(test_dnd_native::DropMovesOnlyWhenMoving {}))
                .with(fixture_test(test_dnd_native::DropExitWhenLeaving {}))
                .with(fixture_test(test_dnd_native::DropExitOnDrop {}))
                .with(fixture_test(test_dnd_native::DropActivateWhenHeld {}))
                .with(fixture_test(test_dnd_native::NoDropActivateAfterLeaving {}))
                .with(fixture_test(
                    test_dnd_native::NestedElementsNeitherEnterNorExit {},
                ))
                .with(fixture_test(test_dnd_native::NestedDragSource {}))
                .with(fixture_test(test_dnd_native::NestedDropTargetDrop {}))
                .with(fixture_test(test_dnd_native::NestedDropTargetEnter {}))
                .with(fixture_test(test_dnd_native::NestedDropTargetExit {}))
                .with(fixture_test(test_dnd_native::NestedDropTargetActivate {}))
                .with(fixture_test(test_dnd_native::NestedDropTargetMove {}))
                .with(fixture_test(test_dnd_native::CustomDataTypes {}))
                .with(fixture_test(test_dnd_native::SeveralItemsOfACustomType {}))
                .with(fixture_test(test_dnd_native::AnItemOfSeveralTypes {}))
                .with(fixture_test(test_dnd_native::SeveralItemsOfSeveralTypes {}))
                .with(fixture_test(test_dnd_native::SeveralNativeTypes {}))
                .with(fixture_test(test_dnd_native::AFile {}))
                .with(fixture_test(test_dnd_native::SeveralFiles {}))
                .with(fixture_test(test_dnd_native::TextAndFiles {}))
                .with(fixture_test(test_dnd_native::ADirectory {}))
                .with(fixture_test(test_dnd_native::AFileOfAnUnknownType {}))
                .with(fixture_test(
                    test_dnd_native::GetDropOperationOverridesTheDefault {},
                ))
                .with(fixture_test(
                    test_dnd_native::AllowedOperationsLimitTheDrop {},
                ))
                .with(fixture_test(test_dnd_native::GetDropOperationCancels {}))
                .with(fixture_test(
                    test_dnd_native::EffectAllowedNarrowedByTheBrowser {},
                ))
                .with(fixture_test(
                    test_dnd_native::EffectAllowedNarrowedToARefusedOperation {},
                ))
                .with(fixture_test(
                    test_dnd_native::ModifierKeysPickTheOperation {},
                ))
                .with(fixture_test(
                    test_dnd_native::WrongEffectAllowedOfTheBrowser {},
                ))
                .with(fixture_test(test_dnd_native::FileTypesBeforeTheDrop {}))
                .with(fixture_test(
                    test_dnd_native::UnknownFileTypesBeforeTheDrop {},
                ))
                .with(fixture_test(test_dnd_native::NoFileTypesBeforeTheDrop {}))
                .with(fixture_test(test_dnd_native::TheItemsJsonIsNoDragType {}))
                .with(fixture_test(test_dnd_native::ADragPreview {}))
                .with(fixture_test(
                    test_dnd_native::ASmallDragPreviewIsCentered {},
                ))
                .with(fixture_test(test_dnd_native::TheDragPreviewOffset {}))
                .with(fixture_test(
                    test_dnd_native::TheDragPreviewOffsetIsClamped {},
                ))
                .with(fixture_test(test_dnd_native::ARemovedDragSourceEndsOnce {}))
                .with(fixture_test(test_dnd_native::ADragEndingInItsFirstFrame {}))
                .with(fixture_test(test_dnd_native::KeyboardCustomDataTypes {}))
                .with(fixture_test(
                    test_dnd_native::KeyboardSeveralItemsOfACustomType {},
                ))
                .with(fixture_test(
                    test_dnd_native::KeyboardAnItemOfSeveralTypes {},
                ))
                .with(fixture_test(
                    test_dnd_native::KeyboardSeveralItemsOfSeveralTypes {},
                ))
                .with(fixture_test(test_dnd_native::KeyboardNestedDropTargets {}))
                .with(fixture_test(
                    test_dnd_native::ASecondEnterStartsNoSecondDrag {},
                )),
        )
        .with_test_group(
            TestGroup::new("dnd_collection")
                .with(fixture_test(test_dnd_collection::BasicDragAndDrop {}))
                .with(fixture_test(test_dnd_collection::ArrowKeyNavigation {}))
                .with(fixture_test(test_dnd_collection::HomeAndEnd {}))
                .with(fixture_test(test_dnd_collection::PageUpAndPageDown {}))
                .with(fixture_test(
                    test_dnd_collection::PageUpAndPageDownSkipInvalidTargets {},
                ))
                .with(fixture_test(
                    test_dnd_collection::AfterTheLastFocusedItem {},
                ))
                .with(fixture_test(test_dnd_collection::AfterTheSelectedItems {}))
                .with(fixture_test(test_dnd_collection::BeforeTheSelectedItems {}))
                .with(fixture_test(test_dnd_collection::OnTheFirstSelectedItem {}))
                .with(fixture_test(test_dnd_collection::OnTheLastSelectedItem {}))
                .with(fixture_test(test_dnd_collection::NativeBasicDragAndDrop {}))
                .with(fixture_test(test_dnd_collection::NativeDropOnAnItem {}))
                .with(fixture_test(
                    test_dnd_collection::ScreenReaderBasicDragAndDrop {},
                ))
                .with(fixture_test(
                    test_dnd_collection::ScreenReaderDescriptions {},
                ))
                .with(fixture_test(
                    test_dnd_collection::ScreenReaderInsertionIndicators {},
                ))
                .with(fixture_test(
                    test_dnd_collection::ScreenReaderHidesRowsNotTakingTheDrop {},
                )),
        )
        .with_test_group(
            TestGroup::new("clipboard")
                .with(fixture_test(test_clipboard::Copies {}))
                .with(fixture_test(test_clipboard::CopiesOnlyWhenFocused {}))
                .with(fixture_test(test_clipboard::NoCopyWithoutItems {}))
                .with(fixture_test(test_clipboard::Cuts {}))
                .with(fixture_test(test_clipboard::CutsOnlyWhenFocused {}))
                .with(fixture_test(test_clipboard::NoCutWithoutItems {}))
                .with(fixture_test(test_clipboard::NoCutWithoutOnCut {}))
                .with(fixture_test(test_clipboard::Pastes {}))
                .with(fixture_test(test_clipboard::PastesOnlyWhenFocused {}))
                .with(fixture_test(test_clipboard::NoPasteWithoutOnPaste {}))
                .with(fixture_test(test_clipboard::CustomTypes {}))
                .with(fixture_test(test_clipboard::MultipleItemsOfACustomType {}))
                .with(fixture_test(test_clipboard::ItemsOfMultipleTypes {}))
                .with(fixture_test(
                    test_clipboard::MultipleItemsOfMultipleTypes {},
                ))
                .with(fixture_test(test_clipboard::TheActionOfACut {}))
                .with(fixture_test(test_clipboard::TheActionOfACopy {})),
        )
        .with_test_group(
            TestGroup::new("clipboard_write")
                .with(fixture_test(test_clipboard_write::WritesText {}))
                .with(fixture_test(test_clipboard_write::WritesTextLoadedLater {}))
                .with(fixture_test(
                    test_clipboard_write::WritesNothingWithoutText {},
                ))
                .with(fixture_test(test_clipboard_write::ADeniedWrite {}))
                .with(fixture_test(test_clipboard_write::NoClipboard {})),
        )
        .with_test_group(
            TestGroup::new("text_field")
                .with(fixture_test(test_text_field::Labelling {}))
                .with(fixture_test(test_text_field::TypingUpdatesTheState {}))
                .with(fixture_test(test_text_field::Validation {}))
                .with(fixture_test(
                    test_text_field::ProgrammaticChangesUpdateTheInput {},
                ))
                .with(fixture_test(
                    test_text_field::FormResetRestoresTheDefault {},
                ))
                .with(fixture_test(test_text_field_atoms::ProvidesSlotsInput {}))
                .with(fixture_test(
                    test_text_field_atoms::ProvidesSlotsTextarea {},
                ))
                .with(fixture_test(test_text_field_atoms::HoverState {}))
                .with(fixture_test(test_text_field_atoms::FocusVisibleState {}))
                .with(fixture_test(
                    test_text_field_atoms::ReadOnlyAndRequiredState {},
                ))
                .with(fixture_test(
                    test_text_field_atoms::NativeValidationErrorsInput {},
                ))
                .with(fixture_test(
                    test_text_field_atoms::NativeValidationErrorsTextarea {},
                ))
                .with(fixture_test(
                    test_text_field_atoms::CustomizedValidationErrors {},
                ))
                .with(fixture_test(
                    test_text_field_atoms::InvalidWithoutMessageRendersNoError {},
                ))
                .with(fixture_test(test_text_field_atoms::IdGoesOnTheInput {}))
                .with(fixture_test(test_text_field_atoms::FormAttribute {}))
                .with(fixture_test(
                    test_text_field_atoms::ServerValidationErrors {},
                ))
                .with(fixture_test(
                    test_text_field_atoms::BoundValuesKeepTheDomInSync {},
                ))
                .with(fixture_test(
                    test_text_field_atoms::FormValidationBehavior {},
                ))
                .with(fixture_test(
                    test_text_field_atoms::NativeValidateFunction {},
                ))
                .with(fixture_test(
                    test_text_field_atoms::NoAutoFocusIfInvalidIsPrevented {},
                ))
                .with(fixture_test(test_text_field_atoms::AriaValidateFunction {}))
                .with(fixture_test(test_text_field_atoms::AriaServerValidation {}))
                .with(fixture_test(test_text_field_atoms::DisabledState {})),
        )
        .with_test_group(
            TestGroup::new("search_field")
                .with(fixture_test(test_search_field::ProvidesSlots {}))
                .with(fixture_test(test_search_field::EnterSubmits {}))
                .with(fixture_test(test_search_field::EscapeClearsOnce {}))
                .with(fixture_test(
                    test_search_field::ClearButtonClearsAndFocusesTheInput {},
                ))
                .with(fixture_test(
                    test_search_field::ClearButtonShowsFocusVisible {},
                ))
                .with(fixture_test(
                    test_search_field::DisabledFieldIgnoresKeysAndTheClearButton {},
                ))
                .with(fixture_test(
                    test_search_field::EnterWithoutOnSubmitSubmitsTheForm {},
                ))
                .with(fixture_test(test_search_field::ValidationErrors {}))
                .with(fixture_test(test_search_field::ReadOnly {}))
                .with(fixture_test(test_search_field::FormAttribute {}))
                .with(fixture_test(test_search_field::InputType {})),
        )
        .with_test_group(
            TestGroup::new("combobox")
                .with(fixture_test(test_combobox::AriaStructure {}))
                .with(fixture_test(
                    test_combobox::TypingFiltersAndKeyboardSelects {},
                ))
                .with(fixture_test(test_combobox::EscapeRevertsTheInput {}))
                .with(fixture_test(
                    test_combobox::ButtonShowsAllOptionsAndClickSelects {},
                ))
                .with(fixture_test(
                    test_combobox::ArrowDownOpensWithTheSelectedOptionFocused {},
                ))
                .with(fixture_test(
                    test_combobox::ClearingTheInputClearsTheValue {},
                ))
                .with(fixture_test(
                    test_combobox::ExternallyChangedValueShowsInTheInput {},
                ))
                .with(fixture_test(
                    test_combobox::ExternallySelectedAddedItemShowsInTheInput {},
                ))
                .with(fixture_test(
                    test_combobox::ItemAddedAfterItsSelectionShowsInTheInput {},
                ))
                .with(fixture_test(
                    test_combobox::ValueAndItemsDerivedFromOneSignal {},
                ))
                .with(fixture_test(test_combobox::ValueChangedByATimer {}))
                .with(fixture_test(test_combobox::ValueWrittenInAnEffect {}))
                .with(fixture_test(
                    test_combobox::PopoverInAModalStaysInteractive {},
                ))
                .with(fixture_test(test_combobox::ButtonIsPressedWhileOpen {}))
                .with(fixture_test(test_combobox::ButtonTogglesThePopover {}))
                .with(fixture_test(test_combobox::ClickingTheInputKeepsItOpen {}))
                .with(fixture_test(test_combobox::ArrowUpOpensOnTheLastOption {}))
                .with(fixture_test(
                    test_combobox::PickingTheSelectedOptionResetsTheText {},
                ))
                .with(fixture_test(test_combobox::TabCommitsTheFocusedOption {}))
                .with(fixture_test(test_combobox::ReadOnly {}))
                .with(fixture_test(test_combobox::Disabled {}))
                .with(fixture_test(test_combobox::ClosesWhenThePageScrolls {}))
                .with(fixture_test(
                    test_combobox::PopoverSpansTheInputAndTheButton {},
                ))
                .with(fixture_test(
                    test_combobox::PopoverContentIsntPartOfTheComboBox {},
                ))
                .with(fixture_test(test_combobox::ServerFilteredOptionsReopen {}))
                .with(fixture_test(
                    test_combobox::ComboBoxValueListsTheSelection {},
                ))
                .with(fixture_test(
                    test_combobox::LeftAndRightClearTheVirtualFocus {},
                ))
                .with(fixture_test(
                    test_combobox::EscapeDoesntPreventTheDefault {},
                ))
                .with(fixture_test(test_combobox::HeldArrowKeysRepeat {}))
                .with(fixture_test(test_combobox_forms::SelectAnOption {}))
                .with(fixture_test(test_combobox_forms::CustomTextOnBlur {}))
                .with(fixture_test(test_combobox_forms::EscapeKeepsCustomText {}))
                .with(fixture_test(test_combobox_forms::EnterCommitsCustomText {}))
                .with(fixture_test(test_combobox_forms::NativeValidation {}))
                .with(fixture_test(test_combobox_forms::AriaValidation {}))
                .with(fixture_test(test_combobox_forms::MultipleSelection {}))
                .with(fixture_test(test_combobox_forms::MultipleFormReset {}))
                .with(fixture_test(
                    test_combobox_forms::RequiredWithMultipleSelection {},
                ))
                .with(fixture_test(test_combobox_forms::FormValue {}))
                .with(fixture_test(test_combobox_forms::FocusTrigger {}))
                .with(fixture_test(test_combobox_forms::ManualTrigger {}))
                .with(fixture_test(test_combobox_forms::FilteringSections {}))
                .with(fixture_test(
                    test_combobox_forms::DisabledOptionIsSkipped {},
                ))
                .with(fixture_test(
                    test_combobox_forms::EnterWithoutAFocusedOption {},
                ))
                .with(fixture_test(
                    test_combobox_forms::SingleSelectionFormReset {},
                ))
                .with(fixture_test(
                    test_combobox_forms::ClickingASectionHeaderKeepsItOpen {},
                ))
                .with(fixture_test(
                    test_combobox_forms::BlurAfterPickingReportsNoChange {},
                )),
        )
        .with_test_group(
            TestGroup::new("checkbox")
                .with(fixture_test(test_checkbox::SelectedState {}))
                .with(fixture_test(test_checkbox::KeyboardAndFocusRing {}))
                .with(fixture_test(test_checkbox::VirtualLabelClick {}))
                .with(fixture_test(test_checkbox::Hover {}))
                .with(fixture_test(test_checkbox::PressState {}))
                .with(fixture_test(test_checkbox::PressStateWithKeyboard {}))
                .with(fixture_test(test_checkbox::IndeterminateState {}))
                .with(fixture_test(test_checkbox::DisabledState {}))
                .with(fixture_test(test_checkbox::ReadOnlyState {}))
                .with(fixture_test(test_checkbox::InvalidState {}))
                .with(fixture_test(test_checkbox::RequiredState {}))
                .with(fixture_test(test_checkbox::BoundState {}))
                .with(fixture_test(test_checkbox::BoundReadOnlyAndOnChange {}))
                .with(fixture_test(test_checkbox::Group {}))
                .with(fixture_test(test_checkbox::GroupDisabledAndReadOnly {}))
                .with(fixture_test(test_checkbox::GroupValidation {}))
                .with(fixture_test(test_checkbox::NativeGroupValidateFunction {}))
                .with(fixture_test(
                    test_checkbox::NativeCheckboxValidateFunction {},
                ))
                .with(fixture_test(test_checkbox::NativeGroupServerValidation {}))
                .with(fixture_test(test_checkbox::CustomNativeErrorMessage {}))
                .with(fixture_test(test_checkbox::AriaGroupValidateFunction {}))
                .with(fixture_test(test_checkbox::AriaCheckboxValidateFunction {}))
                .with(fixture_test(test_checkbox::AriaGroupServerValidation {})),
        )
        .with_test_group(
            TestGroup::new("forms")
                .with(fixture_test(test_forms::FormReset {}))
                .with(fixture_test(test_forms::CanceledFormReset {}))
                .with(fixture_test(test_forms::FormResetWithStoppedPropagation {}))
                .with(fixture_test(test_forms::FormResetCanceledInCapturePhase {}))
                .with(fixture_test(test_forms::ImplicitSubmissionWithEnter {}))
                .with(fixture_test(test_forms::RightToLeftArrowKeys {}))
                .with(fixture_test(test_forms::CheckboxGroupRealtimeValidation {}))
                .with(fixture_test(test_forms::FieldAtoms {})),
        )
        .with_test_group(
            TestGroup::new("radio_group")
                .with(fixture_test(test_radio_group::Structure {}))
                .with(fixture_test(
                    test_radio_group::TabEntersAndLeavesTheGroup {},
                ))
                .with(fixture_test(test_radio_group::SelectionByPress {}))
                .with(fixture_test(test_radio_group::VirtualLabelClick {}))
                .with(fixture_test(test_radio_group::ArrowKeys {}))
                .with(fixture_test(test_radio_group::SelectedRadioIsTheTabStop {}))
                .with(fixture_test(test_radio_group::SkipsDisabledRadios {}))
                .with(fixture_test(test_radio_group::Horizontal {}))
                .with(fixture_test(test_radio_group::DisabledGroup {}))
                .with(fixture_test(test_radio_group::ReadOnlyGroup {}))
                .with(fixture_test(test_radio_group::Validation {}))
                .with(fixture_test(test_radio_group::ValidationWithTheKeyboard {}))
                .with(fixture_test(test_radio_group::Hover {}))
                .with(fixture_test(test_radio_group::PressState {}))
                .with(fixture_test(test_radio_group::PressStateWithKeyboard {}))
                .with(fixture_test(test_radio_group::Controlled {}))
                .with(fixture_test(test_radio_group::LabelContextStaysInside {}))
                .with(fixture_test(test_radio_group::TypedValues {})),
        )
        .with_test_group(
            TestGroup::new("switch")
                .with(fixture_test(test_switch::SelectedState {}))
                .with(fixture_test(test_switch::Keyboard {}))
                .with(fixture_test(test_switch::VirtualLabelClick {}))
                .with(fixture_test(test_switch::Hover {}))
                .with(fixture_test(test_switch::PressState {}))
                .with(fixture_test(test_switch::PressStateWithKeyboard {}))
                .with(fixture_test(test_switch::DisabledState {}))
                .with(fixture_test(test_switch::ReadOnlyState {}))
                .with(fixture_test(test_switch::BoundState {}))
                .with(fixture_test(test_switch::BoundReadOnly {})),
        )
        .with_test_group(
            TestGroup::new("toggle_button")
                .with(fixture_test(test_toggle_button::ToggleButton {}))
                .with(fixture_test(test_toggle_button::DisabledToggleButton {}))
                .with(fixture_test(test_toggle_button::SingleSelection {}))
                .with(fixture_test(test_toggle_button::MultipleSelection {}))
                .with(fixture_test(test_toggle_button::HorizontalNavigation {}))
                .with(fixture_test(test_toggle_button::TabLeavesAndRestores {}))
                .with(fixture_test(test_toggle_button::VerticalNavigation {}))
                .with(fixture_test(test_toggle_button::DisabledGroup {})),
        )
        .with_test_group(
            TestGroup::new("aria_hide_outside")
                .with(fixture_test(
                    test_aria_hide_outside::HidesEverythingButTheTarget {},
                ))
                .with(fixture_test(
                    test_aria_hide_outside::HidesTheCellsOfAHiddenRow {},
                ))
                .with(fixture_test(
                    test_aria_hide_outside::NestedHidesRestoredOutOfOrder {},
                ))
                .with(fixture_test(
                    test_aria_hide_outside::NestedHidesRestoredInOrder {},
                ))
                .with(fixture_test(
                    test_aria_hide_outside::HidesARootWithoutTheTarget {},
                ))
                .with(fixture_test(
                    test_aria_hide_outside::ShowsOverlaysRegisteredLate {},
                ))
                .with(fixture_test(test_aria_hide_outside::AddedOutside {}))
                .with(fixture_test(
                    test_aria_hide_outside::AddedToAHiddenContainer {},
                ))
                .with(fixture_test(
                    test_aria_hide_outside::AddedInsideTheTarget {},
                ))
                .with(fixture_test(
                    test_aria_hide_outside::AddedWithATopLayerElement {},
                ))
                .with(fixture_test(
                    test_aria_hide_outside::ReparentedIntoTheTarget {},
                ))
                .with(fixture_test(
                    test_aria_hide_outside::ReparentedIntoAHiddenContainer {},
                ))
                .with(fixture_test(test_aria_hide_outside::UnhideAfterReorder {}))
                .with(fixture_test(
                    test_aria_hide_outside::InertModeHidesSvgWithAriaHidden {},
                )),
        )
        .with_test_group(
            TestGroup::new("dialog")
                .with(fixture_test(test_dialog::DismissButtonCloses {}))
                .with(fixture_test(test_dialog::EscapeCloses {}))
                .with(fixture_test(test_dialog::AlertDialog {}))
                .with(fixture_test(test_dialog::KeyboardOpenAndCloseFromInside {}))
                .with(fixture_test(test_dialog::KeyboardOpenAndEscape {}))
                .with(fixture_test(test_dialog::NestedModals {}))
                .with(fixture_test(test_dialog::AnimatedModal {}))
                .with(fixture_test(test_dialog::AutoFocus {}))
                .with(fixture_test(test_dialog::RegularDialogNotDescribed {}))
                .with(fixture_test(test_dialog::AlertDialogDescribedbyOverride {}))
                .with(fixture_test(test_dialog::UntitledDialogWarns {}))
                .with(fixture_test(test_dialog::NamedDialogsDontWarn {}))
                .with(fixture_test(test_dialog::KeepsFocusInsideAShadowRoot {})),
        )
        .with_test_group(
            TestGroup::new("toolbar")
                .with(fixture_test(test_toolbar::Structure {}))
                .with(fixture_test(test_toolbar::KeyboardNavigation {}))
                .with(fixture_test(test_toolbar::TabLeavesAndReenters {}))
                .with(fixture_test(test_toolbar::NoWrapping {}))
                .with(fixture_test(test_toolbar::Vertical {}))
                .with(fixture_test(test_toolbar::RightToLeft {}))
                .with(fixture_test(test_toolbar::RightToLeftVertical {}))
                .with(fixture_test(test_toolbar::AriaExampleChildren {}))
                .with(fixture_test(test_toolbar::DefaultClassAndOrientation {}))
                .with(fixture_test(test_toolbar::Dividers {}))
                .with(fixture_test(test_toolbar::AriaLabelWinsOverLabelledby {})),
        )
        .with_test_group(
            TestGroup::new("progress_bar")
                .with(fixture_test(test_progress_bar::Renders {}))
                .with(fixture_test(test_progress_bar::FollowsItsValue {}))
                .with(fixture_test(test_progress_bar::CustomRange {}))
                .with(fixture_test(test_progress_bar::EmptyRange {}))
                .with(fixture_test(test_progress_bar::Indeterminate {}))
                .with(fixture_test(test_progress_bar::CustomTextValue {}))
                .with(fixture_test(
                    test_progress_bar::LabelFollowsTheRenderedLabel {},
                ))
                .with(fixture_test(test_progress_bar::Meter {}))
                .with(fixture_test(test_progress_bar::ZeroRange {}))
                .with(fixture_test(test_progress_bar::MeterCustomRange {}))
                .with(fixture_test(test_progress_bar::MeterEmptyRange {}))
                .with(fixture_test(test_progress_bar::DefaultClassesAndPercent {})),
        )
        .with_test_group(
            TestGroup::new("pressable")
                .with(fixture_test(test_pressable::MergesWithTheChildsHandlers {}))
                .with(fixture_test(test_pressable::MakesTheChildFocusable {}))
                .with(fixture_test(test_pressable::Disabled {}))
                .with(fixture_test(test_pressable::PressResponder {}))
                .with(fixture_test(
                    test_pressable::PressResponderWarnsWithoutPressable {},
                ))
                .with(fixture_test(
                    test_pressable::WarnsAboutChildrenWithoutInteractiveRoles {},
                )),
        )
        .with_test_group(
            TestGroup::new("spin_button")
                .with(fixture_test(test_spin_button::AriaProps {}))
                .with(fixture_test(test_spin_button::DisabledAndReadOnly {}))
                .with(fixture_test(test_spin_button::KeysCallTheirCallbacks {}))
                .with(fixture_test(
                    test_spin_button::ReadOnlyAndDisabledIgnoreKeys {},
                ))
                .with(fixture_test(test_spin_button::AnnouncesValueChanges {})),
        )
        .with_test_group(
            TestGroup::new("submenu")
                .with(fixture_test(test_submenu::SupportsASubmenuTrigger {}))
                .with(fixture_test(test_submenu::SupportsNestedSubmenuTriggers {}))
                .with(fixture_test(test_submenu::Keyboard {}))
                .with(fixture_test(
                    test_submenu::FocusingAnotherItemClosesTheSubmenu {},
                ))
                .with(fixture_test(test_submenu::InteractingOutsideClosesAll {}))
                .with(fixture_test(test_submenu::ContextMenu {}))
                .with(fixture_test(test_submenu::Subdialog {}))
                .with(fixture_test(test_submenu::SubdialogWithDialog {}))
                .with(fixture_test(test_submenu::RightToLeft {}))
                .with(fixture_test(test_submenu::SafeTriangle {}))
                .with(fixture_test(
                    test_submenu::HoveringBackOntoTheTriggerKeepsTheSubmenu {},
                ))
                .with(fixture_test(
                    test_submenu::ClickingInsideTheSubmenuTreeKeepsItOpen {},
                ))
                .with(fixture_test(test_submenu::SubmenuSections {}))
                .with(fixture_test(test_submenu::NestedSubdialogs {}))
                .with(fixture_test(
                    test_submenu::InteractingOutsideClosesAllSubdialogs {},
                )),
        )
        .with_test_group(
            TestGroup::new("link")
                .with(fixture_test(test_link::CurrentPage {}))
                .with(fixture_test(test_link::NewTab {}))
                .with(fixture_test(test_link::TriggerProps {}))
                .with(fixture_test(test_link::Disabled {}))
                .with(fixture_test(test_link::StateAttributes {}))
                .with(fixture_test(test_link::DisabledHookAnchor {}))
                .with(fixture_test(test_link::AnchorLink {}))
                .with(fixture_test(test_link::Replace {}))
                .with(fixture_test(test_link::ClientSideNavigation {})),
        )
        .with_test_group(
            TestGroup::new("localized_atoms")
                .with(fixture_test(test_localized_atoms::SearchField {}))
                .with(fixture_test(test_localized_atoms::NumberField {}))
                .with(fixture_test(test_localized_atoms::Tag {}))
                .with(fixture_test(test_localized_atoms::Select {})),
        )
        .with_test_group(
            TestGroup::new("breadcrumbs")
                .with(fixture_test(test_breadcrumbs::CurrentItem {}))
                .with(fixture_test(test_breadcrumbs::DynamicCollections {}))
                .with(fixture_test(test_breadcrumbs::Disabled {}))
                .with(fixture_test(test_breadcrumbs::Hooks {}))
                .with(fixture_test(test_breadcrumbs::Press {})),
        )
        .with_test_group(
            TestGroup::new("disclosure")
                .with(fixture_test(test_disclosure::TriggerControlsItsPanel {}))
                .with(fixture_test(
                    test_disclosure::AdjacentInteractiveElements {},
                ))
                .with(fixture_test(test_disclosure::TogglesByPressAndEnter {}))
                .with(fixture_test(test_disclosure::FindInPageExpands {}))
                .with(fixture_test(test_disclosure::NestedDisclosures {}))
                .with(fixture_test(test_disclosure::OneExpandedAtATime {}))
                .with(fixture_test(test_disclosure::MultipleExpanded {}))
                .with(fixture_test(test_disclosure::PanelAsLandmark {}))
                .with(fixture_test(test_disclosure::RepeatedKeydownTogglesOnce {}))
                .with(fixture_test(test_disclosure::DisabledGroup {}))
                .with(fixture_test(test_disclosure::FocusRing {}))
                .with(fixture_test(test_disclosure::Controlled {}))
                .with(fixture_test(test_disclosure::DisabledExpanded {}))
                .with(fixture_test(test_disclosure::FindInPageControlledClosed {}))
                .with(fixture_test(test_disclosure::GroupOnExpandedChange {}))
                .with(fixture_test(test_disclosure::GroupControlled {}))
                .with(fixture_test(test_disclosure::NestedGroups {}))
                .with(fixture_test(test_disclosure::RemountedPanel {})),
        )
        .with_test_group(
            TestGroup::new("popover")
                .with(fixture_test(test_popover::TriggerControlsTheDialog {}))
                .with(fixture_test(test_popover::OutsideClickCloses {}))
                .with(fixture_test(
                    test_popover::ClosesOnDocumentAndWindowScroll {},
                ))
                .with(fixture_test(test_popover::ModalStaysOpenOnScroll {}))
                .with(fixture_test(
                    test_popover::NonModalContainsFocusWithADialog {},
                ))
                .with(fixture_test(test_popover::TriggerNamesAnUntitledDialog {}))
                .with(fixture_test(test_popover::StandalonePopoverIsTheDialog {}))
                .with(fixture_test(test_popover::Animated {}))
                .with(fixture_test(test_popover::Scrolling {}))
                .with(fixture_test(test_popover::ContainmentPerOpening {}))
                .with(fixture_test(test_popover::Direction {})),
        )
        .with_test_group(
            TestGroup::new("dismiss_button")
                .with(fixture_test(test_dismiss_button::DefaultLabel {}))
                .with(fixture_test(test_dismiss_button::AriaLabel {}))
                .with(fixture_test(test_dismiss_button::AriaLabelledby {}))
                .with(fixture_test(test_dismiss_button::AriaLabelledbyAndLabel {}))
                .with(fixture_test(test_dismiss_button::ActivatingDismisses {}))
                .with(fixture_test(
                    test_dismiss_button::DoesntSubmitAnEnclosingForm {},
                )),
        )
        .with_test_group(
            TestGroup::new("overlay")
                .with(fixture_test(test_overlay::Dismissable {}))
                .with(fixture_test(test_overlay::NotDismissable {}))
                .with(fixture_test(test_overlay::KeyboardDismissDisabled {}))
                .with(fixture_test(test_overlay::TopMostOnly {}))
                .with(fixture_test(test_overlay::NestedModals {})),
        )
        .with_test_group(
            TestGroup::new("overlay_state")
                .with(fixture_test(
                    test_overlay_state::PopoverStateOverridesTheTrigger {},
                ))
                .with(fixture_test(
                    test_overlay_state::ModalStateOverridesTheTrigger {},
                ))
                .with(fixture_test(test_overlay_state::StandalonePopover {}))
                .with(fixture_test(
                    test_overlay_state::PopoverInAModalClosesAlone {},
                ))
                .with(fixture_test(test_overlay_state::ModalFilterCloses {}))
                .with(fixture_test(test_overlay_state::ModalFilterKeeps {}))
                .with(fixture_test(test_overlay_state::EnterAndExitCallbacks {}))
                .with(fixture_test(
                    test_overlay_state::AutoFocusInAModalOpenedFromAMenu {},
                )),
        )
        .with_test_group(
            TestGroup::new("overlay_position")
                .with(fixture_test(test_overlay_position::PlacedAbove {}))
                .with(fixture_test(test_overlay_position::HiddenUntilPlaced {}))
                .with(fixture_test(
                    test_overlay_position::RepositionsOnPropsChange {},
                ))
                .with(fixture_test(
                    test_overlay_position::RepositionsOnWindowResize {},
                ))
                .with(fixture_test(test_overlay_position::MaxHeightLimits {}))
                .with(fixture_test(test_overlay_position::TargetWithMargin {}))
                .with(fixture_test(test_overlay_position::CrossOffsetShifts {}))
                .with(fixture_test(test_overlay_position::StartAndEndFollowRtl {}))
                .with(fixture_test(
                    test_overlay_position::TargetRectReplacesTheTrigger {},
                ))
                .with(fixture_test(test_overlay_position::ArrowBoundaryOffset {}))
                .with(fixture_test(
                    test_overlay_position::StaysWithinTheBoundary {},
                ))
                .with(fixture_test(test_overlay_position::ReopenedWithArrow {}))
                .with(fixture_test(test_overlay_position::FlipsBelow {}))
                .with(fixture_test(test_overlay_position::ReopenedUnplaced {})),
        )
        .with_test_group(
            TestGroup::new("global_shortcuts")
                .with(fixture_test(
                    test_global_shortcuts::SlashOutsideTextFields {},
                ))
                .with(fixture_test(test_global_shortcuts::SlashInTextField {}))
                .with(fixture_test(test_global_shortcuts::ModKAnywhere {}))
                .with(fixture_test(test_global_shortcuts::ShiftKey {}))
                .with(fixture_test(test_global_shortcuts::LaterBindingWins {}))
                .with(fixture_test(test_global_shortcuts::ShortcutKeys {})),
        )
        .with_test_group(
            TestGroup::new("landmark")
                .with(fixture_test(test_landmark::NavigationOrder {}))
                .with(fixture_test(test_landmark::SkipsInertLandmarks {}))
                .with(fixture_test(test_landmark::WrapEventBackward {}))
                .with(fixture_test(test_landmark::ShiftF6FromOutside {}))
                .with(fixture_test(test_landmark::RestoresLastFocused {}))
                .with(fixture_test(test_landmark::AltF6ToMain {}))
                .with(fixture_test(test_landmark::AddedAndRemoved {}))
                .with(fixture_test(test_landmark::WrapEvent {}))
                .with(fixture_test(test_landmark::LabelUpdates {}))
                .with(fixture_test(test_landmark::NestedOrder {}))
                .with(fixture_test(test_landmark::Controller {}))
                .with(fixture_test(test_landmark::DuplicateRoleWarnings {})),
        )
        .with_test_group(
            TestGroup::new("toast")
                .with(fixture_test(test_toast::TriggerAndClose {}))
                .with(fixture_test(test_toast::Timeouts {}))
                .with(fixture_test(test_toast::KeyboardFocus {}))
                .with(fixture_test(test_toast::ProgrammaticClose {}))
                .with(fixture_test(test_toast::RemainingTimeAfterPause {}))
                .with(fixture_test(test_toast::OneAtATime {}))
                .with(fixture_test(test_toast::FocusedToastAfterNewToast {})),
        )
        .with_test_group(
            TestGroup::new("tooltip")
                .with(fixture_test(test_tooltip::ShowsOnHover {}))
                .with(fixture_test(
                    test_tooltip::WarmTooltipReplacesWithoutAnimation {},
                ))
                .with(fixture_test(test_tooltip::ShowsOnFocus {}))
                .with(fixture_test(
                    test_tooltip::CloseOnPressDisabledAndCloseDelay {},
                ))
                .with(fixture_test(test_tooltip::FocusTriggerMode {}))
                .with(fixture_test(test_tooltip::HideOnScroll {}))
                .with(fixture_test(test_tooltip::OpensAtOnceWithoutDelay {}))
                .with(fixture_test(test_tooltip::StaysOpenWhileHovered {}))
                .with(fixture_test(test_tooltip::ClosesWhenThePointerLeavesIt {}))
                .with(fixture_test(test_tooltip::StaysOpenBackOnTheTrigger {})),
        )
        .with_test_group(
            TestGroup::new("tag_group")
                .with(fixture_test(
                    test_tag_group_atoms::DefaultClassesAndSlots {},
                ))
                .with(fixture_test(
                    test_tag_group_atoms::LabelContextEndsWithTheGroup {},
                ))
                .with(fixture_test(test_tag_group_atoms::FocusRing {}))
                .with(fixture_test(
                    test_tag_group_atoms::TabbingToRemoveButtons {},
                ))
                .with(fixture_test(test_tag_group_atoms::SelectionState {}))
                .with(fixture_test(test_tag_group_atoms::EmptyState {}))
                .with(fixture_test(
                    test_tag_group_atoms::FocusMovesToTheGridWhenNoTagCanTakeIt {},
                ))
                .with(fixture_test(test_tag_group_atoms::Hover {}))
                .with(fixture_test(test_tag_group_atoms::NotInteractive {}))
                .with(fixture_test(test_tag_group_atoms::PressState {}))
                .with(fixture_test(
                    test_tag_group_atoms::DisabledTagsCantBeRemoved {},
                ))
                .with(fixture_test(test_tag_group_atoms::OnAction {}))
                .with(fixture_test(
                    test_tag_group_atoms::OnActionWithReplaceSelection {},
                ))
                .with(fixture_test(test_tag_group_atoms::OrderWhenAdding {}))
                .with(fixture_test(test_tag_group_atoms::RightToLeft {}))
                .with(fixture_test(test_tag_group_atoms::KeyboardSelection {}))
                .with(fixture_test(test_tag_group::AriaStructure {}))
                .with(fixture_test(test_tag_group::KeyboardNavigation {}))
                .with(fixture_test(
                    test_tag_group::RemovingWithTheKeyboardMovesFocusOn {},
                ))
                .with(fixture_test(test_tag_group::RemoveButton {}))
                .with(fixture_test(
                    test_tag_group::RemovingEveryTagFocusesTheGroup {},
                ))
                .with(fixture_test(test_tag_group::HoldingTheRemoveKey {})),
        )
        .with_test_group(
            TestGroup::new("virtual_list")
                .with(fixture_test(test_virtual_list::FollowsItsEnd {}))
                .with(fixture_test(
                    test_virtual_list::AppendedLinesComeIntoView {},
                ))
                .with(fixture_test(test_virtual_list::PageScrollKeepsFollowing {}))
                .with(fixture_test(
                    test_virtual_list::ScrollJumpsRenderRowsInOrder {},
                ))
                .with(fixture_test(
                    test_virtual_list::ScrollingAwayStopsFollowing {},
                ))
                .with(fixture_test(test_virtual_list::SelectedRowStaysRendered {}))
                .with(fixture_test(
                    test_virtual_list::TurningFollowingOnScrollsToTheEnd {},
                ))
                .with(fixture_test(
                    test_virtual_list::RebuiltViewsDropTheOldHandlers {},
                ))
                .with(fixture_test(
                    test_virtual_list::FollowToggleKeepsMeasuredSizes {},
                ))
                .with(fixture_test(
                    test_virtual_list::TextRowsAreMeasuredAgainWhenTheyResize {},
                )),
        )
        .with_test_group(
            TestGroup::new("virtualizer")
                .with(fixture_test(test_virtualizer::RendersTheVisibleOptions {}))
                .with(fixture_test(
                    test_virtualizer::ScrollingRendersOtherOptions {},
                ))
                .with(fixture_test(
                    test_virtualizer::FocusedOptionScrollsIntoView {},
                ))
                .with(fixture_test(test_virtualizer::LogStaysAtItsEnd {}))
                .with(fixture_test(
                    test_virtualizer::PlainListBoxIsNotVirtualized {},
                ))
                .with(fixture_test(
                    test_virtualizer::FocusedOptionStaysRendered {},
                ))
                .with(fixture_test(
                    test_virtualizer::TypeAheadReachesAnUnrenderedOption {},
                ))
                .with(fixture_test(
                    test_virtualizer::PressingAScrolledToOptionSelectsIt {},
                ))
                .with(fixture_test(
                    test_virtualizer::RendersOptionsAfterBeingHidden {},
                )),
        )
        .with_test_group(
            TestGroup::new("tree")
                .with(fixture_test(test_tree::AriaStructure {}))
                .with(fixture_test(test_tree::KeyboardExpansion {}))
                .with(fixture_test(
                    test_tree::ArrowRightOnAnExpandedRowKeepsTheFocus {},
                ))
                .with(fixture_test(test_tree::ExpandButton {}))
                .with(fixture_test(test_tree::PressingAParentTogglesIt {}))
                .with(fixture_test(
                    test_tree::DisabledItemsCanBeExpandedButNotSelected {},
                ))
                .with(fixture_test(test_tree::DisabledItemsCannotBeUsed {}))
                .with(fixture_test(test_tree::RightToLeftExpansionKeys {}))
                .with(fixture_test(
                    test_tree::CollapsingTheParentOfTheFocusedRow {},
                ))
                .with(fixture_test(test_tree::AnItemGettingChildren {}))
                .with(fixture_test(test_tree::TypeAheadSearchesTheVisibleRows {}))
                .with(fixture_test(
                    test_tree::HomeAndEndMoveBetweenTheVisibleRows {},
                ))
                .with(fixture_test(
                    test_tree::SelectableRowsAreNotExpandedByPressingThem {},
                ))
                .with(fixture_test(
                    test_tree::RowsWithAnActionAreNotExpandedByPressingThem {},
                ))
                .with(fixture_test(test_tree::TabIntoAnEmptyTree {}))
                .with(fixture_test(test_tree::EscapeKeepsTheSelection {}))
                .with(fixture_test(test_tree::SelectsOnPressUp {}))
                .with(fixture_test(test_tree::KeysInATextInputStayThere {})),
        )
        .with_test_group(
            TestGroup::new("visually_hidden")
                .with(fixture_test(test_visually_hidden::HidesElement {}))
                .with(fixture_test(
                    test_visually_hidden::UnhidesFocusedFocusable {},
                ))
                .with(fixture_test(test_visually_hidden::ReactiveIsFocusable {})),
        )
        .with_test_group(
            TestGroup::new("separator")
                .with(fixture_test(test_separator::DefaultClass {}))
                .with(fixture_test(test_separator::AccessibilityProps {}))
                .with(fixture_test(test_separator::Orientation {})),
        )
        .with_test_group(
            TestGroup::new("theme")
                .with(fixture_test(test_theme::Context {}))
                .with(fixture_test(test_theme::Switching {}))
                .with(fixture_test(test_theme::SetterWithoutTheme {}))
                .with(fixture_test(test_theme::ControlledWithoutSetter {}))
                .with(fixture_test(
                    test_theme::RootRemovesDocumentThemeOnUnmount {},
                ))
                .with(fixture_test(
                    test_theme::RootRestoresPreviousDocumentTheme {},
                ))
                .with(fixture_test(
                    test_theme::NestedUnmountPreservesDocumentTheme {},
                ))
                .with(fixture_test(
                    test_theme::RootUnmountPreservesExternalDocumentTheme {},
                )),
        );
    let mut hydration = TestGroup::new("hydration");
    for shard in 0..4 {
        hydration = hydration.with(fixture_test(test_hydration_ids::HydrationIdTests {
            shard,
            shards: 4,
        }));
    }
    tests.with_test_group(hydration)
}
