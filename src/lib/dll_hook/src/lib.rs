#![feature(libc)]
 
extern crate winapi;
extern crate kernel32;
#[macro_use] extern crate detour;
#[macro_use] extern crate lazy_static;
extern crate libc;

use detour::*;
use std::{ffi::CString, iter};
use winapi::ctypes::c_int;

type test_fn = unsafe extern "C" fn(f64, f64, f64) -> f64;
type createmove_fn = fn();

struct UserCmd {
    /* dscode here */
}

struct FunctionPtrAddress2 {
    address: test_fn,
}

struct FunctionPtrAddress {
    address: createmove_fn,
}

impl FunctionPtrAddress {
    pub fn from_address(func_address: u32) -> FunctionPtrAddress {
        FunctionPtrAddress {
            address: unsafe {
                std::mem::transmute::<usize, createmove_fn>(func_address as usize)
            }
        }
    }
}

impl FunctionPtrAddress2 {
    pub fn from_address(func_address: u32) -> FunctionPtrAddress2 {
        FunctionPtrAddress2 {
            address: unsafe {
                std::mem::transmute::<usize, test_fn>(func_address as usize)
            }
        }
    }
}

//let unit177_sub_008ae4cc_short_math = unsafe { example!(0x008ae4cc, extern "C" fn()) };
//static ref fn_ptrs: FunctionPtrAddress = FunctionPtrAddress::from_address(0x00401000);
// 0x004111d6

// language thing 0x004324dc 0x00432448 0x00432588 0x0042e49c*
// 0x0042e548- 0x00b2eec4*
// 0x00b23340*
// entry point 0x00b9b09c   0x00b9b0be

lazy_static! {
    static ref fn_ptrs: FunctionPtrAddress = FunctionPtrAddress::from_address(0x00b23340);
    static ref test_fn_ptrs: FunctionPtrAddress2 = FunctionPtrAddress2::from_address(0x008ae4cc);
}

static_detour! {
    static CreateMoveDetour: fn();
}

use winapi::shared::minwindef::{
    HINSTANCE, DWORD, LPVOID, BOOL, TRUE
};
use winapi::um::winnt::{
    DLL_PROCESS_DETACH, DLL_PROCESS_ATTACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH
};
use winapi::um::libloaderapi::{
    DisableThreadLibraryCalls,
};
use winapi::um::winuser::{
    MessageBoxW, MB_OK
};

use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt as _;

fn wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

fn show_message_box(caption: &str, text: &str) {
    unsafe{
        MessageBoxW(
            null_mut() as _, 
            wide_string(text).as_ptr() as _, 
            wide_string(caption).as_ptr() as _, 
            MB_OK
        );
    }
}

// define dllmain to handle the init action
#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(hinst: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
  match reason {
    DLL_PROCESS_DETACH => {
      println!("Remove from main process.");
    }
    DLL_PROCESS_ATTACH => {
        DisableThreadLibraryCalls(hinst);
        init()
    },
    DLL_THREAD_ATTACH => {}
    DLL_THREAD_DETACH => {}
    _ => {}
  };

  return TRUE;
}

fn init() {
    unsafe { 
        kernel32::AllocConsole() 
    };
    println!("Initializing..");
    let closure_for_createmove = || {
        println!("heres the detour. put your code in here");
    
        //println!("...");
        //std::thread::sleep(std::time::Duration::from_secs(1));
        //println!(".");
        //println!("DONE SLEEPING");
        //return unsafe {
        //    CreateMoveDetour.call(i)
        //}
        let result = unsafe { (test_fn_ptrs.address)(0.0,0.0,0.0) };
        println!("result: {}", result);
        let result = unsafe { (test_fn_ptrs.address)(1.0,1.0,1.0) };
        println!("result: {}", result);
        std::process::exit(0)
    };  

    let hook = unsafe { 
        let address: createmove_fn = fn_ptrs.address;
        CreateMoveDetour.initialize(address, closure_for_createmove).unwrap()
    };

    unsafe {
        hook.enable().unwrap();
    }

    println!("Hook enabled..");
}

