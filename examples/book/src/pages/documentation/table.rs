use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageTable(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Table"</H1>

        <P>"Tables..."</P>

        <Code>
            {indoc!(r#"
                <Table>
                    <TableBody />
                </Table>
            "#)}
        </Code>

        <Table>
            <TableBody />
        </Table>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --table-...
            "#)}
        </Code>
    }
}
