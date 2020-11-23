use image::{RgbImage, Rgb};
use std::time::SystemTime;
use std::io;
use std::io::Write;
use std::f64;
use std::fs;
use std::process::Command;

mod colour;
mod fractals;

fn input(message: &str, failure_message: &str) -> u32 {
    let mut input_received = false;
    let mut return_int = 0;

    while !input_received {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input!");
        input = input.trim().to_string();
        match input.parse::<u32>() {
            Ok(n) => {
                return_int = n; 
                input_received = true;
            },
            Err(_) => {
                println!("{}", failure_message);
            },
        }
    }
    return_int
}

fn main() {
    let x_size = input("Input x size: ", "Only input intergers!");
    let y_size = input("Input y size: ", "Only input intergers!");
    let x_limits: [f64; 2] = [-2.0, 2.0];
    let y_limits: [f64; 2] = [-2.0, 2.0];
    let escape_radius = 10;
    let max_iterations = 255;
    let mut img = RgbImage::new(x_size, y_size);
    let start_time = SystemTime::now();

    let max: f64 = f64::consts::PI * 2 as f64;
    let step = 0.01;
    let mut current: f64 = 0.0;
    let mut i: u32 = 0;

    match fs::remove_dir_all("./imgs") {
        Ok(_) => {},
        Err(_) => {},
    }

    match fs::create_dir_all("./imgs") {
        Ok(_) => {},
        Err(msg) => {
            panic!(msg);
        },
    }
    
    // Render Video
    println!("Rendering a {:?} x {:?} animation of the Julia Set", x_size, y_size);
    while current < max {
        for y in 0..y_size {
            let cy = y as f64 * (y_limits[1] - y_limits[0]) / y_size as f64 + y_limits[0];
            for x in 0..x_size {
                let cx = x as f64 * (x_limits[1] - x_limits[0]) / x_size as f64 + x_limits[0];
                let julia_num: u32 = fractals::julia([current.cos(), current.sin()], [cx, cy], escape_radius, max_iterations);
                img.put_pixel(x, y, Rgb(colour::hsl_to_rgb((julia_num as f32*15.0/255.0*360.0) as u32, 100.0, 50.0)));
            }
        }
        img.save("./imgs/".to_owned() + &i.to_string() + ".png").expect("Image failed to save.");
        i += 1;
        current = current + step;
    }
    println!("Finished generating frames");
    println!("Beginning video generation");

    match Command::new("ffmpeg")
            .args(&["-framerate", "60", "-i", "./imgs/%d.png", "-pix_fmt", "yuv420p", "Julia.mp4", "-y"])
            .output() {
        Ok(_) => {
            println!("Finished generating video");
            println!("Finished Julia Set in {:.1} seconds", start_time.elapsed().unwrap().as_secs_f32());
        },
        Err(_) => {
            println!("Failed to make video! Do you have FFmpeg installed to PATH on your system?");
        },
    } 
    match fs::remove_dir_all("./imgs") {
        Ok(_) => {},
        Err(_) => {},
    }

    // Render Julia Set Image
    println!("Rendering image of the Julia Set");
    let start_time = SystemTime::now();
    let mut img = RgbImage::new(x_size, y_size);
    let x_limits: [f64; 2] = [-1.5, 1.5];
    let y_limits: [f64; 2] = [-1.5, 1.5];
    
    for y in 0..y_size {
        let cy = y as f64 * (y_limits[1] - y_limits[0]) / y_size as f64 + y_limits[0];
        for x in 0..x_size {
            let cx = x as f64 * (x_limits[1] - x_limits[0]) / x_size as f64 + x_limits[0];
            let julia_num: u8 = fractals::julia([-0.7, 0.27015], [cx, cy], escape_radius, max_iterations) as u8;
            img.put_pixel(x, y, Rgb([julia_num, julia_num, julia_num]));
        }
    }
    img.save("Julia.png").expect("Image failed to save.");
    println!("Finished Julia Set in {:.1} seconds", start_time.elapsed().unwrap().as_secs_f32());
}
