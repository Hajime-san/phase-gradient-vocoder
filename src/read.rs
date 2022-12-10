use std::{
    io::{
        Read,
    },
    fs::File,
    str,
    string::FromUtf8Error,
};
use core::num::ParseIntError;
use thiserror::Error;

use crate::normalize::{
    GenericNormalize,
};

#[derive(Debug, Clone)]
pub struct Wave {
    pub file_type: String,
    pub riff_chunk_size: usize,
    pub riff_type: String,
    pub format_str: String,
    pub format_data_size: usize,
    pub compression_code: String,
    pub channels: String,
    pub sample_rate: usize,
    pub bytes_per_second: usize,
    pub block_align: usize,
    pub bits_per_sample: usize,
    pub data_str: String,
    pub chunk_data_size: usize,
    pub file_size: usize,
    pub normalized_sample_data: Vec<f64>,
}
#[derive(Error, Debug)]
pub enum WaveParseError {
    #[error("failed to open file")]
    IoError(#[from] std::io::Error),
    #[error("failed to convert bytes to utf-8 string")]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("failed to convert string to hexadecimal integer")]
    ParseIntError(#[from] ParseIntError),
}

pub type WaveResult<T> = Result<T, WaveParseError>;

fn byte_vec_to_num(bytes: &mut Vec<u8>) -> Result<usize, ParseIntError> {
    // reverse byte vector from little-endian
    bytes.reverse();
    let hexadecimal = bytes.iter().map(|n| format!("{:02X}", n)).collect::<String>();
    let number = usize::from_str_radix(&hexadecimal, 16)?;

    Ok(number)
}

fn normalize<T: GenericNormalize>(value: f64) -> f64 {
    let max = T::MAX.to_f64().unwrap();
    let min = T::MIN.to_f64().unwrap();
    let normalized = ((value - min) / (max - min)) * (1.0 - (-1.0)) - 1.0;

    normalized
}

pub fn wav_read(filename: &str) -> WaveResult<Wave> {
    // open local file
    let mut file = File::open(filename)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;
    // restore data from binary
    let file_type = String::from_utf8(buf[0..=3].to_vec())?;
    let riff_chunk_size = byte_vec_to_num(&mut buf[4..=7].to_vec())?;
    let riff_type = String::from_utf8(buf[8..=11].to_vec())?;
    let format_str = String::from_utf8(buf[12..=15].to_vec())?;
    let format_data_size = byte_vec_to_num(&mut buf[16..=19].to_vec())?;
    let compression_code = match byte_vec_to_num(&mut buf[20..=21].to_vec())? {
        1 => "Linear PCM".to_string(),
        2 => "MS-ADPCM".to_string(),
        5 => "IBM CSVD".to_string(),
        _ => "Unkown wave format".to_string(),
    };
    let channels_num = byte_vec_to_num(&mut buf[22..=23].to_vec())?;
    let channels = if channels_num == 1 {
        "Monaural".to_string()
    } else {
        "Stereo".to_string()
    };
    let sample_rate = byte_vec_to_num(&mut buf[24..=27].to_vec())?;
    let bytes_per_second = byte_vec_to_num(&mut buf[28..=31].to_vec())?;
    let block_align = byte_vec_to_num(&mut buf[32..=33].to_vec())?;
    let bits_per_sample = byte_vec_to_num(&mut buf[34..=35].to_vec())?;
    let data_str = String::from_utf8(buf[36..=39].to_vec())?;
    let chunk_data_size = byte_vec_to_num(&mut buf[40..=43].to_vec())?;
    let file_size = riff_chunk_size + chunk_data_size + 8;
    let raw_chunk_data = buf[44..=buf.len() - 1].to_vec();

    let data = if bits_per_sample == 8 {
        Wave {
            file_type,
            riff_chunk_size,
            riff_type,
            format_str,
            format_data_size,
            compression_code,
            channels,
            sample_rate,
            bytes_per_second,
            block_align,
            bits_per_sample,
            data_str,
            chunk_data_size,
            file_size,
            normalized_sample_data: raw_chunk_data.into_iter().map(|a| normalize::<u8>(a as f64)).collect(),
        }
    } else {
        let restored_normalized_sample_data: Vec<f64> = raw_chunk_data
                                    .chunks_exact(2)
                                    .into_iter()
                                    .map(|a| normalize::<i16>(i16::from_le_bytes([a[0], a[1]]) as f64))
                                    .collect();
        Wave {
            file_type,
            riff_chunk_size,
            riff_type,
            format_str,
            format_data_size,
            compression_code,
            channels,
            sample_rate,
            bytes_per_second,
            block_align,
            bits_per_sample,
            data_str,
            chunk_data_size,
            file_size,
            normalized_sample_data: restored_normalized_sample_data,
        }
    };

    Ok(data)
}
