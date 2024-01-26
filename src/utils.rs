pub fn get_terminal_size() -> (u8, u8) {
    let terminal_size = term_size::dimensions().expect("cannot get terinal size");
    let size_columns = terminal_size.0 as u8 / 2;
    let size_rows = terminal_size.1 as u8 - 1;
    (size_columns, size_rows)
}