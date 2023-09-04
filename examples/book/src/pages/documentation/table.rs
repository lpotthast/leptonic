use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageTable(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Table"</H1>

        <P>"Tables..."</P>

        <Code>
            {indoc!(r##"
                <TableContainer>
                    <Table bordered=true hoverable=true>
                        <Thead>
                            <Tr>
                                <Th min_width=true>"#"</Th>
                                <Th>"Name"</Th>
                            </Tr>
                        </Thead>
                        <Tbody>
                            <Tr>
                                <Th>"1"</Th>
                                <Th>"Kevin"</Th>
                            </Tr>
                            // ...
                        </Tbody>
                    </Table>
                </TableContainer>
            "##)}
        </Code>

        <TableContainer>
            <Table bordered=true hoverable=true>
                <Thead>
                    <Tr>
                        <Th min_width=true>"#"</Th>
                        <Th>"Name"</Th>
                    </Tr>
                </Thead>
                <Tbody>
                    <Tr>
                        <Th>"1"</Th>
                        <Th>"Kevin"</Th>
                    </Tr>
                    <Tr>
                        <Th>"2"</Th>
                        <Th>"Bob"</Th>
                    </Tr>
                    <Tr>
                        <Th>"3"</Th>
                        <Th>"Stuart"</Th>
                    </Tr>
                    <Tr>
                        <Th>"4"</Th>
                        <Th>"Otto"</Th>
                    </Tr>
                </Tbody>
            </Table>
        </TableContainer>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --table-...
            "#)}
        </Code>
    }
}
