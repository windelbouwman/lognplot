//! TSDB API
use libc::c_char;
use lognplot::time::TimeStamp;
use lognplot::tsdb::{Observation, Sample, TsDb, TsDbHandle};
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn lognplot_tsdb_new() -> *mut TsDbHandle {
    let handle = TsDb::default().into_handle();
    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn lognplot_tsdb_add_sample(db: *mut TsDbHandle, name: *const c_char, value: f64) {
    if db.is_null() {
        println!("ERROR: db is null");
    } else if name.is_null() {
        println!("ERROR: name is null");
    } else {
        let db: &mut TsDbHandle = unsafe {
            assert!(!db.is_null());
            &mut *db
        };

        let name = unsafe {
            assert!(!name.is_null());
            CStr::from_ptr(name)
        }
        .to_str()
        .unwrap();

        let timestamp = TimeStamp::new(0.0);
        let sample = Sample::new(value);
        let observation = Observation::new(timestamp, sample);
        db.add_value(&name, observation);
    }
}

#[no_mangle]
pub extern "C" fn lognplot_tsdb_query(db: *mut TsDbHandle) {
    if db.is_null() {
        println!("ERROR: db is null");
    } else {
        println!("TODO: query");
    }
}
