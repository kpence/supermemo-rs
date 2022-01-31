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
        MessageBoxW, MB_OK, WM_QUIT
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
    sync::Mutex,
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

    /*
    "Application_InstancePtr": "0x00BB11E8", // Application:TApplication
    "Application_OnMessageOffset": "0x120" // TApplication::FOnMessage:TMessageEvent

    What I need to do is write a wndproc callback at the TApplication end point
    then I need to postMessage to it
    */

    // Implement a delphi trampoline method for detouring from wndproc, and invent some wndproc to use with which you'll write it to this OnMessage thing

    //hijack!("E8 ? ? ? ? 80 7D FB 24", EL_WDW_GO_TO_ELEMENT, ElWdwGoToElement)
    struct ExecutionContext<HookStruct> where HookStruct: TrampolineBase+'static {
        execution_method: &'static HookStruct,
        execution_parameters: HookStruct::Args,
        execution_result: HookStruct::Output,
        //TODO execution_out_parameter
    }

    hijack!(0x0000000, EL_WDW_GO_TO_ELEMENT, ElWdwGoToElement, (arg1: i32, arg2: i32) -> i32);

    enum ExecutionMethod {
        ElWdwGoToElement(ExecutionContext<ElWdwGoToElement>),
    }

    struct WndProc {
        execution_context: ExecutionMethod
    }

    lazy_static!(
        static ref WND_PROC: Mutex<WndProc> = Mutex::new(WndProc {
            execution_context: ExecutionMethod::ElWdwGoToElement(
                ExecutionContext::<ElWdwGoToElement> {
                    execution_method: &EL_WDW_GO_TO_ELEMENT,
                    execution_parameters: (1,2),
                    execution_result: 0,
                }
            )
        });
    );

    WND_PROC.lock().unwrap().execution_context = ExecutionMethod::ElWdwGoToElement(
        ExecutionContext::<ElWdwGoToElement> {
            execution_method: &EL_WDW_GO_TO_ELEMENT,
            execution_parameters: (1,2),
            execution_result: 0,
        }
    );

    impl TrampolineBase for WndProc {
        type Args = (i32, i32, i32);
        type Output = i32;
    }

    impl Trampoline3 for WndProc {
        type Args = <WndProc as TrampolineBase>::Args;
        type Output = <WndProc as TrampolineBase>::Output;

        unsafe extern "C" fn real_func(sm_main: i32, msg_addr: i32, handled: i32) -> i32 {
            // TODO verify msg_ptr isn't null
            if msg_addr == 0 {
                return 0;
            }
            // TODO do this right
            let ExecuteOnMainThread = 0;
            if let Msg { msg: WM_QUIT, wParam: ExecuteOnMainThread, .. } = (msg_addr as *const Msg).read() {
                //let res = int.MinValue;

                (handled as *mut bool).write(true);
            }

            // TODO Use singleton for the execution context, the bellow line won't work
            //println!("parameters={}, result={}", Self::execution_context::execution_parameters, Self::execution_context::execution_result);
            // Instead of singleton, I'm just gonna use lazy static!

            // TODO Here I can write the detour where I test
            // TODO how should I get the execution context
            // ^ I decided what I'll do is put it in the WndProc struct
            0
        }
    }

    let wnd_proc_fn_ptr: usize = WndProc::trampoline as usize;

    // Write the address of wndProc to OnMessage
    let application_ptr = 0x00bb11e8 as usize;

    // postMessage

    // wait for response

    // Write 0 to OnMessage

    println!("Hooks enabled..");
}
