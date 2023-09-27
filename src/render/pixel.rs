use tiny_skia::{Path, PathBuilder, Rect};

use crate::{
    render::window::MainWindow,
    constants,
};

#[derive(Copy, Clone, Debug)]
pub struct PixelData{
    pix_width_height: u16,
}

impl PixelData{
    pub fn get_data(width: u32, _height: u32) -> Self{
        let pix_width_height: u16 = (width / constants::PIXELS_WIDTH) as u16;
        return Self{
            pix_width_height,
        };
    }
    pub fn get_width_height(&self) -> u16{
        return self.pix_width_height;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pixel{
    r: u8,
    g: u8,
    b: u8,
    x_pos: u16,
    y_pos: u16,
    abs_pos: u32,
    been_rendered: bool,
}

impl Pixel{
    pub fn new(r: u8, g: u8, b: u8, x_pos: u16, y_pos: u16) -> Self{
        return Self{
            r,
            g,
            b,
            x_pos,
            y_pos,
            abs_pos: (y_pos as u32 * constants::PIXELS_WIDTH + x_pos as u32) as u32,
            been_rendered: false,
        };
    }
    pub fn pix_rect(&self, pix_data: PixelData) -> Rect{
        let width_height = pix_data.get_width_height();
        let pix_square = Rect::from_xywh(
            (self.x_pos * width_height) as f32,
            (self.y_pos * width_height) as f32,
            width_height as f32,
            width_height as f32,
        );
        return pix_square.unwrap();
    }
    pub fn pix_color(&self) -> (u8, u8, u8, u8){
        return (
            self.r,
            self.g,
            self.b, 
            255,
        );
    }
    pub fn been_rendered(&self) -> bool{
        return self.been_rendered;
    }
    pub fn change_render_status(&mut self){
        self.been_rendered = !self.been_rendered;
    }
    pub fn comp_pixels(&self, other_pixel: Pixel) -> bool{
        if self.abs_pos == other_pixel.abs_pos{
            if self.r == other_pixel.r && self.g == other_pixel.g && self.b == other_pixel.b{
                return true;
            }
            return false;
        }
        return false;
    }
    pub fn get_location(&self) -> (u16, u16){
        return (self.x_pos, self.y_pos);
    }
    pub fn get_abs_loc(&self) -> u32{
        return self.abs_pos;
    }
}

#[derive(Clone, Debug)]
pub struct PixelList{
    pixel_list: Vec<Pixel>,
    index: usize,
    allow_rerender: bool,
}

impl PixelList{
    pub fn new() -> Self{
        let pixel_list: Vec<Pixel> = Vec::new();
        return Self{
            pixel_list,
            index: 0,
            allow_rerender: false,
        };
    }
    pub fn add_pixel(&mut self, pixel: Pixel){
        if self.pixel_list.len() == 0{
            self.pixel_list.push(pixel);
            return;
        }
        let index = self.locate(pixel.get_abs_loc()) as usize;
        if pixel.comp_pixels(self.pixel_list[index]){
            return;
        }
        if index as u32 != u32::MAX && self.pixel_list[index].get_abs_loc() == pixel.get_abs_loc(){
            self.pixel_list[index] = pixel;
        } else{
            self.pixel_list.push(pixel);
            self.sort_pixels();
        }
    }
    pub fn sort_pixels(&mut self){
        self.pixel_list.sort_by(|a, b| a.get_abs_loc().cmp(&b.get_abs_loc()));
    }
    pub fn locate(&self, search_num: u32) -> u32{
        let mut max = self.pixel_list.len() as u32;
        let mut min = 0;
        let element_num = u32::MAX;
        while search_num != element_num{
            let split_index: u32 = (max + min) / 2;
            let element_num = self.pixel_list[split_index as usize].get_abs_loc();
            if element_num == search_num{
                return split_index;
            } else{
                if search_num > element_num{
                    min = split_index;
                } else{
                    max = split_index;
                }
            }
        }
        return u32::MAX;
    }
    pub fn allow_rerender(&mut self){
        self.allow_rerender = true;
    }
    pub fn disable_rerender(&mut self){
        self.allow_rerender = false;
    }
}

impl Iterator for PixelList{
    type Item = Pixel;
    fn next(&mut self) -> Option<Pixel>{
        if self.pixel_list.len() == 0{
            return None;
        }
        while self.index < self.pixel_list.len(){
            if self.allow_rerender || !self.pixel_list[self.index].been_rendered(){
                let first_pixel = self.pixel_list[self.index];
                self.pixel_list[self.index].change_render_status();
                self.index += 1;
                return Some(first_pixel);
            }
            self.index += 1;
        } 
        return None;
    }
}
