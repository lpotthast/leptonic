use leptonic::{components::prelude::*, utils::aria::AriaSort};
use leptos::prelude::*;

#[derive(Clone, PartialEq)]
struct Minion {
    id: u32,
    name: &'static str,
    appearance: &'static str,
    eyes: u32,
}

const MINIONS: [Minion; 4] = [
    Minion { id: 1, name: "Kevin", appearance: "Tall", eyes: 2 },
    Minion { id: 2, name: "Bob", appearance: "Short", eyes: 2 },
    Minion { id: 3, name: "Stuart", appearance: "Medium", eyes: 1 },
    Minion { id: 4, name: "Otto", appearance: "Round", eyes: 2 },
];

#[component]
pub fn TableDemo() -> impl IntoView {
    // Sorted by name, ascending or descending. Pressing the header (or Enter / Space on it) reverses the order.
    let ascending = RwSignal::new(true);
    let minions = Memo::new(move |_| {
        let mut minions = MINIONS.to_vec();
        minions.sort_by_key(|minion| minion.name);
        if !ascending.get() {
            minions.reverse();
        }
        minions
    });

    view! {
        <TableContainer>
            <Table bordered=true hoverable=true>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell>"#"</TableHeaderCell>
                        <TableHeaderCell
                            on_press=move |_| ascending.update(|ascending| *ascending = !*ascending)
                            aria_sort=Signal::derive(move || if ascending.get() { AriaSort::Ascending } else { AriaSort::Descending })
                        >
                            "Name "
                            <span aria-hidden="true">{move || if ascending.get() { "\u{25b2}" } else { "\u{25bc}" }}</span>
                        </TableHeaderCell>
                        <TableHeaderCell min_width=false>"Appearance"</TableHeaderCell>
                        <TableHeaderCell>"Eyes"</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <For each=move || minions.get() key=|minion| minion.id let:minion>
                        <TableRow>
                            <TableCell>{minion.id}</TableCell>
                            <TableCell>{minion.name}</TableCell>
                            <TableCell>{minion.appearance}</TableCell>
                            <TableCell>{minion.eyes}</TableCell>
                        </TableRow>
                    </For>
                </TableBody>
            </Table>
        </TableContainer>
        <p class="demo-status">
            {move || if ascending.get() { "Sorted by name, A to Z." } else { "Sorted by name, Z to A." }}
        </p>
    }
}
