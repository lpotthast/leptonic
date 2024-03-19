pub(crate) fn disable_text_selection(element: &web_sys::Element) {
    match element.set_attribute("data-disable-user-select", "true") {
        Ok(_ok) => {}
        Err(err) => {
            tracing::warn!(?err, "Could not set 'data-disable-user-select' attribute.");
        }
    }
}

pub(crate) fn restore_text_selection(element: &web_sys::Element) {
    match element.remove_attribute("data-disable-user-select") {
        Ok(_ok) => {}
        Err(err) => {
            tracing::warn!(
                ?err,
                "Could not remove 'data-disable-user-select' attribute."
            );
        }
    }
}
