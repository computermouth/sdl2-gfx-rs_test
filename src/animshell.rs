extern crate sdl2;

use sdl2::pixels;


pub struct Anim {
	pub anim: Vec<Slide>
}

pub struct Slide {
	pub shapes: Vec<Shape>
}

pub struct Shape {
	pub x: Vec<i16>,
	pub y: Vec<i16>,
	pub color: pixels::Color
}
