use rltk::{Rltk, VirtualKeyCode};
use specs::shred::FetchMut;
use shrev::EventChannel;


pub fn _send(ctx: &mut Rltk, mut input_channel: FetchMut<EventChannel<VirtualKeyCode>>) {
    let key = ctx.key;
    if let Some(key_pressed) = key {
        input_channel.single_write(key_pressed);
    }
}