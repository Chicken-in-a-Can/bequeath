use std::collections::HashSet;

/*
*   Defines all pixel-related structs
*   Works closely with the window structs
*/
use tiny_skia::Rect;

use crate::constants;


// Information necessary for defining pixels.
// Mainly for interaction between window size and pixel size
#[derive(Copy, Clone, Debug)]
pub struct PixelData{
    // Window width
    window_width: u32,
    // Window height
    window_height: u32,
    // Width and height of individual pixels
    pix_width_height: u16,
}

impl PixelData{
    // Get the info from the width and height of a window
    pub fn get_data(width: u32, height: u32) -> Self{
        let pix_width_height: u16 = (width as f32 / constants::PIXELS_WIDTH as f32) as u16;
        return Self{
            window_width: width,
            window_height: height,
            pix_width_height,
        };
    }
    // Gets the width/height of a pixel
    pub fn get_width_height(&self) -> u16{
        return self.pix_width_height;
    }
    // Gets the window width and height
    pub fn get_screen_dimensions(&self) -> (u32, u32){
        return (self.window_width, self.window_height);
    }
}

// Struct that defines a full pixel
#[derive(Copy, Clone, Debug)]
pub struct Pixel{
    // Red value. 0-255
    r: u8,
    // Green value. 0-255
    g: u8,
    // Blue value. 0-255
    b: u8,
    // X position. in my pixels, not window pixels
    x_pos: u16,
    // Y position. in py pixels, not window pixels
    y_pos: u16,
    // Absolute position. Unique identifier based on each pixel
    abs_pos: u32,
    // Check if it's been rendered
    been_rendered: bool,
}

impl Pixel{
    // Generate a new pixel
    pub fn new(r: u8, g: u8, b: u8, x_pos: u16, y_pos: u16) -> Self{
        return Self{
            // Use defined red value
            r,
            // Use defined green value
            g,
            // Use defined blue value
            b,
            // Use defined x position
            x_pos,
            // Use defined y position
            y_pos,
            // Create the absolute position
            abs_pos: (y_pos as u32 * constants::PIXELS_WIDTH + x_pos as u32) as u32,
            // Since just created, hasn't been rendered
            been_rendered: false,
        };
    }
    // Generate a square(Rect) for rendering
    pub fn pix_rect(&self, pix_data: PixelData) -> Result<Rect, ()>{
        // Get the width and height of individual pixels
        let width_height = pix_data.get_width_height();
        // Get screen dimensions
        let (max_width, max_height) = pix_data.get_screen_dimensions();

        // Check to make sure pixels don't go further than the available window space. 
        // return error if it is too long
        if ((self.x_pos * width_height) + width_height) >= max_width as u16 || ((self.y_pos * width_height) + width_height) >= max_height as u16{
            return Err(());
        }

        // Generate the square "pixel"
        let pix_square = Rect::from_xywh(
            // Get x position by multiplying size of our pixel by it's size
            (self.x_pos * width_height) as f32,
            // Get y position by multiplying size of our pixel by it's size
            (self.y_pos * width_height) as f32,
            // give the width and height of our pixels as the width and height of rectangle to draw
            width_height as f32,
            width_height as f32,
        );
        // Return the rectangle as a result
        return Ok(pix_square.unwrap());
    }
    // Return red, gree, blue, and alpha values of each pixel
    pub fn pix_color(&self) -> (u8, u8, u8, u8){
        return (
            // return own red color
            self.r,
            // return own green color
            self.g,
            // return own blue color
            self.b, 
            // return 255 for alpha, as it's opaque
            255,
        );
    }
    // Check if it's been rendered
    pub fn been_rendered(&self) -> bool{
        return self.been_rendered;
    }
    // Change whether or not it's been rendered
    pub fn change_render_status(&mut self){
        self.been_rendered = !self.been_rendered;
    }
    // Compare pixels. returns true if same pixel
    pub fn comp_pixels(&self, other_pixel: Pixel) -> bool{
        // Check if same absolute position position
        if self.abs_pos == other_pixel.abs_pos{
            // Check if rgb values the same
            if self.r == other_pixel.r && self.g == other_pixel.g && self.b == other_pixel.b{
                // If they are the same, return true
                return true;
            }
            // If rgb values aren't the same, return false
            return false;
        }
        // If not same absolute position, returns false
        return false;
    }
    // Returns x and y positions
    pub fn get_location(&self) -> (u16, u16){
        return (self.x_pos, self.y_pos);
    }
    // Returns absolute position
    pub fn get_abs_loc(&self) -> u32{
        return self.abs_pos;
    }
}

