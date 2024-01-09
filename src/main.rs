extern  crate  sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;
use sdl2::rect::Rect;

fn main() {
   let sdl_context=sdl2::init().expect("SDL initialization failed");
    let video_subsystem=sdl_context.video().expect("could not get video subsystem");
    let window=video_subsystem.window("Rust sdl demo: Video",800,600)
        .position_centered().opengl().build().expect("failed to create window");
    let mut canvas=window.into_canvas().build().expect("failed to convert window into canvas");
    canvas.set_draw_color(Color::RGB(255,0,0));
    canvas.clear();
    canvas.present();
    let texture_creator=canvas.texture_creator();
    const TEXTURE_SIZE:u32=32;
    let mut square_texture=texture_creator.create_texture_target(None,TEXTURE_SIZE,TEXTURE_SIZE).expect("failed to create a texture");

    canvas.with_texture_canvas(&mut  square_texture, |texture|{
        texture.set_draw_color(Color::RGB(0,255,0));
        texture.clear();

    }).expect("failed to color a texture");

    let mut event_pump=sdl_context.event_pump().expect("failed to get sdl event pump");
    'running: loop {
        for event in event_pump.poll_iter(){
            match event{
                Event::Quit {..} |
                Event::KeyDown { keycode:Some(Keycode::Escape),..}=>
                    {
                        break 'running;
                    },
                _ =>{

                }

            }
            canvas.set_draw_color(Color::RGB(255,0,0));
            canvas.clear();
            canvas.copy(&square_texture,None,Rect::new(0,0,TEXTURE_SIZE,TEXTURE_SIZE)).expect("Could not copy Texture into window");
            canvas.present();
            sleep(Duration::new(0,1_000_000_000u32/60));

        }

    }



}
