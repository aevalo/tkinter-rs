extern crate tkinter_rs;

use libc::c_int;
use std::env;

use tkinter_rs::{tcl, tk};

extern "C" fn init_proc(interp: *mut tcl::Tcl_Interp) -> c_int {
  // Initialize Tcl
  let mut ret = tcl::init(interp);
  if ret != tcl::TCL_ERRORS::TCL_OK as c_int {
    println!(
      "Failed to initialize Tcl: {:?}",
      tcl::get_string_result(interp)
    );
    return ret;
  }
  // Initialize Tk
  ret = tk::init(interp);
  if ret != tcl::TCL_ERRORS::TCL_OK as c_int {
    println!(
      "Failed to initialize Tk: {:?}",
      tcl::get_string_result(interp)
    );
    return ret;
  }
  return tcl::TCL_ERRORS::TCL_OK as c_int;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  tk::main(args, Some(init_proc));
}
