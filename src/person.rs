use crate::messages::{CallStatus, Greetings, PersonParams, Response};
use crate::buffer_utils::*;

pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Person {
    pub fn greet(&self, other: &str) -> String {
        format!(
            "Hello {}, my name is {}, I'm {} years old",
            other, self.name, self.age
        )
    }
}

/// # Safety
/// Caller must ensure `person_ptr` is valid.
/// `person_ptr` will be set to point to a `Person` struct on the heap, caller must manually free memory using  the `free_person` function.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_new_person(
    instance_ptr: *mut *mut Person,
    ptr: *const u8,
    len: usize,
) -> i32 {
    let person = match get_call_message::<PersonParams>(ptr, len) {
        Ok(m) => m,
        Err(_) => return CallStatus::DecodeError.into(),
    };
    let person = Box::new(Person {
        name: person.name,
        age: person.age as u8,
    });
    let res_ptr = Box::into_raw(person);
    unsafe {
        println!(
            "RUST: create_new_person called with, res_ptr: {:?}",
            res_ptr
        );
        *instance_ptr = res_ptr;
    }
    0
}

/// # Safety
/// Caller must ensure `instance_ptr` is a valid address provided by the function `create_new_person`.
/// Caller must ensure `ptr` and `len` provide valid information for a bytes buffer that contains an encoded `Greetings` proto message.
/// Caller must ensure `out_ptr` and `out_len` are valid.
/// The `out_ptr` and `out_len` will set the information needed to read a bytes buffer containing an encoded `Response` proto message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn person_greet(
    instance_ptr: *mut Person,
    ptr: *const u8,
    len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    println!(
        "RUST: person_greet called with instance_ptr: {:?}",
        instance_ptr
    );
    let person = unsafe { &mut *instance_ptr };
    let res = match get_call_message::<Greetings>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    set_call_result(
        Response {
            text: person.greet(&res.name),
        },
        out_ptr,
        out_len
    );
    CallStatus::Ok.into()
}

/// # Safety
/// Caller must ensure there are no other references to the structure.
/// `ptr` must be an address provided by the function `create_new_person`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_person(ptr: *mut Person) {
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
