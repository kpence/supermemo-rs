#![feature(libc, asm, asm_sym, naked_functions)]
extern crate detour;
extern crate kernel32;
extern crate libc;
extern crate winapi;
use delphi::*;
use call_native_fn::{
    CallNativeFn,
    EXECUTION_FINISH_EVENT_TRANSMITTER,
    EXECUTION_FINISH_EVENT_RECEIVER,
};
use detour::RawDetour;

mod delphi;
mod dev_helpers;
mod structs;
mod supermemo;
mod call_native_fn;

use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::libloaderapi::DisableThreadLibraryCalls,
    um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH},
};

use std::sync::mpsc::channel;

use once_cell::sync::Lazy;

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
            kernel32::AllocConsole();
            init()
        }
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        _ => {}
    };

    return TRUE;
}

fn init() {
    println!("Initializing..");

    //hijack!("E8 ? ? ? ? 80 7D FB 24", EL_WDW_GO_TO_ELEMENT, ElWdwGoToElement)
    //hijack!(0x00b21df0, EL_WDW_GO_TO_ELEMENT, ElWdwGoToElement, (arg1: i32, arg2: i32));
    hijack!(0x008b1520, MATH_STUFF, MathStuff, (arg1: i32) -> f64);

    let (sender, receiver) = channel();
    *EXECUTION_FINISH_EVENT_TRANSMITTER.lock().unwrap() = Some(sender);
    *EXECUTION_FINISH_EVENT_RECEIVER.lock().unwrap() = Some(receiver);

    if false {
        println!("\n\n\n\n\n\n\n\n\n-------------------------------------------------------------------------Calling native function now!");
        let result = <i32 as CallNativeFn<f64>>::call_native_fn(&((*MATH_STUFF).0), 5);
        println!("\n\n\n\n\n\n\n\n\n--------------------------------------------------directly after calling native function, now gonna wait!");
        std::thread::sleep(std::time::Duration::from_secs(20));
        println!("\n\n\n\n\n\n\n\n\n--------------------------------------------------------------------------done sleeping");
        println!("result is {}", result.unwrap_f64());
    }

    println!("Hooks enabled..");
}
