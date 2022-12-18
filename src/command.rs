use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum, PartialEq, Parser)]
pub enum Mode {
    TimeStretch,
    PitchShift,
}

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
    /// weather its time-stretch or pitch-shift
    #[arg(value_enum, short, long = "mode")]
    pub mode: Mode,
    /// factor ratio
    #[arg(short, long)]
    pub ratio: f64,
    /// input wave file path
    #[arg(short, long)]
    pub i: Option<String>,
    /// output wave file path
    #[arg(short, long)]
    pub o: Option<String>,
    /// frame size that should be power of two
    #[arg(short, long)]
    pub buffer: Option<usize>,
}
