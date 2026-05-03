use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TableConceptDemo() -> impl IntoView {
    view! {
        <TableContainer>
            <Table bordered=true hoverable=true>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell>"Name"</TableHeaderCell>
                        <TableHeaderCell>"Value"</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow>
                        <TableCell>"Alpha"</TableCell>
                        <TableCell>"100"</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"Beta"</TableCell>
                        <TableCell>"200"</TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </TableContainer>
    }
}
