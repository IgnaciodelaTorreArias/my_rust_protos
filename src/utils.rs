use std::mem::ManuallyDrop;

use crate::messages::my_package::Error as CMR;

pub(crate) fn get_call_message<T>(ptr: *const u8, len: usize) -> Result<T, CMR>
where
    T: Default + prost::Message,
{
    if ptr.is_null() {
        Ok(T::default())
    } else {
        let input = unsafe { std::slice::from_raw_parts(ptr, len) };
        match T::decode(input) {
            Ok(msg) => Ok(msg),
            Err(e) => Err(CMR {
                details: e.to_string(),
            }),
        }
    }
}

pub(crate) fn set_call_result<T: prost::Message>(
    res: &T,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> bool {
    let l = res.encoded_len();
    if l == 0 {
        unsafe {
            *out_ptr = std::ptr::null_mut();
            *out_len = 0;
        };
        return true;
    }
    let mut buf: Vec<u8> = Vec::with_capacity(l);
    if res.encode(&mut buf).is_err() {
        unsafe {
            *out_ptr = std::ptr::null_mut();
            *out_len = 0;
        };
        return false;
    }
    let mut buf = ManuallyDrop::new(buf);
    unsafe {
        *out_ptr = buf.as_mut_ptr();
        *out_len = buf.len();
    };
    true
}

/// # Safety
/// Function must be called after a function that has an output.
/// With the same address and len the output was pointed to.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_buffer(ptr: *mut u8, len: usize) {
    let _vec = unsafe { Vec::from_raw_parts(ptr, len, len) };
}
