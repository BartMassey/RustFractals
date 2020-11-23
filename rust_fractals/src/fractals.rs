pub fn julia(c: [f64; 2], z: [f64; 2], escape_radius: u32, max_iterations: usize) -> usize {
    let mut iterations: usize = 1;
    let mut zx: f64 = z[0];
    let mut zy: f64 = z[1];

    while zx * zx + zy * zy < (escape_radius * escape_radius) as f64 && iterations < max_iterations
    {
        let xtemp = zx * zx - zy * zy;
        zy = 2.0 as f64 * zx * zy + c[1];
        zx = xtemp as f64 + c[0];

        iterations += 1;
    }

    return iterations;
}
