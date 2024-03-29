extern crate libc;

#[macro_use]
extern crate redhook;

use env_logger;
use libc::{c_char, c_int};
use log::{error, info, warn};
use std::io::{Read, Write};
use std::{env, fs, process};

hook! {
    unsafe fn fexecve(fd: c_int, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int => detect_fexecve {
        if let Err(_) = env::var("RUST_LOG") {
            env::set_var("RUST_LOG", "warn");
        }
        if let Err(_) = env::var("ACTION") {
            env::set_var("ACTION", "dump");
        }
        env_logger::init();
        info!("hook fexecve!");
        let path = format!("/proc/self/fd/{}", fd);
        match fs::read_link(&path) {
            Ok(link) => {
                let filename = String::from(link.iter().last().unwrap().to_str().unwrap());
                if filename.starts_with("memfd:") {
                    warn!("detected fileless fexecve!!");
                }
                match &env::var("ACTION").unwrap()[..] {
                    "abort" => {
                        warn!("aborting process");
                        std::process::abort();
                    },
                    "dump" => {
                        let mut buf = Vec::new();
                        let mut f = fs::File::open(path).unwrap();
                        f.read_to_end(&mut buf).unwrap();
                        let dump = format!("{}.dump", process::id());
                        warn!("binary dump to -> {}", dump);
                        let mut f = fs::File::create(dump).unwrap_or_else(|e| { error!("{}", e); process::exit(1); });
                        f.write_all(&buf).unwrap_or_else(|e| { error!("{}", e); process::exit(1); });
                        process::exit(0);
                    },
                    _ => info!("detected fileless exec"),
                }
            },
            Err(e) => {
                error!("error in read_link: {}", e);
            }
        }

        real!(fexecve)(fd, argv, envp)
    }
}
