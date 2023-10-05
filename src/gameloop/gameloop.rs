use crate::render::{window::MainWindow, pixel::PixelList};
use crate::sprite::sprite::Sprite;
use crate::audio::audio::Music;
use winit::event::{KeyboardInput, VirtualKeyCode, ModifiersState};
use std::thread;
use std::sync::mpsc;

pub struct GameLoop{
}

#[allow(deprecated)]
impl GameLoop{
    pub fn run(){
        let (pixel_sender, pixel_receiver) = mpsc::sync_channel::<PixelList>(0);
        let (keyboard_event_sender, keyboard_event_receiver) = mpsc::sync_channel::<KeyboardInput>(1);
        let window = MainWindow::new(pixel_receiver, keyboard_event_sender);

        let mut pixel_list = PixelList::new();

        let mut sprite_list: Vec<Sprite> = Vec::new();
        sprite_list.push(Sprite::new("../assets/world/base.png", 0, 0));
        sprite_list.push(Sprite::new("../assets/items/grimoire_idle.png", 0, 0));

        let empty_keycode = VirtualKeyCode::Yen;
        let empty_keycode = Some(empty_keycode);

        let song_file = "../assets/music/main_theme.wav";

        thread::spawn(move ||{
            let music = Music::new(song_file);
            music.run();
            loop{
                let keyboard_entry = keyboard_event_receiver.try_recv();
                let key_pressed = match keyboard_entry {
                    Ok(keypress) => {
                        keypress
                    },
                    Err(_e) => {
                        KeyboardInput { scancode: 0, state: winit::event::ElementState::Pressed, virtual_keycode: empty_keycode, modifiers: ModifiersState::empty()}
                    },
                };
                match key_pressed.virtual_keycode.unwrap(){
                    VirtualKeyCode::Right => {
                        let location = sprite_list[1].get_location();
                        sprite_list[1].set_location(location.0 + 1, location.1);
                    },
                    VirtualKeyCode::Left => {
                        let location = sprite_list[1].get_location();
                        sprite_list[1].set_location(location.0 - 1, location.1);
                    },
                    VirtualKeyCode::Up => {
                        let location = sprite_list[1].get_location();
                        sprite_list[1].set_location(location.0, location.1 - 1);
                    },
                    VirtualKeyCode::Down => {
                        let location = sprite_list[1].get_location();
                        sprite_list[1].set_location(location.0, location.1 + 1);
                    },
                    _ => {},
                }

                for sprite in sprite_list.clone(){
                    sprite.add_to_pixel_list(&mut pixel_list);
                }

                let _result = pixel_sender.send(pixel_list.clone());
            }
        });

        window.run();
    }
}
