# Yet Another Simple Ray Tracer

![The Current Output](https://github.com/mjalen/ray-tracer/blob/main/output.png?raw=true)

This project is a learning exercise with a few objectives:

- Learn the math behind rays, and how to use rays to calculate each pixel color in an image.

- Experiment with the Rust programming language and see how it differs from the C++ approach to programming.

- Have fun! :D

<<<<<<< HEAD
This project is based on the book ['Ray Tracing in One Weekend'](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley. This is my first time reading this book, and it is very helpful and fast. Changes made between this project and the source book are found under 'Project Deviations From Book'.
=======
This project is based on the book ['Ray Tracing in One Weekend'](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley. This is my first time reading this book, and it is very helpful and fast. The changes this project make from 'Ray Tracing in One Weekend' can be seen \[here]\(## Project Deviations From Book).
>>>>>>> 6756faa (Camera is now move-able.)

This project uses the Rust programming language. There are two reasons for this. The first is that I learn quicker by re-writing content in my own words. Translating from C++ to Rust is helpful because it has me THINK about what the code I am copying does. The second reason for using Rust is the hype. I have heard a lot of hype around Rust and was curious how great it really was. Spoilers: Rust is awesome.

## Project Deviations From Book

The source book 'Ray Tracing in One Weekend' implements a ray-tracer in C++. This project is written in Rust, so there are a number of idiomatic changes made in this project:

*   There is no operator overloading. All vector operations have proper implementations. For example, scalar multiplication can be computed using the implemented `Vector3::scalar_mul(self, f32)` function. This is optimal over operator overloading, since it is clear if the goal is scalar multiplication, a dot product, or a cross product.

```rust
let test_vector: Vector3 = Vector3::new(1.0, 1.0, 1.0);
let scalar: f32 = 5.0;

test_vector.scalar_mul(scalar) // results in (5.0, 5.0, 5.0)
```

# To-Do List

*   Add more to 'Project Deviations From Book' section of README.



