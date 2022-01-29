#![feature(libc, asm, asm_sym, naked_functions)]
extern crate detour;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate utf16_lit;
extern crate winapi;
extern crate kernel32;
extern crate libc;
use delphi::*;
use structs::*;
//use dev_helpers::*;
use supermemo::*;
mod delphi;
mod structs;
mod dev_helpers;
mod supermemo;

use winapi::{
    um::processthreadsapi::GetCurrentProcessId,
    shared::minwindef::{
        HINSTANCE, DWORD, LPVOID, BOOL, TRUE
    },
    um::winnt::{
        DLL_PROCESS_DETACH, DLL_PROCESS_ATTACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH
    },
    um::libloaderapi::{
        DisableThreadLibraryCalls,
    },
    um::winuser::{
        MessageBoxW, MB_OK
    },
    um::tlhelp32::{
        CreateToolhelp32Snapshot,
        Thread32First,
        TH32CS_SNAPTHREAD,
        THREADENTRY32,
    }
};

use utf16_lit::utf16_null;

use std::{
    path::Path,
    fs::File,
    //ffi::OsStr,
};

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

    if false {
        {
            // TODO This is for stuff that loads data from my files
            // TODO 961454
        }
        {
            // TODO Calls from OnEntry
            hijack!(0x00b0dc10, FILE_B0DC10, FileB0dc10, () -> i32);
            hijack!(0x0072127c, READ_YOUTUBE_SCRIPT, ReadYoutubeScript, () -> i32);
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
                        let mock_item_info = ItemInfo { ..Default::default() };
                        std::ptr::write(item_info_addr as *mut ItemInfo, mock_item_info);
                        0
                    }
            );

            // TODO Hijack database path, and mount a volume to your docker with the path and then override databasepath to point to that
            // TODO ParamStr - Override this maybe, or make sure you document how it is used??
        }
    }

    hijack!(0x00950f80, GET_OPTIMIZATION_DATA, GetOptimizationData, (database_addr: i32, element_number: i32, item_optization_data_addr: i32) -> i32);
    if false {
        // TODO For testing, but I might keep
        //foreign_fn!(0x00950f80, GET_OPTIMIZATION_DATA_FN, fn(u32, u32, *const DataRecord));
        hijack!(0x00950f80, GET_OPTIMIZATION_DATA, GetOptimizationData,
                (database_addr: i32, element_number: i32, item_optization_data_addr: i32) -> i32 {
                    println!("Detour for GetOptimizationData: ({} {} {})",
                             database_addr, element_number, item_optization_data_addr as u32);
                    let result = GET_OPTIMIZATION_DATA.call_trampoline(database_addr, element_number, item_optization_data_addr);
                    let item_optimization_data = (item_optization_data_addr as *mut ItemOptimizationData).read();
                    println!("RESULT OF GetOptimizationData: {} {:?}", result, item_optimization_data);
                    result
                }
        );
    }


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

    hijack!(0x00955488, ALGORITHM_OUTCOMES, AlgorithmOutcomes, (arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32);
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
    foreign_fn!(0x0040d71c, U_STR_SET_LENGTH_FN, fn(i32, i32) -> i32);
    foreign_fn!(0x00951570, TODAY_FN, fn(i32) -> i32);
    foreign_fn!(0x00933924, GET_FI_CHANGE_FN, fn(i32) -> f64);
    foreign_fn!(0x0093649c, GET_BURDEN_CHANGE_FN, fn(i32, i32, i32) -> f64);
    foreign_fn!(0x009414f0, GET_LAPSES_CHANGE_FN, fn(i32, i32, i32) -> f64);

    // TODO These are temporary and for debugging
    hijack!(0x00933e38, VERIFY_OPTIMIZATION_RECORD_SIZE, VerifyOptimizationRecordSize, (arg1: i32, arg2: i32) -> i32);
    hijack!(0x0094c300, READ_OPTIMIZATION_RECORD, ReadOptimizationRecord,
            (database_ptr: i32) -> i32 {
                println!("Detour for ReadOptimizationRecord: {}", database_ptr);

                // TODO get number of bytes in file:
                {
                    //let opt_record_file = File::open("C:\\sm\\systems\\testitems\\info\\sm8opt.dat").unwrap();
                    //let file_size = opt_record_file.metadata().unwrap().len();
                    //println!("ReadOptimizationRecord: Setting OptimizationRecordSize to = {}", file_size);
                    //std::ptr::write((database_ptr + 3111) as *mut u16, file_size as u16);
                    println!("ReadOptimizationRecord: Reading OptimizationRecordSize before calling: {}", std::ptr::read((database_ptr + 3111) as *mut u16));
                    //std::ptr::write((database_ptr + 3111) as *mut u16, 0);
                }

                let result = READ_OPTIMIZATION_RECORD.call_trampoline(database_ptr);
                read_optimization_record(database_ptr as usize);
                let database = (database_ptr as *const Database).read();
                println!("ReadOptimizationRecord: After calling function, database.OptimizationRecordSize = {}, database.OptimizationRecord (ptr) = {}", database.OptimizationRecordSize, database.OptimizationRecord as u32);
                result
            }
    );
    hijack!(0x0094c0d0, DATABASE_PATH, DatabasePath,
            (database: i32, out_param_ptr: i32) -> i32 {
                println!("Detour for DatabasePath {} {}...", database, out_param_ptr);
                let result = DATABASE_PATH.call_trampoline(database, out_param_ptr);
                println!("DatabasePath: Directly after calling function...");
                let file_name_ptr = (out_param_ptr as *mut *mut u16).read();
                let mut file_name_s: Vec<u16> = Vec::with_capacity(1000);
                for i in 0..256 {
                    let c: u16 = file_name_ptr.add(i).read();
                    if c == 0 {
                        break;
                    }
                    file_name_s.push(c);
                }
                println!("DatabasePath: After calling function, out_param_ptr = {}", &String::from_utf16_lossy(&file_name_s));
                result
            }
    );

    fn read_optimization_record(database_ptr: usize) {
        let opt_record_file = File::open("C:\\sm\\systems\\testitems\\info\\sm8opt.dat").unwrap();
        let mut optimization_record = OptimizationRecord::from_reader(opt_record_file).unwrap();

       //let database_ptr = 0xca60d0;
        let optimization_record_ptr      = (database_ptr + 1084) as *mut *mut OptimizationRecord;
        let optimization_record_size_ptr = (database_ptr + 3111) as *mut u16;
        unsafe {
            std::ptr::write(optimization_record_size_ptr, std::mem::size_of::<OptimizationRecord>() as u16);
            std::ptr::write(optimization_record_ptr,      &mut optimization_record as *mut OptimizationRecord);
        }
    }

    hijack!(0x00998c48, ENTRY_POINT, EntryPoint,
            (smmain: i32) -> i32 {
                let is_user_registered_ptr = 0x00ca3710 as *mut bool;
                std::ptr::write(is_user_registered_ptr, true);
                ENTRY_POINT.call_trampoline(smmain)
            }
    );

    //hijack!(0x0061da6c, FORM_SET_VISIBLE, FormSetVisible,
    //        (arg1: i32, arg2: i32) -> i32 {
    //            println!("SKipping FormSetVisible... {} {}", arg1, arg2);
    //            0
    //        }
    //);

    hijack!(0x00770608, SUB_770608, Sub770608, (arg1: i32) -> i32 { 0 });
    hijack!(0x009a3210, SET_DEFAULT_THEME, SetDefaultTheme,
            (arg1: i32, arg2: i32) -> i32 {println!("Detour for SetDefaultTheme: {} {}", arg1, arg2); 0});
    //hijack!(0x0099401c, ADD_TO_FILE_PICK, AddToFilePick, (arg1: i32, arg2: i32) -> i32 { 0 });
    hijack!(0x009622cc, OPEN_DATABASE, OpenDatabase, (arg1: i32, arg2: i32) -> i32);
    hijack!(0x00964074, QUEUE_GET_ITEM, QueueGetItem, (arg1: i32, arg2: i32) -> i32 {
        QUEUE_GET_ITEM.call_trampoline(arg1, arg2)
    });
    hijack!(0x008bb624, ITEMS_GET_ITEM, ItemsGetItem, (items_ptr: i32, position: i32) -> i32);
    hijack!(0x009991d4, OPEN_NEW_DATABASE, OpenNewDatabase, (sm_main: i32, database_name: i32) -> i32 /*{
        let mut utf16 = UnicodeString::from("C:\\Users\\wineuser\\systems\\testitems");
        OPEN_NEW_DATABASE.call_trampoline(sm_main, utf16.buffer_ptr() as i32)
    }*/);
    //hijack!(0x0061da6c, SET_VISIBLE, SetVisible, (arg1: i32, arg2: i32) -> i32 {0});
    //hijack!(0x00627ee8, APPLICATION_RUN, ApplicationRun,
    hijack!(0x00998c48, ON_ENTRY, OnEntry,
            (smmain: i32) -> i32 {
                println!("Reached detour for OnEntry");
                println!("Calling original OnEntry");
                ON_ENTRY.call_trampoline(smmain);
                println!("Calling my hook after calling original OnEntry");
                let database_ptr = 0xba9c04 as *mut *mut Database;

                //let mut utf16 = UnicodeString::from("C:\\Users\\wineuser\\systems\\testitems");
                //OPEN_NEW_DATABASE.call_trampoline(0x00ca61c0, utf16.as_ptr() as i32);

                let database = database_ptr.read().read();
                println!("database = {:?}", database);
                std::process::exit(0);

                let random_test_items = database.RandomTestItems.read();
                println!("random_test_items = {:?}", random_test_items);
                //let item = ITEMS_GET_ITEM.call_trampoline(random_test_items as i32, 1);
                //println!("random_test_items = {:?}", item);

                //let random_test_items_read = random_test_items.read(); // Why does dereferencing this cause problem happen?

                std::process::exit(0);
                //let size = random_test_items_read.Size;
                //let mut outstanding_elements = Vec::with_capacity(size as usize);
                //TODOfor it in 1..=size {
                //    let mut element_number = QUEUE_GET_ITEM.call_trampoline(randomTestItems as i32, it as i32);
                //    outstanding_elements.push(element_number);
                //}
                //println!("In OnEntry: outstanding_elements: {:?}", outstanding_elements);

                //TODO let mock_recent_grade = 3;
                //ALGORITHM_OUTCOMES.call_trampoline(database_ptr.read() as *const _ as i32,
                //                                   //((elwind_ptr + 3597) as *const i32).read() as i32,
                //                                   1,
                //                                   ((elwind_ptr + 4405) as *const i32).read() as i32,
                //                                   (elwind_ptr + 4406) as i32);
                //println!("Result of algorithm outcomes: Database = {:?}. {:?}", database_ptr.read().read(), database_ptr.read().read().RandomTestItems);

                if false {
                    println!("Press enter to continue execution:");
                    let mut line = String::new();
                    let _ = std::io::stdin().read_line(&mut line).unwrap();
                }
                std::process::exit(0);
                0
            }
    );

    //hijack!(
    //    0x00b23340, ENTRY_POINT, EntryPoint,
    //    () -> i32 {
    //        {
    //            println!("Press enter to continue execution:");
    //            let mut line = String::new();
    //            let _ = std::io::stdin().read_line(&mut line).unwrap();
    //        }
    //        ENTRY_POINT.call_trampoline()
    //    }
    //);

    //hijack!(
    //    0x00b23340, ENTRY_POINT, EntryPoint,
    //    () -> i32 {
    //        ENTRY_POINT.call_trampoline();

    //        // Pausing for input, so I can attach my debugger
    //        //show_message_box("Hit Entry Point", "Hit Entry Point");

    //        //test_test2_fn(1,5);
    //        //let relative_distance: u32 = std::ptr::read(((*TEST2_FN as usize) + 1) as *const u32);
    //        //let address: u32 = (*TEST2_FN as u32) + relative_distance + 5;
    //        //pretty_print_code_at_address(address, 160);
    //        println!("Entered start of Entry Point detour....");
    //        println!("---");

    //        {
    //            println!("Press enter to continue execution:");
    //            let mut line = String::new();
    //            let _ = std::io::stdin().read_line(&mut line).unwrap();
    //        }

    //        //let result = register_call4_f64((*ALGORITHM_OUTCOMES).fn_ptr as usize, 11,22,33,44);
    //        // Set up a proper testing enviroment and taking advice of properties and fixtures

    //        let opt_record_file = File::open("C:\\sm\\systems\\testitems\\info\\sm8opt.dat").unwrap();
    //        let mut optimization_record = OptimizationRecord::from_reader(opt_record_file).unwrap();

    //        //let mock_temporary_optimization_record = OptimizationRecord { ..Default::default() };
    //        //let mock_optimization_record = OptimizationRecord { ..Default::default() };

    //        let _mock_data_record      = DataRecord { ..Default::default() };
    //        let mut mock_primary_space = UnicodeString::from("C:\\sm\\systems\\testitems");
    //        let mut mock_filespace     = Filespace { PrimarySpace: mock_primary_space.buffer_ptr(), ..Filespace::default() };
    //        let mut mock_database      = Database {
    //            OptimizationRecord: &mut optimization_record,
    //            OptimizationRecordSize: std::mem::size_of::<OptimizationRecord>() as u16,
    //            Filespace: &mut mock_filespace,
    //            ..Default::default()
    //        };
    //        let mut smmain_unknown_ptr = 0;
    //        let mut mock_smmain        = SMMain { _unknown_ptr: &mut smmain_unknown_ptr, ..SMMain::default() };
    //        //let mut file_name_s: Vec<u16> = Vec::with_capacity(1000);

    //        // TODO Isolate all these "globals" that are used by the foreign process
    //        let temporary_optimization_records_ptr      = 0xca4548 as *mut *mut OptimizationRecord;
    //        let optimization_records_ptr                = 0xca4544 as *mut *mut OptimizationRecord;
    //        let is_temporary_optimization_records_dirty = 0xca4558 as *mut bool;
    //        let smmain_ptr                              = 0xba9740 as *mut *mut SMMain;
    //        let database_ptr                            = 0xca60d0 as *mut *mut Database;
    //        //std::ptr::write(temporary_optimization_records_ptr,      &mut mock_temporary_optimization_record);
    //        std::ptr::write(temporary_optimization_records_ptr,      std::ptr::null_mut());
    //        //std::ptr::write(optimization_records_ptr,                &mut mock_optimization_record);
    //        std::ptr::write(optimization_records_ptr,                std::ptr::null_mut());
    //        std::ptr::write(is_temporary_optimization_records_dirty, false);
    //        std::ptr::write(smmain_ptr,                              &mut mock_smmain);
    //        std::ptr::write(database_ptr,                            &mut mock_database);

    //        //TODO let _database = open_database(&Path::new("C:\\sm\\systems\\testitems"));

    //        // TODO Compute Used Interval testing
    //        //{
    //        //    let element_number = 1;
    //        //    let today = 1;
    //        //    let last_repetition = 1;
    //        //    let result = register_call3((*COMPUTE_USED_INTERVAL_FN) as usize, element_number,today,last_repetition);
    //        //    println!("Result of calling COMPUTE_USED_INTERVAL_FN(1,1,1) = {}", result);
    //        //}

    //        // TODO Compute New Interval testing
    //        //{
    //        //    let mock_item_optimization_data = ItemOptimizationData { UsedInterval: 1, ..Default::default() };
    //        //    let result = register_call1((*COMPUTE_NEW_INTERVAL_FN) as usize, &mock_item_optimization_data as *const _ as i32);
    //        //    println!("Result of calling COMPUTE_NEW_INTERVAL_FN() = {}", result);
    //        //}



    //        // TODO tests
    //        if false {
    //            // TODO This is the Code that executes when you commit an Item repetition
    //            {
    //                /* TODO
    //                if (1E-8_FP80_00af3e78 < (float10)ItemOptData->PredictedR) {
    //                _Unit138.sub_007b3364_UpdateSM16/SM17Metrics(ItemOptData);
    //            }

    //                // TODO Commit Item Optimization Data


    //                // TODO Must DETOUR all these
    //                {
    //                // TODO WriteDatabaseInfoFile
    //                // TODO SaveOutstanding
    //                // TODO NewParameters
    //                // TODO 0088d0e0
    //                // TODO GetDSRData
    //            }
    //            }
    //                    */
    //            }
    //            // TODO This is the Code that executes when you grade an item
    //            //if false {
    //            //    println!("Database before call: {:?}", mock_database);
    //            //    //let result = register_call4((*ALGORITHM_OUTCOMES_FN) as usize, &mock_database as *const _ as i32, 0, 3, &mock_data_record as *const _ as i32);

    //            //    let mut mock_item_optimization_data = ItemOptimizationData { UsedInterval: 1, ..Default::default() };
    //            //    println!("before running functions: mock_item_optimization_data {:?}", mock_item_optimization_data);

    //            //    //let result = register_call3((*GET_OPTIMIZATION_DATA_FN) as usize, &mock_database as *const _ as i32, 0, &mut mock_item_optimization_data as *mut _ as i32);
    //            //    let mock_recent_grade = 3;

    //            //    let result = register_call4((*ALGORITHM_OUTCOMES_FN) as usize, &mock_database as *const _ as i32, 1, mock_recent_grade, &mut mock_item_optimization_data as *mut _ as i32);
    //            //    if mock_item_optimization_data.Repetitions == 0 {

    //            //        mock_database.dPending = 0xff;
    //            //        mock_database.dMaxOutstanding = 1;
    //            //    }
    //            //    else {
    //            //        let today = register_call1((*TODAY_FN) as usize, &mock_database as *const _ as i32) as u32;
    //            //        let next_scheduled_day = mock_item_optimization_data.LastRepetition + mock_item_optimization_data.OldInterval;
    //            //        if next_scheduled_day <= today {
    //            //            mock_database.dIOutstanding = 0xff;
    //            //        }
    //            //        mock_database.dFI = register_call1_f64((*GET_FI_CHANGE_FN) as usize, &mock_database as *const _ as i32);
    //            //        mock_database.dItemBurden = register_call3_f64((*GET_FI_CHANGE_FN) as usize,
    //            //                                                       &mock_database as *const _ as i32,
    //            //                                                       mock_item_optimization_data.NewInterval as i32,
    //            //                                                       mock_item_optimization_data.OldInterval as i32);
    //            //        mock_database.dLapses = if mock_recent_grade < 3 { 1 } else { 0 };
    //            //        mock_database.dReps = 1;
    //            //        mock_database.dAL = register_call3_f64((*GET_LAPSES_CHANGE_FN) as usize,
    //            //                                               &mock_database as *const _ as i32,
    //            //                                               mock_item_optimization_data.Lapses as i32,
    //            //                                               mock_item_optimization_data.NewLapses as i32);
    //            //    }

    //            //    println!("finished compute repetition parameters: mock_item_optimization_data {:?}", mock_item_optimization_data);
    //            //}

    //            //println!("After calling ALGORITHM_OUTCOMES, this is what mock_tempor:ary_optimization_record = {:?}", mock_temporary_optimization_record);
    //            // TODO Testing UnicodeString
    //            {
    //                //{
    //                //    let mut lower_case_result = empty_delphi_utf16_string();
    //                //    let mut utf16 = into_delphi_utf16_string(String::from("HELLO"));
    //                //    register_call2((*LOWER_CASE_FN) as usize, &mut utf16 as *mut _ as i32, &mut lower_case_result as *mut _ as i32);
    //                //    println!("Test: 'HELLO' = {:?}", utf16);
    //                //    println!("Test: LOWER_CASE('HELLO') = {:?}", lower_case_result);
    //                //    println!("Test: LOWER_CASE('HELLO') = {}", unsafe { &from_delphi_utf16_string(&lower_case_result, 4) });
    //                //    let mut lower_case_result = empty_delphi_utf16_string();
    //                //    let mut utf16 = into_delphi_utf16_string(String::from("HELLO fdsafdsafjlk;a TREWTQWTEWQ FVDSAFDSA LJSDAJFGLKJDSAJL"));
    //                //    register_call2((*LOWER_CASE_FN) as usize, &mut utf16 as *mut _ as i32, &mut lower_case_result as *mut _ as i32);
    //                //    println!("Test2: LOWER_CASE('HELLO') = {:?}", lower_case_result);
    //                //    println!("Test2: LOWER_CASE('HELLO') = {:?}", lower_case_result);
    //                //    println!("Test2: LOWER_CASE('HELLO') = {}", unsafe { &from_delphi_utf16_string(&lower_case_result, 4) });
    //                //}
    //                //let mut empty_string = empty_delphi_utf8_string();
    //                //register_call2((*U_STR_SET_LENGTH_FN) as usize, &mut empty_string as *mut _ as i32, 5);
    //                //println!("Test3: U_STR_SET_LENGTH_FN = {:?}", empty_string);

    //                //unsafe {
    //                //    let p = utf16.as_ptr() as *mut u16;
    //                //    for i in 0..14 {
    //                //        let v = std::ptr::read(p.add(i));
    //                //        println!("Test4 -- HELLO UnicodeString before {}: {:x}", i, v)
    //                //    }
    //                //}

    //                //let mut utf16 = UnicodeString::from("HELLO world");
    //                //let mut result = UnicodeString::default();
    //                ////let mut result = UnicodeString::default();
    //                //register_call2((*LOWER_CASE_FN) as usize, unsafe { utf16.buffer_ptr() } as i32, result.as_ptr() as i32);
    //                //println!("Test4: 'HELLO world' after = {:?}, len: {}", String::from(&result), result.len());

    //                ////let mut utf16 = from_delphi_utf16_string(utf16);
    //                ////println!("Test4: 'HELLO' after = {:?}", utf16);

    //                ////let mut utf16 = into_delphi_utf16_string(String::from("HELLO WORLD"));
    //                ////println!("Test5: 'HELLO WORLD' before = {:?}", utf16);
    //                //let mut utf16 = UnicodeString::from("tELLO");

    //                //let mut utf16 = UnicodeString::from("F");
    //                //unsafe {
    //                //    let p = utf16.as_ptr() as *mut u16;
    //                //    for i in 0..14 {
    //                //        let v = std::ptr::read(p.add(i));
    //                //        println!("Test4 -- HELLO UnicodeString before {}: {:x}", i, v)
    //                //    }
    //                //}

    //                //let mut utf16 = UnicodeString::from("tELLO");
    //                //let mut result = UnicodeString::default();
    //                //let mut utf16 = UnicodeString::from("C:\\FDASFSDAFDS\\FDSAFDSAFDSA\\FDSAFSDAFDS\\fdsa.txt");
    //                //register_call2((*OPEN_DATABASE_FN) as usize, unsafe { utf16.buffer_ptr() } as i32, result.as_ptr() as i32);
    //                //println!("Test4: 'HELLO world' after = {:?}, len: {}", String::from(&result), result.len());

    //                //register_call2((*U_STR_SET_LENGTH_FN) as usize, utf16.as_ptr() as i32, 7);
    //                //println!("Test5: After call..");
    //                //println!("Test5: 'test' = {:?}", String::from(&utf16));
    //                //let mut utf16 = UnicodeString::from("HELLO");
    //                //register_call2((*U_STR_SET_LENGTH_FN) as usize, utf16.as_ptr() as i32, 11);
    //                //println!("Test5: 'HELLO' after = {:?}", String::from(&utf16));
    //            }
    //        }

    //        // TODO Rewriting OpenDatabase
    //        //if false {
    //        //    foreign_fn!(0x0095dc38, CLOSE_DATABASE, fn(*const Database));
    //        //    foreign_fn!(0x0095a6a4, BACKUP_DISALLOWED, fn(*const Database, *const u16, *const u16) -> i32);
    //        //    foreign_fn!(0x00b11900, CONFIRM_SPACE_FOR_FILE, fn(*const Database, i32) -> i32);
    //        //    foreign_fn!(0x00942194, OPEN_KNO_FILE_FN, fn(*const Database, *const ()));
    //        //    foreign_fn!(0x0091b324, FILESPACE_CREATE, fn(i32, i32, *const Database) -> i32);
    //        //    // TODO Now you need to do OpenSystemInfoFile here
    //        //    // TODO For testing I'll just foreign function this for now
    //        //    foreign_fn!(0x00961454, OPEN_SYSTEM_INFO_FILE, fn(*const Database, *const ()) -> i32);
    //        //    // TODO Maybe I can do detour for this
    //        //    foreign_fn!(0x0093489c, CLOSE_KNO_FILE, fn(*const Database));
    //        //    foreign_fn!(0x0093c8d4, READ_COLLECTION_INI_FILE, fn(*const Database));
    //        //    foreign_fn!(0x009445c0, CREATE_QUEUES, fn(*const Database));
    //        //    foreign_fn!(0x007d8450, SLEEP_7D8450, fn());
    //        //    foreign_fn!(0x0094ed34, CREATE_REGISTRIES, fn(*const Database));
    //        //    foreign_fn!(0x0094fd5c, CREATE_COLLECTION_FILES, fn(*const Database));
    //        //    foreign_fn!(0x009450fc, VERIFY_KNO_RECORDS, fn(*const Database));
    //        //    foreign_fn!(0x0093da20, COLLECT_OUTSTANDING_ELEMENTS, fn(*const Database, i32));
    //        //    foreign_fn!(0x0094c600, SECONDARY_STORAGE_OK, fn(*const Database) -> i32);
    //        //    foreign_fn!(0x0095c438, INIT_COLLECTION_VARIABLES, fn(*const Database));
    //        //    foreign_fn!(0x0091ee68, INIT_FILESPACE, fn(*const ()));
    //        //    foreign_fn!(0x0095d5a4, LOAD_RETENTION_DATA, fn(*const Database));
    //        //    foreign_fn!(0x009458dc, UPDATE_USE_STATISTICS, fn(*const Database));
    //        //    foreign_fn!(0x00895bec, VERIFY_RETENTION_DATA_FILE, fn());
    //        //    foreign_fn!(0x0094fd5c, CREATE_COLLECTION_FILES_AND_FOLDERS, fn(*const Database));

    //        //    let file = std::mem::ManuallyDrop::new(UnicodeString::from("C:\\sm\\systems\\testitems.Kno"));
    //        //    let opt_record_file = File::open("C:\\sm\\systems\\testitems\\info\\sm8opt.dat").unwrap();
    //        //    let mut optimization_record = OptimizationRecord::from_reader(opt_record_file).unwrap();

    //        //    // Now running all these
    //        //    if mock_database.Opened {
    //        //        register_call1((*CLOSE_DATABASE) as usize, &mock_database as *const _ as i32);
    //        //    }

    //        //    let is_enough_space = register_call2((*CONFIRM_SPACE_FOR_FILE) as usize, file.buffer_ptr() as i32, 150000);
    //        //    println!("is_enough_space = {}", is_enough_space);
    //        //    //if is_enough_space != 0 {

    //        //    register_call2((*OPEN_KNO_FILE_FN) as usize, &mock_database as *const _ as i32, file.as_ptr() as i32);

    //        //    //if database is 0
    //        //    println!("After open KNO file test.");

    //        //    let _filespace = register_call3((*FILESPACE_CREATE) as usize, 0x0091ab00, 1, &mock_database as *const _ as i32);
    //        //    //mock_database.Filespace = filespace as *mut Filespace;
    //        //    //println!("After FILESPACE_CREATE test.");

    //        //    //// TODO OpenSystemInfoFile
    //        //    {
    //        //        foreign_fn!(0x00b13048, GET_KNO_FILE_DATE, fn(i32) -> f64);

    //        //        //let is_open_system_info_file = register_call2((*OPEN_SYSTEM_INFO_FILE) as usize, &mock_database as *const _ as i32, file.buffer_ptr() as i32);

    //        //        // TODO extract file name

    //        //        std::ptr::write(0xbaa9e0 as *mut u32, 2);
    //        //        println!("Wrote 2 at address 0xbaa9e0...");

    //        //        mock_database.KNOFileDate = register_call1_f64((*GET_KNO_FILE_DATE) as usize, file.buffer_ptr() as i32);
    //        //        println!("KNOFileDate result was = {}", mock_database.KNOFileDate);
    //        //        //foreign_fn!(0x00cbf308, KERNEL32_LREAD, fn() -> i32);
    //        //        foreign_fn!(0x0094ce88, COL_SUB_94CE88, fn(*const Database, i32) -> i32);
    //        //        //let lread_result = register_call0((*KERNEL32_LREAD) as usize);
    //        //        //println!("After KERNEL32_LREAD...");
    //        //        let lread_result = 0x3b6;
    //        //        mock_database.DataRecord.Version = 0x16;
    //        //        if register_call2((*COL_SUB_94CE88) as usize, &mock_database as *const _ as i32, lread_result) != 0 {
    //        //            println!("After COL_SUB_94CE88 != 0...");
    //        //            mock_database.SessionToday = 0;
    //        //            let today = register_call1((*TODAY_FN) as usize, &mock_database as *const _ as i32);
    //        //            println!("Result of TODAY = {}", today);
    //        //            if today < 1 && mock_database.DataRecord.MemorizedCount > 0 {
    //        //                //foreign_fn!(0x00b18e48, SUB_B18E48, fn(i32) -> f64);
    //        //                //let first_day = register_call1_f64((*SUB_B18E48) as usize, mock_database.DataRecord.FirstDay as i32);
    //        //                // TODO Error
    //        //                panic!("Error here");
    //        //            }

    //        //            mock_database.DataRecord.Version = 0x16;
    //        //            foreign_fn!(0x0093c0bc, COL_SUB_93C0BC, fn(*const Database) -> i32);
    //        //            let result = register_call1((*COL_SUB_93C0BC) as usize, &mock_database as *const _ as i32);
    //        //            println!("Result of COL_SUB_93C0BC = {}", result);
    //        //            let result = 0;
    //        //            if result == 0 {
    //        //                foreign_fn!(0x00961388, SUB_961388, fn(*const Database) -> i32);
    //        //                let result = register_call1((*SUB_961388) as usize, &mock_database as *const _ as i32);
    //        //                println!("Result of SUB_961388 = {}", result);
    //        //                if result != 0 {
    //        //                    if mock_database.DataRecord.MemorizedCount == 0 {
    //        //                        foreign_fn!(0x00937200, SET_FIRST_DAY, fn(*const Database));
    //        //                        register_call1((*SET_FIRST_DAY) as usize, &mock_database as *const _ as i32);
    //        //                    } else {
    //        //                        // TODO I don't need to call CompareDates here
    //        //                        println!("I'm not calling CompareDates here...");
    //        //                    }
    //        //                    // TODO Manually read the optimization record!
    //        //                    // "";

    //        //                    foreign_fn!(0x0094c300, READ_OPTIMIZATION_RECORD, fn(*const Database));
    //        //                    register_call1((*READ_OPTIMIZATION_RECORD) as usize, &mock_database as *const _ as i32);
    //        //                    println!("After READ_OPTIMIZATION_RECORD...");
    //        //                    println!("Resulting OPTIMIZATION_RECORD = {:?}", mock_database.OptimizationRecord.read());
    //        //                    mock_database.OptimizationRecord = &mut optimization_record;
    //        //                    foreign_fn!(0x008ad078, ALLOCATE_OPTIMIZATION_MEMORY, fn(*const OptimizationRecord));
    //        //                    register_call1((*ALLOCATE_OPTIMIZATION_MEMORY) as usize, mock_database.OptimizationRecord as i32);
    //        //                    println!("After ALLOCATE_OPTIMIZATION_MEMORY...");
    //        //                    foreign_fn!(0x007aabd0, READ_MEMORY_DATA_FROM_DISK, fn());
    //        //                    register_call0((*READ_MEMORY_DATA_FROM_DISK) as usize);
    //        //                    println!("After READ_MEMORY_DATA_FROM_DISK...");
    //        //                }
    //        //            }
    //        //        }
    //        //    }
    //        //    println!("After open system info file test.");

    //        //    //register_call1((*READ_COLLECTION_INI_FILE) as usize, &mock_database as *const _ as i32);
    //        //    //println!("After READ_COLLECTION_INI_FILE...");
    //        //    register_call1((*CREATE_QUEUES) as usize, &mock_database as *const _ as i32);
    //        //    println!("After CREATE_QUEUES...");
    //        //    register_call0((*SLEEP_7D8450) as usize);
    //        //    println!("After SLEEP_7D8450...");

    //        //    // TODO This is doing the CREATE_COLLECTION_FILES_AND_FOLDERS
    //        //    {
    //        //        //println!("Filespace at this point: {}", mock_database.Filespace);
    //        //        //register_call1((*CREATE_REGISTRIES) as usize, &mock_database as *const _ as i32);
    //        //        //println!("After CREATE_REGISTRIES...");

    //        //        // TODO hijack create item info file element info dat
    //        //        foreign_fn!(0x0094f3b0, INIT_ITEM_INFO, fn(*const Database, *const ItemInfo, i32));
    //        //        hijack!(0x0094f1a8, CREATE_ELEMENT_INFO_DAT, CreateElementInfoDat,
    //        //                (database: i32) -> i32 {
    //        //                    println!("Detour for CreateElementInfoDat: {}", database);
    //        //                    let element_info_file = File::open("C:\\sm\\systems\\testitems\\info\\ElementInfo.dat").unwrap();
    //        //                    let mut element_info = ItemInfo::from_reader(element_info_file).unwrap();
    //        //                    register_call3((*INIT_ITEM_INFO) as usize, database, &mut element_info as *mut ItemInfo as i32, 4);
    //        //                    0
    //        //                }
    //        //        );
    //        //        // TODO Parts of this file need to be called to set the DataRecord

    //        //        hijack!(0x0089e5f4, OPEN_REPETITION_HISTORY_DAT, OpenRepetitionHistoryDat,
    //        //                (arg1: i32) -> i32 {
    //        //                    println!("Detour for OpenRepetitionHistoryDat: {}", arg1);
    //        //                    //let rep_hist_file = File::open("C:\\sm\\systems\\testitems\\info\\RepetitionHistory.dat").unwrap();
    //        //                    //let mut element_info = ItemInfo::from_reader(rep_hist_file).unwrap();
    //        //                    //register_call3((*INIT_ITEM_INFO) as usize, database, &mut element_info as *mut ItemInfo as i32, 4);
    //        //                    // TODO
    //        //                    0
    //        //                }
    //        //        );
    //        //        hijack!(0x00899088, UPDATE_REPETITION_HISTORY, UpgradeRepetitionHistory,
    //        //                () -> i32 {
    //        //                    println!("Detour for UpgradeRepetitionHistory");
    //        //                    // TODO
    //        //                    0
    //        //                }
    //        //        );

    //        //        //register_call1((*CREATE_COLLECTION_FILES_AND_FOLDERS) as usize, &mock_database as *const _ as i32);
    //        //        //println!("After CREATE_COLLECTION_FILES_AND_FOLDERS...");
    //        //    }

    //        //    //register_call1((*VERIFY_KNO_RECORDS) as usize, &mock_database as *const _ as i32);
    //        //    //println!("After VERIFY_KNO_RECORDS...");
    //        //    //register_call2((*COLLECT_OUTSTANDING_ELEMENTS) as usize, &mock_database as *const _ as i32, 0);
    //        //    //println!("After COLLECT_OUTSTANDING_ELEMENTS...");
    //        //    let result = register_call1((*SECONDARY_STORAGE_OK) as usize, &mock_database as *const _ as i32);
    //        //    println!("After SECONDARY_STORAGE_OK = {}", result);
    //        //    assert!(result != 0);
    //        //    //register_call1((*INIT_COLLECTION_VARIABLES) as usize, &mock_database as *const _ as i32);
    //        //    //println!("After INIT_COLLECTION_VARIABLES...");
    //        //    //register_call1((*INIT_FILESPACE) as usize, mock_database.Filespace as i32);
    //        //    //println!("After INIT_FILESPACE...");
    //        //    mock_database.Opened = true;
    //        //    if mock_database.DataRecord.MemorizedCount == 0 {
    //        //        register_call1((*LOAD_RETENTION_DATA) as usize, &mock_database as *const _ as i32);
    //        //        println!("After LOAD_RETENTION_DATA...");
    //        //    }
    //        //    //register_call1((*UPDATE_USE_STATISTICS) as usize, &mock_database as *const _ as i32);
    //        //    //println!("After UPDATE_USE_STATISTICS...");
    //        //    //register_call0((*VERIFY_RETENTION_DATA_FILE) as usize);
    //        //    //println!("After VERIFY_RETENTION_DATA_FILE...");
    //        //    println!("Database is = {:?}", mock_database);
    //        //}


    //        // TODO THIS IS JUST FOR TESTING
    //        if false {
    //            foreign_fn!(0x004073d4, IO_RESULT, fn() -> i32);
    //            hijack!(0x00407fec, SYSTEM_SUB_407FEC, SystemSub407fec,
    //                    (file_ptr: i32, file_name_ptr: i32) -> i32 {
    //                        println!("Detour for SystemSub407fec: {} {}", file_ptr, file_name_ptr);
    //                        let file_name_ptr = file_name_ptr as *mut u16;
    //                        let mut file_name_s: Vec<u16> = Vec::with_capacity(1000);
    //                        for i in 0..256 {
    //                            let c: u16 = file_name_ptr.add(i).read();
    //                            if c == 0 {
    //                                break;
    //                            }
    //                            file_name_s.push(c)
    //                        }
    //                        println!("SystemSub407fec: input parameter, utf16 filename result = {}", &String::from_utf16_lossy(&file_name_s));
    //                        println!("SystemSub407fec: input parameter, file_ptr = {}", (file_ptr as *mut u32).read());
    //                        let result = SYSTEM_SUB_407FEC.call_trampoline(file_ptr, file_name_ptr as i32);
    //                        println!("SystemSub407fec: After calling, value that's in out parameter, file_ptr = {}", (file_ptr as *mut u32).read());
    //                        let io_result = register_call0((*IO_RESULT) as usize);
    //                        println!("SystemSub407fec: After calling, result of IOResult = {}", io_result);
    //                        result
    //                    });
    //        }

    //        // TODO QUICK TEST OF OPEN_DATABASE_FN WITH UNICODESTRING
    //        if true {
    //            // TODO Thigns I want to hook out
    //            hijack!(0x0096214c, UPDATE_APPLICATION_TITLE, UpdateApplicationTitle,
    //                    (arg1: i32) -> i32 {println!("Detour for UpdateApplicationTitle: {}", arg1); 0});
    //            hijack!(0x009a3210, SET_DEFAULT_THEME, SetDefaultTheme,
    //                    (arg1: i32, arg2: i32) -> i32 {println!("Detour for SetDefaultTheme: {} {}", arg1, arg2); 0});
    //            hijack!(0x0095e330, READ_DATABASE_CONFIGURATION, ReadDatabaseConfiguration,
    //                    (arg1: i32) -> i32 {println!("Detour for ReadDatabaseConfiguration: {}", arg1); 0});
    //            hijack!(0x00997e34, READ_TRANSLATION_TABLE, ReadTranslationTable,
    //                    (arg1: i32, arg2: i32) -> i32 {println!("(Skipping) Detour for ReadTranslationTable: {} {}", arg1, arg2); 0});
    //            hijack!(0x009a20dc, UPDATE_TRANSLATED_INTERFACE, UpdateTranslatedInterface,
    //                    (arg1: i32) -> i32 {println!("Detour for UpdateTranslatedInterface: {}", arg1); 0});
    //            // TODO This needs to be hooked out
    //            hijack!(0x0099686c, COMPARE_DATES, CompareDates,
    //                    (arg1: i32) -> i32 {println!("Detour for CompareDates: {}", arg1); 0});
    //            // TODO This may be needed to be hooked out
    //            hijack!(0x009a33c8, MAIN_CLEANUP, MainCleanup,
    //                    () -> i32 {println!("Detour for MainCleanup"); 0});

    //            // TODO This part is JUST for testing
    //            hijack!(0x00942194, OPEN_KNO_FILE, OpenKNOFile, (arg1: i32, arg2: i32) -> i32);
    //            hijack!(0x00961454, OPEN_SYSTEM_INFO_FILE, OpenSystemInfoFile, (arg1: i32, arg2: i32) -> i32);
    //            hijack!(0x0091b324, FILESPACE_CREATE, FilespaceCreate, (arg1: i32, arg2: i32, arg3: i32) -> i32);
    //            hijack!(0x009445c0, CREATE_QUEUES, CreateQueues, (arg1: i32) -> i32);
    //            hijack!(0x0094ed34, CREATE_REGISTRIES, CreateRegistries, (arg1: i32) -> i32 { println!("(Skipping) Detour for CREATE_REGISTRIES called. Not calling original."); 0 });
    //            hijack!(0x00429234, SUB_429234, Sub429234, (arg1: i32, arg2: i32) -> i32);
    //            hijack!(0x00b08bc4, FILE_SUB_B08BC4, FileSubB08bc4, (arg1: i32, arg2: i32) -> i32);
    //            hijack!(0x00b13048, GET_KNO_FILE_DATE, GetKNOFileDate, (arg1: i32) -> f64);
    //            hijack!(0x004297ec, EXTRACT_FILE_NAME, ExtractFileName, (arg1: i32, arg2: i32) -> i32);
    //            hijack!(0x0094ce88, SUB_94CE88, Sub94ce88, (arg1: i32, arg2: i32) -> i32);
    //            hijack!(0x00b09198, FILES_B09198, FilesB09198, (arg1: i32, arg2: i32) -> i32);
    //            hijack!(0x00b13048, FILES_B13048, FilesB13048, (arg1: i32) -> i32);
    //            hijack!(0x00b0a5c8, FILES_B0A5C8, FilesB0a5c8, (arg1: i32, arg2: i32) -> i32 { println!("Detour for FILES_B0A5C8 called. Not calling original."); 0 });
    //            hijack!(0x00b0cb20, FILES_B0CB20, FileB0cb20, (arg1: i32) -> i32);
    //            //hijack!(0x0094c300, READ_OPTIMIZATION_MEMORY, ReadOptimizationMemory,
    //            //        (arg1: i32) -> i32 {
    //            //            println!("Detour for ReadOptimizationMemory: {}", arg1);
    //            //            register_call1(READ_OPTIMIZATION_MEMORY.detour.trampoline() as *const _ as usize, arg1)
    //            //        });
    //            hijack!(0x008ad078, ALLOCATE_OPTIMIZATION_MEMORY, AllocateOptimizationMemory, (arg1: i32) -> i32);
    //            hijack!(0x007aabd0, READ_MEMORY_DATA_FROM_DISK, ReadMemoryDataFromDisk, () -> i32);
    //            hijack!(0x007ab0e8, READ_MEMORY_MATRICES, ReadMemoryMatrices, () -> i32);
    //            hijack!(0x007aaebc, READ_FIRST_FORGETTING_CURVE, ReadFirstForgettingCurve, () -> i32);
    //            hijack!(0x007abab0, READ_LAPSE_INTERVAL_MATRIX, ReadLapseIntervalMatrix, () -> i32);
    //            hijack!(0x007abccc, READ_DIFFICULTY_DISTRIBUTION, ReadDifficultyDistribution, () -> i32);
    //            hijack!(0x008acfb8, SET_TEMP_OPTIMIZATION_RECORD, SetTemporaryOptimizationRecord, () -> i32);
    //            hijack!(0x00932d1c, DATABASE_MUST_BE_READ_ONLY, DatabaseMustBeReadOnly, (arg1: i32) -> i32);


    //            // CreateCollectionFilesAndFolders
    //            //hijack!(0x00895268, SET_PRIORITY_AND_INSERT_ELEMENT, SetPriorityAndInsertElement, () -> i32);
    //            hijack!(0x0094fd5c, CREATE_COLLECTION_FILES_AND_FOLDERS, CreateCollectionFilesAndFolders, (arg1: i32) -> i32 {
    //                println!("(Skipping) CREATE_COLLECTION_FILES_AND_FOLDERS");
    //                let smmain_ptr = 0xba9740 as *mut *mut SMMain;
    //                println!("(Skipping) Detour for CREATE_COLLECTION_FILES_AND_FOLDERS... smmain ptr value = {}", smmain_ptr.read() as u32);
    //                //let smmain_ptr = 0xba9740 as *mut *mut SMMain;
    //                //println!("Testing in CREATE_ITEM_INFORMATION_FILE_ELEMENT_INFO. Result = {}", result);
    //                0
    //            });
    //            hijack!(0x00b14b70, FILES_B14B70, FilesB14b70, (arg1: i32, arg2: i32) -> i32);
    //            hijack!(0x0094f1a8, CREATE_ITEM_INFORMATION_FILE_ELEMENT_INFO, CreateItemInformationFileElementInfo, (database_ptr: i32) -> i32 {
    //                println!("Detour for CreateItemInformationFileElementInfo called, {}", database_ptr);
    //                println!("Detour for CreateItemInformationFileElementInfo ... ignoring for now");
    //                // TODO Now I'm going to replace all the functions called her
    //                //let result = FILES_B14B70.call_trampoline(UnicodeString::from("C:\\sm\\systems\\testitems\\info").buffer_ptr() as i32, 0);
    //                //if result != 0 {
    //                //    //let result = SYSTEM_SUB_407FEC.call_trampoline(database_ptr + 1914, UnicodeString::from("C:\\sm\\systems\\testitems\\info\\ElementInfo.dat").buffer_ptr() as i32);
    //                //    let opt_record_file = File::open("C:\\sm\\systems\\testitems\\info\\sm8opt.dat").unwrap();
    //                //    let mut optimization_record = OptimizationRecord::from_reader(opt_record_file).unwrap();
    //                //    std::ptr::write((database_ptr + 1914) as *mut *mut

    //                //}
    //                0
    //            });
    //            hijack!(0x00899088, UPGRADE_REPETITION_HISTORY, UpgradeRepetitionHistory, () -> i32);
    //            hijack!(0x00407fec, SYSTEM_SUB_407FEC, SystemSub407fec, (arg1: i32, arg2: i32) -> i32);
    //            hijack!(0x00897a6c, SUB_897A6C_UPGRADE, Sub897a6cUpgrade, () -> i32);
    //            hijack!(0x009353e0, GET_INFO_PATH, GetInfoPath, (database: i32, out_param_ptr: i32) -> i32 {
    //                println!("Detour for GetInfoPath {} {}...", database, out_param_ptr);
    //                let result = GET_INFO_PATH.call_trampoline(database, out_param_ptr);
    //                println!("GetInfoPath: Directly after calling function...");
    //                let file_name_ptr = (out_param_ptr as *mut *mut u16).read();
    //                let mut file_name_s: Vec<u16> = Vec::with_capacity(1000);
    //                for i in 0..256 {
    //                    let c: u16 = file_name_ptr.add(i).read();
    //                    if c == 0 {
    //                        break;
    //                    }
    //                    file_name_s.push(c);
    //                }
    //                println!("GetInfoPath: After calling function, out_param_ptr = {}", &String::from_utf16_lossy(&file_name_s));
    //                result
    //            });
    //            hijack!(0x007d8450, SLEEP_7D8450, Sleep7d8450, () -> i32);

    //            // TODO DETOUR THIS 96214c
    //            foreign_fn!(0x009622cc, OPEN_DATABASE_FN, fn(i32, i32) -> i32);

    //            let utf16 = std::mem::ManuallyDrop::new(UnicodeString::from("C:\\sm\\systems\\testitems.kno"));
    //            register_call2((*OPEN_DATABASE_FN) as usize, 0x00ca60d0, utf16.as_ptr() as i32);
    //            println!("Done with OpenDatabase test");
    //            //println!("Database is = {:?}", mock_database);
    //        }

    //        // TODO Replace the same functionality that ComputeItemRepetitionData so you don't have to remake the ElWind struct

    //        std::process::exit(0)
    //    }
    //);

    println!("Hooks enabled..");
}
