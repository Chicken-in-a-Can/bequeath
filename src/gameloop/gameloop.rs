use crate::render::{window::MainWindow, pixel::PixelList};
use crate::sprite::sprite::Sprite;
use winit::event::KeyboardInput;
use std::thread;
use std::sync::mpsc;

pub struct GameLoop{
}

impl GameLoop{
    pub fn run(){
        let (pixel_sender, pixel_receiver) = mpsc::sync_channel::<PixelList>(0);
        let (keyboard_event_sender, keyboard_event_receiver) = mpsc::sync_channel::<KeyboardInput>(0);
        let window = MainWindow::new(pixel_receiver, keyboard_event_sender);

        let mut pixel_list = PixelList::new();

        let mut sprite_list: Vec<Sprite> = Vec::new();
        sprite_list.push(Sprite::new("../assets/world/base.png"));
        sprite_list.push(Sprite::new("../assets/items/grimoire_idle.png"));

        thread::spawn(move ||{
            loop{
                for sprite in sprite_list.clone(){
                    sprite.add_to_pixel_list(&mut pixel_list, 0, 0);
                }
                let _result = pixel_sender.send(pixel_list.clone());
                let _keyboard_entry = keyboard_event_receiver.try_recv();
            }
        });

        window.run();
    }
}
