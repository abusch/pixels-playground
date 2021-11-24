# pixels-playground

A little experiment to play with old-school pixel effects using lua. Gives you a vaguely mode-13h-ish environment: a 320x240 screen, with a 256-color palette.

Uses [`mlua`](https://crates.io/crates/mlua) for lua integration, [`pixels`](https://crates.io/crates/pixels) for rendering. Effects ported over from [this repo](https://seancode.com/demofx/).

## To run:
You need a working Rust environment, then run `cargo run --release -- lua/blobs.lua` (or another script).

## API

Provides a tiny api, vaguely inspired by pico-8:
- `pset(x, y, c)`: sets the pixel at coordinates (x,y) to color index `c`
- `cls(c)`: clear screen with color `c`
- `pal(c, r, g, b)`: set palette index `c` to color (r,g,b)
- more to come...

Lua scripts simply need to define a `Render(t)` function where you can do your rendering. The argument passed in is the current time in milliseconds. You can also define an optional `Init()` function that will be called once at the beginning (and when hot-reloading).

## TODO
- [ ] better error handling
- [x] hot-reload of lua scripts
- [ ] more api

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
