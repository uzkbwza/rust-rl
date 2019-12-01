use crate::components::*;
use specs::prelude::*;

pub struct Naming;

#[derive(SystemData)]
pub struct NamingSystemData<'a> {
    entities: Entities<'a>,
    names: WriteStorage<'a, Name>,
}
impl<'a> System<'a> for Naming {
    type SystemData = NamingSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        for entity in (&data.entities).join() {
            if data.names.get(entity) == None {
                let entity_name = &String::from(format!("Entity{}", entity.id()));
                let entity_name = Name::new(entity_name);
                data.names.insert(entity, entity_name);
            }
        }
    }
}
