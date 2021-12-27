#![feature(libc)]
 
extern crate winapi;
extern crate kernel32;
#[macro_use] extern crate detour;
#[macro_use] extern crate lazy_static;
extern crate libc;

use detour::*;
use std::{ffi::CString, iter};
use winapi::ctypes::c_int;

struct UserCmd {
    /* dscode here */
}

struct FnPtrAddress<T> {
    address: T,
}

trait Function {
    type FnSig;
}

trait DetourHook {
    fn hook_detour() -> Result<()>;
}

trait FromAddress<F> where F: Function {
    fn from_address(func_address: u32) -> FnPtrAddress<F::FnSig>;
}

impl<F> FromAddress<F> for F where F: Function {
    fn from_address(func_address: u32) -> FnPtrAddress<F::FnSig> {
        FnPtrAddress::<F::FnSig> {
            address: unsafe { std::mem::transmute_copy::<u32, F::FnSig>(&func_address) }
        }
    }
}

impl DetourHook for EntryPointFn {
    fn hook_detour() -> Result<()> {

        let closure_for_createmove = || {
            println!("heres the detour. put your code in here");
        
            //println!("...");
            //std::thread::sleep(std::time::Duration::from_secs(1));
            //println!(".");
            //println!("DONE SLEEPING");
            //return unsafe {
            //    CreateMoveDetour.call(i)
            //}
            //let result = unsafe { (TEST_FN_PTR)(0.0,0.0,0.0) };
            //println!("result: {}", result);
            //let result = unsafe { (TEST_FN_PTR)(1.0,1.0,1.0) };
            //println!("result: {}", result);
            std::process::exit(0)
        };  

        let hook = unsafe { 
            CreateMoveDetour.initialize(ENTRY_POINT_FN_PTR.address, closure_for_createmove).unwrap()
            //TestDetour.initialize(TEST_DETOUR_FN_PTR, closure_for_createmove).unwrap()
        };

        unsafe {
            hook.enable().unwrap();
        }

        Ok(())
    }
}

macro_rules! impl_fn_ptr {
    ($S:ident, $F:ty) => {
        struct $S { }
        impl Function for $S {
            type FnSig = $F;
        }
    };
}

impl_fn_ptr!(TestFn, unsafe extern "C" fn(f64, f64, f64) -> f64);
impl_fn_ptr!(TestDetourFn, unsafe extern "C" fn(u8) -> f64);
impl_fn_ptr!(EntryPointFn, fn());

static_detour! {
    static CreateMoveDetour: fn();
    static TestDetour: unsafe extern "C" fn(u8) -> f64;
}

lazy_static! {
    static ref ENTRY_POINT_FN_PTR: FnPtrAddress<<EntryPointFn as Function>::FnSig> = EntryPointFn::from_address(0x00b23340);
    static ref TEST_FN_PTR: FnPtrAddress<<TestFn as Function>::FnSig> = TestFn::from_address(0x008ae4cc);
}

//let unit177_sub_008ae4cc_short_math = unsafe { example!(0x008ae4cc, extern "C" fn()) };
//static ref fn_ptrs: FunctionPtrAddress = FunctionPtrAddress::from_address(0x00401000);
// 0x004111d6

// language thing 0x004324dc 0x00432448 0x00432588 0x0042e49c* 0x0042e548- 0x00b2eec4*
// 0x00b23340*
// entry point 0x00b9b09c   0x00b9b0be

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
    // TODO Make it so you encapsulate this closure using traits, make a trait for function
    // addreses for whether they have detours
    
    <EntryPointFn as DetourHook>::hook_detour().unwrap();

    println!("Hook enabled..");
}

