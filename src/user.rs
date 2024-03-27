use crate::effect::Effect;
use crate::entity::Entity;
use crate::order::Order;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct User {
    pub id: Uuid,
    pub orders: HashMap<Uuid, Order>,
}

pub enum Action {
    CreateOrder(Uuid),
    RemoveOrder(Uuid),
}

impl Entity for User {
    type Action = Action;

    fn reduce(&mut self, action: Self::Action) -> Effect {
        match action {
            Action::CreateOrder(id) => {
                self.orders.insert(id, Order { id });
                Effect::Message("Вы создали заказ!".to_string())
            }
            Action::RemoveOrder(id) => {
                self.orders.remove(&id);
                Effect::Message("Вы избавились от заказа...".to_string())
            }
        }
    }
}
