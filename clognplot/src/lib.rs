use libc::c_char;
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
