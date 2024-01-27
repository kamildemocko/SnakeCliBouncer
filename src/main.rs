use crate::configer::Config;

mod utils;
mod land;
mod configer;

fn main()
{
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    let config: Config = Config::new();

    let use_terminal_size: bool = false;
    let size_columns: u8;
    let size_rows: u8;

    if use_terminal_size {
        (size_columns, size_rows) = utils::get_terminal_size();
    } else {
        (size_columns, size_rows) = (58, 14);
    }

    let mut land = land::Land::new(&config, size_columns, size_rows);
    land.add_food();
    land.run();
}
