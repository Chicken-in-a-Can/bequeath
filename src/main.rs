use bequeath::render::{window, pixel::PixelList};
use bequeath::sprite::sprite;
use std::thread;
use std::sync::mpsc;

fn main() {
    let (pixel_sender, pixel_receiver) = mpsc::sync_channel::<PixelList>(0);
    let window = window::MainWindow::new(pixel_receiver);

    let mut pix_vec = PixelList::new();

    let player_sprite = sprite::Sprite::new("../assets/player/idle.png");
    let background_sprite = sprite::Sprite::new("../assets/world/base.png");

    thread::spawn(move ||{
        loop{
            background_sprite.add_to_pixel_list(&mut pix_vec, 0, 0);
            player_sprite.add_to_pixel_list(&mut pix_vec, 0, 0);
            let _result = pixel_sender.send(pix_vec.clone());
        }
    });

    window.run();
}
