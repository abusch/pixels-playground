use std::path::Path;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use anyhow::Result;
use lazy_static::lazy_static;
use mlua::prelude::*;
use mlua::Function;
use parking_lot::Mutex;

use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;

lazy_static! {
    static ref SCREEN: Mutex<Screen> = Mutex::new(Screen::new(SCREEN_WIDTH, SCREEN_HEIGHT));
}

pub struct LuaEffect {
    lua: Lua,
    frame_count: u64,
    script_path: PathBuf,
}

impl LuaEffect {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let lua = Lua::new();
        LuaEffect {
            lua,
            frame_count: 0,
            script_path: path.as_ref().to_path_buf(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        // Set up Lua environment
        let globals = self.lua.globals();
        globals.set("W", SCREEN_WIDTH)?;
        globals.set("H", SCREEN_HEIGHT)?;
        globals.set("time", self.frame_count)?;

        // put API functions into global context
        // cls
        let cls = self.lua.create_function(|_, c: u8| {
            SCREEN.lock().cls(c);
            Ok(())
        })?;
        globals.set("cls", cls)?;
        // pset
        let pset = self
            .lua
            .create_function(|_, (x, y, c): (usize, usize, u8)| {
                SCREEN.lock().pset(x, y, c);
                Ok(())
            })?;
        globals.set("pset", pset)?;
        // pal
        let pal = self
            .lua
            .create_function(|_, (i, r, g, b): (u8, u8, u8, u8)| {
                SCREEN.lock().pal(i, Rgb(r, g, b));
                Ok(())
            })?;
        globals.set("pal", pal)?;

        // Load and execute script
        let script = std::fs::read_to_string(&self.script_path)?;
        self.lua.load(&script).exec()?;

        // Run init function, if there is one
        let init_func: Option<Function> = self.lua.globals().get("_Init")?;
        if let Some(f) = init_func {
            println!("Calling _Init() function...");
            f.call(())?;
        } else {
            println!("No _Init() function to call...");
        }

        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        let globals = self.lua.globals();
        self.frame_count += 1;
        globals.set("time", self.frame_count)?;
        let time = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!")
            .as_millis();

        // Run effect's update function
        let update: Option<Function> = globals.get("_Update")?;
        if let Some(f) = update {
            f.call::<_, ()>(time)?;
        }

        Ok(())
    }

    pub fn draw(&self, pixels: &mut [u8]) -> Result<()> {
        let screen = SCREEN.lock();
        screen.draw(pixels);
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Rgb(u8, u8, u8);

#[derive(Clone, Debug)]
struct Screen {
    width: usize,
    height: usize,
    screen: Box<[u8]>,
    palette: Box<[Rgb]>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        assert!(width != 0 && height != 0);

        Self {
            width,
            height,
            screen: vec![0u8; width * height].into_boxed_slice(),
            palette: vec![Rgb::default(); 256].into_boxed_slice(),
        }
    }

    fn draw(&self, screen: &mut [u8]) {
        for (c, pix) in self.screen.iter().zip(screen.chunks_exact_mut(4)) {
            let Rgb(r, g, b) = self.palette[*c as usize];
            let color = [r, g, b, 255];
            pix.copy_from_slice(&color);
        }
    }

    fn cls(&mut self, col: u8) {
        self.screen.iter_mut().for_each(|p| *p = col);
    }

    pub fn pset(&mut self, x: usize, y: usize, c: u8) {
        self.screen[x + self.width * y] = c;
    }

    pub fn pal(&mut self, idx: u8, c: Rgb) {
        self.palette[idx as usize] = c;
    }
}
