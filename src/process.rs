use std::f64::consts::{ PI };

pub fn principal_argument(phase_in: f64) -> f64 {
    let a = phase_in / (2.0 * PI);
    let k = a.round();
    let phase_out = phase_in - k * (2.0 * PI);

    phase_out
}

pub fn hanning_window(n: usize) -> Vec<f64> {
    let mut window = vec![0.0; n];

    let two_pi = 2.0 * PI;

    for i in 0..n {
        window[i] = 0.5 - 0.5 * (two_pi * i as f64 / n as f64).cos()
    }

    window
}

fn interpolation(fft_size: &usize, interpolate_length: &usize, synthesized_buffer: &Vec<f64>, ratio: &f64) -> Vec<f64> {
    let factor = 1.0 / ratio;
    let mut x1 = 0.0;
    let mut buffer = vec![0.0; *interpolate_length];

    for i in 0..*fft_size {
        if i + 1 >= *fft_size {
            break;
        }
        let y1 = synthesized_buffer[i];
        let x2 = x1 + factor;
        let y2 = synthesized_buffer[i + 1];
        for j in 0..(factor.floor() + 1.0) as usize {
            let xt = x1 + j as f64;
            let yt = (y2 - y1) / (x2 - x1) * (xt - x1) + y1;
            if xt < *interpolate_length as f64 {
                buffer[xt as usize] = yt;
            }
        }
        x1 = x2;
    }

    buffer
}
