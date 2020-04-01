//! Client API.

use libc::{c_char, size_t};
use std::ffi::CStr;

use lognplot::net::TcpClient;

const RESULT_OK: u32 = 0;
const RESULT_ERR_OTHER: u32 = 1;
const RESULT_ERR_INVALID_CLIENT_PTR: u32 = 2;
const RESULT_ERR_INVALID_ARGUMENT: u32 = 3;

#[no_mangle]
pub extern "C" fn lognplot_client_new(address: *const c_char) -> *mut TcpClient {
    if address.is_null() {
        println!("Error: address was NULL");
        std::ptr::null_mut()
    } else {
        let addr = process_c_string(address);

        println!("Connecting to: {}", addr);
        match TcpClient::new(addr) {
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
pub extern "C" fn lognplot_client_close(client_ptr: *mut TcpClient) -> u32 {
    if client_ptr.is_null() {
        RESULT_ERR_INVALID_CLIENT_PTR
    } else {
        let client = process_client(client_ptr);

        if let Err(err) = client.close() {
            println!("Error closing client: {:?}", err);
            // TODO: return error and not free memory?
        }

        // Drop client:
        unsafe {
            Box::from_raw(client_ptr);
        }
        RESULT_OK
    }
}

#[no_mangle]
pub extern "C" fn lognplot_client_send_sample(
    client_ptr: *mut TcpClient,
    name: *const c_char,
    t: f64,
    value: f64,
) -> u32 {
    if client_ptr.is_null() {
        RESULT_ERR_INVALID_CLIENT_PTR
    } else if name.is_null() {
        RESULT_ERR_INVALID_ARGUMENT
    } else {
        let client = process_client(client_ptr);
        let name = process_c_string(name);

        if let Err(err) = client.send_sample(name, t, value) {
            println!("Error: {:?}", err);
            RESULT_ERR_OTHER
        } else {
            RESULT_OK
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
) -> u32 {
    if client_ptr.is_null() {
        RESULT_ERR_INVALID_CLIENT_PTR
    } else if name.is_null() {
        RESULT_ERR_INVALID_ARGUMENT
    } else {
        let client = process_client(client_ptr);
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

        if let Err(err) = client.send_samples(name, samples) {
            println!("Error: {:?}", err);
            RESULT_ERR_OTHER
        } else {
            RESULT_OK
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
) -> u32 {
    if client_ptr.is_null() {
        RESULT_ERR_INVALID_CLIENT_PTR
    } else if name.is_null() {
        RESULT_ERR_INVALID_ARGUMENT
    } else {
        let client = process_client(client_ptr);
        let name = process_c_string(name);

        let values: Vec<f64> = unsafe { std::slice::from_raw_parts(values, count) }.to_vec();

        if let Err(err) = client.send_sampled_samples(name, t0, dt, values) {
            println!("Error: {:?}", err);
            RESULT_ERR_OTHER
        } else {
            RESULT_OK
        }
    }
}

#[no_mangle]
pub extern "C" fn lognplot_client_send_text(
    client_ptr: *mut TcpClient,
    name: *const c_char,
    t: f64,
    text: *const c_char,
) -> u32 {
    if client_ptr.is_null() {
        RESULT_ERR_INVALID_CLIENT_PTR
    } else if name.is_null() {
        RESULT_ERR_INVALID_ARGUMENT
    } else if text.is_null() {
        RESULT_ERR_INVALID_ARGUMENT
    } else {
        let client = process_client(client_ptr);
        let name = process_c_string(name);
        let text = process_c_string(text);

        if let Err(err) = client.send_text(name, t, text.to_owned()) {
            println!("Error: {:?}", err);
            RESULT_ERR_OTHER
        } else {
            RESULT_OK
        }
    }
}

fn process_c_string<'a>(s: *const c_char) -> &'a str {
    unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    }
    .to_str()
    .unwrap()
}

fn process_client<'a>(client_ptr: *mut TcpClient) -> &'a mut TcpClient {
    let client: &mut TcpClient = unsafe {
        assert!(!client_ptr.is_null());
        &mut *client_ptr
    };

    client
}
