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
                println!("Detour for GetOptimizationData: ({} {} {})", param1, param2, param3);
                0
            }
    );
    hijack!(0x0088ce7c, SUB_88ce7c, Sub88ce7c,
            (param1: i32, param2: i32, param3: i32) -> i32 {
                println!("Detour for Sub88ce7c: ({} {} {})", param1, param2, param3);
                0
            }
    );
    hijack!(0x008b09d8, COMPUTE_REPETITION_PARAMETERS, ComputeRepetitionParameters,
            (param1: i32) -> i32 {
                println!("Detour for ComputeRepetitionParameters: ({})", param1);
                0
            }
    );
    hijack!(0x00955488, ALGORITHM_OUTCOMES, AlgorithmOutcomes,
            (param1: i32, param2: i32, param3: i32, param4: i32) -> f64 {
                println!("Detour for AlgorithmOutcomesFN: ({} {} {} {})", param1, param2, param3, param4);
                1000.0
            }
    );

    foreign_fn!(0x008ae4cc, TEST_FN_PTR, fn(f64, f64, f64) -> f64);

    hijack!(0x008b0530, TEST_DETOUR, TestDetour,
        (param1: i32) -> f64 {
            println!("Detour for TestDetourFN: ({})", param1);
            1000.0
        }
    );
    hijack!(0x008b03b8, TEST_DETOUR2, TestDetour2,
        (param1: i32) -> f64 {
            println!("Detour for TestDetour2FN: ({})", param1);
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
            let result = register_call4_f64((*ALGORITHM_OUTCOMES).fn_ptr as usize, 11,22,33,44);
            // Set up a proper testing enviroment and taking advice of properties and fixtures
            let mock_item_optimization_data = ItemOptimizationData {
                ElementNo: 0,
                Today: 0,
                TheGrade: 0,
                TheFirstGrade: 0,
                LastRepetition: 0,
                OldInterval: 0,
                UsedInterval: 0,
                VirtualUsedInterval: 0,
                OI16: 0,
                NI16: 0,
                OI17: 0.0,
                NI17: 0.0,
                NewInterval: 0,
                Repetitions: 0,
                NewRepetitions: 0,
                Lapses: 0,
                NewLapses: 0,
                RequestedFI: 0,
                Ordinal: 0.0,
                AFactor: 0.0,
                NewAFactor: 0.0,
                UFactor: 0.0,
                NewUFactor: 0.0,
                OldRF: 0.0,
                NewRF: 0.0,
                OldOF: 0.0,
                NewOF: 0.0,
                Cases: 0,
                EstimatedFI: 0.0,
                ExpectedFI: 0.0,
                NormalizedGrade: 0.0,
                NGMin: 0.0,
                NGMax: 0.0,
                RepetitionsCategory: 0.0,
                RepetitionsHistory: 0,
                ExpR: 0.0,
                UsualR: 0.0,
                PredictedR: 0.0,
                Postpones: 0,
                _ItemDifficulty: 0.0,
            };
            let result = register_call4_f64((*ALGORITHM_OUTCOMES).detour.trampoline() as *const _ as usize, 1,2,3,&mock_item_optimization_data as *const _ as i32);
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
