use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::selection::SelectionDemo;

#[component]
pub fn PageUseSelection() -> impl IntoView {
    view! {
        <Article>
            <h1 id="selection" class="anchor">
                "Selection System"
                <AnchorLink href="#selection" description="Direct link to article header"/>
            </h1>

            <p>
                "The selection system provides a collection of hooks for managing item selection in lists, grids, and other collections. "
                "See the "<Link href=crate::routes::doc::SelectionDomain.materialize()>"Selection overview"</Link>" for domain guidance."
            </p>

            <h2 id="use_selection_state" class="anchor">
                "use_selection_state"
                <AnchorLink href="#use_selection_state" description="Direct link to use_selection_state"/>
            </h2>

            <p>"Core hook for managing selection state with support for single, multiple, and no selection modes."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    let UseSelectionStateReturn {
                        selected_keys,
                        is_selected,
                        select,
                        toggle,
                        clear_selection,
                        select_all,
                        ..
                    } = use_selection_state(UseSelectionStateInput {
                        selection_mode: SelectionMode::Multiple,
                        selection_behavior: SelectionBehavior::Toggle,
                        disabled: Signal::derive(|| false),
                        ..Default::default()
                    });
                ")}
            </Code>

            <DemoShell source=include_str!("demos/selection.rs")>
                <SelectionDemo />
            </DemoShell>

            <h2 id="related-hooks" class="anchor">
                "Related Hooks"
                <AnchorLink href="#related-hooks" description="Direct link to related hooks"/>
            </h2>

            <h3>"use_selectable_collection"</h3>
            <p>"Provides keyboard navigation and focus management for collections with selection. Handles arrow keys, Home/End, and type-ahead selection."</p>

            <h3>"use_selectable_list"</h3>
            <p>"Specialization for lists with vertical keyboard navigation. Handles ArrowUp/ArrowDown for single-axis navigation."</p>

            <h3>"use_selectable_item"</h3>
            <p>"Handles individual item selection. Manages focus, selection state, and click/keyboard interactions for a single item."</p>

            <h3>"use_type_select"</h3>
            <p>"Enables type-ahead selection where users can type to jump to items starting with those characters."</p>

            <h2 id="selection-modes" class="anchor">
                "Selection Modes"
                <AnchorLink href="#selection-modes" description="Direct link to selection modes"/>
            </h2>

            <ul>
                <li><strong>"None"</strong> " - No selection allowed, items are read-only"</li>
                <li><strong>"Single"</strong> " - Only one item can be selected at a time"</li>
                <li><strong>"Multiple"</strong> " - Multiple items can be selected simultaneously"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Three selection modes: none, single, multiple"</li>
                <li>"Disabled keys support"</li>
                <li>"Selection change callbacks"</li>
                <li>"Select all / clear operations"</li>
                <li>"Toggle individual items"</li>
                <li>"Type-ahead selection in collections"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::SelectionDomain.materialize()>"Selection overview"</Link></li>
                <li><Link href=crate::routes::doc::Listbox.materialize()>"Listbox"</Link>" \u{2014} uses the selection system"</li>
                <li><Link href=crate::routes::doc::Grid.materialize()>"Grid"</Link>" \u{2014} uses the selection system"</li>
                <li><Link href=crate::routes::doc::Table.materialize()>"Table"</Link>" \u{2014} uses the selection system"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Selection System", link: "#selection" },
                Toc::Leaf { title: "use_selection_state", link: "#use_selection_state" },
                Toc::Leaf { title: "Related Hooks", link: "#related-hooks" },
                Toc::Leaf { title: "Selection Modes", link: "#selection-modes" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
