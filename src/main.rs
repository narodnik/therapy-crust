use input::{Libinput, LibinputInterface, event::{TabletToolEvent, tablet_tool::{ProximityState, TabletToolEventTrait}}};
use input as libinput;
use std::{fs::{File, OpenOptions},
os::unix::{fs::OpenOptionsExt, io::OwnedFd},
path::Path, time::Duration
};

use sdl2::{
    pixels::Color,
    rect::Point,
    event::Event, keyboard::Keycode, gfx::primitives::DrawRenderer
};

extern crate libc;
use libc::{O_RDONLY, O_RDWR, O_WRONLY};

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }

    // This is in the crate's official docs, so I just disable the warning
    #[allow(unused_must_use, unused_unsafe)]
    fn close_restricted(&mut self, fd: OwnedFd) {
        unsafe {
            File::from(fd);
        }
    }
}

fn old_main() {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    loop {
        input.dispatch().unwrap();
        for event in &mut input {
            match event {
                libinput::Event::Keyboard(event) => {
                    //println!("keyboard: {:?}", ev);
                }
                libinput::Event::Tablet(event) => {
                    match event {
                        TabletToolEvent::Proximity(event) => {
                            match event.proximity_state() {
                                ProximityState::Out => {
                                    println!("out");
                                }
                                ProximityState::In => {
                                    println!("in");
                                }
                            }
                        }
                        TabletToolEvent::Axis(event) => {
                            let (x, y) = (event.x_transformed(1), event.y_transformed(1));
                        }
                        _ => {
                            // Ignored
                        }
                    }
                }
                _ => {
                    // Ignored
                }
            }
        }
    }
}

pub fn main() {
    let ctx = zmq::Context::new();
    //let socket = ctx.socket(zmq::REQ).unwrap();
    //socket.connect("tcp://127.0.0.1:1234").unwrap();
    //socket.send("hello world!", 0).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .borderless()
        .resizable()
        .maximized()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let (width, height) = {
            let view = canvas.viewport();
            (view.width(), view.height())
        };

        canvas.set_draw_color((0, 0, 0));
        canvas.clear();
        // Draw a line from center point to lower right
        let (center_x, center_y) = (width/2, height/2);
        canvas.thick_line(center_x as i16, center_y as i16, width as i16, height as i16, 4, Color::RGB(255, 255, 255));
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

