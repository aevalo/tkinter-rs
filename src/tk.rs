/*
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/tk-bindings.rs"));
*/

use crate::tcl::{create_interp, Tcl_AppInitProc, Tcl_Interp, TCL_ERRORS};
use libc::{c_char, c_int};
use std::ffi::CString;
use std::vec::Vec;
use std::{mem, ptr};

#[repr(C)]
#[derive(Copy, Clone)]
struct Tk_Window_ {
  _unused: [u8; 0],
}
pub type Tk_Window = *mut Tk_Window_;

#[link(name = "tk")]
extern "C" {
  fn Tk_MainLoop();
  fn Tk_MainEx(
    argc: c_int,
    argv: *mut *mut c_char,
    appInitProc: Tcl_AppInitProc,
    interp: *mut Tcl_Interp,
  );
  fn Tk_Init(interp: *mut Tcl_Interp) -> c_int;
  fn Tk_MainWindow(interp: *mut Tcl_Interp) -> Tk_Window;
}

pub fn main_loop() {
  unsafe {
    Tk_MainLoop();
  }
}

unsafe fn free_string_array(ptr: *mut *mut c_char, len: c_int) {
  println!("String array @ {:?} with length {:?}", ptr, len);
  let len = len as usize;

  // Get back our vector.
  // Previously we shrank to fit, so capacity == length.
  let v = Vec::from_raw_parts(ptr, len, len);

  // Now drop one string at a time.
  for elem in v {
    // From the docs:
    // "(CString::from_raw()) Retakes ownership of a CString that was transferred to C via CString::into_raw."
    let _ = CString::from_raw(elem);
  }

  // Afterwards the vector will be dropped and thus freed.
}

pub fn main_ex(args: Vec<String>, appInitProc: Tcl_AppInitProc, interp: *mut Tcl_Interp) {
  unsafe {
    let mut v = vec![];
    // Let's fill a vector with null-terminated strings
    for arg in args {
      v.push(CString::new(arg.as_str()).unwrap());
    }
    // Turning each null-terminated string into a pointer.
    // `into_raw` takes ownershop, gives us the pointer and does NOT drop the data.
    let mut out = v.into_iter().map(|s| s.into_raw()).collect::<Vec<_>>();
    // Make sure we're not wasting space.
    out.shrink_to_fit();
    assert!(out.len() == out.capacity());
    // Get the pointer to our vector.
    let argc = out.len();
    let argv = out.as_mut_ptr();
    mem::forget(out);
    // Start Tk main loop
    Tk_MainEx(argc as c_int, argv, appInitProc, interp);
    println!("String array @ {:?} with length {:?}", argv, argc);

    free_string_array(argv, argc as c_int);
  }
}

pub fn main(args: Vec<String>, appInitProc: Tcl_AppInitProc) {
  unsafe {
    let interp = create_interp();
    main_ex(args, appInitProc, interp);
  }
}

pub fn init(interp: *mut Tcl_Interp) -> c_int {
  unsafe {
    return Tk_Init(interp);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
}
