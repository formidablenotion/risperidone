use crate::effect::Effect;
use crate::entity::Entity;
use crate::user::{self, User};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct State {
    pub users: HashMap<Uuid, User>,
}

pub enum Action {
    CreateUser { user_id: Uuid },
    User(user::Action, Uuid),
}

impl Entity for State {
    type Action = Action;

    fn reduce(&mut self, action: Self::Action) -> Effect {
        match action {
            Action::CreateUser { user_id } => {
                self.users.insert(
                    user_id,
                    User {
                        id: user_id,
                        orders: Default::default(),
                    },
                );
                Effect::Message("Теперь у вас есть аккаунт...".into())
            }
            Action::User(action, user_id) => self
                .users
                .get_mut(&user_id)
                .map(|u| u.reduce(action))
                .unwrap_or(Effect::None),
        }
    }
}
