extern crate tkinter_rs;

use libc::c_int;

use tkinter_rs::{tcl, tk};

fn main() {
  let interp = tcl::create_interp();
  // Initialize Tcl
  if tcl::init(interp) != tcl::TCL_ERRORS::TCL_OK as c_int {
    println!(
      "Failed to initialize Tcl: {:?}",
      tcl::get_string_result(interp)
    );
    return;
  }
  // Initialize Tk
  if tk::init(interp) != tcl::TCL_ERRORS::TCL_OK as c_int {
    println!(
      "Failed to initialize Tk: {:?}",
      tcl::get_string_result(interp)
    );
    return;
  }
  // Start Tk main loop
  tk::main_loop();
}
