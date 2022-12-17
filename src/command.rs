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
    #[arg(value_enum, long = "mode")]
    pub mode: Mode,

    #[arg(short, long)]
    pub ratio: f64,
    /// input wave file path
    #[arg(short, long)]
    pub i: Option<String>,
    /// output wave file path
    #[arg(short, long)]
    pub o: Option<String>,

    #[arg(short, long)]
    pub buffer: Option<usize>,
}
