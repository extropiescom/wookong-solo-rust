extern crate libloading as lib;
extern crate libc;
extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use serde::Serialize;


use std::ffi::{CString, CStr};
use libc::c_int;
use libc::c_char;

fn try_me() -> lib::Result<u32> {
    let lib = lib::Library::new("/home/chester/wookong-solo-rust/src/libSolo.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern fn()-> u32> = lib.get(b"try_me")?;
        Ok(func())
    }
}

fn get() ->lib::Result<String>  {
    let lib = lib::Library::new("/home/chester/wookong-solo-rust/src/libSolo.so")?;
    unsafe {
        let s = CString::new("count").expect("CString::new failed");
        let func: lib::Symbol<unsafe extern fn(url: &CStr, port: c_int)-> *const c_char> = lib.get(b"get")?;
        Ok(CStr::from_ptr(func( &s, 0 as c_int)).to_string_lossy().into_owned())
    }
}

#[derive(Debug, Serialize, Deserialize)]
 struct Count {
    count: u32,
    }

#[derive(Debug, Serialize, Deserialize)]
  struct Res {
    code:u32,
    result:Count
    }

fn main() {
    println!("Hello, world!");
    let mut _result = try_me();
    println!("{:?}",_result);
    let result;
    match  get() {
        Ok(_n)=> result =_n,
        Err(_err) => panic!(),
    };
    println!("{:#?}",result);
    let u : Res = serde_json::from_str(&result).unwrap();
    println!("{:#?}",u);
    println!("{:?}",u.result.count);
    
}