use specs::prelude::*;
use crate::components::*;
use serde::Deserialize;
use std::fs::{File, read_dir};
use std::fs;
use std::fs::FileType;
use std::io::prelude::*;
use std::io::Read;
use specs::Builder;
use ron::de::from_reader;
use std::collections::HashMap;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};

pub type EntityLoadQueue = Vec<(String, Option<Position>)>;

#[derive(Debug, Clone)]
struct BlueprintStorage {
    pub blueprint: Option<EntityBlueprint>,
    pub path: PathBuf,
}


pub struct EntityFactory {
    blueprints: HashMap<String, BlueprintStorage>,
}


impl EntityFactory {
    pub fn new(path: &str) -> Self {
        let mut factory = EntityFactory {
            blueprints: HashMap::new()
        };
        let path = PathBuf::from(path);
        factory.build_map(&path);
        factory
    }

    pub fn build(&mut self, name: String, world: &mut World, pos: Option<Position>) -> Option<Entity> {
        if !self.blueprints.contains_key(&name) {
            //            println!("Could not build blueprint: {}", &name);
            return None
        }

        let mut blueprint_entry = self.blueprints.get_mut(&name);
        if let Some(mut blueprint_storage) = blueprint_entry {
            if let Some(blueprint) = &blueprint_storage.blueprint {
                let mut blueprint = blueprint.clone();
                blueprint.position = pos;
//                println!{"BUILDING: {:?}", &name}
                let entity = blueprint.build(world);
                return Some(entity)
            }
        }
        None
    }

    fn build_map(&mut self, path: &PathBuf) {
        let file_paths = get_blueprint_paths(path);
        let mut names = Vec::new();

        // make empty map of blueprints
        for ref entry in file_paths {
            let name = format_path_name(entry).split_off((path.to_string_lossy().len() + 1)); // remove topmost parent name from formatted name (e.g. "blueprints.")
            names.push(name.clone());
            let storage = BlueprintStorage {
                blueprint: None,
                path: entry.clone(),
            };

            self.blueprints.insert(name, storage);
        }

        for name in names {
            let mut storage = self.blueprints.get_mut(&name);
            if let Some(storage) = storage {
                let mut storage = storage.clone();
                let blueprint = self.load(name.clone());
                storage.blueprint = Some(blueprint.clone());
                self.blueprints.insert(name, storage);
            }
        }
    }
}


fn get_blueprint_paths(path_buf: &PathBuf) -> Vec<PathBuf> {
//    println!("{:?}", path);
    let mut paths = Vec::new();
    for entry in fs::read_dir(path_buf)
        .expect(&format!("Problem reading path: {:?}", path_buf))
        {
            if let Ok(entry) = entry {
                let ref path = entry.path();
                let metadata = entry
                    .metadata()
                    .expect(&format!("Problem reading file metadata: {:?}", path));

                if metadata.is_dir() {
                    paths.extend(get_blueprint_paths(path))
                }

                else if metadata.is_file() {
//                    println!("{}", &formatted_path_name);
                    paths.push(path.clone());
                }
            }
        }
    paths
}

fn format_path_name(path: &PathBuf) -> String {
    let mut path_name = String::new();
    for ancestor in path.ancestors() {
        match ancestor.file_stem() {
            Some(ancestor_name) => {
                if path_name.is_empty() {
                    path_name = String::from(ancestor_name.to_string_lossy());
                } else if ancestor.parent() != None {
                    path_name = format!("{}.{}", ancestor_name.to_string_lossy(), path_name);
                }
            },
            None => (),
        }
    }
    println!("REGISTERING NAME: {:?}", path_name);
    path_name
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
            pub fn load(path: &PathBuf) -> Self {
                let mut file = File::open(path)
                    .expect(&format!("blueprint file not found: {:?}", path));

                let mut blueprint: Self = from_reader(file).expect(&format!("could not create blueprint: {:?}", path));

                // recursively apply parent blueprints
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

        impl EntityFactory {
            fn load(&mut self, name: String) -> EntityBlueprint {
                let ref path = self.blueprints.get(&name).unwrap().path;
                let mut file = File::open(path)
                    .expect(&format!("blueprint file not found: {:?}", path));
                let mut blueprint: EntityBlueprint = from_reader(file).expect(&format!("could not create blueprint: {:?}", path));
                println!("LOADING: {:?}", path);
                // recursively apply child blueprints on top of parent
                if let Some(name) = blueprint.extends {
                    println!("^ EXTENDS: {:?}", name);
                    let mut parent =  self.load(name);
                    $(
                        if let Some(c) = blueprint.$compname {
                        parent.$compname = Some(c);
                    };
                    )+
                    return parent
                }
                blueprint
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