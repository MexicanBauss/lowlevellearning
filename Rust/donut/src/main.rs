use std::{f64::consts::PI, time::Duration};
use termion::terminal_size;

const THETA: f64 = 0.07;
const PHI: f64 = 0.02;

const R1: f64 = 1.0;
const R2: f64 = 2.0;
const K2: f64 = 5.0;

fn calculate_k1(screen_width: f64) -> f64 {
    screen_width * K2 * 3.0 / (8.0 * (R1 + R2))
}

fn render_frame(a: f64, b: f64) {
    let (width, height) = terminal_size().unwrap();
    let screen_width = width as f64 * (1.0/3.0);
    let screen_height = width as f64 * (1.0/3.0);
    let k1: f64 = calculate_k1(screen_width);

    let (sin_a, cos_a) = a.sin_cos();
    let (sin_b, cos_b) = b.sin_cos();

    let mut z_buffer = vec![vec![0.0; width as usize]; height as usize];
    let mut output_buffer = vec![vec![' '; width as usize]; height as usize];

    let mut theta = 0.0;

    while theta < 2.0 * PI {
        let (sintheta, costheta) = theta.sin_cos();
        let mut phi = 0.0;
        while phi < 2.0 * PI {
            let (sinphi, cosphi) = phi.sin_cos();

            let circle_x = R2 + R1 * costheta;
            let circle_y = R1 * sintheta;

            let x = circle_x * (cos_b * cosphi + sin_a * sin_b * sinphi) - circle_y * cos_a * sin_b;
            let y = circle_x * (sin_b * cosphi - sin_a * cos_b * sinphi) + circle_y * cos_a * cos_b;
            let z = K2 + cos_a * circle_x * sinphi + circle_y * sin_a;
            let ooz = 1.0 / z;

            let xp = (screen_width / 2.0 + k1 * ooz * x) as usize;
            let yp = (screen_height / 2.0 - k1 * ooz * y) as usize;

            let l = cosphi * costheta * sin_b - cos_a * costheta * sinphi - sin_a * sintheta
                + cos_b * (cos_a * sintheta - costheta * sin_a * sinphi);

            if l > 0.0 {
                if xp < width as usize && yp < height as usize && ooz > z_buffer[yp][xp] {
                    z_buffer[yp][xp] = ooz;
                    let luminance_idx = (l * 8.0) as usize;
                    output_buffer[yp][xp] = ".,-~:;=!*#$@".chars().nth(luminance_idx).unwrap();
                }
            }

            phi += PHI;
        }
        theta += THETA;
    }

    // Now, dump output_buffer to the screen.
    // Bring cursor to "home" location.
    print!("\x1b[H");
    for row in output_buffer {
        let row_string: String = row.into_iter().collect();
        println!("{}", row_string);
    }
}

fn main() {
    let (mut a, mut b) = (0.0, 0.0);
    loop {
        render_frame(a, b);
        a+=0.01;
        b+=0.01;
        std::thread::sleep(Duration::from_micros(900));
    }
}
