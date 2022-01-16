use super::connection::Connection;
use super::parse_args::Settings;
use crate::message::debug::debug_p;

pub struct Server<'a> {
    setting: &'a Settings,
    conns: Vec<Connection>,
    count: i32,
}

impl Server<'_> {
    pub fn new(setting: &Settings) -> Server {
        Server {
            setting: setting,
            conns: vec![],
            count: -1,
        }
    }

    fn add_connection(&mut self, conn: Connection) {
        self.conns.push(conn);
    }

    pub fn run(&mut self) {
        debug_p("Server running...");
    }
}
