use chrono::prelude::*;

static mut VERBOSE_MODE: bool = false;

pub fn set_log(vb: bool) {
    unsafe {
        VERBOSE_MODE = vb;
    }
}

pub fn debug_p<T: std::fmt::Display>(con: T) {
    unsafe {
        if VERBOSE_MODE {
            let now: DateTime<Local> = Local::now();
            let mills: i64 = now.timestamp_millis();
            println!("{} {}", Local.timestamp_millis(mills), con);
        }
    }
}
