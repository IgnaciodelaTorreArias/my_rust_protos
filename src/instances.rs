use std::ffi::c_void;
use std::sync::{OnceLock, RwLock};

static INSTANCES: OnceLock<RwLock<Vec<usize>>> = OnceLock::new();

fn get_instances() -> &'static RwLock<Vec<usize>> {
    INSTANCES.get_or_init(|| RwLock::new(Vec::new()))
}

pub(crate) fn register_instance(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let mut list = get_instances()
        .write()
        .expect("Corrupted instances cache while registering instance");
    list.push(ptr as usize);
}

pub(crate) fn instance_exists(ptr: *mut c_void) -> bool {
    if ptr.is_null() {
        return false;
    }
    let list = get_instances()
        .read()
        .expect("Corrupted instances cache while searching instance");
    list.contains(&(ptr as usize))
}
