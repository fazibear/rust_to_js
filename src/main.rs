extern crate sdl2;

use std::process;
use sdl2::rect::{Rect};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window  = match video_ctx
        .window("rust_to_js", 640, 480)
        .position_centered()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err)   => panic!("failed to create window: {}", err)
        };

    let mut canvas = match window
        .into_canvas()
        .build() {
            Ok(canvas) => canvas,
            Err(err) => panic!("failed to create canvas: {}", err)
        };

    let mut rect = Rect::new(10, 10, 10, 10);
    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);

    let mut events = ctx.event_pump().unwrap();

    let mut main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    rect.x -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    rect.x += 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    rect.y -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    rect.y += 10;
                },
                _ => {}
            }
        }

        let _ = canvas.set_draw_color(black);
        let _ = canvas.clear();
        let _ = canvas.set_draw_color(white);
        let _ = canvas.fill_rect(rect);
        let _ = canvas.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}
