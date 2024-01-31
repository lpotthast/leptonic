use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageTable() -> impl IntoView {
    view! {
        <H1>"Table"</H1>

        <P>"Tables..."</P>

        <Code>
            {indoc!(r##"
                <TableContainer>
                    <Table bordered=true hoverable=true>
                        <TableHeader>
                            <TableRow>
                                <TableHeaderCell min_width=true>"#"</TableHeaderCell>
                                <TableHeaderCell>"Name"</TableHeaderCell>
                                <TableHeaderCell>"Appearance"</TableHeaderCell>
                                <TableHeaderCell>"Num. eyes"</TableHeaderCell>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <TableRow>
                                <TableCell>"1"</TableCell>
                                <TableCell>"Kevin"</TableCell>
                                <TableCell>"Tall"</TableCell>
                                <TableCell>"2"</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>"2"</TableCell>
                                <TableCell>"Bob"</TableCell>
                                <TableCell>"Short"</TableCell>
                                <TableCell>"2"</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>"3"</TableCell>
                                <TableCell>"Stuart"</TableCell>
                                <TableCell>"Medium"</TableCell>
                                <TableCell>"1"</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>"4"</TableCell>
                                <TableCell>"Otto"</TableCell>
                                <TableCell>"Round"</TableCell>
                                <TableCell>"2"</TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </TableContainer>
            "##)}
        </Code>

        <TableContainer>
            <Table bordered=true hoverable=true>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell min_width=true>"#"</TableHeaderCell>
                        <TableHeaderCell>"Name"</TableHeaderCell>
                        <TableHeaderCell>"Appearance"</TableHeaderCell>
                        <TableHeaderCell>"Num. eyes"</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableCell>"1"</TableCell>
                        <TableCell>"Kevin"</TableCell>
                        <TableCell>"Tall"</TableCell>
                        <TableCell>"2"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"2"</TableCell>
                        <TableCell>"Bob"</TableCell>
                        <TableCell>"Short"</TableCell>
                        <TableCell>"2"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"3"</TableCell>
                        <TableCell>"Stuart"</TableCell>
                        <TableCell>"Medium"</TableCell>
                        <TableCell>"1"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"4"</TableCell>
                        <TableCell>"Otto"</TableCell>
                        <TableCell>"Round"</TableCell>
                        <TableCell>"2"</TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </TableContainer>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r"
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
            ")}
        </Code>
    }
}
