use crate::color::Color;
use crate::completions::CompletionsArgs;
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ArgsCli {
    #[command(subcommand)]
    pub subcommand: Cmds,
}

#[derive(Debug, Subcommand)]
pub enum Cmds {
    Completions(CompletionsArgs),
    Static(StaticArgs),
    SingleOn(StdArgs),
    SingleOff(StdArgs),
    Glittering(StdArgs),
    Rain(StdArgs),
    Colorful(StdArgs),
    Breath(StdArgs),
    Spectrum(SpectrumArgs),
    CentrifugalWave(StdArgs),
    VerticalWave(VerticalWaveArgs),
    HorizontalWave(HorizontalWaveArgs),
    Rotating(StdArgs),
    Explosion(StdArgs),
    Launch(StdArgs),
    Ripples(StdArgs),
    Snake(StdArgs),
    Pulse(StdArgs),
    Tilt(StdArgs),
    Shuttle(StdArgs),
}

#[derive(Args, Debug)]
pub struct StaticArgs {
    #[clap(value_parser)]
    pub color: Color,
    #[arg(short, long, default_value_t = 16, help = "Brightness of the effect (min 0, max 16)")]
    pub brightness: u8,
    #[clap(long, short, action)]
    pub full_color: bool,
}

#[derive(Args, Debug)]
pub struct StdArgs {
    #[clap(value_parser)]
    pub color: Color,
    #[arg(short, long, default_value_t = 16, help = "Brightness of the effect (min 0, max 16)")]
    pub brightness: u8,
    #[arg(short, long, default_value_t = 8, help = "Speed of the effect (min 1, max 16)")]
    pub speed: u8,
    #[clap(long, short, action)]
    pub full_color: bool,
}

#[derive(Args, Debug)]
pub struct SpectrumArgs {
    #[arg(short, long, default_value_t = 16, help = "Brightness of the effect (min 0, max 16)")]
    pub brightness: u8,
    #[arg(short, long, default_value_t = 8, help = "Speed of the effect (min 1, max 16)")]
    pub speed: u8,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum VerticalDirections {
    Down,
    Up,
}

impl VerticalDirections {
    pub fn to_u8(&self) -> u8 {
        match self {
            VerticalDirections::Down => 2,
            VerticalDirections::Up => 3
        }
    }
}

#[derive(Args, Debug)]
pub struct VerticalWaveArgs {
    #[clap(value_parser)]
    pub color: Color,
    #[arg(short, long, default_value_t = 16, help = "Brightness of the effect (min 0, max 16)")]
    pub brightness: u8,
    #[arg(short, long, default_value_t = 8, help = "Speed of the effect (min 1, max 16)")]
    pub speed: u8,
    #[clap(value_enum)]
    pub direction: VerticalDirections,
    #[clap(long, short, action)]
    pub full_color: bool,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum HorizontalDirections {
    Left,
    Right,
}

impl HorizontalDirections {
    pub fn to_u8(&self) -> u8 {
        match self {
            HorizontalDirections::Left => 0,
            HorizontalDirections::Right => 1
        }
    }
}

#[derive(Args, Debug)]
pub struct HorizontalWaveArgs {
    #[clap(value_parser)]
    pub color: Color,
    #[arg(short, long, default_value_t = 16, help = "Brightness of the effect (min 0, max 16)")]
    pub brightness: u8,
    #[arg(short, long, default_value_t = 8, help = "Speed of the effect (min 1, max 16)")]
    pub speed: u8,
    #[clap(value_enum)]
    pub direction: HorizontalDirections,
    #[clap(long, short, action)]
    pub full_color: bool,
}