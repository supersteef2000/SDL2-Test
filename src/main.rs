use std::path::Path;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let width = 800;
    let height = 600;
    let window = video_subsystem.window("Window", width as u32, height as u32).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp")).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&temp_surface)
        .map_err(|e| e.to_string()).unwrap();
    let timer = sdl_context.timer().unwrap();
    let mut rect1 = Rect::new(0, 32, 32, 32);
    let mut rect2 = Rect::new(0, 32, 128, 128);
    let mut flipped = false;
    rect2.center_on(Point::new(width / 2, height / 2));
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut j = 0;
    let mut current_x = width / 2;
    'running: loop {
        if j == 0 {
            i = i + 1;
        }
        if j == 1 {
            i = i - 1;
        }
        if i == 0 {
            j = 0;
        }
        if i == 255 {
            j = 1;
        }
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        let ticks = timer.ticks() as i32;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    flipped = true;
                    current_x -= 2;
                    println!("{}", current_x);
                    rect1.set_x(32 * ((ticks / 100) % 4));
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    flipped = false;
                    current_x += 2;
                    println!("{}", current_x);
                    rect1.set_x(current_x);
                    rect1.set_x(32 * ((ticks / 100) % 4));
                },
                _ => ()
            }
        }


        rect2.set_x(current_x);
        // set the current frame for time
        canvas.copy_ex(&texture, rect1, rect2, 0.0, None, flipped, false);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1000000000u32 / 90));
    }
}
