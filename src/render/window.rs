/*
*   Defines all window related structs
*   Works closely with the pixel structs
*/


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

// Struct that is our main program window
pub struct MainWindow{
    // Event loop in an option so I can pass it to the run function
    event_loop: Option<EventLoop<()>>,
    // The actual window
    window: Window,
    // The surface we draw on
    surface: Surface,
    // A receiver to receive the pixels we are to draw
    pixel_receiver: Receiver<PixelList>,
}

impl MainWindow{
    // Create a new window struct
    pub fn new(pixel_receiver: Receiver<PixelList>) -> Self{
        // Create new event loop
        let event_loop = EventLoop::new();
        // uild our window using the event loop we made
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        // Create the context to create the surface
        let context = unsafe{Context::new(&window)}.unwrap();
        // Create the surface using the context and the window
        let surface = unsafe{Surface::new(&context, &window)}.unwrap();
        // Wrap the event loop in an option 
        // so that we can take the actual object out in our run function
        let event_loop = Some(event_loop);
        // Return this to the user
        // Also return the receiver for our pixel list
        return Self{
            event_loop,
            window,
            surface,
            pixel_receiver,
        }
    }
    // Run the window that we just created
    pub fn run(mut self){
        // let our handler equal our event loop that we wrapped in an option
        let handler: EventLoop<()> = self.event_loop.take().unwrap();
        // boolean to check if it's the first run
        let mut first_run: bool = true;

        // Start the control flow
        handler.run(move |event, _, control_flow|{
            // If it's the first run, initialize the control flow info
            if first_run{
                *control_flow = ControlFlow::Wait;
                // Get window dimensions
                let (width, height) = {
                    let size = self.window.inner_size();
                    (size.width, size.height)
                };
                // Create surface with these dimensions
                self.surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap()
                    )
                    .unwrap();
                // Since this was run, make first run boolean false
                first_run = false;
            }
            // Match an event that happens to windows
            match event{
                // If a window redraw is requested, run this branch
                // For window size changes, focus changes
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    // Get new width and height
                    let (width, height) = {
                        let size = self.window.inner_size();
                        (size.width, size.height)
                    };
                    // Resize the surface to fit new dimensions
                    self.surface
                        .resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap()
                        )
                        .unwrap();

                    // Create a pixel map
                    let mut pixmap = Pixmap::new(width, height).unwrap();

                    // try to get the pixel list from our receiver
                    let pixel_list_result = self.pixel_receiver.try_recv();
                    // Match what we get
                    let pixel_list = match pixel_list_result{
                        // If it's a list, feed a list to the pixel_list variable
                        Ok(pix_list) => pix_list,
                        // If not, feed a blank list to the pixel_list variable
                        Err(_e) => PixelList::new(),
                    };
                    
                    // Get the pixel data from the screen we have
                    let pix_data = PixelData::get_data(width, height);
                    // Create a new paint instance so we can paint on the pixels
                    let mut paint = Paint::default();
                    // Parse through the pixel list
                    for pixel in pixel_list{
                        // Get the red, green, blue, and alpha values from the pixel
                        let (r, g, b, a) = pixel.pix_color();
                        // Set the paints color to that of the pixel
                        paint.set_color_rgba8(r, g, b, a);
                        // request the rectangle, match the result
                        match pixel.pix_rect(pix_data.clone()){
                            // If we can draw the rectangle, draw the rectangle
                            Ok(pixel_rect) => {
                                pixmap.fill_rect(pixel_rect, &paint, Transform::identity(), None);
                            },
                            // If we can't retreive the rectangle, it's out of bounds
                            // so don't draw it
                            Err(_e) => {},
                        }
                    }

                    // Create a buffer to interact with our window
                    let mut buffer = self.surface.buffer_mut().unwrap();
                    // parse through our painted window and give to the buffer
                    for index in 0..(width * height) as usize{
                        buffer[index] = pixmap.data()[index * 4 + 2] as u32
                            | (pixmap.data()[index * 4 + 1] as u32) << 8
                            | (pixmap.data()[index * 4] as u32) << 16;
                    }
                    // Give the buffer to the window so it draws it on the window
                    buffer.present().unwrap();
                },
                // If the user requests to close the window, close the window
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::CloseRequested,
                } if window_id == self.window.id() => {
                    *control_flow = ControlFlow::Exit;
                },
                // If the user does something else to the window that isn't a request redraw or a
                // close, ignore it
                _ => {},
            }
            // Start a loop (just for a goto/break point
            'frame_loop: loop{
                // Get a width and height of the window
                let (width, height): (u32, u32) = (self.window.inner_size().width, self.window.inner_size().height);
                // Create a new pixel map
                let mut pixmap = Pixmap::new(width, height).unwrap();

                // Try to receive the pixel list
                let pixel_list_result = self.pixel_receiver.try_recv();
                // Match the returned result
                let pixel_list = match pixel_list_result{
                    // If it's ok, feed the returned value to pixel_list
                    Ok(pix_list) => pix_list,
                    Err(_e) => {
                    // If it's not ok, just break out of the loop and start over
                        break 'frame_loop;
                    },
                };
                
                // Get new pixel data
                let pix_data = PixelData::get_data(width, height);
                // Create a new paint variable
                let mut paint = Paint::default();
                // Parse through our returned pixel list
                for pixel in pixel_list{
                    // get red, green, blue, and alpha values of the pixel
                    let (r, g, b, a) = pixel.pix_color();
                    // Set our brush color to the pixel colors
                    paint.set_color_rgba8(r, g, b, a);
                    // Match the rectangle of the pixel
                    match pixel.pix_rect(pix_data.clone()){
                        // If we get the rectangle, draw the rectangle
                        Ok(pixel_rect) => {
                            pixmap.fill_rect(pixel_rect, &paint, Transform::identity(), None);
                        },
                        // If we don't, then it's out of bounds, so don't draw
                        Err(_e) => {},
                    }
                }

                // Create a buffer to interact between our surface and the window
                let mut buffer = self.surface.buffer_mut().unwrap();
                // Parse through the surface we drew on and feed to the buffer
                for index in 0..(width * height) as usize{
                    buffer[index] = pixmap.data()[index * 4 + 2] as u32
                        | (pixmap.data()[index * 4 + 1] as u32) << 8
                        | (pixmap.data()[index * 4] as u32) << 16;
                }
                // Feed the buffer to the window
                buffer.present().unwrap();
                // Exit out of the loop
                break 'frame_loop;
            }
        });
    }
    // Gets the dimensions of the window
    pub fn get_dimensions(self) -> (u32, u32){
        let (width, height) = {
            let size = self.window.inner_size();
            (size.width, size.height)
        };
        return (width, height);
    }
}
