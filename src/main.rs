mod command;
mod effect;
mod entity;
mod order;
mod state;
mod user;

use crate::command::{Action, Command};
use crate::entity::Entity;
use crate::state::State;
use uuid::Uuid;
use crate::effect::Effect;

fn main() {
    let mut state = State::default();
    let user = Uuid::new_v4();
    let order = Uuid::new_v4();
    let commands = vec![
        Command {
            user,
            action: Action::Start,
        },
        Command {
            user,
            action: Action::CreateOrder(order),
        },
        Command {
            user,
            action: Action::RemoveOrder(order),
        }
    ];
    for command in commands {
        println!("Command: {}", command);
        match state.reduce(command.into()) {
            Effect::Message(message) => {
                println!("Message: {}", message)
            }
            Effect::None => {}
        }
        println!();
    }
}
