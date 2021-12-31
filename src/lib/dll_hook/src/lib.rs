#![feature(libc, asm, asm_sym, naked_functions)]

extern crate winapi;
extern crate kernel32;
#[macro_use] extern crate detour;
#[macro_use] extern crate lazy_static;
extern crate libc;

use detour::*;
use std::{ffi::CString, iter};
use winapi::ctypes::c_int;

unsafe extern "C" fn simple_ret() -> i32 {
    println!("test");
    2220
}

#[naked]
unsafe extern "C" fn simple_ret_naked(arg1: i32, arg2: i32) -> i32 {
    unsafe {
        core::arch::asm!(
            "mov eax, 222; ret",
            options(noreturn)
        );
    }
}

#[naked]
unsafe extern "C" fn detour_register2() -> i32 {
    unsafe {
        core::arch::asm!(
            "push ebp",
            "mov ebp, esp",
            "push edx",
            "push eax",
            "call {}",
            "add esp, 8",
            "mov esp, ebp",
            "pop ebp",
            "ret",
            sym my_func,
            clobber_abi("C"),
            options(noreturn)
        );
    }
}

extern "C" fn my_func(arg1: i32, arg2: i32) -> i32 {
    println!("my_func called with: {}, {}", arg1, arg2);
    222
}

fn register_call2(ptr: usize, arg1: i32, arg2: i32) -> i32 {
    let ret_val: i32;
    unsafe {
        core::arch::asm!(
            "finit; call {f}",
            f = in(reg) ptr,
            in("eax") arg1,
            in("edx") arg2,
            lateout("eax") ret_val,
            out("ecx") _,
            lateout("edx") _,
            out("st(0)") _,
            out("st(1)") _,
            out("st(2)") _,
            out("st(3)") _,
            out("st(4)") _,
            out("st(5)") _,
            out("st(6)") _,
            out("st(7)") _,
            out("xmm0") _,
            out("xmm1") _,
            out("xmm2") _,
            out("xmm3") _,
            out("xmm4") _,
            out("xmm5") _,
            out("xmm6") _,
            out("xmm7") _
        );
    }
    println!("register_call2");
    ret_val
}

