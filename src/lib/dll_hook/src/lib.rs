#![feature(libc, asm, asm_sym, naked_functions)]
#[macro_use] extern crate detour;
#[macro_use] extern crate lazy_static;

extern crate winapi;
extern crate kernel32;
extern crate libc;
use delphi::*;
use structs::*;
mod delphi;
mod structs;

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
    },
    DLL_THREAD_ATTACH => {}
    DLL_THREAD_DETACH => {}
    _ => {}
  };

  return TRUE;
}

fn init() {
    println!("Initializing..");

    hijack!(0x00950f80, GET_OPTIMIZATION_DATA, GetOptimizationData,
            (param1: i32, param2: i32, param3: i32) -> i32 {
                println!("Detour for GetOptimizationDataFN: param1: ({} {} {})", param1, param2, param3);
                0
            }
    );
    hijack!(0x008b09d8, COMPUTE_REPETITION_PARAMETERS, ComputeRepetitionParameters,
            (param1: i32, param2: i32, param3: i32) -> i32 {
                println!("Detour for GetOptimizationDataFN: param1: ({} {} {})", param1, param2, param3);
                0
            }
    );
    hijack!(0x00955488, ALGORITHM_OUTCOMES, AlgorithmOutcomes,
            (param1: i32, param2: i32, param3: i32, param4: i32) -> f64 {
                println!("Detour for AlgorithmOutcomesFN: param1: ({} {} {} {})", param1, param2, param3, param4);
                1000.0
            }
    );
    //unsafe { (*ALGORITHM_OUTCOMES).detour.disable().unwrap() };

    foreign_fn!(0x008ae4cc, TEST_FN_PTR, fn(f64, f64, f64) -> f64);

    hijack!(0x008b0530, TEST_DETOUR, TestDetour,
        (param1: i32) -> f64 {
            println!("Detour for TestDetourFN: param1: ({})", param1);
            1000.0
        }
    );
    hijack!(0x008b03b8, TEST_DETOUR2, TestDetour2,
        (param1: i32) -> f64 {
            println!("Detour for TestDetour2FN: param1: ({})", param1);
            500.0
        }
    );

    foreign_fn!(0x008b0630, TEST2_FN_PTR, fn(i32, i32) -> f64);

    hijack!(
        0x00b23340, ENTRY_POINT, EntryPoint,
        () -> i32 {
            //test_test2_fn(1,5);
            //let relative_distance: u32 = std::ptr::read(((*TEST2_FN_PTR as usize) + 1) as *const u32);
            //let address: u32 = (*TEST2_FN_PTR as u32) + relative_distance + 5;
            //pretty_print_code_at_address(address, 160);
            println!("--- ");
            //let result = register_call4_f64((*ALGORITHM_OUTCOMES_FN_PTR) as usize, 11,22,33,44);
            let result = register_call4_f64((*ALGORITHM_OUTCOMES).fn_ptr as usize, 11,22,33,44);
            let _ = register_call3((*GET_OPTIMIZATION_DATA).fn_ptr as usize, 11,22,33);
            println!("heres the detour. put your code in here");
            let result: f64 = (*TEST_FN_PTR)(0.0,0.0,0.0);
            println!("result: {}", result);
            let result: f64 = (*TEST_FN_PTR)(1.0,1.0,1.0);
            println!("result: {}", result);
            let result: f64 = (*TEST_FN_PTR)(5.0,1.5,1.5);
            println!("result: {}", result);
            let result = register_call2_f64(*TEST2_FN_PTR as usize, 55, 64);
            println!("result(55,64): {}", result);
            let result = register_call2_f64(*TEST2_FN_PTR as usize, 40, 1);
            println!("result(40,0): {}", result);
            let result = register_call2_f64(*TEST2_FN_PTR as usize, 30, 0);
            println!("result(30,0): {}", result);

            //let relative_distance: u32 = std::ptr::read(((*TEST_DETOUR2_FN_PTR as usize) + 1) as *const u32);
            //let address: u32 = (*TEST_DETOUR2_FN_PTR as u32) + relative_distance + 5;
            //pretty_print_code_at_address(address, 160);
            //println!("--- ");
            //let relative_distance: u32 = std::ptr::read(((*TEST2_FN_PTR as usize) + 1) as *const u32);
            //let address: u32 = (*TEST2_FN_PTR as u32) + relative_distance + 5;
            //pretty_print_code_at_address(address, 160);
            //println!("--- ");
            //let address: u32 = test_detour2_fn as u32;
            //pretty_print_code_at_address(address, 100);
            //println!("--- ");
            //let address: u32 = *TEST2_FN_PTR as u32;
            //pretty_print_code_at_address(address, 148);
            //println!("--- ");

            //println!("F({}, {}) result: {}", 1, 0, result);
            std::process::exit(0)
        }
    );

    println!("Hooks enabled..");
}
