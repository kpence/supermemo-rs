#![feature(libc)]

extern crate winapi;
extern crate kernel32;
#[macro_use] extern crate detour;
#[macro_use] extern crate lazy_static;
extern crate libc;

use detour::*;
use std::{ffi::CString, iter};
use winapi::ctypes::c_int;

macro_rules! hook {
    (
        $ADDR:expr,
        $FN_PTR_NAME:ident,
        $DETOUR_NAME: ident,
        fn($($FARGS:tt)*) $(-> $FRET:ty)?,
        $DETOUR:expr
    ) => {
        hook!($ADDR, $FN_PTR_NAME, unsafe extern "C" fn($($FARGS)*) $(-> $FRET)?);
        static_detour! { static $DETOUR_NAME: unsafe extern "C" fn($($FARGS)*) $(-> $FRET)?; }
        unsafe { $DETOUR_NAME.initialize(*$FN_PTR_NAME, $DETOUR).unwrap().enable().unwrap() };
    };
    ($ADDR:expr, $FN_PTR_NAME:ident, $FN_SIGNATURE:ty) => {
        lazy_static! {
            static ref $FN_PTR_NAME: $FN_SIGNATURE =
                unsafe { std::mem::transmute::<u32, $FN_SIGNATURE>($ADDR) };
        }
    };
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

fn show_message_box(caption: &str, text: &str) {
    fn wide_string(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(Some(0)).collect()
    }

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
        unsafe {kernel32::AllocConsole()};
        init()
    },
    DLL_THREAD_ATTACH => {}
    DLL_THREAD_DETACH => {}
    _ => {}
  };

  return TRUE;
}

fn init() {
    println!("Initializing..");

    hook!(0x008ae4cc, TEST_FN_PTR, fn(f64, f64, f64) -> f64);
    hook!(0x008b0530, TEST_DETOUR_FN_PTR, TestDetour,
          fn(u8) -> f64,
        |param1| {
            println!("Detour for TestDetourFN: param1: ({})", param1);
            //unsafe { (TEST_DETOUR_FN_PTR)(param1) }
            19.0
        }
    );
    hook!(0x008b03b8, TEST_DETOUR2_FN_PTR, TestDetour2,
          fn(u8, u8, u8, u8) -> f64,
        |param1, param2, param3, param4| {
            println!("Detour for TestDetourFN2: param1: ({})", param1);
            //unsafe { (TEST_DETOUR2_FN_PTR)(param1) }
            1000.0
        }
    );
    hook!(0x008b0630, TEST2_FN_PTR, fn(u32, u8) -> f64);
    hook!(0x00b23340, ENTRY_POINT_FN_PTR, EntryPointDetour,
          fn(),
        || {
            println!("heres the detour. put your code in here");
            let result: f64 = unsafe { (*TEST_FN_PTR)(0.0,0.0,0.0) };
            println!("result: {}", result);
            let result: f64 = unsafe { (*TEST_FN_PTR)(1.0,1.0,1.0) };
            println!("result: {}", result);
            let result: f64 = unsafe { (*TEST_FN_PTR)(5.0,1.5,1.5) };
            println!("result: {}", result);
            for i in 1..100 {
                for j in 0..2 {
                    let result = unsafe { (*TEST2_FN_PTR)(i, j) };
                    println!("F({}, {}) result: {}", i, j, result);
                }
            }
            std::process::exit(0)
        }
    );

    println!("Hooks enabled..");
}

