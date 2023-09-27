use std::{
    num::NonZeroU32,
    sync::mpsc::Receiver,
};

use crate::render::pixel::PixelList;

use softbuffer::{Context, Surface};
use tiny_skia::{Pixmap, Paint, Transform};
use winit::{
    event::{
        Event,
        WindowEvent,
    },
    event_loop::{
        ControlFlow,
        EventLoop,
    },
    window::{
        Window,
        WindowBuilder,
    },
};

use super::pixel::PixelData;

pub struct MainWindow{
    event_loop: Option<EventLoop<()>>,
    window: Window,
    surface: Surface,
    pixel_receiver: Receiver<PixelList>,
}

impl MainWindow{
    pub fn new(pixel_receiver: Receiver<PixelList>) -> Self{
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let context = unsafe{Context::new(&window)}.unwrap();
        let surface = unsafe{Surface::new(&context, &window)}.unwrap();
        let event_loop = Some(event_loop);
        return Self{
            event_loop,
            window,
            surface,
            pixel_receiver,
        }
    }
    pub fn run(mut self){
        let handler: EventLoop<()> = self.event_loop.take().unwrap();
        let mut first_run: bool = true;
        handler.run(move |event, _, control_flow|{
            if first_run{
                *control_flow = ControlFlow::Wait;
                let (width, height) = {
                    let size = self.window.inner_size();
                    (size.width, size.height)
                };
                self.surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap()
                    )
                    .unwrap();
                first_run = false;
            }
            match event{
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    let (width, height) = {
                        let size = self.window.inner_size();
                        (size.width, size.height)
                    };
                    self.surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap()
                        )
                        .unwrap();

                    let mut pixmap = Pixmap::new(width, height).unwrap();

                    let pixel_list_result = self.pixel_receiver.try_recv();
                    let pixel_list = match pixel_list_result{
                        Ok(pix_list) => pix_list,
                        Err(_e) => PixelList::new(),
                    };
                    
                    let pix_data = PixelData::get_data(width, height);
                    let mut paint = Paint::default();
                    for pixel in pixel_list{
                        let (r, g, b, a) = pixel.pix_color();
                        paint.set_color_rgba8(r, g, b, a);
                        match pixel.pix_rect(pix_data.clone()){
                            Ok(pixel_rect) => {
                                pixmap.fill_rect(pixel_rect, &paint, Transform::identity(), None);
                            },
                            Err(_e) => {},
                        }
                    }

                    let mut buffer = self.surface.buffer_mut().unwrap();
                    for index in 0..(width * height) as usize{
                        buffer[index] = pixmap.data()[index * 4 + 2] as u32
                            | (pixmap.data()[index * 4 + 1] as u32) << 8
                            | (pixmap.data()[index * 4] as u32) << 16;
                    }
                    buffer.present().unwrap();
                },
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::CloseRequested,
                } if window_id == self.window.id() => {
                    *control_flow = ControlFlow::Exit;
                },
                _ => {},
            }
            'frame_loop: loop{
                let (width, height): (u32, u32) = (self.window.inner_size().width, self.window.inner_size().height);
                let mut pixmap = Pixmap::new(width, height).unwrap();
                let pixel_list_result = self.pixel_receiver.try_recv();
                let pixel_list = match pixel_list_result{
                    Ok(pix_list) => pix_list,
                    Err(_e) => {
                        break 'frame_loop;
                    },
                };
                
                let pix_data = PixelData::get_data(width, height);
                let mut paint = Paint::default();
                for pixel in pixel_list{
                    let (r, g, b, a) = pixel.pix_color();
                    paint.set_color_rgba8(r, g, b, a);
                    match pixel.pix_rect(pix_data.clone()){
                        Ok(pixel_rect) => {
                            pixmap.fill_rect(pixel_rect, &paint, Transform::identity(), None);
                        },
                        Err(_e) => {},
                    }
                }

                let mut buffer = self.surface.buffer_mut().unwrap();
                for index in 0..(width * height) as usize{
                    buffer[index] = pixmap.data()[index * 4 + 2] as u32
                        | (pixmap.data()[index * 4 + 1] as u32) << 8
                        | (pixmap.data()[index * 4] as u32) << 16;
                }
                buffer.present().unwrap();
                break 'frame_loop;
            }
        });
    }
    pub fn get_dimensions(self) -> (u32, u32){
        let (width, height) = {
            let size = self.window.inner_size();
            (size.width, size.height)
        };
        return (width, height);

    }
}
