//! C wrapper around the rust internals.
//!
//! See also this excellent book:
//! http://jakegoulding.com/rust-ffi-omnibus/slice_arguments/
//!

use libc::{c_char, size_t};
use std::ffi::CStr;
use std::panic::catch_unwind;

use lognplot::tsdb::{TsDb, TsDbHandle};
use lognplot::TcpClient;

// TSDB API
#[no_mangle]
pub extern "C" fn lognplot_tsdb_new() -> TsDbHandle {
    println!("Hello in rust!");
    TsDb::default().into_handle()
}

#[no_mangle]
pub extern "C" fn lognplot_tsdb_add_sample(db: TsDbHandle, name: &str, value: f64) {
    println!("Hello in rust!");
}

#[no_mangle]
pub extern "C" fn lognplot_tsdb_query(db: TsDbHandle) {
    println!("Hello in rust!");
}

// CLIENT API

#[no_mangle]
pub extern "C" fn lognplot_client_new(address: *const c_char) -> *mut TcpClient {
    let result = catch_unwind(|| {
        let addr = unsafe {
            assert!(!address.is_null());
            CStr::from_ptr(address)
        }
        .to_str()
        .unwrap();
        println!("Connecting to: {}", addr);
        let client = TcpClient::new(addr);
        println!("Client created!");
        client
    });

    match result {
        Ok(client) => Box::into_raw(Box::new(client)),
        Err(e) => {
            println!("Error: {:?}", e);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn lognplot_client_send_sample(
    client_ptr: *mut TcpClient,
    name: *const c_char,
    t: f64,
    value: f64,
) {
    if client_ptr.is_null() {
        println!("Client is null, not sending data!");
    } else {
        let result = catch_unwind(|| {
            let client: &mut TcpClient = unsafe {
                assert!(!client_ptr.is_null());
                &mut *client_ptr
            };

            let name = unsafe {
                assert!(!name.is_null());
                CStr::from_ptr(name)
            }
            .to_str()
            .unwrap();

            client.send_sample(name, t, value);
        });

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn lognplot_client_send_samples(
    client_ptr: *mut TcpClient,
    name: *const c_char,
    count: size_t,
    times: *const f64,
    values: *const f64,
) {
    if client_ptr.is_null() {
        println!("Client is null, not sending data!");
    } else {
        let result = catch_unwind(|| {
            let client: &mut TcpClient = unsafe {
                assert!(!client_ptr.is_null());
                &mut *client_ptr
            };

            let name = unsafe {
                assert!(!name.is_null());
                CStr::from_ptr(name)
            }
            .to_str()
            .unwrap();

            let samples = {
                let times = unsafe { std::slice::from_raw_parts(times, count) };
                let values = unsafe { std::slice::from_raw_parts(values, count) };
                let mut samples: Vec<(f64, f64)> = vec![];
                for (t, v) in times.into_iter().zip(values.iter()) {
                    samples.push((*t, *v));
                }
                samples
            };

            client.send_samples(name, samples);
        });

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn lognplot_client_send_sampled_samples(
    client_ptr: *mut TcpClient,
    name: *const c_char,
    t0: f64,
    dt: f64,
    count: size_t,
    values: *const f64,
) {
    if client_ptr.is_null() {
        println!("Client is null, not sending data!");
    } else {
        let result = catch_unwind(|| {
            let client: &mut TcpClient = unsafe {
                assert!(!client_ptr.is_null());
                &mut *client_ptr
            };

            let name = unsafe {
                assert!(!name.is_null());
                CStr::from_ptr(name)
            }
            .to_str()
            .unwrap();

            let values: Vec<f64> = unsafe { std::slice::from_raw_parts(values, count) }.to_vec();

            client.send_sampled_samples(name, t0, dt, values);
        });

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
