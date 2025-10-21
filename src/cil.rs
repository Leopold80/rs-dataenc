use clap::{ValueEnum, Parser};

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Cil {
    #[arg(required = true)]
    pub files: Vec<String>,

    #[arg(value_enum, short, long, required = true)]
    pub mode: EncDecMode,

    #[arg(short, long, required = true)]
    pub passwd: String,

    // #[arg(long, short)]
    // pub dir: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum EncDecMode {
    Enc,
    Dec,
}