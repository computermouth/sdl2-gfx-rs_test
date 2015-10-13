extern crate sdl2;
extern crate sdl2_gfx;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;

use sdl2_gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("rust-sdl2_gfx: draw line & FPSManager", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(pixels::Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    let mut events = sdl_context.event_pump().unwrap();
	
    'main: loop {
		renderer.set_draw_color(pixels::Color::RGB(0, 0, 0));
		renderer.clear();
		
        for event in events.poll_iter() {
            match event {

                Event::Quit {..} => break 'main,

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape {
                        break 'main
                    }
                }

                _ => {}
            }
        }
        let vx = vec![100, 400, 400, 100];
        let vy = vec![100, 100, 400, 400];
        let color = pixels::Color::RGBA(255, 0, 0, 110);
        let _ = renderer.filled_polygon(&vx, &vy, color);
        
        let vx2 = vec![200, 500, 500, 200];
        let vy2 = vec![200, 200, 500, 500];
        let color2 = pixels::Color::RGBA(255, 0, 0, 255);
        let _ = renderer.filled_polygon(&vx2, &vy2, color2);
        
        renderer.present();
    }
}
