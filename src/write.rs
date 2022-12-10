use std::{
    fs::File,
    io::{
        prelude::Write,
    },
};

use crate::normalize::{
    GenericNormalize,
};

pub fn normalize<T: GenericNormalize>(value: f64) -> Option<T> {
    let max = T::MAX.to_f64()?;
    let min = T::MIN.to_f64()?;
    // 0.0 is median value between -1.0 and 1.0
    let checked_value = if value.is_nan() {
        let median = 0.0;
        median
    } else {
        value
    };
    let normalized = (checked_value - (-1.0)) / (1.0 - (-1.0)) * (max - min) + min;

    let wrapping = if normalized > T::MAX.to_f64()? {
        T::MAX.to_f64()?
    } else if normalized < T::MIN.to_f64()? {
        T::MIN.to_f64()?
    } else {
        normalized
    };

    Some(T::from_f64(wrapping)?)
}

pub fn wav_write(filename: &str, buffer: Vec<f64>, size: usize, fs: usize, bit: usize) -> std::io::Result<()> {
    let mut head: Vec<u8> = vec![0; 44];

    // Chunk ID
    let riff_to_bytes = b"RIFF";
    head[0] = riff_to_bytes[0];
    head[1] = riff_to_bytes[1];
    head[2] = riff_to_bytes[2];
    head[3] = riff_to_bytes[3];

    let filesize = head.len() + size;
    // Chunk Data Size
    head[4] = ((filesize - 8).rotate_right(0) & 0xff) as u8;
    head[5] = ((filesize - 8).rotate_right(8) & 0xff) as u8;
    head[6] = ((filesize - 8).rotate_right(16) & 0xff) as u8;
    head[7] = ((filesize - 8).rotate_right(24) & 0xff) as u8;

    // RIFF Type
    let wave_to_bytes = b"WAVE";
    head[8] = wave_to_bytes[0];
    head[9] = wave_to_bytes[1];
    head[10] = wave_to_bytes[2];
    head[11] = wave_to_bytes[3];

    // Chunk ID
    let fmt_to_bytes = b"fmt ";
    head[12] = fmt_to_bytes[0];
    head[13] = fmt_to_bytes[1];
    head[14] = fmt_to_bytes[2];
    head[15] = fmt_to_bytes[3];

    // Chunk Data Size
    head[16] = 16;
    head[17] = 0;
    head[18] = 0;
    head[19] = 0;

    // Compression Code
    head[20] = 1;
    head[21] = 0;

    // Number of channels
    head[22] = 1;
    head[23] = 0;

    // Sample rate
    head[24] = (fs.rotate_right(0) & 0xff) as u8;
    head[25] = (fs.rotate_right(8) & 0xff) as u8;
    head[26] = (fs.rotate_right(16) & 0xff) as u8;
    head[27] = (fs.rotate_right(24) & 0xff) as u8;

    // Average bytes per second
    head[28] = ((fs * (bit / 8)).rotate_right(0) & 0xff) as u8;
    head[29] = ((fs * (bit / 8)).rotate_right(8) & 0xff) as u8;
    head[30] = ((fs * (bit / 8)).rotate_right(16) & 0xff) as u8;
    head[31] = ((fs * (bit / 8)).rotate_right(24) & 0xff) as u8;

    // Block align
    head[32] = ((bit / 8).rotate_right(0) & 0xff) as u8;
    head[33] = ((bit / 8).rotate_right(8) & 0xff) as u8;

    // Significant bits per sample
    head[34] = (bit.rotate_right(0) & 0xff) as u8;
    head[35] = (bit.rotate_right(8) & 0xff) as u8;

    // Chunk ID
    let data_to_bytes = b"data";
    head[36] = data_to_bytes[0];
    head[37] = data_to_bytes[1];
    head[38] = data_to_bytes[2];
    head[39] = data_to_bytes[3];

    // chunk size
    head[40] = (size.rotate_right(0) & 0xff) as u8;
    head[41] = (size.rotate_right(8) & 0xff) as u8;
    head[42] = (size.rotate_right(16) & 0xff) as u8;
    head[43] = (size.rotate_right(24) & 0xff) as u8;

    // write
    let mut file = File::create(filename)?;
    file.write_all(&head)?;
    if bit == 8 {
        for i in 0..buffer.len() {
            let sample = normalize::<u8>(buffer[i]).unwrap_or(0);
            let byte = sample.to_le_bytes();
            file.write_all(&byte)?;
        }
    } else {
        for i in 0..buffer.len() {
            let sample = normalize::<i16>(buffer[i]).unwrap_or(0);
            let byte = sample.to_le_bytes();
            file.write_all(&byte)?;
        }
    };
    file.flush()?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalize_i16() {
        let result: i16 = normalize(f64::NAN).unwrap_or(0);
        assert_eq!(0, result);

        let result: u8 = normalize(f64::NAN).unwrap_or(0);
        assert_eq!(127, result);
    }
}
