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
    any::Any,
    //ffi::OsStr,
};

use once_cell::{
    sync::Lazy,
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

static EXECUTION_METHOD: Lazy<Mutex<Option<Box<&'static (dyn HookBase + 'static + Send + Sync)>,>>> = Lazy::new(|| Mutex::new(None));

//static execution_parameters: Lazy<HookStruct::Args> = Lazy::;
//static execution_result: Lazy<HookStruct::Output>;

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
    hijack!(0x0000000, EL_WDW_GO_TO_ELEMENT, ElWdwGoToElement, (arg1: i32, arg2: i32) -> i32);

    *EXECUTION_METHOD.lock().unwrap() = Some(Box::new(&*EL_WDW_GO_TO_ELEMENT));

    struct WndProc { }

    impl Trampoline3 for WndProc {
        unsafe extern "C" fn real_func(sm_main: i32, msg_addr: i32, handled: i32) -> i32 {
            let ExecuteOnMainThread = 0; // TODO nocheckin
            if msg_addr == 0 {
                return 0;
            } else if let Msg {
                msg: WM_QUIT,
                wParam: ExecuteOnMainThread,
                ..
            } = (msg_addr as *const Msg).read() {

                if let Some(execution_method) = &*EXECUTION_METHOD.lock().unwrap() {
                    let execution_context = execution_method.get_execution_context();
                    //let result = (*execution_method).call_trampoline_tuple(execution_parameters);
                    //*execution_result = result;
                    //(handled as *mut bool).write(true);
                }

            }

            // TODO Here I can write the detour where I test
            // TODO how should I get the execution context
            // ^ I decided what I'll do is put it in the WndProc struct
            0
        }
    }

    //let wnd_proc_fn_ptr: usize = WndProc::trampoline as usize;

    // Write the address of wndProc to OnMessage
    let application_ptr = 0x00bb11e8 as usize;

    // postMessage

    // wait for response

    // Write 0 to OnMessage

    println!("Hooks enabled..");
}
