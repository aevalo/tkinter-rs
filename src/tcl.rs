use core::str::Utf8Error;
use libc::{c_char, c_double, c_int, c_long, c_longlong, c_ulong, c_void};
use std::ffi::{CStr, CString};
use std::option::Option;

type Tcl_WideInt = c_longlong;

/*
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/tcl-bindings.rs"));
*/

pub enum TCL_ERRORS {
  TCL_OK = 0,
  TCL_ERROR = 1,
}

#[repr(C)]
pub struct Tcl_Interp {
  pub resultDontUse: *mut c_char,
  pub freeProcDontUse: unsafe extern "C" fn(arg1: *mut c_char),
  pub errorLineDontUse: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TwoPtrStuct {
  pub ptr1: *mut c_void,
  pub ptr2: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PtrAndLongStruct {
  pub ptr: *mut c_void,
  pub value: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Tcl_ObjInternalRep {
  pub longValue: c_long,
  pub doubleValue: c_double,
  pub otherValuePtr: *mut c_void,
  pub wideValue: Tcl_WideInt,
  pub twoPtrValue: TwoPtrStuct,
  pub ptrAndLongRep: PtrAndLongStruct,
}

#[repr(C)]
pub struct Tcl_Obj {
  pub refCount: c_int,
  pub bytes: *mut c_char,
  pub length: c_int,
  pub typePtr: *const Tcl_ObjType,
  pub internalRep: Tcl_ObjInternalRep,
}

pub type Tcl_AppInitProc = Option<extern "C" fn(interp: *mut Tcl_Interp) -> c_int>;
type Tcl_FreeInternalRepProc = Option<extern "C" fn(objPtr: *mut Tcl_Obj)>;
type Tcl_DupInternalRepProc = Option<extern "C" fn(srcPtr: *mut Tcl_Obj, dupPtr: *mut Tcl_Obj)>;
type Tcl_UpdateStringProc = Option<extern "C" fn(objPtr: *mut Tcl_Obj)>;
type Tcl_SetFromAnyProc =
  Option<extern "C" fn(interp: *mut Tcl_Interp, objPtr: *mut Tcl_Obj) -> c_int>;
type Tcl_ObjCmdProc = Option<
  extern "C" fn(
    clientData: ClientData,
    interp: *mut Tcl_Interp,
    objc: c_int,
    objv: *const *mut Tcl_Obj,
  ) -> c_int,
>;
type Tcl_CmdDeleteProc = Option<extern "C" fn(clientData: ClientData)>;

pub type ClientData = *mut c_void;

#[repr(C)]
pub struct Tcl_ObjType {
  pub name: *const c_char,
  pub freeIntRepProc: Tcl_FreeInternalRepProc,
  pub dupIntRepProc: Tcl_DupInternalRepProc,
  pub updateStringProc: Tcl_UpdateStringProc,
  pub setFromAnyProc: Tcl_SetFromAnyProc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tcl_Command_ {
  _unused: [u8; 0],
}
pub type Tcl_Command = *mut Tcl_Command_;

#[link(name = "tcl")]
extern "C" {
  fn Tcl_FindExecutable(argv0: *const c_char);
  fn Tcl_CreateInterp() -> *mut Tcl_Interp;
  fn Tcl_Init(interp: *mut Tcl_Interp) -> c_int;
  fn Tcl_Finalize();
  fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
  fn Tcl_DuplicateObj(objPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
  fn Tcl_AppendObjToObj(objPtr: *mut Tcl_Obj, appendObjPtr: *mut Tcl_Obj);
  fn Tcl_CreateObjCommand(
    interp: *mut Tcl_Interp,
    cmdName: *const c_char,
    proc_: Tcl_ObjCmdProc,
    clientData: ClientData,
    deleteProc: Tcl_CmdDeleteProc,
  ) -> Tcl_Command;
  fn Tcl_GetStringResult(interp: *mut Tcl_Interp) -> *const c_char;
  fn Tcl_Eval(interp: *mut Tcl_Interp, script: *const c_char) -> c_int;
  fn Tcl_EvalFile(interp: *mut Tcl_Interp, fileName: *const c_char) -> c_int;
}

pub fn find_executable(argv0: &str) {
  let argv0_c = CString::new(argv0).expect("CString::new failed");
  unsafe {
    Tcl_FindExecutable(argv0_c.as_ptr());
  }
}

pub fn create_interp() -> *mut Tcl_Interp {
  unsafe {
    return Tcl_CreateInterp();
  }
}

pub fn init(interp: *mut Tcl_Interp) -> c_int {
  unsafe {
    return Tcl_Init(interp);
  }
}

pub fn finalize() {
  unsafe {
    Tcl_Finalize();
  }
}

pub fn set_obj_result(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj) {
  unsafe {
    Tcl_SetObjResult(interp, resultObjPtr);
  }
}

pub fn is_shared(objPtr: *mut Tcl_Obj) -> bool {
  unsafe {
    return (*objPtr).refCount > 1;
  }
}

pub fn duplicate_obj(objPtr: *mut Tcl_Obj) -> *mut Tcl_Obj {
  unsafe {
    return Tcl_DuplicateObj(objPtr);
  }
}

pub fn append_obj_to_obj(objPtr: *mut Tcl_Obj, appendObjPtr: *mut Tcl_Obj) {
  unsafe {
    Tcl_AppendObjToObj(objPtr, appendObjPtr);
  }
}

pub fn create_obj_command(
  interp: *mut Tcl_Interp,
  cmdName: &str,
  proc_: Tcl_ObjCmdProc,
  clientData: ClientData,
  deleteProc: Tcl_CmdDeleteProc,
) -> Tcl_Command {
  let cmdName_c = CString::new(cmdName).expect("CString::new failed");
  unsafe {
    return Tcl_CreateObjCommand(interp, cmdName_c.as_ptr(), proc_, clientData, deleteProc);
  }
}

pub fn get_string_result(interp: *mut Tcl_Interp) -> Result<String, Utf8Error> {
  unsafe {
    match CStr::from_ptr(Tcl_GetStringResult(interp)).to_str() {
      Err(error) => Err(error),
      Ok(result) => Ok(String::from(result)),
    }
  }
}

pub fn eval(interp: *mut Tcl_Interp, script: &str) -> c_int {
  let script_c = CString::new(script).expect("CString::new failed");
  unsafe {
    return Tcl_Eval(interp, script_c.as_ptr());
  }
}
pub fn eval_file(interp: *mut Tcl_Interp, fileName: &str) -> c_int {
  let fileName_c = CString::new(fileName).expect("CString::new failed");
  unsafe {
    return Tcl_EvalFile(interp, fileName_c.as_ptr());
  }
}

#[cfg(test)]
mod tests {
  use super::*;
}
