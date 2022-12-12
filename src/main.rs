pub mod normalize;
pub mod process;
pub mod fft;
pub mod read;
pub mod write;
pub mod heap;
pub mod command;

use std::f64::consts::{ PI };
use std::collections::BinaryHeap;
use clap::Parser;
use rand::Rng;

use read::{ wav_read, WaveResult };
use process::{ hanning_window, principal_argument, interpolation };
use fft::{ fft };
use write::{ wav_write };
use heap::MaxHeap;
use command::{ Args };

fn main() -> WaveResult<()> {
	// get settings from cli
    let args = Args::parse();

	let mode = args.mode;
	let ratio = args.ratio;
	let input_path = args.i.unwrap_or("./Hyper Bass (feat. Yunomi).wav".to_string());
	let output_path = args.o.unwrap_or("./output.wav".to_string());
	let buffer = args.buffer.unwrap_or(4096);
	let wave_size_ratio = if mode == command::Mode::TimeStretch {
		ratio
	} else {
		1.0
	};

    let source = wav_read(&input_path)?;

    let fs = source.sample_rate;
    let bit = source.bits_per_sample;
	let channels = 1;
    let input: Vec<f64> = source.normalized_sample_data;
    let input_len = input.len();

    let size = ((fs * (bit / 8) * channels * (input_len / fs)) as f64 * wave_size_ratio) as usize;

    let frame_size = buffer;
    let fft_size = 2 * frame_size;
    let synthesis_hopsize = frame_size as f64 / 4.0;
    let analysis_hopsize = (synthesis_hopsize / ratio).round();
    let analysis_frequency_step = input_len as f64 / fft_size as f64;
    let scalling_factor = synthesis_hopsize / synthesis_hopsize / ratio;
    let synthesis_frequency_step = (scalling_factor * analysis_frequency_step).round();
    let number_of_frame = input_len / analysis_hopsize as usize;

    let mut result_buffer: Vec<f64> = vec![0.0; (input_len as f64 * wave_size_ratio) as usize];
    let mut x_real: Vec<f64> = vec![0.0; fft_size];
    let mut x_imag: Vec<f64> = vec![0.0; fft_size];
    let mut y_real: Vec<f64> = vec![0.0; fft_size];
    let mut y_imag: Vec<f64> = vec![0.0; fft_size];

    let mut magnitude: Vec<Vec<f64>> = vec![vec![0.0; fft_size]; number_of_frame];
    let mut phase: Vec<Vec<f64>> = vec![vec![0.0; fft_size]; number_of_frame];
    let mut alter_phase: Vec<Vec<f64>> = vec![vec![0.0; fft_size]; number_of_frame];
    let omega: Vec<f64> = vec![0.0; fft_size].iter().enumerate().map(|(i, _)| ((2.0 * PI) * analysis_hopsize * i as f64) / fft_size as f64).collect();
    let mut time_delta_phi: Vec<Vec<f64>> = vec![vec![0.0; fft_size]; number_of_frame];
    let mut frequency_delta_phi: Vec<Vec<f64>> = vec![vec![0.0; fft_size]; number_of_frame];
    let mut frequency_forward_delta_phi: Vec<Vec<f64>> = vec![vec![0.0; fft_size]; number_of_frame];
    let mut frequency_backward_delta_phi = vec![vec![0.0; fft_size]; number_of_frame];

    let analysis_window = hanning_window(frame_size);

	let relative_tolerance = 10.0_f64.powi(-6);
	let mut max_heap: BinaryHeap<MaxHeap> = BinaryHeap::new();
	let mut rng = rand::thread_rng();

    let mut offset = 0;
    let mut alter_offset = 0;

    for i in 0..number_of_frame {

        offset = analysis_hopsize as usize * i;

        // Zero padding
        x_real.fill(0.0);
        x_imag.fill(0.0);
        // Windowning real signal
        for j in 0..frame_size {
            if offset + j >= input_len {
                break;
            } else {
                x_real[j] = input[offset + j] * analysis_window[j];
            }
        }
        // Shift real signal to center
        x_real.rotate_right(frame_size);
        // FFT
        fft(&mut x_real, &mut x_imag, fft_size, false);

        // In its essence, the method proceeds by pro-cessing one frame at a time computing the synthesis phase of the current n-th frame φs(·,n).
        // It requires storing the already computed phase φs(·,n −1) and the time derivative (∆tφa) (·,n−1) of the previous (n−1)-th frame and further,
        // it requires access to the coefficients of the previous, current and one "future" frame (c(·,n−1), c(·,n) and c(·,n+1)) assuming the centered differentiation scheme
        for j in 0..fft_size {
            magnitude[i][j] = (x_real[j] * x_real[j] + x_imag[j] * x_imag[j]).sqrt();
            phase[i][j] = x_imag[j].atan2(x_real[j]);
        }
    }

    for i in 0..number_of_frame {

		// Determine the ratio that time-stretch needs synthesis hop size overlapping.
		alter_offset = if mode == command::Mode::TimeStretch {
			synthesis_hopsize as usize * i
		} else {
			analysis_hopsize as usize * i
		};

        // (∆tφa) (m,n) and (∆fφa) (m,n) are computed for all m and current n
        for j in 0..fft_size {

            // It cannot calculate center value, if the bin or vector of the bin placed in the edge of buffer,
            // So it should pick the very value.

            time_delta_phi[i][j] = if
            i as isize - 2 <= 0
            || i + 1 >= number_of_frame {
                synthesis_hopsize *
                ( ( (1.0 / analysis_hopsize) * principal_argument(phase[i][j] - omega[j]) + ((2.0 * PI * j as f64) / fft_size as f64) )
                )
            } else {
                synthesis_hopsize / 2.0 *
                ( ( 1.0 / analysis_hopsize * principal_argument(phase[i - 1][j] - phase[i - 2][j] - omega[j]) + ((2.0 * PI * j as f64) / fft_size as f64) )
                + ( 1.0 / analysis_hopsize * principal_argument(phase[i + 1][j] - phase[i][j] - omega[j]) + ((2.0 * PI * j as f64) / fft_size as f64) )
                )
            };

            frequency_delta_phi[i][j] = if
            j + 1 >= fft_size
            || j as isize - 1 < 0 {
                synthesis_frequency_step *
                ( 1.0 / analysis_frequency_step * principal_argument(phase[i][j])
                )
            } else {
                synthesis_frequency_step / 2.0 *
                ( 1.0 / analysis_frequency_step * principal_argument(phase[i][j] - phase[i][j - 1])
                + 1.0 / analysis_frequency_step * principal_argument(phase[i][j + 1] - phase[i][j])
                )
            };

            frequency_forward_delta_phi[i][j] = if
            j + 2 >= fft_size
            || j as isize - 1 < 0 {
                frequency_delta_phi[i][j]
            } else {
                synthesis_frequency_step / 2.0 *
                ( 1.0 / analysis_frequency_step * principal_argument(phase[i][j] - phase[i][j - 1])
                + 1.0 / analysis_frequency_step * principal_argument(phase[i][j + 2] - phase[i][j + 1])
                )
            };

            frequency_backward_delta_phi[i][j] = if
            j + 1 >= fft_size
            || j as isize - 2 < 0 {
                frequency_delta_phi[i][j]
            } else {
                synthesis_frequency_step / 2.0 *
                ( 1.0 / analysis_frequency_step * principal_argument(phase[i][j - 1] - phase[i][j - 2])
                + 1.0 / analysis_frequency_step * principal_argument(phase[i][j + 1] - phase[i][j])
                )
            };
        }


		// Start calculate phase gradiation.
		(|| -> () {
			// Return current frame's phase due to there're no the last two frame information until it's third frame.
			if i as isize - 1 <= 0 {
				for j in 0..fft_size {
					alter_phase[i][j] = time_delta_phi[i][j];
				}
				return;
			}

			// Preprocessing for heap sort.

			// abstol ← tol·max(s(m,n) ∪ s(m,n - 1))
			let absolute_tolerance = relative_tolerance * f64::max(
										magnitude[i].clone().into_iter().fold(0.0/0.0, f64::max),
										magnitude[i - 1].clone().into_iter().fold(0.0/0.0, f64::max)
										);
			// set I = { m: s(m,n) > abstol }
			let mut frequency_indices: Vec<usize> = magnitude[i]
														.iter().enumerate()
														.filter(|(_, &x)| x > *&absolute_tolerance)
														.map(|(i, _) | i).collect();
			// Assign random values to φs(m,n) for m ∉ I
			let phase_advance: Vec<usize> = phase[i]
											.iter().enumerate()
											.map(|(i, _)| i).collect();
			let difference: Vec<usize> = phase_advance
										.into_iter()
										.filter(|bin | !frequency_indices.contains(bin)).collect();
			difference.iter().for_each(| j | alter_phase[i][*j] = rng.gen());

			// Construct a self-sorting max heap for (m,n) tuples
			// Insert (m,n - 1) for m ∈ I into the heap
			frequency_indices.iter()
							.for_each(|j|
								max_heap.push(MaxHeap { magnitude: magnitude[i - 1][*j], frequency_index: *j, frame: i - 1 } )
							);

			while !frequency_indices.is_empty() {
				while !max_heap.is_empty() {
					let max = max_heap.pop().unwrap();
					let frequency_index = max.frequency_index;

					// propagate the phase in the time direction
					if max.frame == i - 1 {

						// (mh,n) ∈ I
						if frequency_indices.contains(&frequency_index) {
							alter_phase[i][frequency_index] = alter_phase[i - 1][frequency_index] +
																	time_delta_phi[i][frequency_index];
							// Remove (mh,n) from I
							let set_index = frequency_indices.iter().position(|&v| v == frequency_index).unwrap();
							frequency_indices.remove(set_index);
							// Insert (mh,n) into the heap
							max_heap.push(
							MaxHeap { magnitude: magnitude[i][frequency_index], frequency_index: frequency_index, frame: i }
							);

						}
					}

					// propagate the phase in the frequency direction
					if max.frame == i {

						if frequency_index + 1 >= fft_size {
							alter_phase[i][frequency_index] = alter_phase[i][frequency_index] +
																	frequency_forward_delta_phi[i][frequency_index];

							if frequency_indices.contains(&frequency_index) {
								// Remove (mh,n) from I
								let set_index = frequency_indices.iter().position(|&v| v == frequency_index).unwrap();
								frequency_indices.remove(set_index);
								// Insert (mh,n) into the heap
								max_heap.push(
								MaxHeap { magnitude: magnitude[i][frequency_index], frequency_index: frequency_index, frame: i }
								);
							}
							continue;
						}

						if frequency_index as isize - 1 < 0 {
							alter_phase[i][frequency_index] = alter_phase[i][frequency_index] -
																	frequency_backward_delta_phi[i][frequency_index];
							if frequency_indices.contains(&frequency_index) {
								// Remove (mh,n) from I
								let set_index = frequency_indices.iter().position(|&v| v == frequency_index).unwrap();
								frequency_indices.remove(set_index);
								// Insert (mh,n) into the heap
								max_heap.push(
								MaxHeap { magnitude: magnitude[i][frequency_index], frequency_index: frequency_index, frame: i }
								);
							}
							continue;
						}

						// (mh + 1,n) ∈ I
						if frequency_indices.contains(&(frequency_index + 1)) {
							alter_phase[i][frequency_index + 1] = alter_phase[i][frequency_index] +
																		frequency_forward_delta_phi[i][frequency_index];
							// Remove (mh + 1,n) from I
							let set_index = frequency_indices.iter().position(|&v| v == frequency_index + 1).unwrap();
							frequency_indices.remove(set_index);
							// Insert (mh + 1,n) into the heap
							max_heap.push(
							MaxHeap { magnitude: magnitude[i][frequency_index + 1], frequency_index: frequency_index + 1, frame: i }
							);
						}

						// (mh - 1,n) ∈ I
						if frequency_indices.contains(&(frequency_index - 1)) {
							alter_phase[i][frequency_index - 1] = alter_phase[i][frequency_index] -
																		frequency_backward_delta_phi[i][frequency_index];
							// Remove (mh - 1,n) from I
							let set_index = frequency_indices.iter().position(|&v| v == frequency_index - 1).unwrap();
							frequency_indices.remove(set_index);
							// Insert (mh - 1,n) into the heap
							max_heap.push(
							MaxHeap { magnitude: magnitude[i][frequency_index - 1], frequency_index: frequency_index - 1, frame: i }
							);
						}
					}
				}
			}
		}());

        // Resynthesis
        for j in 0..fft_size {
            y_real[j] = magnitude[i][j] * alter_phase[i][j].cos();
            y_imag[j] = magnitude[i][j] * alter_phase[i][j].sin();
        }

        // IFFT
        fft(&mut y_real, &mut y_imag, fft_size, true);

        // Shift real signal to lead
        y_real.rotate_left(frame_size);

        // Windowning real signal
        for j in 0..frame_size {
            y_real[j] = y_real[j] * analysis_window[j];
        }

		// Pitch-shift needs interpolation of audio signal.
		let synthesized_buffer = if mode == command::Mode::PitchShift {
			let interpolate_length = (fft_size as f64 * analysis_hopsize / synthesis_hopsize) as usize;
			interpolation(&fft_size, &interpolate_length, &y_real, &ratio)
		} else {
			y_real.clone()
		};

        for j in 0..frame_size {
            if alter_offset + j >= result_buffer.len() {
                break;
            }
            result_buffer[alter_offset + j] = result_buffer[alter_offset + j] + synthesized_buffer[j];
        }
    }

    let result = wav_write(&output_path, result_buffer, size, fs, bit)?;

    Ok(result)
}
