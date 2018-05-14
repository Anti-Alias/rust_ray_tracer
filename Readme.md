# Rust Ray Tracer
This is an experimental ray tracer I've written in my spare time.
It will support a variety of primitive shapes (Spheres, boxes, cones).
In the future, it will support complex shapes like meshes.

## To Build
Ensure that Rust with Cargo is installed from [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)
On the command line, navigate to the ray_tracer project and write the following:
```bash
cargo run --release
```

Navigate to ./images and see the generated images.

To produce a video, install ffmpeg and run the following command:
```bash
ffmpeg -i images/frame_%05d.png -vf fps=60 -pix_fmt yuv420p -vcodec mpeg4 -b 50000k animation.mp4
```