macro_rules! hook {
    (
        $ADDR:expr,
        $FN_PTR_NAME:ident,
        $DETOUR_NAME: ident,
        fn($($FARGS:tt)*) $(-> $FRET:ty)?,
        $DETOUR:expr
    ) => {
        hook!($ADDR, $FN_PTR_NAME, fn($($FARGS)*) $(-> $FRET)?);
        static_detour! { static $DETOUR_NAME: fn($($FARGS)*) $(-> $FRET)?; }
        unsafe { $DETOUR_NAME.initialize(*$FN_PTR_NAME, $DETOUR).unwrap().enable().unwrap() };
    };
    ($ADDR:expr, $FN_PTR_NAME:ident, $FN_SIGNATURE:ty) => {
        lazy_static! {
            static ref $FN_PTR_NAME: $FN_SIGNATURE =
                unsafe { std::mem::transmute::<u32, $FN_SIGNATURE>($ADDR) };
        }
    };
}

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

    hook!(0x008ae4cc, TEST_FN_PTR, fn(f64, f64, f64) -> f64);
    //hook!(0x008b0530, TEST_DETOUR_FN_PTR, TestDetour,
    //      fn(u8, u8, u8, u8, u8) -> f64,
    //    move |param1, param2, param3, param4, param5| {
    //        println!("Detour for TestDetourFN: param1: ({} {} {} {} {})", param1, param2, param3, param4, param5);
    //        //(*TEST_DETOUR_FN_PTR)(param1)
    //        500.0
    //    }
    //);
    //hook!(0x008b03b8, TEST_DETOUR2_FN_PTR, TestDetour2,
    //      fn(i8) -> f64,
    //    |i8| {
    //        
    //});

    /*#[naked]
    fn test_test2_fn(param1: u32, param2: u32) -> ! {
        let address: u32 = *TEST2_FN_PTR as u32;
        unsafe {
            asm!(
                "mov dl, 5",
                "mov al, 1",
                "jmp ecx", in("ecx") address,
                options(noreturn)
            )
        }
    }*/

    hook!(0x008b0630, TEST2_FN_PTR, fn(i32, i32) -> i32);
    lazy_static! {
        static ref TEST2_HOOK: RawDetour = unsafe {
            RawDetour::new(*TEST2_FN_PTR as *const (), detour_register2 as *const ()).unwrap()
        };
    };
    unsafe { TEST2_HOOK.enable().unwrap() };

    //raw_hook!(TEST2_FN_PTR, Test2, test2_fn);
    hook!(0x00b23340, ENTRY_POINT_FN_PTR, EntryPointDetour,
          fn(),
        || {
            //test_test2_fn(1,5);
            //let relative_distance: u32 = std::ptr::read(((*TEST2_FN_PTR as usize) + 1) as *const u32);
            //let address: u32 = (*TEST2_FN_PTR as u32) + relative_distance + 5;
            //pretty_print_code_at_address(address, 160);
            println!("--- ");
            println!("heres the detour. put your code in here");
            //let result: f64 = (*TEST_FN_PTR)(0.0,0.0,0.0);
            //println!("result: {}", result);
            //let result: f64 = (*TEST_FN_PTR)(1.0,1.0,1.0);
            //println!("result: {}", result);
            //let result: f64 = (*TEST_FN_PTR)(5.0,1.5,1.5);
            //println!("result: {}", result);
            let result = register_call2(*TEST2_FN_PTR as usize, 55, 64);
            println!("result(1,0): {}", result);
            let result = register_call2(*TEST2_FN_PTR as usize, 40, 0);
            println!("result(40,0): {}", result);
            let result = register_call2(*TEST2_FN_PTR as usize, 30, 0);
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

fn pretty_print_code_at_address(address: u32, size: u32) {
    let mut code: Vec<u8> = Vec::with_capacity(size as usize);
    for i in 0..size {
        let byte: u8 = unsafe { std::ptr::read((address + i) as *const u8) };
        code.push(byte);
    }
    pretty_print_code(&code[..]);
}

fn pretty_print_code(code: &[u8]) {
    use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, NasmFormatter};
    let code: &[u8] = &code[..];
    const HEXBYTES_COLUMN_BYTE_LENGTH: usize = 10;
    const EXAMPLE_CODE_RIP: u64 = 0x0000_7FFA_C46A_CDA4;

    let mut decoder =
        Decoder::with_ip(32, code, 0x0000_7FFA_C46A_CDA4, DecoderOptions::NONE);
    let mut formatter = NasmFormatter::new();

    // Change some options, there are many more
    formatter.options_mut().set_digit_separator("`");
    formatter.options_mut().set_first_operand_char_index(10);

    // String implements FormatterOutput
    let mut output = String::new();

    // Initialize this outside the loop because decode_out() writes to every field
    let mut instruction = Instruction::default();

    // The decoder also implements Iterator/IntoIterator so you could use a for loop:
    //      for instruction in &mut decoder { /* ... */ }
    // or collect():
    //      let instructions: Vec<_> = decoder.into_iter().collect();
    // but can_decode()/decode_out() is a little faster:
    while decoder.can_decode() {
        // There's also a decode() method that returns an instruction but that also
        // means it copies an instruction (40 bytes):
        //     instruction = decoder.decode();
        decoder.decode_out(&mut instruction);

        // Format the instruction ("disassemble" it)
        output.clear();
        formatter.format(&instruction, &mut output);

        // Eg. "00007FFAC46ACDB2 488DAC2400FFFFFF     lea       rbp,[rsp-100h]"
        print!("{:016X} ", instruction.ip());
        let start_index = (instruction.ip() - EXAMPLE_CODE_RIP) as usize;
        let instr_bytes = &code[start_index..start_index + instruction.len()];
        for b in instr_bytes.iter() {
            print!("{:02X}", b);
        }
        if instr_bytes.len() < HEXBYTES_COLUMN_BYTE_LENGTH {
            for _ in 0..HEXBYTES_COLUMN_BYTE_LENGTH - instr_bytes.len() {
                print!("  ");
            }
        }
        println!(" {}", output);
    }
}
