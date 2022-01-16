use crate::message::debug::debug_p;
use crate::message::debug::set_log;

pub fn run_main() {
    let main_setting = super::parse_args::parse_args();
    let mut main_server = super::server::Server::new(&main_setting);
    set_log(main_setting.get_verbose());
    debug_p("run: main_server.run()");
    main_server.run();
}
