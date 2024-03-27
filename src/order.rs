use crate::effect::Effect;
use crate::entity::Entity;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct Order {
    pub id: Uuid,
}

pub enum Action {}

impl Entity for Order {
    type Action = Action;

    fn reduce(&mut self, _: Self::Action) -> Effect {
        Effect::None
    }
}
