#![deny(clippy::all)]
#![forbid(unsafe_code)]

use anyhow::Result;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

use crate::utils::create_window;

mod lua;
mod utils;

const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 240;

fn main() -> Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) = create_window("Demo effects", &event_loop);

    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);

    let script_name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "lua/plasma.lua".to_owned());
    let mut lua = lua::LuaEffect::new(script_name);
    lua.init()?;
    let mut pixels = Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)?;
    let mut paused = false;

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            if lua
                .draw(pixels.get_frame())
                .map_err(|e| error!("lua.draw() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            };
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if input.key_pressed(VirtualKeyCode::P) {
                paused = !paused;
            }
            if input.key_pressed(VirtualKeyCode::Space) {
                // Space is frame-step, so ensure we're paused
                paused = true;
            }

            // Adjust high DPI factor
            if let Some(factor) = input.scale_factor_changed() {
                _hidpi_factor = factor;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
            if !paused || input.key_pressed(VirtualKeyCode::Space) {
                lua.update().unwrap();
            }
            window.request_redraw();
        }
    });
}
