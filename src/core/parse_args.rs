use crate::message::error::*;

pub struct Settings {
    port: i32,
    verbose: bool,
}

impl Settings {
    pub fn new() -> Settings {
        return Settings {
            port: -1,
            verbose: false,
        };
    }

    pub fn set_port(&mut self, port: i32) {
        if self.port != -1 {
            // 端口已被修改
            raise_error("Port has already been set.");
        }
        self.port = port;
    }

    pub fn turn_on_verbose(&mut self) {
        if self.verbose {
            show_warning("Verbose mode has already turned on.");
        }
        self.verbose = true;
    }

    pub fn get_port(&self) -> i32 {
        self.port
    }

    pub fn get_verbose(&self) -> bool {
        self.verbose
    }
}

pub fn parse_args() -> Settings {
    let args_from_cmd = std::env::args();
    let mut settings = Settings::new();

    let mut idx = 0;
    let mut is_non_running = false; // 不运行模式标志

    for arg in args_from_cmd {
        match idx {
            // 命令名,省略
            0 => {}
            // 必须是端口号
            1 => {
                settings.set_port(arg.parse::<i32>().expect("Cannot parse port"));
                if settings.port == 0 {
                    // 不运行模式
                    is_non_running = true;
                } else if settings.port < 0 {
                    raise_error("Invalid port");
                }
            }
            2 => {
                // 保证没问题,因为如果后接参数已经在第一个match-special里处理了
                match arg.as_str() {
                    "-h" => {
                        if is_non_running {
                            crate::message::basic::print_help();
                            std::process::exit(0);
                        } else {
                            show_warning("Didn't take argument `-h` because port != 0");
                        }
                    }
                    "-v" => {
                        if is_non_running {
                            crate::message::basic::print_version();
                            std::process::exit(0);
                        } else {
                            show_warning("Didn't take argument `-v` because port != 0");
                        }
                    }
                    "-V" => {
                        settings.turn_on_verbose();
                    }
                    _ => {
                        raise_error("Unknown argument.");
                    }
                };
            }
            _ => {
                raise_error("Too many arguments.");
            }
        };
        idx += 1;
    }
    return settings;
}
