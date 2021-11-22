#![deny(clippy::all)]
#![forbid(unsafe_code)]


use anyhow::Result;
use log::error;
use pixels::{ Pixels, SurfaceTexture};
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

    let mut lua = lua::LuaEffect::new();
    lua.init()?;
    // let mut plasma = Plasma::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);
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

/* #[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rgb(u8, u8, u8);

#[derive(Clone, Debug)]
struct Plasma {
    width: usize,
    height: usize,
    screen: Box<[u8]>,
    palette: Box<[Rgb]>,
    frame_count: u64,
}

impl Plasma {
    fn new(width: usize, height: usize) -> Self {
        assert!(width != 0 && height != 0);
        let palette: Vec<Rgb> = (0..=255)
            .map(|c| {
                let i = c % 16;
                let j = c / 16;
                Rgb(i * 16, 0u8, j * 16)
            })
            .collect();

        Self {
            width,
            height,
            screen: vec![0u8; width * height].into_boxed_slice(),
            palette: palette.into_boxed_slice(),
            frame_count: 0,
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;

        let time = self.frame_count as f32 / 10.0;
        for (i, pix) in self.screen.iter_mut().enumerate() {
            let x = i % self.width;
            let y = i / self.width;
            let dx = (x as f32 / self.width as f32) - 0.5;
            let dy = (y as f32 / self.height as f32) - 0.5;

            let mut v = f32::sin(dx * 10.0 + time);
            let cx = dx + 0.5 * f32::sin(time / 5.0);
            let cy = dy + 0.5 * f32::cos(time / 3.0);
            v += f32::sin(f32::sqrt(50.0 * (cx * cx + cy * cy) + 1.0 + time));
            v += f32::cos(f32::sqrt(dx * dx + dy * dy) - time);

            let r = ((v * PI).sin() * 15.0).floor() as u8;
            let b = ((v * PI).cos() * 15.0).floor() as u8;

            let color = r * 16 + b;
            *pix = color;
        }
    }

    fn draw(&self, screen: &mut [u8]) {
        for (c, pix) in self.screen.iter().zip(screen.chunks_exact_mut(4)) {
            let Rgb(r, g, b) = self.palette[*c as usize];
            let color = [r, g, b, 255];
            pix.copy_from_slice(&color);
        }
    }
} */
