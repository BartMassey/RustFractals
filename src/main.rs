use std::f64;
use std::fs;
use std::process::Command;
use std::time::SystemTime;
use std::path::{Path, PathBuf};

mod colour;
mod fractals;
mod ppm;
mod par;

#[derive(Clone)]
struct RenderParams<'a> {
    x_size: usize,
    y_size: usize,
    escape_radius: f64,
    max_iterations: u8,
    x_limits: [f64; 2],
    y_limits: [f64; 2],
    coord: [f64; 2],
    coloring: &'a (dyn Fn(u8)->ppm::Rgb + Sync),
}

fn julia_render(filename: PathBuf, params: RenderParams) {
    let mut img = ppm::PPM::new(params.x_size, params.y_size);
    for y in 0..params.y_size {
        let cy = y as f64 * (params.y_limits[1] - params.y_limits[0]) / params.y_size as f64 + params.y_limits[0];
        for x in 0..params.x_size {
            let cx = x as f64 * (params.x_limits[1] - params.x_limits[0]) / params.x_size as f64 + params.x_limits[0];
            let julia_num: u8 =
                fractals::julia(params.coord, [cx, cy], params.escape_radius, params.max_iterations) as u8;
            img.put_pixel(x, y, (params.coloring)(julia_num));
        }
    }
    img.save(filename.as_path()).expect(&format!("{} failed to save.", filename.as_path().to_string_lossy()));
}

fn main() {
    let mut args = std::env::args();
    let x_size: usize = args.nth(1).unwrap().parse().unwrap();
    let y_size: usize = args.next().unwrap().parse().unwrap();
    let x_limits = [-2.0, 2.0];
    let y_limits = [-2.0, 2.0];
    let escape_radius = 10.0;
    let max_iterations = 255;
    let start_time = SystemTime::now();

    let max: f64 = f64::consts::PI * 2.0;
    let step = 0.01;
    let mut current: f64 = 0.0;

    let _ = fs::remove_dir_all("./imgs");
    fs::create_dir_all("./imgs").unwrap();

    // Render Video
    println!(
        "Rendering a {:?} x {:?} animation of the Julia Set",
        x_size, y_size
    );
    let coloring = & |julia_num| colour::hsl_to_rgb(julia_num as f32 * 15.0 / 255.0 * 360.0, 100.0, 50.0);
    let mut params = RenderParams {
        x_size,
        y_size,
        escape_radius,
        max_iterations,
        x_limits,
        y_limits,
        coord: [0.0, 0.0],
        coloring,
    };
    let mut i = 0;
    let mut par = par::Par::new(4);
    while current < max {
        params.coord = [current.cos(), current.sin()];
        let filename = Path::new(&format!("./imgs/{}.ppm", i)).to_owned();
        let params = params.clone();
        let r = move || julia_render(filename, params);
        par.run(r);
        i += 1;
        current += step;
    }
    drop(par);
    println!("Finished generating frames");
    println!("Beginning video generation");

    match Command::new("ffmpeg")
        .args(&[
            "-framerate",
            "60",
            "-i",
            "./imgs/%d.ppm",
            "-pix_fmt",
            "yuv420p",
            "Julia.mp4",
            "-y",
        ])
        .output()
    {
        Ok(_) => {
            println!("Finished generating video");
            println!(
                "Finished Julia Set in {:.1} seconds",
                start_time.elapsed().unwrap().as_secs_f32()
            );
        }
        Err(_) => {
            println!("Failed to make video! Do you have FFmpeg installed to PATH on your system?");
        }
    }
    // XXX Uncomment this to remove source images at end.
    // Probably want to preserve these in case something went wrong.
    // let _ = fs::remove_dir_all("./imgs");

    // Render Julia Set Image
    println!("Rendering image of the Julia Set");
    let start_time = SystemTime::now();
    let coloring = & |julia_num| [julia_num, julia_num, julia_num];
    let params = RenderParams {
        x_size,
        y_size,
        escape_radius,
        max_iterations,
        x_limits: [-1.5, 1.5],
        y_limits: [-1.5, 1.5],
        coord: [-0.7, 0.27015],
        coloring,
    };
    julia_render(Path::new("Julia.ppm").to_owned(), params);
    println!(
        "Finished Julia Set in {:.1} seconds",
        start_time.elapsed().unwrap().as_secs_f32()
    );
}
