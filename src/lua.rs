use anyhow::Result;
use rlua::prelude::*;
use rlua::Function;

use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;

/* #[derive(Error, Debug)]
pub enum LuaEffectError {
    #[error("Could not acquire screen lock")]
    ScreenLock,
} */

pub struct LuaEffect {
    lua: Lua,
    frame_count: u64,
}

impl LuaEffect {
    pub fn new() -> Self {
        LuaEffect {
            lua: Lua::new(),
            frame_count: 0,
        }
    }

    pub fn init(&mut self) -> Result<()> {
        self.lua.context(|ctx| {
            let globals = ctx.globals();
            let screen = Screen::new(SCREEN_WIDTH, SCREEN_HEIGHT);
            globals.set("screen", screen)?;
            globals.set("W", SCREEN_WIDTH)?;
            globals.set("H", SCREEN_HEIGHT)?;
            globals.set("time", self.frame_count)?;

            let script = std::fs::read_to_string("lua/plasma.lua")?;
            ctx.load(&script).exec()?;

            Ok(())
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.lua.context(|ctx| {
            let globals = ctx.globals();
            // update screen
            /* let mut screen: Screen = globals.get("screen")?;
            screen.update();
            globals.set("screen", screen)?; */
            self.frame_count += 1;
            globals.set("time", self.frame_count)?;

            // Run effect's update function
            let update: Function = globals.get("_Update")?;

            // TODO update state (inputs, time, etc...)
            update.call::<_, ()>(())?;

            Ok(())
        })
    }

    pub fn draw(&self, pixels: &mut [u8]) -> Result<()> {
        self.lua.context(|ctx| {
            let screen: Screen = ctx.globals().get("screen")?;
            screen.draw(pixels);
            Ok(())
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rgb(u8, u8, u8);

#[derive(Clone, Debug)]
struct Screen {
    width: usize,
    height: usize,
    screen: Box<[u8]>,
    palette: Box<[Rgb]>,
    frame_count: u64,
}

impl Screen {
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
}

impl LuaUserData for Screen {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("t", |_, this, _: ()| {
            Ok(this.frame_count)
        });

        methods.add_method_mut("cls", |_, this, col: u8| {
            this.cls(col);
            Ok(())
        });

        methods.add_meta_method(LuaMetaMethod::Index, |_, this, idx: usize| {
            Ok(this.screen[idx])
        });

        methods.add_meta_method_mut(LuaMetaMethod::NewIndex, |_, this, (idx, value): (usize, u8)| {
            this.screen[idx] = value;
            Ok(())
        });
    }
}
