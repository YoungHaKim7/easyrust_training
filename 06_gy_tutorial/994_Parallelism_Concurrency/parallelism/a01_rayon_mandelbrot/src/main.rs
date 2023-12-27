use bin::mandelbrot::{compute_iterations, save_image};

mod bin;

fn main() {
    let width: u32 = 900;
    let height: u32 = 900;

    let xa: f32 = -2.0;
    let xb: f32 = 1.0;
    let ya: f32 = -1.5;
    let yb: f32 = 1.5;
    let max_iterations = 256;

    let nb_iterations = compute_iterations(width, height, xa, xb, ya, yb, max_iterations);
    save_image(
        &nb_iterations,
        width,
        height,
        max_iterations,
        "mandelbrot.png",
    );
}
