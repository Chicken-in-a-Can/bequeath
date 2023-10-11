/*
*   Defines all of the Sprite work and classes
*   For PNG images, though probably works with some other types
*/


use image::{io::Reader, Rgba};

use crate::render::pixel::{PixelList, Pixel};

// Define the sprite struct
#[derive(Clone, Debug)]
pub struct Sprite{
    pix_matrix: Vec<Vec<[u8; 4]>>,
    start_x: u16,
    start_y: u16,
}

impl Sprite{
    // Create a new sprite
    pub fn new(file_name: &str, start_x: u16, start_y: u16) -> Self{
        // Create a new empty matrix
        let mut pix_matrix: Vec<Vec<[u8; 4]>> = Vec::new();

        // Read in a new image file
        let sprite_image = match Reader::open(file_name){
            Ok(image) => image.decode().unwrap(),
            // Panic if it can't be read
            Err(_e) => panic!("Could not read image file \"{}\"", file_name),
        };

        // Turn from DynamicImage to ImageBuffer
        let rgba_image = sprite_image.into_rgba8();
        // Get dimensions of the ImageBuffer
        let (width, height) = rgba_image.dimensions();
        // Iterate through both the width and the height of the image
        for x_pos in 0..width{
            // Push in a new vector as we about to go vertically though the image
            pix_matrix.push(Vec::new());
            for y_pos in 0..height{
                // Create a specific rgba pixel for the pixel
                let rgba_pixel: Rgba<u8> = *rgba_image.get_pixel(x_pos, y_pos);
                // Put that in our array
                let rgba_pixel_arr: [u8; 4] = rgba_pixel.0;
                // Put that array in our matrix
                pix_matrix[x_pos as usize].push(rgba_pixel_arr);
            }
        }
        // Return our filled matrix
        return Self{
            pix_matrix,
            start_x,
            start_y,
        };
    }

    // Add a sprite to our pixel list
    // Pixel list is borrowed as mutable
    pub fn add_to_pixel_list(&self, pixel_list: &mut PixelList){
        // Parse through both the x and y dimensions
        for x_pos in 0..self.pix_matrix.len(){
            for y_pos in 0..self.pix_matrix[x_pos].len(){
                // Only add to the pixel list if it's opaque
                // So just ignore clear pixels
                if self.pix_matrix[x_pos][y_pos][3] == 255{
                    // Get the r, g, and b values out of the
                    // sprite object's pixel matrix
                    let (r, g, b) = (self.pix_matrix[x_pos][y_pos][0], self.pix_matrix[x_pos][y_pos][1], self.pix_matrix[x_pos][y_pos][2]);
                    // Create a new pixel with our locations and the rgb info
                    let rgb_pix = Pixel::new(
                        r,
                        g,
                        b,
                        x_pos as u16 + self.start_x,
                        y_pos as u16 + self.start_y,
                    );
                    // Add it to the borrowed list
                    pixel_list.add_pixel(rgb_pix);
                }
            }
        }
    }
    // Returns the location of the sprite
    pub fn get_location(&self) -> (u16, u16){
        return (self.start_x, self.start_y);
    }
    // Sets the sprite to the desired location
    pub fn set_location(&mut self, start_x: u16, start_y: u16){
        self.start_x = start_x;
        self.start_y = start_y;
    }
}
