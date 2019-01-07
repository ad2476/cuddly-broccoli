# Cuddly Broccoli 🥦

OpenGL things in Rust. Aims to provide safe abstractions on top of OpenGL functions, data structures and types.
The goal is to be a starting point for more advanced graphics tasks.

## Features

A running list of what's been implemented so far:

* Window creation, keybindings.
* Low-level buffer operations: Abstractions on VBOs, IBOs, vertex attributes.
* Abstractions on shader compiling and linking, setting uniforms.
* Very basic resource loading system for reading files from disk such as shaders, images, other assets.
* Perspective camera with methods for zooming and orbiting.
* Abstractions on shape primitives. Sphere and cylinder vertex generators which are generic over vertex layout.
* Generic 3D mesh shape with normals.
* Abstractions on OpenGL textures: 2D texture and cubemap targets
* Skybox as cubemapped cube.
* Basic lighting model with ambient and diffuse illumination.

## Documentation

Run `cargo doc` from the project root to generate documentation.

## Build requirements

You will need the latest stable version of Rust, preferably installed via [rustup](https://www.rust-lang.org/tools/install).
Install [SDL2](https://www.libsdl.org/) through your package manager (e.g. `apt-get`, `dnf`, `pacman`, `brew`).

This project is confirmed to work on OS X up to 10.14.1 (Mojave) or Linux. No guarantees for Windows.

A discrete GPU not required, but can be very useful.

## Screenshot

![Textured sphere with skybox](https://arundreli.ch/files/screenshot0.png)

## References

* [Brown CS1230: Introduction to Computer Graphics](http://cs.brown.edu/courses/cs123/)
* [OpenGL in Rust from Scratch](http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-00-setup.html)
* [Learn OpenGL](https://learnopengl.com/)
