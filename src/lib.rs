#[macro_use]
extern crate lazy_static;

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

    println!("INIT: {:p} {:p}", start, stop);

    let mut x = start;
    while x < stop {
        unsafe {
            N += 1;
            *x = N;
            x = x.add(1);
        }
    }

    unsafe {
        libc::atexit(__dumper_death);
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
