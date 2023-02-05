//use std::task::Ready;
use std::time::Duration;

//extern crate sdl2;
//https://sunjay.dev/learn-game-dev/refactor-traditional-game-loop.html
use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    // canvas.set_draw_color(color);
    // canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2 );
    let screen_rect = Rect::from_center(screen_position, player.sprite.width()*4, player.sprite.height()*4);
    
    canvas.copy(texture, player.sprite, screen_rect)?;
    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);

    let window = video_subsystem
        .window("Game Tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("Error no video");

    let mut canvas = window.into_canvas().build().expect("Canvas falhou");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("../assets/bardo.png")?;

    //let mut current_color = Color::RGB(0, 0, 0);

    let player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        //Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
        //Update
        i = (i + 1) % 255;

        let current_color = Color::RGB(i, 64, 255 - i);
        
        render(&mut canvas, current_color, &texture, &player)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
