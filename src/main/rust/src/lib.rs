#![crate_type="dylib"]
#![feature(libc)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

extern crate libc;

extern crate jni_native;

use libc::c_void;
use std::ffi::{CStr, CString};
use std::ptr;

use jni_native::*;

#[no_mangle]
pub extern fn Java_org_github_hyunsik_NativeInvoke_procedure(jre: *mut JNIEnv,
    class: *const c_void) {
  println!("Invoked native method, jre: {:p}, class: {:p}", jre, class);
  unsafe {
    let v = ((**jre).GetVersion)(jre);
    println!(">> version: {:?}", v);
  }
}

#[no_mangle]
pub extern fn Java_org_github_hyunsik_NativeInvoke_stringArg(jre: *mut JNIEnv,
    class: *const c_void, name: jstring) {
  unsafe {
      let string = ((**jre).GetStringUTFChars)(jre, name, ptr::null_mut());
      let c_str: &CStr = CStr::from_ptr(string);
      let bytes = c_str.to_bytes();
      println!("{}", std::str::from_utf8(bytes).unwrap());

       ((**jre).ReleaseStringUTFChars)(jre, name, string);
  }
}

#[no_mangle]
pub extern fn Java_org_github_hyunsik_NativeInvoke_returnString(jre: *mut JNIEnv,
    class: *const c_void) -> jstring {
  unsafe {
      let c_str = CString::new("jni native".as_bytes()).unwrap();
      return ((**jre).NewStringUTF)(jre, c_str.as_ptr());
  }
}
