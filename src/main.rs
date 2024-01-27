use clap::Parser;
use crate::argser::Args;
use crate::configer::Config;

mod utils;
mod land;
mod configer;
mod argser;

fn main()
{
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    let config: Config = Config::new();
    let args = Args::parse();

    let use_terminal_size: bool = args.use_term_size;
    let size_columns: u8;
    let size_rows: u8;

    if use_terminal_size {
        (size_columns, size_rows) = utils::get_terminal_size();
    } else {
        (size_columns, size_rows) = (args.columns, args.rows);
    }

    let mut land = land::Land::new(&config, size_columns, size_rows);
    land.add_food();
    land.run();
}
