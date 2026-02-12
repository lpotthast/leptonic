use super::dom_ext::{EventAccessors, EventTargetExt};

#[derive(Debug, Clone, Copy)]
pub struct RectPrecise {
    top: f64,
    right: f64,
    bottom: f64,
    left: f64,
}

impl From<web_sys::DomRect> for RectPrecise {
    fn from(value: web_sys::DomRect) -> Self {
        Self {
            top: value.top(),
            right: value.right(),
            bottom: value.bottom(),
            left: value.left(),
        }
    }
}

pub trait InteractionRect {
    /// Returns the contact-patch for the interaction relative to the viewport.
    ///
    /// Expands the concrete point of interaction, as returned by the events `client_x` and
    /// `client_y` functions by the half-size of the contact dimensions in each direction.
    fn get_interaction_client_rect(&self) -> RectPrecise;
}

impl InteractionRect for web_sys::MouseEvent {
    fn get_interaction_client_rect(&self) -> RectPrecise {
        let client_x = f64::from(self.client_x());
        let client_y = f64::from(self.client_y());

        let contact_width = 1.0;
        let contact_height = 1.0;

        let offset_x = contact_width / 2.0;
        let offset_y = contact_height / 2.0;

        RectPrecise {
            top: client_y - offset_y,
            right: client_x + offset_x,
            bottom: client_y + offset_y,
            left: client_x - offset_x,
        }
    }
}

impl InteractionRect for web_sys::PointerEvent {
    fn get_interaction_client_rect(&self) -> RectPrecise {
        let client_x = f64::from(self.client_x());
        let client_y = f64::from(self.client_y());

        let contact_width = f64::from(self.width());
        let contact_height = f64::from(self.height());

        let offset_x = contact_width / 2.0;
        let offset_y = contact_height / 2.0;

        RectPrecise {
            top: client_y - offset_y,
            right: client_x + offset_x,
            bottom: client_y + offset_y,
            left: client_x - offset_x,
        }
    }
}

impl InteractionRect for web_sys::Touch {
    fn get_interaction_client_rect(&self) -> RectPrecise {
        let client_x = f64::from(self.client_x());
        let client_y = f64::from(self.client_y());

        let contact_width = f64::from(self.radius_x());
        let contact_height = f64::from(self.radius_y());

        let offset_x = contact_width / 2.0;
        let offset_y = contact_height / 2.0;

        RectPrecise {
            top: client_y - offset_y,
            right: client_x + offset_x,
            bottom: client_y + offset_y,
            left: client_x - offset_x,
        }
    }
}

impl InteractionRect for web_sys::KeyboardEvent {
    fn get_interaction_client_rect(&self) -> RectPrecise {
        let rect = self
            .expect_current_target()
            .as_element()
            .expect("element")
            .get_bounding_client_rect();

        rect.into()
    }
}

// TODO: This should be used for Touch events!
pub fn is_over(action: &impl InteractionRect, element: &web_sys::Element) -> bool {
    overlapping(
        element.get_bounding_client_rect().into(),
        action.get_interaction_client_rect(),
    )
}

fn overlapping(a: RectPrecise, b: RectPrecise) -> bool {
    if a.left > b.right || b.left > a.right {
        return false;
    }
    // NOTE: Coordinate system starts in upper-left corner!
    if a.top > b.bottom || b.top > a.bottom {
        return false;
    }
    true
}
