use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 100)]
    pub time: u64,

    #[arg(long, default_value_t = 50)]
    pub height: usize,

    #[arg(long, default_value_t = 50)]
    pub width: usize,
}
