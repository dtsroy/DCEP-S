pub fn raise_error(con: &str) {
    println!("\x1b[1;31mDCEP-SERVER: error: {}\x1b[0m", con);
    std::process::exit(1);
}

pub fn show_warning(con: &str) {
    println!("\x1b[1;33mwarning: {}\x1b[0m", con);
}
