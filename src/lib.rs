pub use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Use real data instead of example data input file(s)
    #[arg(short = 'r', long)]
    pub real: bool,

    #[arg(short = '2', long)]
    pub part_two: bool,
}
