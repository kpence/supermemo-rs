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

    foreign_fn!(0x00950f80, GET_OPTIMIZATION_DATA_FN_PTR, fn(u32, u32, *const DataRecord));

    // These have been confirmed to be necessary for avoiding errors and side effects
    // These also are TODO and not fully tested and verified
    {
        foreign_fn!(0x0042c3e8, DATE_NOW_FN, fn() -> f64);

        hijack!(0x0094da30, GET_ITEM_INFO, GetItemInfo,
                (database: i32, element_number: i32) -> i32 {
                    println!("Detour for GetItemInfo: ({} {})", database, element_number);
                    // TODO This I think I'll lock in
                    /*
                    TODO
                    What this will need to do:
                    Set Database's ItemInfoOpened
                    ...... TODO
                    */
                    0
                }
        );
    }

    hijack!(0x008b09d8, COMPUTE_REPETITION_PARAMETERS, ComputeRepetitionParameters,
            (param1: i32) -> i32 {
                println!("Detour for ComputeRepetitionParameters: ({})", param1);
                // TODO
                0
            }
    );

    hijack!(
        0x00b23340, ENTRY_POINT, EntryPoint,
        () -> i32 {
            //test_test2_fn(1,5);
            //let relative_distance: u32 = std::ptr::read(((*TEST2_FN_PTR as usize) + 1) as *const u32);
            //let address: u32 = (*TEST2_FN_PTR as u32) + relative_distance + 5;
            //pretty_print_code_at_address(address, 160);
            println!("--- ");
            //let result = register_call4_f64((*ALGORITHM_OUTCOMES).fn_ptr as usize, 11,22,33,44);
            // Set up a proper testing enviroment and taking advice of properties and fixtures
            let mock_optimization_record = OptimizationRecord { ..Default::default() };
            let mock_item_optimization_data = ItemOptimizationData { ..Default::default() };
            let mock_data_record = DataRecord { ..Default::default() };
            //let result = register_call4_f64((*ALGORITHM_OUTCOMES).detour.trampoline() as *const _ as usize, 1,2,3,&mock_item_optimization_data as *const _ as i32);
            //println!("The new value of item opt data's grade: {}", mock_item_optimization_data.TheGrade);
            //let _ = register_call3((*GET_OPTIMIZATION_DATA).fn_ptr as usize, 11,22,33);
            //println!("heres the detour. put your code in here");
            //let result: f64 = (*TEST_FN_PTR)(0.0,0.0,0.0);
            //println!("result: {}", result);
            //let result: f64 = (*TEST_FN_PTR)(1.0,1.0,1.0);
            //println!("result: {}", result);
            //let result: f64 = (*TEST_FN_PTR)(5.0,1.5,1.5);
            //println!("result: {}", result);
            //let result = register_call2_f64(*TEST2_FN_PTR as usize, 55, 64);
            //println!("result(55,64): {}", result);
            //let result = register_call2_f64(*TEST2_FN_PTR as usize, 40, 1);
            //println!("result(40,0): {}", result);
            //let result = register_call2_f64(*TEST2_FN_PTR as usize, 30, 0);
            //println!("result(30,0): {}", result);

            //for i in 0..10 {
            //    let result = register_call1(*GRADE_BIT_FLAG_FN_PTR as usize, i);
            //    println!("Grade bit flag result({}): {}", i, result);
            //}

            //let result = register_call1((*COMPUTE_REPETITION_PARAMETERS).detour.trampoline() as *const _ as usize, &mock_item_optimization_data as *const _ as i32);
            let new_optimization_records_ptr = 0xca4548 as *mut OptimizationRecord;
            unsafe {
                std::ptr::write(new_optimization_records_ptr, mock_optimization_record);
            }
            let result = register_call3((*GET_OPTIMIZATION_DATA_FN_PTR) as usize, 0, 0, &mock_data_record as *const _ as i32);
            println!("finished compute repetition parameters: MockDataRecord {:?}", mock_item_optimization_data);

            //let result = register_call1((*FIRST_INTERVAL_VECTOR).detour.trampoline() as *const _ as usize, &mock_optimization_record as *const _ as i32);
            //let result = register_call1((*D_FACTOR_1).detour.trampoline() as *const _ as usize, &mock_optimization_record as *const _ as i32);
            //let result = register_call1((*RECOMPUTE_O_F_MATRIX).detour.trampoline() as *const _ as usize, &mock_optimization_record as *const _ as i32);
            //println!("finished compute repetition parameters: NewAFactor {}", mock_item_optimization_data.NewAFactor);

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
