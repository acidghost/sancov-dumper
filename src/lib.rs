#[macro_use]
extern crate lazy_static;

use nix::sys::signal::{self as sig, signal, SigHandler};
use serde::Serialize;

use std::collections::HashMap;
use std::sync::Mutex;

mod sancov {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

lazy_static! {
    static ref MAP: Mutex<HashMap<u32, usize>> = Mutex::new(HashMap::new());
}

#[no_mangle]
extern "C" fn __sanitizer_cov_trace_pc_guard_init(start: *mut u32, stop: *mut u32) {
    static mut N: u32 = 0;

    if start == stop || unsafe { *start } != 0 {
        return;
    }

    println!("[sancov-dumper] INIT: {:p} {:p}", start, stop);

    let mut x = start;
    while x < stop {
        unsafe {
            N += 1;
            *x = N;
            x = x.add(1);
        }
    }

    if unsafe { nix::libc::atexit(__dumper_death) } != 0 {
        eprintln!("[sancov-dumper] failed to set atexit handler!");
    }

    if std::env::var("SANCOV_SKIP_SIGNALS").is_err() {
        let shandle = SigHandler::Handler(__catch_signal);
        if let Err(e) = unsafe {
            signal(sig::SIGUSR1, shandle)
                .and_then(|_| signal(sig::SIGINT, shandle))
                .and_then(|_| signal(sig::SIGSEGV, shandle))
                .and_then(|_| signal(sig::SIGABRT, shandle))
                .and_then(|_| signal(sig::SIGTERM, shandle))
        } {
            eprintln!("[sancov-dumper] failed to set signal handlers: {}", e);
        }
    }
}

#[no_mangle]
extern "C" fn __sanitizer_cov_trace_pc_guard(guard: *mut u32) {
    if unsafe { *guard } == 0 {
        return;
    }
    // println!("guard: {:p} {:X}", guard, unsafe { *guard });
    *MAP.lock().unwrap().entry(unsafe { *guard }).or_insert(0) += 1;
}

#[no_mangle]
extern "C" fn __catch_signal(signal: i32) {
    println!("[sancov-dumper] received signal {}", signal);
    __dumper_death();
    unsafe { nix::libc::_exit(0) };
}

#[no_mangle]
extern "C" fn __dumper_death() {
    let fname = std::env::var("SANCOV_OUT_FILE").unwrap_or_else(|_| String::from("cov.csv"));
    let mut writer = csv::Writer::from_path(fname).expect("failed to init CSV writer");
    for (&id, &counter) in MAP.lock().unwrap().iter() {
        writer
            .serialize(Record { id, counter })
            .expect("failed to serialize record");
    }
}

#[derive(Serialize)]
struct Record {
    id: u32,
    counter: usize,
}
