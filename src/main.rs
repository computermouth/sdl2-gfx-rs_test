extern crate sdl2;
extern crate sdl2_gfx;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;

use sdl2_gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {

	let face_X = vec![ 178, 524, 616, 621, 597, 549, 531, 508, 238, 102, 32, 23, 50, 47, 93]; 
	let face_Y = vec![ 85, 193, 465, 570, 632, 667, 645, 697, 878, 842, 689, 534, 413, 344, 199];
					   
	let leftNostrilX = vec![ 111, 133, 125, 110]; 
	let leftNostrilY = vec![ 521, 532, 543, 533];

	let rightNostrilX = vec![ 161, 186, 201, 195]; 
	let rightNostrilY = vec![ 529, 525, 533, 548];

	let lipsX = vec![ 112, 132, 174, 210, 268, 185, 117, 83]; 
	let lipsY = vec![ 609, 615, 605, 637, 663, 703, 697, 655];

	let teethX = vec![ 123, 139, 179, 199, 164]; 
	let teethY = vec![ 644, 637, 631, 648, 656];

	let leftBrowX = vec![ 43, 93, 150, 141, 49]; 
	let leftBrowY = vec![ 332, 314, 325, 342, 349];

	let rightBrowX = vec![ 298, 375, 445, 366, 293]; 
	let rightBrowY = vec![ 305, 311, 354, 348, 338];

	let leftWhiteX = vec![ 85, 113, 145, 141, 85, 72]; 
	let leftWhiteY = vec![ 374, 373, 385, 397, 408, 394];

	let rightWhiteX = vec![ 320, 361, 394, 370, 321, 297]; 
	let rightWhiteY = vec![ 372, 378, 403, 406, 407, 393];

	let leftIrisX = vec![ 92, 109, 121, 104, 84, 79]; 
	let leftIrisY = vec![ 369, 371, 388, 402, 401, 382];

	let rightIrisX = vec![ 318, 342, 362, 359, 342, 319, 311]; 
	let rightIrisY = vec![ 369, 370, 381, 394, 409, 406, 391];

	let leftPupilX = vec![ 97, 104, 97, 92]; 
	let leftPupilY = vec![ 374, 387, 396, 387];

	let rightPupilX = vec![ 334, 342, 335, 326]; 
	let rightPupilY = vec![ 377, 389, 397, 386];

	let hairX = vec![ 92, 73, 109, 212, 321, 438, 519, 597, 622, 618, 572, 564, 528, 532, 495, 484, 422, 323, 198, 169, 128]; 
	let hairY = vec![ 203, 174, 98, 29, 13, 42, 102, 215, 319, 476, 456, 521, 531, 455, 336, 226, 174, 144, 159, 141, 163];


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
        let _ = renderer.filled_polygon(&faceX, &faceY, color);
        color = pixels::Color::RGBA(148, 101, 101, 255);
        let _ = renderer.filled_polygon(&leftNostrilX, &leftNostrilY, color);
        color = pixels::Color::RGBA(148, 101, 101, 255);
        let _ = renderer.filled_polygon(&rightNostrilX, &rightNostrilY, color);
        color = pixels::Color::RGBA(249, 249, 249, 255);
        let _ = renderer.filled_polygon(&lipsX, &lipsY, color);
        color = pixels::Color::RGBA(249, 249, 249, 255);
        let _ = renderer.filled_polygon(&teethX, &teethY, color);
        color = pixels::Color::RGBA(191, 148, 0, 255);
        let _ = renderer.filled_polygon(&leftBrowX, &leftBrowY, color);
        color = pixels::Color::RGBA(191, 148, 0, 255);
        let _ = renderer.filled_polygon(&rightBrowX, &rightBrowY, color);
        color = pixels::Color::RGBA(249, 249, 249, 255);
        let _ = renderer.filled_polygon(&leftWhiteX, &leftWhiteY, color);
        color = pixels::Color::RGBA(249, 249, 249, 255);
        let _ = renderer.filled_polygon(&rightWhiteX, &rightWhiteY, color);
        color = pixels::Color::RGBA(100, 157, 97, 255);
        let _ = renderer.filled_polygon(&leftIrisX, &leftIrisY, color);
        color = pixels::Color::RGBA(100, 157, 97, 255);
        let _ = renderer.filled_polygon(&rightIrisX, &rightIrisY, color);
        color = pixels::Color::RGBA(10, 10, 10, 255);
        let _ = renderer.filled_polygon(&leftPupilX, &leftPupilY, color);
        color = pixels::Color::RGBA(10, 10, 10, 255);
        let _ = renderer.filled_polygon(&rightPupilX, &rightPupilY, color);
        color = pixels::Color::RGBA(180, 155, 0, 255);
        let _ = renderer.filled_polygon(&hairX, &hairY, color);
        
        renderer.present();
    }
}
