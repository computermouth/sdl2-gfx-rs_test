extern crate sdl2;
extern crate sdl2_gfx;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;

use sdl2_gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {

	let face_x = vec![ 178, 524, 616, 621, 597, 549, 531, 508, 238, 102, 32, 23, 50, 47, 93]; 
	let face_y = vec![ 85, 193, 465, 570, 632, 667, 645, 697, 878, 842, 689, 534, 413, 344, 199];
					   
	let left_nostril_x = vec![ 111, 133, 125, 110]; 
	let left_nostril_y = vec![ 521, 532, 543, 533];

	let right_nostril_x = vec![ 161, 186, 201, 195]; 
	let right_nostril_y = vec![ 529, 525, 533, 548];

	let lips_x = vec![ 112, 132, 174, 210, 268, 185, 117, 83]; 
	let lips_y = vec![ 609, 615, 605, 637, 663, 703, 697, 655];

	let teeth_x = vec![ 123, 139, 179, 199, 164]; 
	let teeth_y = vec![ 644, 637, 631, 648, 656];

	let left_brow_x = vec![ 43, 93, 150, 141, 49]; 
	let left_brow_y = vec![ 332, 314, 325, 342, 349];

	let right_brow_x = vec![ 298, 375, 445, 366, 293]; 
	let right_brow_y = vec![ 305, 311, 354, 348, 338];

	let left_white_x = vec![ 85, 113, 145, 141, 85, 72]; 
	let left_white_y = vec![ 374, 373, 385, 397, 408, 394];

	let right_white_x = vec![ 320, 361, 394, 370, 321, 297]; 
	let right_white_y = vec![ 372, 378, 403, 406, 407, 393];

	let left_iris_x = vec![ 92, 109, 121, 104, 84, 79]; 
	let left_iris_y = vec![ 369, 371, 388, 402, 401, 382];

	let right_iris_x = vec![ 318, 342, 362, 359, 342, 319, 311]; 
	let right_iris_y = vec![ 369, 370, 381, 394, 409, 406, 391];

	let left_pupil_x = vec![ 97, 104, 97, 92]; 
	let left_pupil_y = vec![ 374, 387, 396, 387];

	let right_pupil_x = vec![ 334, 342, 335, 326]; 
	let right_pupil_y = vec![ 377, 389, 397, 386];

	let hair_x = vec![ 92, 73, 109, 212, 321, 438, 519, 597, 622, 618, 572, 564, 528, 532, 495, 484, 422, 323, 198, 169, 128]; 
	let hair_y = vec![ 203, 174, 98, 29, 13, 42, 102, 215, 319, 476, 456, 521, 531, 455, 336, 226, 174, 144, 159, 141, 163];


    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("sdl2-gfx-rs_test", SCREEN_WIDTH, SCREEN_HEIGHT)
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
        
        let mut color = pixels::Color::RGBA(249, 188, 188, 255);
        let _ = renderer.filled_polygon(&face_x, &face_y, color);
        color = pixels::Color::RGBA(148, 101, 101, 255);
        let _ = renderer.filled_polygon(&left_nostril_x, &left_nostril_y, color);
        color = pixels::Color::RGBA(148, 101, 101, 255);
        let _ = renderer.filled_polygon(&right_nostril_x, &right_nostril_y, color);
        color = pixels::Color::RGBA(249, 249, 249, 255);
        let _ = renderer.filled_polygon(&lips_x, &lips_y, color);
        color = pixels::Color::RGBA(249, 249, 249, 255);
        let _ = renderer.filled_polygon(&teeth_x, &teeth_y, color);
        color = pixels::Color::RGBA(191, 148, 0, 255);
        let _ = renderer.filled_polygon(&left_brow_x, &left_brow_y, color);
        color = pixels::Color::RGBA(191, 148, 0, 255);
        let _ = renderer.filled_polygon(&right_brow_x, &right_brow_y, color);
        color = pixels::Color::RGBA(249, 249, 249, 255);
        let _ = renderer.filled_polygon(&left_white_x, &left_white_y, color);
        color = pixels::Color::RGBA(249, 249, 249, 255);
        let _ = renderer.filled_polygon(&right_white_x, &right_white_y, color);
        color = pixels::Color::RGBA(100, 157, 97, 255);
        let _ = renderer.filled_polygon(&left_iris_x, &left_iris_y, color);
        color = pixels::Color::RGBA(100, 157, 97, 255);
        let _ = renderer.filled_polygon(&right_iris_x, &right_iris_y, color);
        color = pixels::Color::RGBA(10, 10, 10, 255);
        let _ = renderer.filled_polygon(&left_pupil_x, &left_pupil_y, color);
        color = pixels::Color::RGBA(10, 10, 10, 255);
        let _ = renderer.filled_polygon(&right_pupil_x, &right_pupil_y, color);
        color = pixels::Color::RGBA(180, 155, 0, 255);
        let _ = renderer.filled_polygon(&hair_x, &hair_y, color);
        
        renderer.present();
    }
}
