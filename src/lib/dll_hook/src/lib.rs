#![feature(libc)]
 
extern crate winapi;
extern crate kernel32;
#[macro_use] extern crate detour;
#[macro_use] extern crate lazy_static;
extern crate libc;

use detour::*;

type createmove_fn = fn(f32, *mut UserCmd) -> bool;

struct UserCmd {
    /* dscode here */
}

struct FunctionPtrAddress {
    addy: createmove_fn
}

lazy_static! {
    static ref fn_ptrs: FunctionPtrAddress = FunctionPtrAddress {
        addy: unsafe {
            std::mem::transmute::<usize, createmove_fn>(0x004111d6)
        }
    };
}

static_detour! {
    static CreateMoveDetour: fn(f32, *mut UserCmd) -> bool;
}
use winapi::shared::minwindef::{
    HINSTANCE, DWORD, LPVOID, BOOL, TRUE
};
use winapi::um::winnt::{
    DLL_PROCESS_DETACH, DLL_PROCESS_ATTACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH
};
use winapi::um::libloaderapi::{
    DisableThreadLibraryCalls
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
    DLL_PROCESS_ATTACH => init(),
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

    println!("Initializing...");

    let closure_for_createmove = |input_sample_time, cmd| {
        println!("heres the detour. put your code in here");

        std::thread::sleep(std::time::Duration::from_secs(30));
    
        //return (fn_ptrs.addy)(input_sample_time, cmd);
        return true;
    };  

    let mut hook = unsafe { 
        CreateMoveDetour.initialize(createmove_hook, closure_for_createmove).unwrap() 
    };

    unsafe {
        hook.enable().unwrap();
    }

    createmove_hook(1.0, std::ptr::null_mut()); // call this so hook.call works
    hook.call(100.0, std::ptr::null_mut());
}

fn createmove_hook(input_sample_time: f32, cmd: *mut UserCmd) -> bool {
    println!("original function");

    return true;//(fn_ptrs.addy)(input_sample_time, cmd);
}
