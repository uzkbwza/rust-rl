use specs::prelude::*;
use crate::components::*;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use specs::Builder;
use ron::de::from_reader;

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
                    .expect("blueprint file not found!");

                from_reader(file).expect(&format!("could not create blueprint. "))
            }

            pub fn build(&mut self, world: &mut World) -> Entity {
                let mut builder = world.create_entity();
                $(
                    if let Some(c) = self.$compname.clone() {
                        builder = builder.with(c);
                    }
                )+
                builder.build()
            }

            pub fn load_with_position (filename: String, x: i32, y: i32) -> Self {
                let mut blueprint = Self::load(filename);
                let position = Position::new(x, y);
                blueprint.position = Some(position);
                blueprint
            }
        }
    }
}

make_entity_blueprint_template! {
    name: Name,
    actor: Actor,
    renderable: Renderable,
    corporeal: Corporeal,
    seeing: Seeing,
    ai_control: AiControl,
    mobile: Mobile,
    position: Position,
    invulnerable: Invulnerable,
}