//! `use_list_collection` alone (the fixture of `use_collection`'s performance test,
//! [`CollectionFixture`], building the collection from a list of values), for its performance
//! test (`leptonic/tests/perf_tests/hooks/use_list_collection.rs`).

use leptos::prelude::*;

use super::use_collection::{CollectionFixture, Hook};

#[component]
pub fn PagePerf() -> impl IntoView {
    view! { <CollectionFixture title="use_list_collection performance" hook=Hook::ListCollection /> }
}
