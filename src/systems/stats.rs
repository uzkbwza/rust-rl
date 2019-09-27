use specs::prelude::*;
use crate::components::*;
use crate::BASE_TURN_TIME;

pub struct StatCalculator;

#[derive(SystemData)]
pub struct StatsUpdaterSystemData<'a> {
    actors: WriteStorage<'a, Actor>,
    quicknesses: WriteStorage<'a, Quickness>,
}

pub struct QuicknessSystem;
impl QuicknessSystem {
    fn quickness_from_agility(agility: i32) -> i32 {
        ((agility - (BASE_TURN_TIME / 100))) * (BASE_TURN_TIME / 20)
    }
}

impl<'a> System<'a> for QuicknessSystem {
    type SystemData = StatsUpdaterSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        for (actor, quickness) in (&data.actors, &mut data.quicknesses).join() {
            quickness.modify_quickness(Self::quickness_from_agility(actor.stats.agility));
        }
    }
}
