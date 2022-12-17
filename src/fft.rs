use std::f64::consts::PI;

fn pow2(x: usize) -> usize {
    let y = if x == 0 { 1 } else { 2 << (x - 1) };
    y
}

pub fn fft(x_real: &mut Vec<f64>, x_imag: &mut Vec<f64>, sample: usize, inverse: bool) {
    let mut n = 0;
    let mut m = 0;
    let mut r = 0;
    let number_of_stage = (sample as f64).log2() as usize;
    let mut a_real = 0.0;
    let mut a_imag = 0.0;
    let mut b_real = 0.0;
    let mut b_imag = 0.0;
    let mut c_real = 0.0;
    let mut c_imag = 0.0;
    let mut real = 0.0;
    let mut imag = 0.0;

    let two_pi = 2.0 * PI;

    for stage in 1..(number_of_stage + 1) {
        for i in 0..pow2(stage - 1) {
            for j in 0..pow2(number_of_stage - stage) {
                n = pow2(number_of_stage - stage + 1) * i + j;
                m = pow2(number_of_stage - stage) + n;
                r = pow2(stage - 1) * j;
                a_real = x_real[n];
                a_imag = x_imag[n];
                b_real = x_real[m];
                b_imag = x_imag[m];
                c_real = ((two_pi * r as f64) / sample as f64).cos();
                c_imag = if inverse {
                    ((two_pi * r as f64) / sample as f64).sin()
                } else {
                    ((two_pi * r as f64) / sample as f64).sin() * -1.0
                };
                if stage < number_of_stage {
                    x_real[n] = a_real + b_real;
                    x_imag[n] = a_imag + b_imag;
                    x_real[m] = (a_real - b_real) * c_real - (a_imag - b_imag) * c_imag;
                    x_imag[m] = (a_imag - b_imag) * c_real + (a_real - b_real) * c_imag;
                } else {
                    x_real[n] = a_real + b_real;
                    x_imag[n] = a_imag + b_imag;
                    x_real[m] = a_real - b_real;
                    x_imag[m] = a_imag - b_imag;
                }
            }
        }
    }

    let mut index = vec![0; sample];
    for stage in 1..(number_of_stage + 1) {
        for i in 0..pow2(stage - 1) {
            index[pow2(stage - 1) + i] = index[i] + pow2(number_of_stage - stage);
        }
    }

    for k in 0..sample {
        if index[k] > k {
            real = x_real[index[k]];
            imag = x_imag[index[k]];
            x_real[index[k]] = x_real[k];
            x_imag[index[k]] = x_imag[k];
            x_real[k] = real;
            x_imag[k] = imag;
        }
    }

    if inverse {
        for k in 0..sample {
            x_real[k] /= sample as f64;
            x_imag[k] /= sample as f64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fft_test() {
        let sample = 16;
        let REAL: Vec<f64> = vec![
            0.0, 0.5, -0.2, 0.88, -0.025, 0.0, -0.33456, 0.1, 0.023, 0.92, -0.132, 0.03, 0.5223,
            0.056, -0.618234, 0.2294,
        ];
        let mut real = REAL.clone();
        let mut image: Vec<f64> = vec![0.0; real.len()];

        // FFT
        fft(&mut real, &mut image, sample, false);
        // IFFT
        fft(&mut real, &mut image, sample, true);

        println!("{:?}", real);
    }
}
