use std::fmt::{Display, Formatter};
use crate::{state, user};
use uuid::Uuid;

pub enum Action {
    Start,
    CreateOrder(Uuid),
    RemoveOrder(Uuid),
}

pub struct Command {
    pub user: Uuid,
    pub action: Action,
}

impl From<Command> for state::Action {
    fn from(Command { user, action }: Command) -> Self {
        match action {
            Action::Start => Self::CreateUser { user_id: user },
            Action::CreateOrder(id) => Self::User(user::Action::CreateOrder(id), user),
            Action::RemoveOrder(id) => Self::User(user::Action::RemoveOrder(id), user),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.action {
            Action::Start => "/start",
            Action::CreateOrder(_) => "/create_order",
            Action::RemoveOrder(_) => "/remove_order"
        })
    }
}
