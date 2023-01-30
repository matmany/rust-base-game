//extern crate sdl2;
//https://sunjay.dev/learn-game-dev/refactor-traditional-game-loop.html
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

fn render(canvas: &mut WindowCanvas, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
}

fn main() -> Result<(), String>{ 

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Game Tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("Error no video");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Canvas falhou");

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        //Handle events
        for event in event_pump.poll_iter(){
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        //Update
        i = (i + 1) % 255;

        render(&mut canvas, Color::RGB(i, 64, 255 - i));
        println!("{}", i);
        println!("Hello, world!")
    }
    Ok(())
}