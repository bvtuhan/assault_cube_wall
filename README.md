# Assault Cube Wall

- Simple ESP (not the best) written in Rust for Assault Cube. This project helped me to understand how low-level system fundamentals work (such as reading/writing memory and putting pixels on screen) in Rust. It could maybe guide you for your project as well.

![illustration](./screenshots/1.PNG)

## Compiling the program 

- You have to compile it for the 32-bit architecture. For MSCV toolchain:

```bash
rustup target add i686-pc-windows-msvc
cargo build --target=i686-pc-windows-msvc --release # or just cargo run
```

- If you are using GNU toolchain:

```bash
rustup target add i686-pc-windows-gnu
cargo build --target=i686-pc-windows-gnu --release
```

## Configuration 

- Positioning of the red boxes is not the finest, but configurable based on your preference:
 
```rust
// main.rs
let x = screen.x; // do not touch
let y = screen.y; // do not touch

let left = x as i32 - 24 / 2;
let right = x as i32 + 24 / 2;
let top = y as i32 - 16 / 2;
let bottom = y as i32 + 128;
let color = [255, 0, 0, 255]; // Red RGBA

rect(frame, *SCREEN_WIDTH as i32, left, top, right, bottom, &color);
```

## Sources 

- [cs2-dumper](https://github.com/a2x/cs2-dumper)
- [Unknowncheats Post 1](https://www.unknowncheats.me/forum/3712980-post3.html)
- [Reddit Post 1](https://www.reddit.com/r/rust/comments/1cj5ppa/what_would_be_the_simplest_way_to_simply_put)
- [Reddit Post 2](https://stackoverflow.com/questions/75630785/how-can-i-make-a-click-through-overlay-in-rust-that-still-captures-input)
- [Pixels Minimal Winit Example](https://github.com/parasyte/pixels/blob/main/examples/minimal-winit/src/main.rs)
- [Memflow Documentation](https://docs.rs/memflow/latest/memflow/mem/memory_view/trait.MemoryView.html#)