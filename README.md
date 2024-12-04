# Rust Raycaster
![Raycaster Demo](https://github.com/MajorTom3K1M/rust-raycaster/blob/main/screenshots/raycaster-1-demo.gif)

A simple raycaster implemented in Rust to explore Rust programming and OpenGL graphics.

## Overview

This project is a basic raycaster written in Rust, utilizing OpenGL for rendering. It's designed as a learning exercise to understand the Rust language and graphics programming with OpenGL.

## Features

- Raycasting rendering technique
- Leverages Rust's performance and safety features
- OpenGL rendering with `glow` and `glutin` crates
- Window creation and event handling using `winit`

## Dependencies

The project uses the following Rust crates:

- [`glow`](https://crates.io/crates/glow) - Safe OpenGL bindings for Rust
- [`glutin`](https://crates.io/crates/glutin) - Cross-platform OpenGL context creation
- [`winit`](https://crates.io/crates/winit) - Window handling library

## Getting Started

### Prerequisites

- Rust programming language (edition 2021)
- Cargo package manager

### Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/MajorTom3K1M/rust-raycaster.git
   ```

2. **Navigate to the project directory**

   ```bash
   cd rust-raycaster
   ```

3. **Build the project**

   ```bash
   cargo build --release
   ```

### Running the Application

Run the application using Cargo:

```bash
cargo run --release
```

## Project Structure

- `src/` - Main source code directory
- `Cargo.toml` - Project configuration and dependencies

## Learning Resources

This project was developed as a learning exercise. Here are some resources that might be helpful:

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Learn OpenGL](https://learnopengl.com/) (adapted for Rust)
- [Ray-Casting For Game Development](https://permadi.com/1996/05/ray-casting-tutorial-table-of-contents/)