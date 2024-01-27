use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, help = "for the box, use size of the terminal")]
    pub use_term_size: bool,
    #[arg(short, long, help = "number of columns for the box, use_term_size cannot be specified", default_value = "38")]
    pub columns: u8,
    #[arg(short, long, help = "number of rows for the box, use_term_size cannot be specified", default_value = "13")]
    pub rows: u8,
}