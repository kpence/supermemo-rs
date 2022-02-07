use crate::delphi::*;
use crate::structs::*;

use winapi::um::winuser::PostMessageW;

use once_cell::sync::Lazy;

use std::sync::{
    Mutex,
    mpsc::{Sender, Receiver},
};

pub static EXECUTION_METHOD: Lazy<Mutex<Option<&'static Hook>>> =
    Lazy::new(|| Mutex::new(None));
pub static EXECUTION_PARAMETERS: Lazy<Mutex<HookParameters>> =
    Lazy::new(|| Mutex::new(HookParameters::Args0));
pub static EXECUTION_RESULT: Lazy<Mutex<HookResult>> =
    Lazy::new(|| Mutex::new(HookResult::i32(0)));
pub static EXECUTION_FINISH_EVENT_TRANSMITTER: Lazy<Mutex<Option<Sender<()>>>> =
Lazy::new(|| Mutex::new(None));
pub static EXECUTION_FINISH_EVENT_RECEIVER: Lazy<Mutex<Option<Receiver<()>>>> =
Lazy::new(|| Mutex::new(None));

struct WndProc { }

impl Trampoline3 for WndProc {
    unsafe extern "C" fn real_func(_sm_main: i32, msg_addr: i32, handled: i32) -> i32 {
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
                sender.send(()).expect("Failed to send message to calling thread");
                (handled as *mut bool).write(true);
            }
        }
        0
    }
}

pub trait CallNativeFn<Output> {
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
        println!("\n---------------------------------------\nAfter setting execution params");

        // I need to get the handle that I need
        let smmain_ptr = 0x00ca61c0 as *mut *mut usize;
        println!("address is {}", unsafe { smmain_ptr.read() as u32 });
        let handle = unsafe { smmain_ptr.read().add(0x288).read() };
        println!("\n---------------------------------------\nTest1");

        let application_ptr = 0x00bb11e8 as *mut usize;
        let on_message_ptr = unsafe { application_ptr.add(0x120) };
        let wnd_proc_fn_addr = WndProc::trampoline as usize;
        println!("\n---------------------------------------\nTest2");

        // Write the address of wndProc to OnMessage
        unsafe { on_message_ptr.write(wnd_proc_fn_addr) };
        println!("\n---------------------------------------\nTest3");

        // TODO then I need to post message
        const MESSAGE_PARAM_EXECUTE_MAIN_THREAD: u32 = 9100101; // TODO put this somewhere properly
        const MESSAGE_ID_SMA: u32 = 2345;
        let msg_id = MESSAGE_ID_SMA;
        let unknown_variable = 0;
        println!("\n---------------------------------------\nBefore PostMessageW");
        let _result = unsafe {
            PostMessageW(
                handle as *mut _,
                MESSAGE_PARAM_EXECUTE_MAIN_THREAD,
                &msg_id as *const _ as usize,
                &unknown_variable as *const _ as isize,
            )
        };
        println!("\n---------------------------------------\nAfter PostMessageW and before receiving");

        // wait for response
        let response = EXECUTION_FINISH_EVENT_RECEIVER.lock().unwrap().as_ref().unwrap().recv().unwrap();
        println!("\n---------------------------------------\nAfter receiving");

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
