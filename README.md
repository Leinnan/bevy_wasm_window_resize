# Bevy WASM window resize

[![crates.io](https://img.shields.io/crates/v/bevy_wasm_window_resize.svg)](https://crates.io/crates/bevy_wasm_window_resize)
[![license](https://img.shields.io/crates/l/bevy_wasm_window_resize)](https://github.com/Leinnan/bevy_wasm_window_resize#license)
[![crates.io](https://img.shields.io/crates/d/bevy_wasm_window_resize.svg)](https://crates.io/crates/bevy_wasm_window_resize)

This crate makes [Bevy](https://github.com/bevyengine/bevy) application canvas match window size.

Original idea and code behind: [ManevilleF](https://github.com/ManevilleF)

# Install

```
cargo add bevy_wasm_window_resize

```

# Usage

Just add `WindowResizePlugin` during app creation process.

```rust
use bevy::prelude::*;
use bevy_wasm_window_resize::WindowResizePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WindowResizePlugin)
        .run();
}
```

# Bevy compatibility table
Bevy version | crate version
--- | ---
0.14 | 0.4
0.13 | 0.3
0.12 | 0.2
0.10 | 0.1
