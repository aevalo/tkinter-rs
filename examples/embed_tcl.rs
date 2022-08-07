extern crate tkinter_rs;

use libc::c_int;
use std::env;
use std::vec::Vec;

use tkinter_rs::tcl;

extern "C" fn string_cat_cmd(
  dummy: tcl::ClientData,         // Not used.
  interp: *mut tcl::Tcl_Interp,   // Current interpreter.
  objc: c_int,                    // Number of arguments.
  objv: *const *mut tcl::Tcl_Obj, // Argument objects.
) -> c_int {
  unsafe {
    let obj_values = std::slice::from_raw_parts(objv, objc as usize);
    //let obj_values: Vec<*mut tcl::Tcl_Obj> = Vec::from_raw_parts(objv, objc as usize, objc as usize);
    if objc < 2 {
      /*
       * If there are no args, the result is an empty object.
       * Just leave the preset empty interp result.
       */
      return tcl::TCL_ERRORS::TCL_OK as c_int;
    }
    if objc == 2 {
      /*
       * Other trivial case, single arg, just return it.
       */
      tcl::set_obj_result(interp, obj_values[1]);
      return tcl::TCL_ERRORS::TCL_OK as c_int;
    }
    let mut objResultPtr = obj_values[1];
    if tcl::is_shared(objResultPtr) {
      objResultPtr = tcl::duplicate_obj(objResultPtr);
    }
    let mut i: i32 = 2;
    loop {
      tcl::append_obj_to_obj(objResultPtr, obj_values[i as usize]);
      i += 1;
      if i >= objc {
        break;
      }
    }
    tcl::set_obj_result(interp, objResultPtr);
    return tcl::TCL_ERRORS::TCL_OK as c_int;
  }
}

fn extend_tcl(interp: *mut tcl::Tcl_Interp) -> c_int {
  let command = tcl::create_obj_command(
    interp,
    "stringcat",
    Some(string_cat_cmd),
    std::ptr::null_mut(),
    None,
  );
  if command == std::ptr::null_mut() {
    return tcl::TCL_ERRORS::TCL_ERROR as c_int;
  }
  return tcl::TCL_ERRORS::TCL_OK as c_int;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  tcl::find_executable(args[0].as_str());
  let interp = tcl::create_interp();
  if tcl::init(interp) != tcl::TCL_ERRORS::TCL_OK as c_int {
    return;
  }
  if extend_tcl(interp) != tcl::TCL_ERRORS::TCL_OK as c_int {
    println!("Tcl_Init error: {:?}", tcl::get_string_result(interp));
  }
  tcl::eval(interp, "stringcat foo bar");
  println!("{:?}", tcl::get_string_result(interp).unwrap());
  tcl::finalize();
}
