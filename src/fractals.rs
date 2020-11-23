pub fn julia(c: [f64; 2], z: [f64; 2], escape_radius: f64, max_iterations: u8) -> u8 {
    let mut iterations = 1u8;
    let mut zx: f64 = z[0];
    let mut zy: f64 = z[1];

    while zx * zx + zy * zy < escape_radius * escape_radius && iterations < max_iterations {
        let xtemp = zx * zx - zy * zy;
        zy = 2.0 as f64 * zx * zy + c[1];
        zx = xtemp as f64 + c[0];

        iterations += 1;
    }

    iterations
}
