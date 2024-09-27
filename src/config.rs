use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
pub enum Mode {
    NormalMode,
    UserListMode,
}

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "Stefan Kornemann")]
pub struct Opts {
    #[clap(short, long, value_enum, default_value = "normal-mode")]
    pub mode: Mode,
    #[clap(short, long, value_delimiter=',')]
    pub numbers: Option<Vec<i64>>,
}
