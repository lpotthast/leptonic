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
                                <Th>"Appearance"</Th>
                                <Th>"Num. eyes"</Th>
                            </Tr>
                        </Thead>
                        <Tbody>
                            <Tr>
                                <Td>"1"</Td>
                                <Td>"Kevin"</Td>
                                <Td>"Tall"</Td>
                                <Td>"2"</Td>
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
                        <Th>"Appearance"</Th>
                        <Th>"Num. eyes"</Th>
                    </Tr>
                </Thead>
                <Tbody>
                    <Tr>
                        <Td>"1"</Td>
                        <Td>"Kevin"</Td>
                        <Td>"Tall"</Td>
                        <Td>"2"</Td>
                    </Tr>
                    <Tr>
                        <Td>"2"</Td>
                        <Td>"Bob"</Td>
                        <Td>"Short"</Td>
                        <Td>"2"</Td>
                    </Tr>
                    <Tr>
                        <Td>"3"</Td>
                        <Td>"Stuart"</Td>
                        <Td>"Medium"</Td>
                        <Td>"1"</Td>
                    </Tr>
                    <Tr>
                        <Td>"4"</Td>
                        <Td>"Otto"</Td>
                        <Td>"Round"</Td>
                        <Td>"2"</Td>
                    </Tr>
                </Tbody>
            </Table>
        </TableContainer>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                // Table wrapper
                --table-wrapper-box-shadow-color

                // Table
                --table-color
                --table-background-color
                --table-background-color-on-hover
                --table-background-color-of-striped-rows
                --table-header-background-color
                --table-border-color
                --table-cell-box-shadow-on-hover
                --table-column-background-if-ordered
                --table-header-cell-padding
                --table-body-cell-padding
            "#)}
        </Code>
    }
}
