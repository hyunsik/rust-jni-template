#![crate_type="dylib"]
#![feature(libc)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

extern crate jni_native;

use libc::c_void;

use jni_native::JNIEnv;

#[no_mangle]
pub extern fn Java_org_github_hyunsik_NativeMain_helloJre(jre: *mut JNIEnv, class: *const c_void) {
  println!("Invoked native method, jre: {:p}, class: {:p}", jre, class);
  unsafe {
    let v = ((**jre).GetVersion)(jre);
    println!(">> version: {:?}", v);
  }
}
