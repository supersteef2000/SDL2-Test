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
    let wasd_map = sdl2::surface::Surface::load_bmp(Path::new("assets/wasd.bmp")).unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&temp_surface)
        .map_err(|e| e.to_string()).unwrap();
    let wasd_texture = texture_creator
        .create_texture_from_surface(&wasd_map)
        .map_err(|e| e.to_string()).unwrap();
    let timer = sdl_context.timer().unwrap();
    let mut rect1 = Rect::new(0, 24, 19, 24);
    let mut rect2 = Rect::new(0, 24, 57, 72);
    let mut n = Rect::new(0, 24 - height, 57, 72);
    let mut w = Rect::new(0 - width, 24, 57, 72);
    let mut s = Rect::new(0, 24 + height, 57, 72);
    let mut e = Rect::new(0 + width, 24, 57, 72);
    let mut nw = Rect::new(0 - width, 24 - height, 57, 72);
    let mut ne = Rect::new(0 - width, 24 + height, 57, 72);
    let mut sw = Rect::new(0 + width, 24 + height, 57, 72);
    let mut se = Rect::new(0 + width, 24 - height, 57, 72);
    let mut wasd1 = Rect::new(0, 0 - width, 6, 2);
    let mut wasd2 = Rect::new(0, 0 - width, 180, 60);
    let mut flipped = false;
    rect2.center_on(Point::new(width / 2, height / 2));
    wasd2.center_on(Point::new(120, 60));
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut j = 0;
    let mut r = 224;
    let mut g = 32;
    let mut b = 32;
    let mut bg_state = 1;
    let bg_speed = 1;
    let mut current_x = (width as f32 - 57.0) / 2.0;
    let mut current_y = (height as f32 - 72.0) / 2.0;
    let mut key = 0;
    let movement_speed: f32 = 2.8125;
    let diagonal_speed = (movement_speed.powf(2.0) / 2.0).sqrt();
    let mut speed_mod = 1.0;
    let mut character_select = 1;
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
        match bg_state % 6 {
            1 => {
                g += bg_speed;
                if g == 191 {
                    bg_state += 1;
                }
            },
            2 => {
                r -= bg_speed;
                if r == 64 {
                    bg_state += 1;
                }
            },
            3 => {
                b += bg_speed;
                if b == 191 {
                    bg_state += 1;
                }
            },
            4 => {
                g -= bg_speed;
                if g == 64 {
                    bg_state += 1;
                }
            },
            5 => {
                r += bg_speed;
                if r == 191 {
                    bg_state += 1;
                }
            },
            _ => {
                b -= bg_speed;
                if b == 64 {
                    bg_state += 1;
                }
            },
        }
        canvas.set_draw_color(Color::RGB(r as u8, g as u8, b as u8));
        canvas.clear();
        let ticks = timer.ticks() as i32;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => key += 1,
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => key += 2,
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => key += 4,
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => key += 8,
                Event::KeyDown { keycode: Some(Keycode::LShift), repeat: false, .. } => speed_mod = 2.0,
                Event::KeyDown { keycode: Some(Keycode::Num1), repeat: false, .. } => character_select = 0,
                Event::KeyDown { keycode: Some(Keycode::Num2), repeat: false, .. } => character_select = 1,
                Event::KeyDown { keycode: Some(Keycode::Num3), repeat: false, .. } => character_select = 2,
                Event::KeyUp { keycode: Some(Keycode::W), .. } => key -= 1,
                Event::KeyUp { keycode: Some(Keycode::A), .. } => key -= 2,
                Event::KeyUp { keycode: Some(Keycode::S), .. } => key -= 4,
                Event::KeyUp { keycode: Some(Keycode::D), .. } => key -= 8,
                Event::KeyUp { keycode: Some(Keycode::LShift), .. } => speed_mod = 1.0,
                _ => ()
            }
        }

        match key {
            // W and S cancel each other out, so do A and D, so no need to deal with those possibilities separately
            0 | 5 | 10 | 15 => {
                // No keys pressed, WS, AD, WASD
                rect1.set_x(0);
            }
            1 | 11 => {
                // W, WAD
                current_y -= movement_speed * speed_mod;
                rect1.set_x(19 * ((ticks / 100) % 4));
            }
            2 | 7 => {
                // A, WAS
                flipped = true;
                current_x -= movement_speed * speed_mod;
                rect1.set_x(19 * ((ticks / 100) % 4));
            }
            3 => {
                // WA
                flipped = true;
                current_x -= diagonal_speed * speed_mod;
                current_y -= diagonal_speed * speed_mod;
                rect1.set_x(19 * ((ticks / 100) % 4));
            }
            4 | 14 => {
                // S, ASD
                current_y += movement_speed * speed_mod;
                rect1.set_x(19 * ((ticks / 100) % 4));
            }
            6 => {
                // AS
                flipped = true;
                current_x -= diagonal_speed * speed_mod;
                current_y += diagonal_speed * speed_mod;
                rect1.set_x(19 * ((ticks / 100) % 4));
            }
            8 | 13 => {
                // D, WSD
                flipped = false;
                current_x += movement_speed * speed_mod;
                rect1.set_x(19 * ((ticks / 100) % 4));
            }
            9 => {
                // WD
                flipped = false;
                current_x += diagonal_speed * speed_mod;
                current_y -= diagonal_speed * speed_mod;
                rect1.set_x(19 * ((ticks / 100) % 4));
            }
            12 => {
                // SD
                flipped = false;
                current_x += diagonal_speed * speed_mod;
                current_y += diagonal_speed * speed_mod;
                rect1.set_x(19 * ((ticks / 100) % 4));
            }
            _ => {
                // This should never happen
                println!("Impossible key combination {} found. Please report to author. Exiting.",  key);
                break 'running;
            }
        }

        wasd1.set_x(key * 6);
        if speed_mod == 2.0 {
            wasd1.set_y(2);
        } else {
            wasd1.set_y(0);
        }

        rect1.set_y(character_select * 24);

        rect2.set_x(current_x as i32);
        rect2.set_y(current_y as i32);

        if rect2.x() < 0 {
            current_x += width as f32;
        }
        if rect2.x() > width {
            current_x -= width as f32;
        }
        if rect2.y() < 0 {
            current_y += height as f32;
        }
        if rect2.y() > height {
            current_y -= height as f32;
        }

        n.set_x(rect2.x());
        n.set_y(rect2.y() - height);
        w.set_x(rect2.x() - width);
        w.set_y(rect2.y());
        s.set_x(rect2.x());
        s.set_y(rect2.y() + height);
        e.set_x(rect2.x() + width);
        e.set_y(rect2.y());
        nw.set_x(w.x());
        nw.set_y(n.y());
        ne.set_x(e.x());
        ne.set_y(n.y());
        sw.set_x(w.x());
        sw.set_y(s.y());
        se.set_x(e.x());
        se.set_y(s.y());

        canvas.copy_ex(&texture, rect1, rect2, 0.0, None, flipped, false).unwrap();
        canvas.copy_ex(&texture, rect1, n, 0.0, None, flipped, false).unwrap();
        canvas.copy_ex(&texture, rect1, w, 0.0, None, flipped, false).unwrap();
        canvas.copy_ex(&texture, rect1, s, 0.0, None, flipped, false).unwrap();
        canvas.copy_ex(&texture, rect1, e, 0.0, None, flipped, false).unwrap();
        canvas.copy_ex(&texture, rect1, nw, 0.0, None, flipped, false).unwrap();
        canvas.copy_ex(&texture, rect1, ne, 0.0, None, flipped, false).unwrap();
        canvas.copy_ex(&texture, rect1, sw, 0.0, None, flipped, false).unwrap();
        canvas.copy_ex(&texture, rect1, se, 0.0, None, flipped, false).unwrap();
        canvas.copy_ex(&wasd_texture, wasd1, wasd2, 0.0, None, false, false).unwrap();
        canvas.present();
        std::thread::sleep(Duration::new(0, 1000000000u32 / 60));
    }
}
