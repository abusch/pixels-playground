#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::f32::consts::PI;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 240;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) = create_window("Demo effects", &event_loop);

    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);

    let mut plasma = Plasma::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);
    let mut pixels = Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)?;
    let mut paused = false;

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            plasma.draw(pixels.get_frame());
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
                plasma.update();
            }
            window.request_redraw();
        }
    });
}

// COPYPASTE: ideally this could be shared.

/// Create a window for the game.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
///
/// # Returns
///
/// Tuple of `(window, surface, width, height, hidpi_factor)`
/// `width` and `height` are in `PhysicalSize` units.
fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = SCREEN_WIDTH as f64;
    let height = SCREEN_HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        if let Some(monitor) = window.current_monitor() {
            let size = monitor.size().to_logical(hidpi_factor);
            (size.width, size.height)
        } else {
            (width, height)
        }
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round().max(1.0);

    // Resize, center, and display the window
    let min_size: winit::dpi::LogicalSize<f64> =
        PhysicalSize::new(width, height).to_logical(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
}
