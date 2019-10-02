use specs::prelude::*;
use crate::components::*;
use crate::BASE_TURN_TIME;

pub struct _StatCalculator;

#[derive(SystemData)]
pub struct StatsUpdaterSystemData<'a> {
    actors: WriteStorage<'a, Actor>,
    quicknesses: WriteStorage<'a, Quickness>,
}

pub struct QuicknessSystem;
impl QuicknessSystem {
    fn quickness_from_agility(agility: u32) -> i32 {
        let agility = agility as i32;
        let base = BASE_TURN_TIME as i32;

        if agility > 9 {
            (agility - 10) * (base as i32 / 100)
        } else {
            - ((10 - agility) * (base as i32 / 100))
        }
    }
}

impl<'a> System<'a> for QuicknessSystem {
    type SystemData = StatsUpdaterSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        for (actor, quickness) in (&data.actors, &mut data.quicknesses).join() {
            let modifier = Self::quickness_from_agility(actor.stats.agility);
            // println!("{}", modifier);
            quickness.modify_quickness(modifier);
        }
    }
}
