extern crate libc;

use native::*;
use libc::c_char;
use std;
use std::ffi::{CStr, CString};

#[inline]
pub fn str_to_chars(s: &str) -> *const c_char {
  CString::new(s.as_bytes()).unwrap().as_ptr()
}

#[inline]
pub unsafe fn str_to_jstring(jre: *mut JNIEnv, s: &str) -> jstring {
  ((**jre).NewStringUTF)(jre, CString::new(s.as_bytes()).unwrap().as_ptr())
}

#[inline]
pub unsafe fn chars_to_str<'a>(chars: *const c_char) -> &'a str {
  let c_str: &CStr = CStr::from_ptr(chars);
  let bytes = c_str.to_bytes();
  std::str::from_utf8(bytes).unwrap()
}
