use clap::{Parser, ValueEnum};
use colored::Color;

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum CliColor {
    White,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

impl Into<Color> for CliColor {
    fn into(self) -> Color {
        match self {
            Self::Black => Color::Black,
            Self::Red => Color::Red,
            Self::Green => Color::Green,
            Self::Yellow => Color::Yellow,
            Self::Blue => Color::Blue,
            Self::Magenta => Color::Magenta,
            Self::Cyan => Color::Cyan,
            Self::White => Color::White,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The cron expression to use
    pub cron_expr: String,
    /// An optional title that replaces the default date range display
    #[arg(short, long)]
    pub title: Option<String>,
    /// The width of the progress bar
    #[arg(short, long, default_value_t = 50)]
    pub width: usize,
    /// The color of the progress bar
    #[arg(value_enum, short, long, default_value_t = CliColor::White)]
    pub color: CliColor,
}
