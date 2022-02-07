#![feature(libc, asm, asm_sym, naked_functions)]
extern crate detour;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate utf16_lit;
extern crate kernel32;
extern crate libc;
extern crate winapi;
use delphi::*;
use structs::*;
//use dev_helpers::*;
use supermemo::*;
mod delphi;
mod dev_helpers;
mod structs;
mod supermemo;

use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::libloaderapi::DisableThreadLibraryCalls,
    um::processthreadsapi::GetCurrentProcessId,
    um::tlhelp32::{CreateToolhelp32Snapshot, Thread32First, TH32CS_SNAPTHREAD, THREADENTRY32},
    um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH},
    um::winuser::{MessageBoxW, MB_OK, PostMessageW},
};

use utf16_lit::utf16_null;

use std::{
    any::Any,
    //ffi::OsStr,
    fs::File,
    path::Path,
    sync::{
        Mutex,
        RwLock,
        Arc,
        mpsc::{channel, Sender, Receiver},
    },
};

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

static EXECUTION_METHOD: Lazy<Mutex<Option<&'static Hook>>> =
    Lazy::new(|| Mutex::new(None));
static EXECUTION_PARAMETERS: Lazy<Mutex<HookParameters>> =
    Lazy::new(|| Mutex::new(HookParameters::Args0));
static EXECUTION_RESULT: Lazy<Mutex<HookResult>> =
    Lazy::new(|| Mutex::new(HookResult::i32(0)));
static EXECUTION_FINISH_EVENT_TRANSMITTER: Lazy<Mutex<Option<Sender<()>>>> =
Lazy::new(|| Mutex::new(None));
static EXECUTION_FINISH_EVENT_RECEIVER: Lazy<Mutex<Option<Receiver<()>>>> =
Lazy::new(|| Mutex::new(None));

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

    //hijack!(0x00b21df0, EL_WDW_GO_TO_ELEMENT, ElWdwGoToElement, (arg1: i32, arg2: i32));
    hijack!(0x008b1520, MATH_STUFF, MathStuff, (arg1: i32) -> f64);

    let (sender, receiver) = channel();
    *EXECUTION_FINISH_EVENT_TRANSMITTER.lock().unwrap() = Some(sender);
    *EXECUTION_FINISH_EVENT_RECEIVER.lock().unwrap() = Some(receiver);
    //*EXECUTION_METHOD.lock().unwrap() = Some(&((*MATH_STUFF).0));

    struct WndProc { }

    impl Trampoline3 for WndProc {
        unsafe extern "C" fn real_func(sm_main: i32, msg_addr: i32, handled: i32) -> i32 {
            const MESSAGE_PARAM_EXECUTE_MAIN_THREAD: u32 = 9100101; // TODO put this somewhere properly
            const MESSAGE_ID_SMA: u32 = 2345;
            if msg_addr == 0 {
                return 0;
            } else if let Msg {
                msg: MESSAGE_ID_SMA,
                wParam: MESSAGE_PARAM_EXECUTE_MAIN_THREAD,
                ..
            } = (msg_addr as *const Msg).read() {
                if let Some(execution_method) = *EXECUTION_METHOD.lock().unwrap() {
                    *EXECUTION_RESULT.lock().unwrap() = execution_method.call_detour(&*EXECUTION_PARAMETERS.lock().unwrap());
                    let sender = EXECUTION_FINISH_EVENT_TRANSMITTER.lock().unwrap().as_ref().unwrap().clone();
                    sender.send(()).expect("Failed to send message to calling thread"); // TODO set up an enum to pass
                }
            }
            0
        }
    }

    trait CallNativeFn<Output> {
        fn call_native_fn(hook: &'static Hook, parameters: Self) -> HookResult where Self: Sized {
            *EXECUTION_METHOD.lock().unwrap() = Some(hook);
            *EXECUTION_RESULT.lock().unwrap() = match hook.fn_signature {
                FnSignature::Sig0 | FnSignature::Sig1 | FnSignature::Sig2 | FnSignature::Sig3 | FnSignature::Sig4 => {
                    HookResult::i32(0)
                },
                FnSignature::Sig0F64 | FnSignature::Sig1F64 | FnSignature::Sig2F64 | FnSignature::Sig3F64 | FnSignature::Sig4F64 => {
                    HookResult::f64(0.0)
                },
            };
            Self::set_execution_parameters(parameters);

            // I need to get the handle that I need
            let smmain_ptr = 0x00ca61c0 as *mut *mut usize;
            let handle = unsafe { (smmain_ptr.read().add(0x288)).read() };

            let application_ptr = 0x00bb11e8 as *mut usize;
            let on_message_ptr = unsafe { application_ptr.add(0x120) };
            let wnd_proc_fn_addr = WndProc::trampoline as usize;

            // Write the address of wndProc to OnMessage
            unsafe { on_message_ptr.write(wnd_proc_fn_addr) };

            // TODO then I need to post message
            const MESSAGE_PARAM_EXECUTE_MAIN_THREAD: u32 = 9100101; // TODO put this somewhere properly
            const MESSAGE_ID_SMA: u32 = 2345;
            let msg_id = MESSAGE_ID_SMA;
            let unknown_variable = 0;
            let result = unsafe {
                PostMessageW(
                    handle as *mut _,
                    MESSAGE_PARAM_EXECUTE_MAIN_THREAD,
                    &msg_id as *const _ as usize,
                    &unknown_variable as *const _ as isize,
                )
            };

            // wait for response
            let response = EXECUTION_FINISH_EVENT_RECEIVER.lock().unwrap().as_ref().unwrap().recv().unwrap();

            // Write 0 to OnMessage
            unsafe { (on_message_ptr as *mut usize).write(0) };

            *EXECUTION_RESULT.lock().unwrap()
        }

        fn set_execution_parameters(parameters: Self);
    }

    impl<Output> CallNativeFn<Output> for () {
        fn set_execution_parameters(_: ()) {
            *EXECUTION_PARAMETERS.lock().unwrap() = HookParameters::Args0;
        }
    }

    impl<Output> CallNativeFn<Output> for i32 {
        fn set_execution_parameters(arg1: Self) {
            *EXECUTION_PARAMETERS.lock().unwrap() = HookParameters::Args1(arg1);
        }
    }

    impl<Output> CallNativeFn<Output> for (i32, i32) {
        fn set_execution_parameters((arg1, arg2): Self) {
            *EXECUTION_PARAMETERS.lock().unwrap() = HookParameters::Args2(arg1, arg2);
        }
    }

    impl<Output> CallNativeFn<Output> for (i32, i32, i32) {
        fn set_execution_parameters((arg1, arg2, arg3): Self) {
            *EXECUTION_PARAMETERS.lock().unwrap() = HookParameters::Args3(arg1, arg2, arg3);
        }
    }

    impl<Output> CallNativeFn<Output> for (i32, i32, i32, i32) {
        fn set_execution_parameters((arg1, arg2, arg3, arg4): Self) {
            *EXECUTION_PARAMETERS.lock().unwrap() = HookParameters::Args4(arg1, arg2, arg3, arg4);
        }
    }

    let result = <i32 as CallNativeFn<f64>>::call_native_fn(&((*MATH_STUFF).0), 5);

    std::thread::sleep(std::time::Duration::from_secs(20));

    match result {
        HookResult::f64(ret_val) => {
            println!("result is {}", ret_val);
        },
        _ => {
            panic!("oh noes");
        },
    }

    println!("Hooks enabled..");
}
