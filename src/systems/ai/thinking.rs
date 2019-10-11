use specs::prelude::*;
use crate::command::Command;
use super::AiSystemData;


pub trait Thinking {
    fn get_command(entity: Entity, data: &AiSystemData) -> Vec<Command>;
}