extern crate sdl2;
extern crate sdl2_gfx;

pub struct God {
	pub sdl_context: SdlResult<Sdl>,
	pub video_subsys,
	pub window,
	pub renderer,
	pub events
}
