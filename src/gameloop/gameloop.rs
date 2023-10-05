/*
* Main game loop
* Accesses pretty much all other files
*/

use crate::render::{window::MainWindow, pixel::PixelList};
use crate::sprite::sprite::Sprite;
use crate::audio::audio::Music;
use winit::event::{KeyboardInput, VirtualKeyCode, ModifiersState};
use std::thread;
use std::sync::mpsc;

// Empty struct for now. Really jsut cause everything else is ooped
pub struct GameLoop{
}

// One of the fields of a struct is required, but is outdated
// So it gives me a warning. This is annoying, so i just force allow it
#[allow(deprecated)]
// The methods used for GameLoop
impl GameLoop{
    // This runs the GameLoop
    pub fn run(){
        // Create the sender and receiver for pixels. Used to communicate with window
        let (pixel_sender, pixel_receiver) = mpsc::sync_channel::<PixelList>(0);
        // Keyboard event sender and receiver. Also used to communicate with window
        let (keyboard_event_sender, keyboard_event_receiver) = mpsc::sync_channel::<KeyboardInput>(1);
        // Window. Used to communicate with self
        let window = MainWindow::new(pixel_receiver, keyboard_event_sender);

        // Create a list of pixels
        let mut pixel_list = PixelList::new();

        // Create list of sprites
        let mut sprite_list: Vec<Sprite> = Vec::new();
        // Add sprites to list
        // Add background sprite
        sprite_list.push(Sprite::new("../assets/world/base.png", 0, 0));
        // Add idle gromoire sprite
        sprite_list.push(Sprite::new("../assets/items/grimoire_idle.png", 0, 0));

        // Create blank keycodes to assign when no key is received
        let empty_keycode = VirtualKeyCode::Yen;
        let empty_keycode = Some(empty_keycode);

        // Read a song file
        let song_file = "../assets/music/main_theme.wav";

        thread::spawn(move ||{
            // Run the music from the song file
            let music = Music::new(song_file);
            music.run();
            // Loop indefinately
            loop{
                // Get the output of the keyboard input thread form the window
                let keyboard_entry = keyboard_event_receiver.try_recv();
                // Match the output
                let key_pressed = match keyboard_entry {
                    // If the thread provides a valid keycode, assign it to the variable
                    Ok(keypress) => {
                        keypress
                    },
                    // If the thread doesn't provice a valid keycode, assign our blank keycode
                    Err(_e) => {
                        KeyboardInput { scancode: 0, state: winit::event::ElementState::Pressed, virtual_keycode: empty_keycode, modifiers: ModifiersState::empty()}
                    },
                };
                // Match the key pressed
                match key_pressed.virtual_keycode.unwrap(){
                    // On right key, move sprite right
                    VirtualKeyCode::Right => {
                        let location = sprite_list[1].get_location();
                        sprite_list[1].set_location(location.0 + 1, location.1);
                    },
                    // On left key, move sprite left
                    VirtualKeyCode::Left => {
                        let location = sprite_list[1].get_location();
                        sprite_list[1].set_location(location.0 - 1, location.1);
                    },
                    // On up key, move sprite up
                    VirtualKeyCode::Up => {
                        let location = sprite_list[1].get_location();
                        sprite_list[1].set_location(location.0, location.1 - 1);
                    },
                    // On down key, move sprite down
                    VirtualKeyCode::Down => {
                        let location = sprite_list[1].get_location();
                        sprite_list[1].set_location(location.0, location.1 + 1);
                    },
                    // Ignore anything else
                    _ => {},
                }

                // For the sprites in the list, add to our pixel list
                for sprite in sprite_list.clone(){
                    sprite.add_to_pixel_list(&mut pixel_list);
                }

                // Send the pixel list to our window
                let _result = pixel_sender.send(pixel_list.clone());
            }
        });

        // Run the window
        window.run();
    }
}
