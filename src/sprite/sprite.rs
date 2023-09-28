use image::{io::Reader, Rgba};

use crate::render::pixel::{PixelList, Pixel};

pub struct Sprite{
    pix_matrix: Vec<Vec<[u8; 4]>>,
}

impl Sprite{
    pub fn new(file_name: &str) -> Self{
        let mut pix_matrix: Vec<Vec<[u8; 4]>> = Vec::new();
        let sprite_image = match Reader::open(file_name){
            Ok(image) => image.decode().unwrap(),
            Err(_e) => panic!("Could not read image file \"{}\"", file_name),
        };
        let rgba_image = sprite_image.into_rgba8();
        let (width, height) = rgba_image.dimensions();
        for x_pos in 0..width{
            pix_matrix.push(Vec::new());
            for y_pos in 0..height{
                let rgba_pixel: Rgba<u8> = *rgba_image.get_pixel(x_pos, y_pos);
                let rgba_pixel_matrix: [u8; 4] = rgba_pixel.0;
                pix_matrix[x_pos as usize].push(rgba_pixel_matrix);
            }
        }
        return Self{
            pix_matrix,
        };
    }
    pub fn add_to_pixel_list(&self, pixel_list: &mut PixelList, start_x: u16, start_y: u16){
        for x_pos in 0..self.pix_matrix.len(){
            for y_pos in 0..self.pix_matrix[x_pos].len(){
                if self.pix_matrix[x_pos][y_pos][3] == 255{
                    let (r, g, b) = (self.pix_matrix[x_pos][y_pos][0], self.pix_matrix[x_pos][y_pos][1], self.pix_matrix[x_pos][y_pos][2]);
                    let rgb_pix = Pixel::new(
                        r,
                        g,
                        b,
                        x_pos as u16 + start_x,
                        y_pos as u16 + start_y,
                    );
                    pixel_list.add_pixel(rgb_pix);
                }
            }
        }
    }
}
