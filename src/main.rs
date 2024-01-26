mod utils;
mod land;
mod consts;

fn main()
{
    let use_terminal_size: bool = false;
    let size_columns: u8;
    let size_rows: u8;

    if use_terminal_size {
        (size_columns, size_rows) = utils::get_terminal_size()
    } else {
        (size_columns, size_rows) = (25, 14);
    }

    let mut land = land::Land::new(size_columns, size_rows);
    land.run();
}
