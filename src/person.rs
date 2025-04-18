
use std::os::raw::c_void;

use crate::messages::my_package::{PersonParams, Greetings, Response};
use crate::utils::*;

use crate::instances::*;

pub struct Person{
    pub name: String,
    pub age: u8,
}

impl Person {
    pub fn greet(&self, other: &str) -> String{
        format!("Hello {}, my name is {}, I'm {} years old", other, self.name, self.age)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn create_new_person(person_ptr: *mut *mut Person, ptr: *const u8, len: usize) -> i32 {
    let person = match get_call_message::<PersonParams>(ptr, len) {
        Ok(m) => m,
        Err(_) => return -3
    };
    let person = Box::new(Person {
        name: person.name,
        age: person.age as u8,
    });
    let res_ptr = Box::into_raw(person);
    register_instance(res_ptr as *mut c_void);
    unsafe {
        *person_ptr = res_ptr;
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn person_greet(instance_ptr: *mut Person, ptr: *const u8, len: usize, out_ptr: *mut *mut u8, out_len: *mut usize) -> i32{
    if !instance_exists(instance_ptr as *mut c_void){
        return -5;
    }
    let person = unsafe { &mut *instance_ptr };
    let res = match get_call_message::<Greetings>(ptr, len){
        Ok(msg) => msg,
        Err(err) => {
            if set_call_result(&err, out_ptr, out_len){
                return -2;
            }
            else {
                return -3;
            }
        }
    };
    let response = Response{ text: person.greet(&res.name) };
    if !set_call_result(&response, out_ptr, out_len){
        return -1;
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn free_person(ptr: *mut Person){
    if !instance_exists(ptr as *mut c_void){
        return;
    }
    unsafe {
        drop(Box::from_raw(ptr));
    }
}