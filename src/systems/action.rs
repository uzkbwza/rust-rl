use crate::command::{Command, CommandEvent};
use crate::components::flags::requests::*;
use crate::components::*;
use crate::State;
use crate::systems::movement::Dir;
use shrev::{EventChannel, ReaderId};
use specs::prelude::*;

pub struct ActionHandler {
    command_event_reader: Option<ReaderId<CommandEvent>>,
}

impl ActionHandler {
    pub fn new() -> Self {
        ActionHandler {
            command_event_reader: None,
        }
    }
}

#[derive(SystemData)]
pub struct ActionHandlerSystemData<'a> {
    move_requests: WriteStorage<'a, MoveRequest>,
    players: WriteStorage<'a, PlayerControl>,
    attack_requests: WriteStorage<'a, AttackRequest>,
    my_turns: WriteStorage<'a, MyTurn>,
    game_state: WriteExpect<'a, crate::GameState>,

    command_event_reader: WriteExpect<'a, ReaderId<CommandEvent>>,

    // read event channels
    command_event_channel: Read<'a, EventChannel<CommandEvent>>,

}

impl<'a> System<'a> for ActionHandler {
    type SystemData = ActionHandlerSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {


        let command_events = data
            .command_event_channel
            .read(&mut data.command_event_reader);

        for command_event in command_events {
            let entity = command_event.entity;
            // println!("{:?}: {:?}", command_event.entity, &command_event.command);
            match command_event.command {
                Command::Move(dir) => {
                    let (dx, dy) = Dir::dir_to_pos(dir);
                    let move_request = MoveRequest::new(dx, dy);
                    if let Err(err) = data.move_requests.insert(entity, move_request) {
                        error!("Failed to insert move request: {}", err)
                    }
                    // println!("added move request");
                }

                Command::Attack(dir) => {
                    let attack_request = AttackRequest::new(dir);
                    if let Err(err) = data.attack_requests.insert(entity, attack_request) {
                        error!("Failed to insert attack request: {}", err)
                    }
                    // println!("added attack request");
                }
                _ => (),
            }
            data.my_turns.remove(entity);
            if let Some(_) = data.players.get(entity) {
                data.game_state.transition(State::TurnProcess);
                // println!("turned off", );
            }
            // println!("removed my turn", );
        }
    }

    fn setup(&mut self, world: &mut World) {
    }
}
