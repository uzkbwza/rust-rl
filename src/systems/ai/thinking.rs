use super::AiSystemData;
use crate::command::Command;
use specs::prelude::*;

pub trait Thinking {
    fn get_command(entity: Entity, data: &AiSystemData) -> Vec<Command>;
}