// A struct that is just a list of all pixels, with some fun methods
#[derive(Clone, Debug)]
pub struct PixelList{
    // Storage vector for the pixels
    pixel_list: Vec<Pixel>,
    // Index for when we're parsing the list
    index: usize,
    // Permits or disables rerending of pixels
    allow_rerender: bool,
}

impl PixelList{
    // Just generates a blank pixel list
    pub fn new() -> Self{
        // Generate a blank vector
        let pixel_list: Vec<Pixel> = Vec::new();
        return Self{
            // Return blank vector
            pixel_list,
            // Index starts as 0
            index: 0,
            // Rerednering disabled by default
            allow_rerender: false,
        };
    }
    // Adds a pixel to the list, but "smartly"
    pub fn add_pixel(&mut self, pixel: Pixel){
        // If the length is 0, just add it
        if self.pixel_list.len() == 0{
            self.pixel_list.push(pixel);
            return;
        }

        // get the index of where the pixel should go
        let index = self.locate(pixel.get_abs_loc()) as usize;
        // if the pixels are the same at that index, just return
        if index != u32::MAX as usize && pixel.comp_pixels(self.pixel_list[index]){
            return;
        }
        // if there's a pixel there, but it's a different pixel, replace it
        if index as u32 != u32::MAX && self.pixel_list[index].get_abs_loc() == pixel.get_abs_loc(){
            self.pixel_list[index] = pixel;
        } else{
            // If there's not a pixel there, push the pixel to the back
            self.pixel_list.push(pixel);
            // Sort the pixel list
            self.sort_pixels();
        }
    }
    // Sort the list based on the absolute position of the pixels
    pub fn sort_pixels(&mut self){
        self.pixel_list.sort_by(|a, b| a.get_abs_loc().cmp(&b.get_abs_loc()));
    }
    // Locate the index of a specific absolute value
    pub fn locate(&self, search_num: u32) -> u32{
        // Create a max and a min of the vector
        let mut max = self.pixel_list.len() as u32;
        let mut min = 0;
        let mut element_num = u32::MAX;
        // Create a halfway point
        let mut split_index: u32 = (max + min) / 2;
        // loop while we don't have the index
        let mut iterations = 0;
        while search_num != element_num && (split_index != (self.pixel_list.len() - 1) as u32) && split_index != 0{
            // update the halfway point
            split_index = (max + min) / 2;
            // get the absolute position of the midway point
            element_num = self.pixel_list[split_index as usize].get_abs_loc();
            // If the midway point equals the point we're looking for, return the index
            if element_num == search_num{
                return split_index;
            } else{
                // If not, create new min or max value based on whether too high or low
                if search_num > element_num{
                    min = split_index;
                } else{
                    max = split_index;
                }
            }
            iterations += 1;
            if iterations >= self.pixel_list.len(){
                break;
            }
            // Repeat the loop
        }
        return u32::MAX;
    }
    // Make the allow rerender boolean true
    pub fn allow_rerender(&mut self){
        self.allow_rerender = true;
    }
    // Make the allow rerender boolean false
    pub fn disable_rerender(&mut self){
        self.allow_rerender = false;
    }
    pub fn clean(&mut self){
        let mut abs_pos_hs: HashSet<u32> = HashSet::new();
        let mut count = 0;
        for pixel in self.pixel_list.clone(){
            if abs_pos_hs.contains(&pixel.get_abs_loc()){
                self.pixel_list.remove(count);
            } else{
                abs_pos_hs.insert(pixel.get_abs_loc());
            }
            count += 1;
        }
    }
}

// The iterator implementation so I can for loop
impl Iterator for PixelList{
    // Define the type returned by the iterator
    type Item = Pixel;
    fn next(&mut self) -> Option<Pixel>{
        // If the length of the array is 0, just return nothing
        if self.pixel_list.len() == 0{
            return None;
        }
        // while the index is less that the length of the pixel list
        while self.index < self.pixel_list.len(){
            // if rerendering is allowed, or the pixel hasn't been rendered
            if self.allow_rerender || !self.pixel_list[self.index].been_rendered(){
                // get the pixel at the index of the list
                let first_pixel = self.pixel_list[self.index];
                // change the render status of the pixel
                self.pixel_list[self.index].change_render_status();
                // Increase the index by 1
                self.index += 1;
                // Return the pixel
                return Some(first_pixel);
            }
            // If we're not allowed to render the pixel, just increment the quantity
            self.index += 1;
        } 
        return None;
    }
}
