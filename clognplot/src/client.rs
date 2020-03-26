//! Client API.

use libc::{c_char, size_t};
use std::ffi::CStr;

use lognplot::net::TcpClient;

#[no_mangle]
pub extern "C" fn lognplot_client_new(address: *const c_char) -> *mut TcpClient {
    if address.is_null() {
        println!("Error: address was NULL");
        std::ptr::null_mut()
    } else {
        let addr = process_c_string(address);

        println!("Connecting to: {}", addr);
        match TcpClient::new(&addr) {
            Ok(client) => {
                println!("Client created!");
                Box::into_raw(Box::new(client))
            }
            Err(err) => {
                println!("Error: {:?}", err);
                std::ptr::null_mut()
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn lognplot_client_close(client_ptr: *mut TcpClient) {
    if client_ptr.is_null() {
        println!("client_ptr is null, not sending data!");
    } else {
        let client: &mut TcpClient = unsafe {
            assert!(!client_ptr.is_null());
            &mut *client_ptr
        };

        if let Err(err) = client.close() {
            println!("Error closing client: {:?}", err);
        }

        // TODO: drop client!
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
        println!("client_ptr is null, not sending data!");
    } else if name.is_null() {
        println!("name is null, not sending data!");
    } else {
        let client: &mut TcpClient = unsafe {
            assert!(!client_ptr.is_null());
            &mut *client_ptr
        };

        let name = process_c_string(name);

        match client.send_sample(&name, t, value) {
            Ok(_) => {}
            Err(err) => {
                println!("Error: {:?}", err);
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
    } else if name.is_null() {
        println!("name is null, not sending data!");
    } else {
        let client: &mut TcpClient = unsafe {
            assert!(!client_ptr.is_null());
            &mut *client_ptr
        };

        let name = process_c_string(name);

        let samples = {
            let times = unsafe { std::slice::from_raw_parts(times, count) };
            let values = unsafe { std::slice::from_raw_parts(values, count) };
            let mut samples: Vec<(f64, f64)> = vec![];
            for (t, v) in times.iter().zip(values.iter()) {
                samples.push((*t, *v));
            }
            samples
        };

        match client.send_samples(&name, samples) {
            Ok(_) => {}
            Err(err) => {
                println!("Error: {:?}", err);
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
    } else if name.is_null() {
        println!("name is null, not sending data!");
    } else {
        let client: &mut TcpClient = unsafe {
            assert!(!client_ptr.is_null());
            &mut *client_ptr
        };

        let name = process_c_string(name);

        let values: Vec<f64> = unsafe { std::slice::from_raw_parts(values, count) }.to_vec();

        match client.send_sampled_samples(&name, t0, dt, values) {
            Ok(_) => {}
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn lognplot_client_send_text(
    client_ptr: *mut TcpClient,
    name: *const c_char,
    t: f64,
    text: *const c_char,
) {
    if client_ptr.is_null() {
        println!("client_ptr is null, not sending data!");
    } else if name.is_null() {
        println!("name is null, not sending data!");
    } else if text.is_null() {
        println!("text is null, not sending data!");
    } else {
        let client: &mut TcpClient = unsafe {
            assert!(!client_ptr.is_null());
            &mut *client_ptr
        };

        let name = process_c_string(name);
        let text = process_c_string(text);

        match client.send_text(&name, t, text) {
            Ok(_) => {}
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}

fn process_c_string(s: *const c_char) -> String {
    unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    }
    .to_str()
    .unwrap()
    .to_owned()
}
