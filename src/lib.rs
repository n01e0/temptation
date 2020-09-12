extern crate libc;

#[macro_use]
extern crate redhook;

use env_logger;
use log::{error, warn, info};
use libc::{c_int, c_char, c_long};
use std::{fs, env};

redhook::hook! {
    unsafe fn fexecve(fd: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int => detect_fileless {
        if let Err(_) = env::var("RUST_LOG") {
            env::set_var("RUST_LOG", "warn");
        }
        env_logger::init();
        info!("hook fexecve!");
        match fs::read_link(&format!("/proc/self/fd/{}", fd)) {
            Ok(link) => {
                let filename = String::from(link.iter().last().unwrap().to_str().unwrap());
                if filename.starts_with("memfd:") {
                    warn!("detected fileless fexecve!!");
                }
            },
            Err(e) => {
                error!("error in read_link: {}", e);
            }
        }

        real!(fexecve)(fd, argv, envp)
    }
}

