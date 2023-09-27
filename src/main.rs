use bequeath::render::{window, pixel::{PixelList, Pixel}};
use std::thread;
use std::sync::mpsc;

fn main() {
    let (pixel_sender, pixel_receiver) = mpsc::sync_channel::<PixelList>(0);
    let window = window::MainWindow::new(pixel_receiver);

    let mut pix_vec = PixelList::new();

    thread::spawn(move ||{
        loop{
            for j in 0..99{
                let mut count = 0;
                for i in 0..159{
                    count += 1;
                    if j % 2 == 0{
                        pix_vec.add_pixel(Pixel::new(255, count, 255, i, j));
                    } else{
                        pix_vec.add_pixel(Pixel::new(255, 255, count, i, j));
                    }
                }
                let _result = pixel_sender.send(pix_vec.clone());
            }
            pix_vec = PixelList::new();
        }
    });

    window.run();
}
