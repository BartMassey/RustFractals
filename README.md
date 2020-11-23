# Rust Fractals
 
A quick
[project](https://github.com/IsaacMarovitz/RustFractals)
Isaac Marovitz made in Rust that generates an animated video
of the Julia Set and a picture of the Julia Set.

Bart Massey removed all dependencies to make a standalone
version, and added thread-parallel rendering. Run with

    cargo run --release 2000 2000 4

to get a 2000×2000 movie using 4 threads.

Movie produced in about 35 minutes on Bart's 12-core box
with 32 threads.
