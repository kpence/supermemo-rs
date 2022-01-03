#![feature(libc, asm, asm_sym, naked_functions)]
#[macro_use] extern crate detour;
#[macro_use] extern crate lazy_static;

extern crate winapi;
extern crate kernel32;
extern crate libc;
use delphi::*;
use structs::*;
use dev_helpers::*;
mod delphi;
mod structs;
mod dev_helpers;

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
use std::os::raw::c_char;
use std::ffi::{CString, CStr, OsStr};
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

    {
        // TODO This is for stuff that loads data from my files
        // TODO 961454
    }
    {
        // TODO Calls from OnEntry
        hijack!(0x00b0dc10, FILE_B0DC10, FileB0dc10, () -> i32 {println!("Detour for FileB0dc10..."); 0});
        hijack!(0x0072127c, READ_YOUTUBE_SCRIPT, ReadYoutubeScript, () -> i32 {println!("Detour for ReadYoutubeScript..."); 0});
    }
    {
        // These have been confirmed to be necessary for avoiding errors and side effects
        // These also are TODO and not fully tested and verified
        foreign_fn!(0x0042c3e8, DATE_NOW_FN, fn() -> f64);

        hijack!(0x0094da30, GET_ITEM_INFO, GetItemInfo,
                (database_addr: i32, element_number: i32, item_info_addr: i32) -> i32 {
                    println!("Detour for GetItemInfo: ({} {} {})", database_addr, element_number, item_info_addr);

                    // TODO This function will be incredibly important
                    // TODO this is currently just sending mock data into item info (just for testing)
                    let mut mock_item_info = ItemInfo { LastRepetition: 15330, EStatus:1, ..Default::default() };
                    unsafe {
                        std::ptr::write(item_info_addr as *mut ItemInfo, mock_item_info)
                    };
                    //println!("Resulting item_info: {:?}", mock_item_info);
                    0
                }
        );
        // TODO Hijack database path, and mount a volume to your docker with the path and then override databasepath to point to that
        // TODO ParamStr - Override this maybe, or make sure you document how it is used??
    }

    // TODO For testing, but I might keep
    //foreign_fn!(0x00950f80, GET_OPTIMIZATION_DATA_FN, fn(u32, u32, *const DataRecord));
    hijack!(0x00950f80, GET_OPTIMIZATION_DATA, GetOptimizationData,
            (database_addr: i32, element_number: i32, item_optization_data_addr: i32) -> i32 {
                println!("Detour for GetOptimizationData: ({} {} {})",
                         database_addr, element_number, item_optization_data_addr as u32);
                let result = register_call3((*GET_OPTIMIZATION_DATA).detour.trampoline() as *const _ as usize, database_addr, element_number, item_optization_data_addr);
                let item_optimization_data = unsafe {
                    std::ptr::read(item_optization_data_addr as *mut ItemOptimizationData)
                };
                println!("RESULT OF GetOptimizationData: {} {:?}", result, item_optimization_data);
                result
            }
    );


    // TODO for testing purpose only, I might delete this soon
    //foreign_fn!(0x008b09d8, COMPUTE_REPETITION_PARAM_FN, fn(u32));
    //hijack!(0x008b09d8, COMPUTE_REPETITION_PARAM, ComputeRepetitionParam,
    //        (item_optimization_data_addr: i32) -> i32 {
    //            let item_optimization_data = unsafe {
    //                std::ptr::read(item_optimization_data_addr as *mut ItemOptimizationData)
    //            };
    //            println!("Detour for ComputeRepetitionParam: {}{:?}", item_optimization_data_addr, item_optimization_data);
    //            let result = register_call1((*COMPUTE_REPETITION_PARAM).detour.trampoline() as *const _ as usize, item_optimization_data_addr);
    //            let item_optimization_data = unsafe {
    //                std::ptr::read(item_optimization_data_addr as *mut ItemOptimizationData)
    //            };
    //            println!("RESULT OF ComputeRepetitionParam: {} {:?}", result, item_optimization_data);
    //            result
    //        }
    //);
    //hijack!(0x0088ce7c, UPDATE_MEMORY_MATRICES, UpdateMemoryMatrices,
    //        (item_optimization_data_addr: i32) -> i32 {
    //            let item_optimization_data = unsafe {
    //                std::ptr::read(item_optimization_data_addr as *mut ItemOptimizationData)
    //            };
    //            println!("Detour for UPDATE_MEMORY_MATRICES: {}{:?}", item_optimization_data_addr, item_optimization_data);
    //            //let result = register_call1((*COMPUTE_REPETITION_PARAM).detour.trampoline() as *const _ as usize, item_optimization_data_addr);
    //            //let item_optimization_data = unsafe {
    //            //    std::ptr::read(item_optimization_data_addr as *mut ItemOptimizationData)
    //            //};
    //            //println!("RESULT OF ComputeRepetitionParam: {} {:?}", result, item_optimization_data);
    //            //result
    //            0
    //        }
    //);

    foreign_fn!(0x008afbc4, COMPUTE_NEW_INTERVAL_FN, fn(i32) -> i32);
    //hijack!(0x008b158c, COMPUTE_USED_INTERVAL, ComputeUsedInterval,
    //        (element_number: i32, today: i32, last_repetition: i32) -> i32 {
    //            println!("Detour for ComputeUsedInterval: ({} {} {})",
    //                     element_number, today, last_repetition);
    //            // TODO
    //            2
    //        }
    //);

    foreign_fn!(0x008b158c, COMPUTE_USED_INTERVAL_FN, fn(i32, i32, i32) -> i32);
    //hijack!(0x008b158c, COMPUTE_USED_INTERVAL, ComputeUsedInterval,
    //        (element_number: i32, today: i32, last_repetition: i32) -> i32 {
    //            println!("Detour for ComputeUsedInterval: ({} {} {})",
    //                     element_number, today, last_repetition);
    //            // TODO
    //            2
    //        }
    //);

    foreign_fn!(0x00955488, ALGORITHM_OUTCOMES_FN, fn(i32, i32, i32, i32) -> i32);
    //hijack!(0x00955488, ALGORITHM_OUTCOMES, AlgorithmOutcomes,
    //        (database: i32, element_number: i32, grade: i32, item_optimization_data: i32) -> i32 {
    //            println!("Detour for AlgorithmOutcomes: ({} {} {} {})",
    //                     database, element_number, grade, item_optimization_data);
    //            // TODO
    //            0
    //        }
    //);

    //hijack!(0x008b09d8, COMPUTE_REPETITION_PARAMETERS, ComputeRepetitionParameters,
    //        (param1: i32) -> i32 {
    //            println!("Detour for ComputeRepetitionParameters: ({})", param1);
    //            // TODO
    //            0
    //        }
    //);
    // TODO this is just for a quick test of string types
    foreign_fn!(0x00427548, LOWER_CASE_FN, fn(i32, i32) -> i32);
    foreign_fn!(0x0040d71c, U_STR_SET_LENGTH, fn(i32, i32) -> i32);

    hijack!(
        0x00b23340, ENTRY_POINT, EntryPoint,
        () -> i32 {
            //test_test2_fn(1,5);
            //let relative_distance: u32 = std::ptr::read(((*TEST2_FN as usize) + 1) as *const u32);
            //let address: u32 = (*TEST2_FN as u32) + relative_distance + 5;
            //pretty_print_code_at_address(address, 160);
            println!("--- ");
            //let result = register_call4_f64((*ALGORITHM_OUTCOMES).fn_ptr as usize, 11,22,33,44);
            // Set up a proper testing enviroment and taking advice of properties and fixtures

            let mut mock_temporary_optimization_record = OptimizationRecord { ..Default::default() };
            mock_temporary_optimization_record.OFM[0] = 69;
            let mut mock_optimization_record = OptimizationRecord { ..Default::default() };
            mock_optimization_record.OFM[0] = 42;
            let mock_data_record = DataRecord { ..Default::default() };
            let mut mock_database = Database { SessionToday: 1, ..Default::default() };

            // TODO Isolate all these "globals" that are used by the foreign process
            let temporary_optimization_records_ptr      = 0xca4548 as *mut *mut OptimizationRecord;
            let optimization_records_ptr                = 0xca4544 as *mut *mut OptimizationRecord;
            let is_temporary_optimization_records_dirty = 0xca4558 as *mut bool;
            unsafe {
                std::ptr::write(temporary_optimization_records_ptr,      &mut mock_temporary_optimization_record);
                std::ptr::write(optimization_records_ptr,                &mut mock_optimization_record);
                std::ptr::write(is_temporary_optimization_records_dirty, true)
            };
            let database_ptr = 0xca60d0 as *mut *mut Database;
            unsafe {
                std::ptr::write(database_ptr, &mut mock_database)
            };

            // TODO Compute Used Interval testing
            //{
            //    let element_number = 1;
            //    let today = 1;
            //    let last_repetition = 1;
            //    let result = register_call3((*COMPUTE_USED_INTERVAL_FN) as usize, element_number,today,last_repetition);
            //    println!("Result of calling COMPUTE_USED_INTERVAL_FN(1,1,1) = {}", result);
            //}

            // TODO Compute New Interval testing
            //{
            //    let mock_item_optimization_data = ItemOptimizationData { UsedInterval: 1, ..Default::default() };
            //    let result = register_call1((*COMPUTE_NEW_INTERVAL_FN) as usize, &mock_item_optimization_data as *const _ as i32);
            //    println!("Result of calling COMPUTE_NEW_INTERVAL_FN() = {}", result);
            //}

            // TODO Testing Get optimization data
            {
                println!("Database before call: {:?}", mock_database);
                //let result = register_call4((*ALGORITHM_OUTCOMES_FN) as usize, &mock_database as *const _ as i32, 0, 3, &mock_data_record as *const _ as i32);

                let mut mock_item_optimization_data = ItemOptimizationData { UsedInterval: 1, ..Default::default() };
                println!("before running functions: mock_item_optimization_data {:?}", mock_item_optimization_data);

                //let result = register_call3((*GET_OPTIMIZATION_DATA_FN) as usize, &mock_database as *const _ as i32, 0, &mut mock_item_optimization_data as *mut _ as i32);

                let result = register_call4((*ALGORITHM_OUTCOMES_FN) as usize, &mock_database as *const _ as i32, 1, 2, &mut mock_item_optimization_data as *mut _ as i32);
                println!("finished compute repetition parameters: mock_item_optimization_data {:?}", mock_item_optimization_data);
                println!("After calling ALGORITHM_OUTCOMES, this is what mock_tempor:ary_optimization_record = {:?}", mock_temporary_optimization_record);

                // TODO Testing UnicodeString
                {
                    //{
                    //    let mut lower_case_result = empty_delphi_utf16_string();
                    //    let mut utf16 = into_delphi_utf16_string(String::from("HELLO"));
                    //    register_call2((*LOWER_CASE_FN) as usize, &mut utf16 as *mut _ as i32, &mut lower_case_result as *mut _ as i32);
                    //    println!("Test: 'HELLO' = {:?}", utf16);
                    //    println!("Test: LOWER_CASE('HELLO') = {:?}", lower_case_result);
                    //    println!("Test: LOWER_CASE('HELLO') = {}", unsafe { &from_delphi_utf16_string(&lower_case_result, 4) });
                    //    let mut lower_case_result = empty_delphi_utf16_string();
                    //    let mut utf16 = into_delphi_utf16_string(String::from("HELLO fdsafdsafjlk;a TREWTQWTEWQ FVDSAFDSA LJSDAJFGLKJDSAJL"));
                    //    register_call2((*LOWER_CASE_FN) as usize, &mut utf16 as *mut _ as i32, &mut lower_case_result as *mut _ as i32);
                    //    println!("Test2: LOWER_CASE('HELLO') = {:?}", lower_case_result);
                    //    println!("Test2: LOWER_CASE('HELLO') = {:?}", lower_case_result);
                    //    println!("Test2: LOWER_CASE('HELLO') = {}", unsafe { &from_delphi_utf16_string(&lower_case_result, 4) });
                    //}
                    //let mut empty_string = empty_delphi_utf8_string();
                    //register_call2((*U_STR_SET_LENGTH) as usize, &mut empty_string as *mut _ as i32, 5);
                    //println!("Test3: U_STR_SET_LENGTH = {:?}", empty_string);

                    //unsafe {
                    //    let p = utf16.as_ptr() as *mut u16;
                    //    for i in 0..14 {
                    //        let v = std::ptr::read(p.add(i));
                    //        println!("Test4 -- HELLO UnicodeString before {}: {:x}", i, v)
                    //    }
                    //}

                    //let mut utf16 = UnicodeString::from("HELLO world");
                    //let mut result = UnicodeString::default();
                    ////let mut result = UnicodeString::default();
                    //register_call2((*LOWER_CASE_FN) as usize, unsafe { utf16.buffer_ptr() } as i32, result.as_ptr() as i32);
                    //println!("Test4: 'HELLO world' after = {:?}, len: {}", String::from(&result), result.len());

                    ////let mut utf16 = from_delphi_utf16_string(utf16);
                    ////println!("Test4: 'HELLO' after = {:?}", utf16);

                    ////let mut utf16 = into_delphi_utf16_string(String::from("HELLO WORLD"));
                    ////println!("Test5: 'HELLO WORLD' before = {:?}", utf16);
                    //let mut utf16 = UnicodeString::from("tELLO");

                    //let mut utf16 = UnicodeString::from("F");
                    //unsafe {
                    //    let p = utf16.as_ptr() as *mut u16;
                    //    for i in 0..14 {
                    //        let v = std::ptr::read(p.add(i));
                    //        println!("Test4 -- HELLO UnicodeString before {}: {:x}", i, v)
                    //    }
                    //}

                    //let mut utf16 = UnicodeString::from("tELLO");
                    let mut result = UnicodeString::default();
                    let mut utf16 = UnicodeString::from("C:\\FDASFSDAFDS\\FDSAFDSAFDSA\\FDSAFSDAFDS\\fdsa.txt");
                    register_call2((*LOWER_CASE_FN) as usize, unsafe { utf16.buffer_ptr() } as i32, result.as_ptr() as i32);
                    println!("Test4: 'HELLO world' after = {:?}, len: {}", String::from(&result), result.len());

                    //register_call2((*U_STR_SET_LENGTH) as usize, utf16.as_ptr() as i32, 7);
                    //println!("Test5: After call..");
                    //println!("Test5: 'test' = {:?}", String::from(&utf16));
                    //let mut utf16 = UnicodeString::from("HELLO");
                    //register_call2((*U_STR_SET_LENGTH) as usize, utf16.as_ptr() as i32, 11);
                    //println!("Test5: 'HELLO' after = {:?}", String::from(&utf16));
                }

                println!("Reached end of entry point tests.");
            }

            std::process::exit(0)
        }
    );

    println!("Hooks enabled..");
}
