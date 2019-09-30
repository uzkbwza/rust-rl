use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs::shred::FetchMut;
use shrev::EventChannel;


pub fn send(ctx: &mut Rltk, mut input_channel: FetchMut<EventChannel<VirtualKeyCode>>) {
    let mouse_pos = ctx.mouse_pos();
    let key = ctx.key;
    if let Some(key_pressed) = key {
        input_channel.single_write(key_pressed);
    }
}