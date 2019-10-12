use toml::{Value, de::Error};
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {

    pub screen_width: i32,
    pub screen_height: i32,

    pub viewport_width: i32,
    pub viewport_height: i32,

    pub viewport_x: i32,
    pub viewport_y: i32,

    pub map_width: i32,
    pub map_height: i32,

    pub base_turn_time: u32,
    pub min_turn_time: u32,

    pub log_turn_start: bool,

    pub debug_vision: bool,
}

impl Config {
    pub fn open() -> Self {
        let mut file = File::open("CONFIG.toml").expect("Config file not found!");
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        println!("{}", contents);

        let config: Config = toml::from_str(&contents).expect("Problem reading config");
        config
    }
}