#![crate_type="dylib"]
#![feature(libc)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

extern crate libc;

extern crate jni;

use libc::c_void;
use std::ptr;

use jni::native::*;
use jni::helper::*;

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
      println!("{}", chars_to_str(string));

       ((**jre).ReleaseStringUTFChars)(jre, name, string);
  }
}

#[no_mangle]
pub extern fn Java_org_github_hyunsik_NativeInvoke_returnString(jre: *mut JNIEnv,
    class: *const c_void) -> jstring {
  unsafe {
      return str_to_jstring(jre, "jni native");
  }
}
