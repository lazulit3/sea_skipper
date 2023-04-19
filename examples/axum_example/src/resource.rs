//! Describe [`Resource`] for [`Cake`] at `/cakes/`.

use sea_skipper::{Location, Resource};

use crate::entity::{cake, prelude::Cake};

/// Implement [`sea_skipper::Resource`] for [`Cake`] entity.
impl Resource for Cake {
    type ActiveModel = cake::ActiveModel;
    type Data = cake::Model;
    type Id = i32;
}

impl Location for cake::Model {
    fn location(&self) -> String {
        format!("/cakes/{}", self.id)
    }
}
