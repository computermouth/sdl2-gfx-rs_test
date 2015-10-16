extern crate sdl2;
extern crate sdl2_gfx;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;

use sdl2_gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

mod animshell;


fn main() {
		  
    let face_x : Vec<i16> = vec![ 178, 524, 616, 621, 597, 549, 531, 508, 238, 102, 32, 23, 50, 47, 93]; 
    let face_y : Vec<i16> = vec![ 85, 193, 465, 570, 632, 667, 645, 697, 878, 842, 689, 534, 413, 344, 199];
    let face_color = pixels::Color::RGBA(249, 188, 188, 255);
    let face = animshell::Shape { x: face_x, y: face_y, color: face_color};
			   
    let left_nostril_x  : Vec<i16> = vec![ 111, 133, 125, 110]; 
    let left_nostril_y  : Vec<i16> = vec![ 521, 532, 543, 533];
    let left_nostril_color = pixels::Color::RGBA(148, 101, 101, 255);
    let left_nostril = animshell::Shape { x: left_nostril_x, y: left_nostril_y, color: left_nostril_color};
	
    let right_nostril_x : Vec<i16> = vec![ 161, 186, 201, 195]; 
    let right_nostril_y : Vec<i16> = vec![ 529, 525, 533, 548];
    let right_nostril_color = pixels::Color::RGBA(148, 101, 101, 255);
    let right_nostril = animshell::Shape { x: right_nostril_x, y: right_nostril_y, color: right_nostril_color};

    let lips_x : Vec<i16> = vec![ 112, 132, 174, 210, 268, 185, 117, 83]; 
    let lips_y : Vec<i16> = vec![ 609, 615, 605, 637, 663, 703, 697, 655];
    let lips_color = pixels::Color::RGBA(249, 249, 249, 255);
    let lips = animshell::Shape { x: lips_x, y: lips_y, color: lips_color};

    let teeth_x : Vec<i16> = vec![ 123, 139, 179, 199, 164]; 
    let teeth_y : Vec<i16> = vec![ 644, 637, 631, 648, 656];
    let teeth_color = pixels::Color::RGBA(249, 249, 249, 255);
    let teeth = animshell::Shape { x: teeth_x, y: teeth_y, color: teeth_color};

    let left_brow_x : Vec<i16> = vec![ 43, 93, 150, 141, 49]; 
    let left_brow_y : Vec<i16> = vec![ 332, 314, 325, 342, 349];
    let left_brow_color = pixels::Color::RGBA(191, 148, 0, 255);
    let left_brow = animshell::Shape { x: left_brow_x, y: left_brow_y, color: left_brow_color};

    let right_brow_x : Vec<i16> = vec![ 298, 375, 445, 366, 293]; 
    let right_brow_y : Vec<i16> = vec![ 305, 311, 354, 348, 338];
    let right_brow_color = pixels::Color::RGBA(191, 148, 0, 255);
    let right_brow = animshell::Shape { x: right_brow_x, y: right_brow_y, color: right_brow_color};

    let left_white_x : Vec<i16> = vec![ 85, 113, 145, 141, 85, 72]; 
    let left_white_y : Vec<i16> = vec![ 374, 373, 385, 397, 408, 394];
    let left_white_color = pixels::Color::RGBA(249, 249, 249, 255);
    let left_white = animshell::Shape { x: left_white_x, y: left_white_y, color: left_white_color};

    let right_white_x : Vec<i16> = vec![ 320, 361, 394, 370, 321, 297]; 
    let right_white_y : Vec<i16> = vec![ 372, 378, 403, 406, 407, 393];
    let right_white_color = pixels::Color::RGBA(249, 249, 249, 255);
    let right_white = animshell::Shape { x: right_white_x, y: right_white_y, color: right_white_color};

    let left_iris_x : Vec<i16> = vec![ 92, 109, 121, 104, 84, 79]; 
    let left_iris_y : Vec<i16> = vec![ 369, 371, 388, 402, 401, 382];
    let left_iris_color = pixels::Color::RGBA(100, 157, 97, 255);
    let left_iris = animshell::Shape { x: left_iris_x, y: left_iris_y, color: left_iris_color};

    let right_iris_x : Vec<i16> = vec![ 318, 342, 362, 359, 342, 319, 311]; 
    let right_iris_y : Vec<i16> = vec![ 369, 370, 381, 394, 409, 406, 391];
    let right_iris_color = pixels::Color::RGBA(100, 157, 97, 255);
    let right_iris = animshell::Shape { x: right_iris_x, y: right_iris_y, color: right_iris_color};

    let left_pupil_x : Vec<i16> = vec![ 97, 104, 97, 92]; 
    let left_pupil_y : Vec<i16> = vec![ 374, 387, 396, 387];
    let left_pupil_color = pixels::Color::RGBA(10, 10, 10, 255);
    let left_pupil = animshell::Shape { x: left_pupil_x, y: left_pupil_y, color: left_pupil_color};

    let right_pupil_x : Vec<i16> = vec![ 334, 342, 335, 326]; 
    let right_pupil_y : Vec<i16> = vec![ 377, 389, 397, 386];
    let right_pupil_color = pixels::Color::RGBA(10, 10, 10, 255);
    let right_pupil = animshell::Shape { x: right_pupil_x, y: right_pupil_y, color: right_pupil_color};

    let hair_x : Vec<i16> = vec![ 92, 73, 109, 212, 321, 438, 519, 597, 622, 618, 572, 564, 528, 532, 495, 484, 422, 323, 198, 169, 128]; 
    let hair_y : Vec<i16> = vec![ 203, 174, 98, 29, 13, 42, 102, 215, 319, 476, 456, 521, 531, 455, 336, 226, 174, 144, 159, 141, 163];
    let hair_color = pixels::Color::RGBA(180, 155, 0, 255);
    let hair = animshell::Shape { x: hair_x, y: hair_y, color: hair_color};

	let face_slide_0 = animshell::Slide { shapes: (vec![face, left_nostril, 
		right_nostril, lips, teeth, left_brow, right_brow, left_white, right_white,
		left_iris, right_iris, left_pupil, right_pupil, hair])};
	let face_anim = animshell::Anim { slides: (vec![face_slide_0])};

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
        
        for i in 0..face_anim.slides[0].shapes.len(){
			println!("{}", i);	
		}
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[0].x, 
			&face_anim.slides[0].shapes[0].y, 
			face_anim.slides[0].shapes[0].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[1].x, 
			&face_anim.slides[0].shapes[1].y, 
			face_anim.slides[0].shapes[1].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[2].x, 
			&face_anim.slides[0].shapes[2].y, 
			face_anim.slides[0].shapes[2].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[3].x, 
			&face_anim.slides[0].shapes[3].y, 
			face_anim.slides[0].shapes[3].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[4].x, 
			&face_anim.slides[0].shapes[4].y, 
			face_anim.slides[0].shapes[4].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[5].x, 
			&face_anim.slides[0].shapes[5].y, 
			face_anim.slides[0].shapes[5].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[6].x, 
			&face_anim.slides[0].shapes[6].y, 
			face_anim.slides[0].shapes[6].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[7].x, 
			&face_anim.slides[0].shapes[7].y, 
			face_anim.slides[0].shapes[7].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[8].x, 
			&face_anim.slides[0].shapes[8].y, 
			face_anim.slides[0].shapes[8].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[9].x, 
			&face_anim.slides[0].shapes[9].y, 
			face_anim.slides[0].shapes[9].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[10].x, 
			&face_anim.slides[0].shapes[10].y, 
			face_anim.slides[0].shapes[10].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[11].x, 
			&face_anim.slides[0].shapes[11].y, 
			face_anim.slides[0].shapes[11].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[12].x, 
			&face_anim.slides[0].shapes[12].y, 
			face_anim.slides[0].shapes[12].color);
        let _ = renderer.filled_polygon(
			&face_anim.slides[0].shapes[13].x, 
			&face_anim.slides[0].shapes[13].y, 
			face_anim.slides[0].shapes[13].color);
        
        renderer.present();
    }
}
