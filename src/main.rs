use bequeath::render::{window, pixel::{PixelList, Pixel}};
use bequeath::constants;
use std::thread;
use std::sync::mpsc;
use std::time;

fn main() {
    let (pixel_sender, pixel_receiver) = mpsc::sync_channel::<PixelList>(0);
    let window = window::MainWindow::new(pixel_receiver);

    let mut pix_vec = PixelList::new();
    pix_vec.allow_rerender();
    pix_vec.add_pixel(Pixel::new(255, 0, 0, 0, 0));

    thread::spawn(move ||{
        thread::sleep(time::Duration::from_millis(1000));
        let _result = pixel_sender.send(pix_vec);
    });

    window.run();
}
