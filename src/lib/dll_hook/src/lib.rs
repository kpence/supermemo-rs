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

impl DetourHook for TestDetour2Fn {
    fn hook_detour() -> Result<()> {
        let closure_detour = move |param1: u8| {
            println!("Detour for TestDetour2FN: param1: {}", param1);
            //unsafe { (TEST_DETOUR_FN_PTR.address)(param1) }
            1000.0
        };  

        let hook = unsafe { 
            TestDetour2.initialize(TEST_DETOUR2_FN_PTR.address, closure_detour).unwrap()
        };

        unsafe {
            hook.enable().unwrap();
        }

        Ok(())
    }
}

impl DetourHook for TestDetourFn {
    fn hook_detour() -> Result<()> {
        let closure_detour = move |param1: u8| {
            println!("Detour for TestDetourFN: param1: {}", param1);
            //unsafe { (TEST_DETOUR_FN_PTR.address)(param1) }
            19.0
        };  

        let hook = unsafe { 
            TestDetour.initialize(TEST_DETOUR_FN_PTR.address, closure_detour).unwrap()
        };

        unsafe {
            hook.enable().unwrap();
        }

        Ok(())
    }
}

impl DetourHook for EntryPointFn {
    fn hook_detour() -> Result<()> {
        let closure_detour = || {
            println!("heres the detour. put your code in here");
            let result = unsafe { (TEST_FN_PTR.address)(0.0,0.0,0.0) };
            println!("result: {}", result);
            let result = unsafe { (TEST_FN_PTR.address)(1.0,1.0,1.0) };
            println!("result: {}", result);
            let result = unsafe { (TEST_FN_PTR.address)(5.0,1.5,1.5) };
            println!("result: {}", result);
            let result = unsafe { (TEST2_FN_PTR.address)(0, 1) };
            println!("test2(0) with topic result: {}", result);
            let result = unsafe { (TEST2_FN_PTR.address)(1, 1) };
            println!("test2(10) with topic result: {}", result);
            let result = unsafe { (TEST2_FN_PTR.address)(40, 0) };
            println!("test2(0) with item result: {}", result);
            let result = unsafe { (TEST2_FN_PTR.address)(50, 0) };
            println!("test2(10) with item result: {}", result);
            std::process::exit(0)
        };  

        let hook = unsafe { 
            EntryPointDetour.initialize(ENTRY_POINT_FN_PTR.address, closure_detour).unwrap()
        };

        unsafe {
            hook.enable().unwrap();
        }

        Ok(())
    }
}

macro_rules! impl_fn_ptr {
    ($S:ident, $ST:ident, $D: ident, $(fn($($FARGS:tt)*) $(-> $FRET:ty)?)+, $ADDR:expr) => {
        impl_fn_ptr!($S, $ST, unsafe extern "C" $(fn($($FARGS)*) $(-> $FRET)?)+, $ADDR);
        static_detour! { static $D: unsafe extern "C" $(fn($($FARGS)*) $(-> $FRET)?)+; }
    };
    ($S:ident, $F:ty) => {
        struct $S { }
        impl Function for $S {
            type FnSig = $F;
        }
    };
    ($S:ident, $ST:ident, $F:ty, $ADDR:expr) => {
        impl_fn_ptr!($S, $F);
        impl_fn_ptr!($S, $ST, $ADDR);
    };
    ($S:ident, $ST:ident, $ADDR:expr) => {
        lazy_static! {
            static ref $ST: FnPtrAddress<<$S as Function>::FnSig> = $S::from_address($ADDR);
        }
    };
}

impl_fn_ptr!(TestFn, TEST_FN_PTR, fn(f64, f64, f64) -> f64, 0x008ae4cc);
impl_fn_ptr!(TestDetourFn, TEST_DETOUR_FN_PTR, TestDetour, fn(u8) -> f64, 0x008b0530);
impl_fn_ptr!(TestDetour2Fn, TEST_DETOUR2_FN_PTR, TestDetour2, fn(u8) -> f64, 0x008b03b8);
impl_fn_ptr!(Test2Fn, TEST2_FN_PTR, fn(u8, u8) -> f64, 0x008b0630);
impl_fn_ptr!(EntryPointFn, ENTRY_POINT_FN_PTR, EntryPointDetour, fn(), 0x00b23340);

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
    
    <EntryPointFn as DetourHook>::hook_detour().unwrap();
    <TestDetourFn as DetourHook>::hook_detour().unwrap();
    <TestDetour2Fn as DetourHook>::hook_detour().unwrap();

    println!("Hook enabled..");
}

