# snake

A terminal Snake game written in Rust.

## Controls

| Key | Action |
|---|---|
| `↑` `↓` `←` `→` or `W` `A` `S` `D` | Turn |
| `1` `2` `3` | Choose difficulty in the menu |
| `R` | Restart |
| `M` | Back to menu |
| `Q`, `Esc`, `Ctrl+C` | Quit |

## Building and running

Requires [Rust](https://rustup.rs).

```sh
git clone https://github.com/eoftgge/termsnake
cd termsnake
cargo run --release
```

On Windows, use Windows Terminal: the legacy console (`cmd.exe`) may render emoji incorrectly.

## Project structure

```
src/
├── main.rs            game loop, input, states
├── render.rs          terminal output
├── game.rs            game logic
└── game/
    ├── snake.rs       the snake and its movement
    ├── difficulty.rs  speed per difficulty level
    ├── ground.rs      frame buffer
    └── pixel.rs       cell rendering
```

## Dependencies

- [crossterm](https://crates.io/crates/crossterm): terminal handling
- [rand](https://crates.io/crates/rand): apple placement
- [windows-sys](https://crates.io/crates/windows-sys): clean shutdown when the window is closed on Windows

## License

Nothing