use crate::effect::Effect;

pub trait Entity {
    type Action;

    fn reduce(&mut self, action: Self::Action) -> Effect;
}
