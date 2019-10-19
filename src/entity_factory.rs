use specs::prelude::*;
use crate::components::*;
use serde::Deserialize;
use std::fs::File;
use std::fs::FileType;
use std::io::prelude::*;
use std::io::Read;
use specs::Builder;
use ron::de::from_reader;
use std::collections::HashMap;
use walkdir::WalkDir;

pub type EntityLoadQueue = Vec<(String, Option<Position>)>;

// TODO: deserialize all entity blueprints on startup so i can just instantiate them later (see comments)
// probably add an "entity factory" that contains a hashmap of all filenames
// (like "creatures/base_creature") to a copyable blueprint instance
// and takes in the entity load queue

pub struct EntityFactory {
    blueprints: HashMap<String, EntityBlueprint>,
}

impl EntityFactory {
    pub fn new(blueprints_path: &str) -> Self {

        let mut map = HashMap::new();

        for entry in WalkDir::new(blueprints_path) {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_file() {
                let mut path = String::from(entry.path().to_str().unwrap());
                let start = blueprints_path.len();
                let end = path.len() - 4;
                path.truncate(end);
                path = path.split_off(start);

                println!("Adding blueprint {}", path.clone());
                map.insert(path.clone(), EntityBlueprint::load(path));
            }
        }

        EntityFactory {
            blueprints: map
        }
    }

    pub fn build(&mut self, name: String, world: &mut World, pos: Option<Position>) -> Option<Entity> {
        if !self.blueprints.contains_key(&name) {
            println!("Could not build blueprint: {}", &name);
            return None
        }

        let mut blueprint = self.blueprints.get_mut(&name).unwrap();
        blueprint.position = pos;
        let entity = blueprint.build(world);
        Some(entity)
    }
}

#[macro_export]
macro_rules! make_entity_blueprint_template {
    {
        $($compname:ident: $comptype:ty,)+
    } => {

        #[derive(Clone, Debug, Deserialize, Default)]
        pub struct EntityBlueprint {
            pub extends: Option<String>,
            $(
                #[serde(default)]
                pub $compname: Option<$comptype>
            ),+
        }

        impl EntityBlueprint {
            pub fn load(filename: String) -> Self {
                let filename = format!("blueprints/{}.ron", filename);
                let mut file = File::open(&filename)
                    .expect(&format!("blueprint file not found: {}", filename));

                let mut blueprint: Self = from_reader(file).expect(&format!("could not create blueprint: {}", filename));

                // recursively apply parent blueprints
                if let Some(path) = blueprint.extends {
                    println!("^ EXTENDS: {}", path.clone());
                    let mut base =  Self::load(path);
                    $(
                        if let Some(c) = blueprint.$compname {
                            base.$compname = Some(c);
                        };
                    )+
                    return base
                }
                blueprint
            }

            pub fn load_and_place (filename: String, x: i32, y: i32) -> Self {
                let mut blueprint = Self::load(filename);
                let position = Position::new(x, y);
                blueprint.position = Some(position);
                blueprint
            }

            pub fn build(&self, world: &mut World) -> Entity {
                let mut builder = world.create_entity();
                $(
                    if let Some(c) = self.$compname.clone() {
                        builder = builder.with(c);
                    }
                )+
                builder.build()
            }
        }
    }
}

make_entity_blueprint_template! {
    name: Name,
    actor: Actor,
    player: PlayerControl,
    camera: Camera,
    renderable: Renderable,
    random_renderable: RandomRenderable,
    corporeal: Corporeal,
    seeing: Seeing,
    ai_control: AiControl,
    mobile: Mobile,
    position: Position,
    invulnerable: Invulnerable,
    blocks_movement: BlockMovement,
    blocks_sight: BlockSight,
}