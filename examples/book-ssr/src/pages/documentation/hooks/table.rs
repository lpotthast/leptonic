use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::table::TableDemo;

#[component]
pub fn PageUseTableHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="table" class="anchor">
                "Table Hooks"
                <AnchorLink href="#table" description="Direct link to article header"/>
            </h1>

            <p>
                "Hooks for creating accessible data tables with selection, sorting, and keyboard navigation. "
                "See the "<Link href=crate::routes::doc::Table.materialize()>"Table overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useTable.html" target=LinkTarget::_Blank>
                    "useTable"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell source=include_str!("demos/table.rs")>
                <TableDemo />
            </DemoShell>

            <h2 id="use_table" class="anchor">
                "use_table"
                <AnchorLink href="#use_table" description="Direct link to use_table"/>
            </h2>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseTableReturn { table_attrs, state } = use_table(UseTableInput {
                        label: Some("Users".into()),
                        selection_mode: TableSelectionMode::Multiple,
                        selected_keys: selected_rows.into(),
                        sort_descriptor: None,
                        on_selection_change: Some(Callback::new(move |keys| set_selected_rows.set(keys))),
                        on_sort_change: Some(Callback::new(move |sort| { /* handle sort */ })),
                    });
                "#)}
            </Code>

            <h2 id="related-hooks" class="anchor">
                "Related Hooks"
                <AnchorLink href="#related-hooks" description="Direct link to related hooks"/>
            </h2>

            <h3>"use_table_header"</h3>
            <p>"For the thead element with role=\"rowgroup\"."</p>

            <h3>"use_table_column_header"</h3>
            <p>"For sortable column headers with aria-sort."</p>

            <h3>"use_table_row"</h3>
            <p>"For table rows with selection and navigation."</p>

            <h3>"use_table_cell"</h3>
            <p>"For individual cells with focus management."</p>

            <h3>"use_table_checkbox_cell"</h3>
            <p>"Special cell type for row selection checkboxes."</p>

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><strong>"Arrow keys"</strong> " - Navigate between cells"</li>
                <li><strong>"Home/End"</strong> " - Jump to first/last cell in row"</li>
                <li><strong>"Ctrl+Home/End"</strong> " - Jump to first/last cell in table"</li>
                <li><strong>"Space"</strong> " - Toggle row selection"</li>
                <li><strong>"Ctrl+A"</strong> " - Select all rows"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"None/single/multiple selection modes"</li>
                <li>"Sortable columns with aria-sort"</li>
                <li>"Keyboard navigation (grid pattern)"</li>
                <li>"Row selection checkboxes"</li>
                <li>"Header rowgroup semantics"</li>
                <li>"Proper ARIA grid attributes"</li>
            </ul>
            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Table.materialize()>"Table overview"</Link></li>
                <li><Link href=crate::routes::doc::table::Component.materialize()>"Table component"</Link></li>
                <li><Link href=crate::routes::doc::Grid.materialize()>"Grid concept"</Link></li>
                <li><Link href=crate::routes::doc::hooks::Selection.materialize()>"Selection hooks"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Table Hooks", link: "#table" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "use_table", link: "#use_table" },
                Toc::Leaf { title: "Related Hooks", link: "#related-hooks" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
