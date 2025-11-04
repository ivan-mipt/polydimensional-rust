# Polydimensional Rust
Lightweight library for Rust created to make work with **matrixes** and **vectors** in 2D, 3D and 4D better and faster!
![Downloads](https://img.shields.io/crates/d/polydimensional_rust)

## Using
Just add this to your `Cargo.toml`:
```toml
[dependencies]
polydimensional_rust = "0.1"
```
So, now you can do this in your `main.rs` for example:
```rust
use polydimensional_rust::linear_algebra::{Vector3, Matrix4, Matrix3, Vector2};

fn main() {
    let vector_product = Vector3::new(3.0, 5.0, 6.0)
        .cross_product(&Vector3::new(8.0, 4.0, 2.0)); // library will give you product to your variable
    println!("Result: {:?}", vector_product);
}
```
## Features
- It's so cool for work with **Vulkan** and **OpenGL**
- It's killing waste of time by doing this by your own hands
- 100% Rust code with no `unsafe` functions(you can check it on my repo)
- High performance

## About feedback
Please, when you want to leave feedback, give me a star - I will be very grateful to you!



