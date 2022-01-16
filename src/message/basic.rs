pub fn print_version() {
    println!("DCEP-SERVER version 0.1.0 (*-*-*)");
    println!("rustc 1.56.1 (59eed8a2a 2021-11-01)");
    println!("cargo 1.56.0 (4ed5d137b 2021-10-04)");
}

fn print_with_tab(con: &str) {
    println!("    {}", con);
}

pub fn print_help() {
    println!("Usage: dcep_server PORT [OPTIONS]");
    println!("Port:");
    print_with_tab("Local port to be binded.");
    print_with_tab("\x1b[1;33mNOTE:Setting port to `0` won't run server.\x1b[0m");
    println!("Options:");
    print_with_tab("-h, --help      Display this message.");
    print_with_tab("-v, --version   Display DCEP-SERVER version message.");
    print_with_tab("\x1b[1;33mNOTE:Two options above are only used with `port==0`.\x1b[0m");
    print_with_tab("-V, --verbose   Output log messages; default to `off`.");
}
