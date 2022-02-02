pub use detour::*;
//use crate::structs::{UnicodeString, UnicodeStringData};
use std::{
    ptr,
    slice,
    mem,
    alloc::Layout,
};

// source: https://gist.github.com/thomcc/af7f05e308b95a1c2b935cbd1a8cf3b6
#[repr(C)]
struct UnicodeStringData {
    pub buffer_ptr: *mut u16,
    pub code_page: u16,
    pub bytes_per_char: u16,
    pub rc: u32,
    pub len: u32,
    pub buffer: [u16; 0],
    // _pin: core::marker::PhantomPinned,
}

#[repr(transparent)]
pub struct UnicodeString(*mut UnicodeStringData);

impl UnicodeString {
    /// Includes the nul terminator
    pub fn full_len(&self) -> usize {
        unsafe { ptr::addr_of!((*self.0).len).read() as usize }
    }
    pub fn len(&self) -> usize {
        let len = self.full_len();
        if len > 0 { len - 1 } else { 0 }
    }
    pub fn as_utf16(&self) -> &[u16] {
        let full = self.as_utf16_with_nul();
        &full[..full.len() - 1]
    }
    pub fn as_utf16_with_nul(&self) -> &[u16] {
        unsafe {
            let len = self.full_len();
            assert_eq!(ptr::addr_of!((*self.0).bytes_per_char).read(), 2);
            let u16s_ptr = ptr::addr_of!((*self.0).buffer_ptr).read();
            slice::from_raw_parts(u16s_ptr, len)
        }
    }
    pub fn as_ptr(&self) -> *const () {
        self.0 as *const ()
    }
    pub fn buffer_ptr(&self) -> *mut u16 {
        unsafe {
            (*self.0).buffer_ptr
        }
    }
}

impl Default for UnicodeString {
    fn default() -> Self {
        unsafe {
            use ptr::addr_of_mut;

            let layout_arr = Layout::array::<u16>(1).unwrap();
            let (layout_string, buffer_start_offset) = Layout::new::<UnicodeStringData>()
                .extend(layout_arr)
                .unwrap();
            assert_eq!(
                buffer_start_offset,
                mem::size_of::<UnicodeStringData>()
            );

            let s = std::alloc::alloc(layout_string) as *mut UnicodeStringData;
            if s.is_null() {
                std::alloc::handle_alloc_error(layout_string);
            }
            addr_of_mut!((*s).code_page).write(0x04b0);
            addr_of_mut!((*s).bytes_per_char).write(2);
            addr_of_mut!((*s).rc).write(1);
            addr_of_mut!((*s).len).write(0);
            addr_of_mut!((*s).buffer_ptr).write(ptr::null_mut());
            UnicodeString(s)
        }
    }
}

impl Drop for UnicodeString {
    fn drop(&mut self) {
        unsafe {
            let len = ptr::addr_of!((*self.0).len).read() as usize;
            let layout_arr = Layout::array::<u16>(len).unwrap();
            let (layout_string, _) = Layout::new::<UnicodeStringData>()
                .extend(layout_arr)
                .unwrap();
            std::alloc::dealloc(self.0 as *mut u8, layout_string);
        }
    }
}

impl From<&str> for UnicodeString {
    fn from(s: &str) -> Self {
        unsafe {
            use ptr::addr_of_mut;
            let mut utf16: Vec<u16> = s.encode_utf16().collect();
            utf16.push(0);

            let layout_arr = Layout::array::<u16>(utf16.len()).unwrap();
            let (layout_string, buffer_start_offset) = Layout::new::<UnicodeStringData>()
                .extend(layout_arr)
                .unwrap();
            assert_eq!(
                buffer_start_offset,
                mem::size_of::<UnicodeStringData>()
            );

            let s = std::alloc::alloc(layout_string) as *mut UnicodeStringData;
            if s.is_null() {
                std::alloc::handle_alloc_error(layout_string);
            }
            addr_of_mut!((*s).code_page).write(0x04b0);
            addr_of_mut!((*s).bytes_per_char).write(2);
            addr_of_mut!((*s).rc).write(1);
            addr_of_mut!((*s).len).write(utf16.len() as u32);
            let bufptr = addr_of_mut!((*s).buffer).cast::<u16>();
            assert_eq!(
                bufptr.cast::<u8>(),
                s.cast::<u8>().wrapping_add(buffer_start_offset),
            );

            addr_of_mut!((*s).buffer_ptr).write(bufptr);
            ptr::copy_nonoverlapping(utf16.as_ptr(), bufptr, utf16.len());
            UnicodeString(s)
        }
    }
}

impl From<&UnicodeString> for String {
    fn from(u: &UnicodeString) -> Self {
        if u.full_len() == 0 {
            String::default()
        } else {
            String::from_utf16_lossy(u.as_utf16())
        }
    }
}

pub trait HookBase {
    //TODO fn get_execution_context(&self) -> RegisterCall;
}

pub trait Trampoline0 {
    unsafe extern "C" fn real_func() -> i32;

    #[naked]
    unsafe extern "C" fn trampoline() -> i32 {
        core::arch::asm!(
            "push ebp;
                mov ebp, esp",
            "call {}",
            "mov esp, ebp;
                pop ebp;
                ret",
            sym Self::real_func,
            //clobber_abi("C"),
            options(noreturn)
        );
    }
}

pub trait Trampoline0F64 {
    unsafe extern "C" fn real_func() -> f64;

    #[naked]
    unsafe extern "C" fn trampoline() -> f64 {
        core::arch::asm!(
            "push ebp;
                mov ebp, esp",
            "call {}",
            "sub esp, 8;
                fstp qword ptr [esp];
                movsd xmm0, qword ptr [esp];
                add esp, 8",
            "mov esp, ebp;
                pop ebp;
                ret",
            sym Self::real_func,
            //clobber_abi("C"),
            options(noreturn)
        );
    }
}

pub trait Trampoline1 {
    unsafe extern "C" fn real_func(arg1: i32) -> i32;

    #[naked]
    unsafe extern "C" fn trampoline() -> i32 {
        core::arch::asm!(
            "push ebp;
                mov ebp, esp",
            "push eax;
                call {}",
            "mov esp, ebp;
                pop ebp;
                ret",
            sym Self::real_func,
            //clobber_abi("C"),
            options(noreturn)
        );
    }
}

pub trait Trampoline1F64 {
    unsafe extern "C" fn real_func(arg1: i32) -> f64;

    #[naked]
    unsafe extern "C" fn trampoline() -> f64 {
        core::arch::asm!(
            "push ebp;
                mov ebp, esp",
            "push eax;
                call {}",
            "sub esp, 8;
                fstp qword ptr [esp];
                movsd xmm0, qword ptr [esp];
                add esp, 8",
            "mov esp, ebp;
                pop ebp;
                ret",
            sym Self::real_func,
            //clobber_abi("C"),
            options(noreturn)
        );
    }
}

pub trait Trampoline2 {
    unsafe extern "C" fn real_func(arg1: i32, arg2: i32) -> i32;

    #[naked]
    unsafe extern "C" fn trampoline() -> i32 {
        core::arch::asm!(
            "push ebp;
                mov ebp, esp",
            "push edx;
                push eax;
                call {}",
            "mov esp, ebp;
                pop ebp;
                ret",
            sym Self::real_func,
            //clobber_abi("C"),
            options(noreturn)
        );
    }
}

pub trait Trampoline2F64 {
    unsafe extern "C" fn real_func(arg1: i32, arg2: i32) -> f64;

    #[naked]
    unsafe extern "C" fn trampoline() -> f64 {
        core::arch::asm!(
            "push ebp;
                mov ebp, esp",
            "push edx;
                push eax;
                call {}",
            "sub esp, 8;
                fstp qword ptr [esp];
                movsd xmm0, qword ptr [esp];",
            "mov esp, ebp;
                pop ebp;
                ret",
            sym Self::real_func,
            //clobber_abi("C"),
            options(noreturn)
        );
    }
}

pub trait Trampoline3 {
    unsafe extern "C" fn real_func(arg1: i32, arg2: i32, arg3: i32) -> i32;

    #[naked]
    unsafe extern "C" fn trampoline() -> i32 {
        core::arch::asm!(
            "push ebp;
                mov ebp, esp",
            "push ecx;
                push edx;
                push eax;
                call {}",
            "mov esp, ebp;
                pop ebp;
                ret",
            sym Self::real_func,
            //clobber_abi("C"),
            options(noreturn)
        );
    }
}

pub trait Trampoline3F64 {
    unsafe extern "C" fn real_func(arg1: i32, arg2: i32, arg3: i32) -> f64;

    #[naked]
    unsafe extern "C" fn trampoline() -> f64 {
        core::arch::asm!(
            "push ebp;
                mov ebp, esp",
            "push ecx;
                push edx;
                push eax;
                call {}",
            "sub esp, 8;
                fstp qword ptr [esp];
                movsd xmm0, qword ptr [esp];
                add esp, 8",
            "mov esp, ebp;
                pop ebp;
                ret",
            sym Self::real_func,
            //clobber_abi("C"),
            options(noreturn)
        );
    }
}

pub trait Trampoline4 {
    unsafe extern "C" fn real_func(arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32;

    #[naked]
    unsafe extern "C" fn trampoline() -> i32 {
        core::arch::asm!(
            "push ebp;
                mov ebp, esp",
            "push dword ptr [esp+8];
                push ecx;
                push edx;
                push eax;
                call {}",
            "mov esp, ebp;
                pop ebp;
                ret",
            sym Self::real_func,
            //clobber_abi("C"),
            options(noreturn)
        );
    }
}

pub trait Trampoline4F64 {
    unsafe extern "C" fn real_func(arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> f64;

    #[naked]
    unsafe extern "C" fn trampoline() -> f64 {
        core::arch::asm!(
            "push ebp;
                mov ebp, esp",
            "push dword ptr [esp+8];
                push ecx;
                push edx;
                push eax;
                call {}",
            "sub esp, 8;
                fstp qword ptr [esp];
                movsd xmm0, qword ptr [esp];
                add esp, 8",
            "mov esp, ebp;
                pop ebp;
                ret",
            sym Self::real_func,
            //clobber_abi("C"),
            options(noreturn)
        );
    }
}

pub struct RegisterExecutionContext<Args, Output> {
    parameters: Args,
    result: Output,
    fn_addr: usize,
    detour: RawDetour,
}

pub enum RegisterCallEnum {
    RegisterCall0(RegisterExecutionContext<(), i32>),
    RegisterCall0F64(RegisterExecutionContext<(), f64>),
    RegisterCall1(RegisterExecutionContext<i32, i32>),
    RegisterCall1F64(RegisterExecutionContext<i32, f64>),
    RegisterCall2(RegisterExecutionContext<(i32, i32), i32>),
    RegisterCall2F64(RegisterExecutionContext<(i32, i32), f64>),
    RegisterCall3(RegisterExecutionContext<(i32, i32, i32), i32>),
    RegisterCall3F64(RegisterExecutionContext<(i32, i32, i32), f64>),
    RegisterCall4(RegisterExecutionContext<(i32, i32, i32, i32), i32>),
    RegisterCall4F64(RegisterExecutionContext<(i32, i32, i32, i32), f64>),
}

pub enum RegisterCallResult {
    i32(i32),
    f64(f64),
}

impl RegisterCallEnum {
    pub fn call_detour(&self) -> std::result::Result<(), &'static str> {
        match self {
            RegisterCallEnum::RegisterCall0(RegisterExecutionContext::<(), i32> {
                fn_addr, ref mut result, ..
            }) => {
                *result = register_call0(*fn_addr, ());
                Ok(())
            },
            RegisterCallEnum::RegisterCall0F64(RegisterExecutionContext::<(), f64> {
                fn_addr, ref mut result, ..
            }) => {
                *result = register_call0_f64(*fn_addr, ());
                Ok(())
            },
            RegisterCallEnum::RegisterCall1(RegisterExecutionContext::<i32, i32> {
                parameters, fn_addr, ref mut result, ..
            }) => {
                *result = register_call1(*fn_addr, *parameters);
                Ok(())
            },
            RegisterCallEnum::RegisterCall1F64(RegisterExecutionContext::<i32, f64> {
                parameters, fn_addr, ref mut result, ..
            }) => {
                *result = register_call1_f64(*fn_addr, *parameters);
                Ok(())
            },
            RegisterCallEnum::RegisterCall2(RegisterExecutionContext::<(i32, i32), i32> {
                parameters, fn_addr, ref mut result, ..
            }) => {
                *result = register_call1(*fn_addr, *parameters);
                Ok(())
            },
            RegisterCallEnum::RegisterCall2F64(RegisterExecutionContext::<(i32, i32), f64> {
                parameters, fn_addr, ref mut result, ..
            }) => {
                *result = register_call2_f64(*fn_addr, *parameters);
                Ok(())
            },
            RegisterCallEnum::RegisterCall3(RegisterExecutionContext::<(i32, i32, i32), i32> {
                parameters, fn_addr, ref mut result, ..
            }) => {
                *result = register_call3(*fn_addr, *parameters);
                Ok(())
            },
            RegisterCallEnum::RegisterCall3F64(RegisterExecutionContext::<(i32, i32, i32), f64> {
                parameters, fn_addr, ref mut result, ..
            }) => {
                *result = register_call3_f64(*fn_addr, *parameters);
                Ok(())
            },
            RegisterCallEnum::RegisterCall4(RegisterExecutionContext::<(i32, i32, i32, i32), i32> {
                parameters, fn_addr, ref mut result, ..
            }) => {
                *result = register_call4(*fn_addr, *parameters);
                Ok(())
            },
            RegisterCallEnum::RegisterCall4F64(RegisterExecutionContext::<(i32, i32, i32, i32), f64> {
                parameters, fn_addr, ref mut result, ..
            }) => {
                *result = register_call4_f64(*fn_addr, *parameters);
                Ok(())
            },
            _ => { Err("Error, did not match on RegisterCall variant in call") },
        }
    }
}

/*
            RegisterCall0(RegisterExecutionContext<(), i32> {
                fn_addr, ref mut result, ..
            }) => { result = register_call0(fn_addr, ()); },
            RegisterCall0F64(RegisterExecutionContext<(), f64> {
                fn_addr, ref mut result, ..
            }) => { result = register_call0_f64(fn_addr, ()); },
            RegisterCall1(RegisterExecutionContext<(i32,), i32> {
                parameters, fn_addr, ref mut result, ..
            }) => { result = register_call1(fn_addr, parameters); },
            RegisterCall1F64(RegisterExecutionContext<(i32,), f64> {
                parameters, fn_addr, ref mut result, ..
            }) => { result = register_call1_f64(fn_addr, parameters); },
            RegisterCall2(RegisterExecutionContext<(i32, i32), i32> {
                parameters, fn_addr, ref mut result, ..
            }) => { result = register_call1(fn_addr, parameters); },
            RegisterCall2F64(RegisterExecutionContext<(i32, i32), f64> {
                parameters, fn_addr, ref mut result, ..
            }) => { result = register_call2_f64(fn_addr, parameters); },
            RegisterCall3(RegisterExecutionContext<(i32, i32, i32), i32> {
                parameters, fn_addr, ref mut result, ..
            }) => { result = register_call3(fn_addr, parameters); },
            RegisterCall3F64(RegisterExecutionContext<(i32, i32, i32), f64> {
                parameters, fn_addr, ref mut result, ..
            }) => { result = register_call3_f64(fn_addr, parameters); },
            RegisterCall4(RegisterExecutionContext<(i32, i32, i32, i32), i32> {
                parameters, fn_addr, ref mut result, ..
            }) => { result = register_call4(fn_addr, parameters); },
            RegisterCall4F64(RegisterExecutionContext<(i32, i32, i32, i32), f64> {
                parameters, fn_addr, ref mut result, ..
            }) => { result = register_call4_f64(fn_addr, parameters); },
*/


pub trait RegisterCall<Output> {
    pub fn register_call(fn_ptr: usize, args: Self) -> Output;
}

impl RegisterCall<i32> for () {
    pub fn register_call(fn_ptr: usize, _: Self) -> i32 {
        let ret_val: i32;
        unsafe {
            core::arch::asm!(
                "finit; call {f}",
                f = in(reg) fn_ptr,
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
        ret_val
    }
}

impl RegisterCall<f64> for () {
    pub fn register_call(fn_ptr: usize, _: Self) -> f64 {
        let ret_val: f64;
        unsafe {
            core::arch::asm!(
                "finit; call {f}",
                f = in(reg) fn_ptr,
                lateout("eax") _,
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
                out("xmm0") ret_val,
                out("xmm1") _,
                out("xmm2") _,
                out("xmm3") _,
                out("xmm4") _,
                out("xmm5") _,
                out("xmm6") _,
                out("xmm7") _
            );
        }
        ret_val
    }
}

impl RegisterCall<i32> for i32 {
    pub fn register_call(fn_ptr: usize, arg1: Self) -> i32 {
        let ret_val: i32;
        unsafe {
            core::arch::asm!(
                "finit; call {f}",
                f = in(reg) fn_ptr,
                in("eax") arg1,
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
        ret_val
    }
}

impl RegisterCall<f64> for i32 {
    pub fn register_call(fn_ptr: usize, arg1: Self) -> f64 {
        let ret_val: f64;
        unsafe {
            core::arch::asm!(
                "finit; call {f}",
                f = in(reg) fn_ptr,
                in("eax") arg1,
                lateout("eax") _,
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
                out("xmm0") ret_val,
                out("xmm1") _,
                out("xmm2") _,
                out("xmm3") _,
                out("xmm4") _,
                out("xmm5") _,
                out("xmm6") _,
                out("xmm7") _
            );
        }
        ret_val
    }
}

impl RegisterCall<i32> for (i32, i32) {
    pub fn register_call(fn_ptr: usize, arg1: Self) -> i32 {
        let ret_val: i32;
        unsafe {
            core::arch::asm!(
                "finit; call {f}",
                f = in(reg) fn_ptr,
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
        ret_val
    }
}

impl RegisterCall<f64> for (i32, i32) {
    pub fn register_call(fn_ptr: usize, (arg1, arg2): Self) -> f64 {
        let ret_val: f64;
        unsafe {
            core::arch::asm!(
                "finit; call {f}",
                f = in(reg) fn_ptr,
                in("eax") arg1,
                in("edx") arg2,
                lateout("eax") _,
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
                out("xmm0") ret_val,
                out("xmm1") _,
                out("xmm2") _,
                out("xmm3") _,
                out("xmm4") _,
                out("xmm5") _,
                out("xmm6") _,
                out("xmm7") _
            );
        }
        ret_val
    }
}

impl RegisterCall<i32> for (i32, i32, i32) {
    pub fn register_call(fn_ptr: usize, (arg1, arg2, arg3): Self) -> i32 {
        let ret_val: i32;
        unsafe {
            core::arch::asm!(
                "finit; call {f}",
                f = in(reg) fn_ptr,
                in("eax") arg1,
                in("edx") arg2,
                in("ecx") arg3,
                lateout("eax") ret_val,
                lateout("ecx") _,
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
        ret_val
    }
}

impl RegisterCall<f64> for (i32, i32, i32) {
    pub fn register_call(fn_ptr: usize, (arg1, arg2, arg3): Self) -> f64 {
        let ret_val: f64;
        unsafe {
            core::arch::asm!(
                "finit; call {f}",
                f = in(reg) fn_ptr,
                in("eax") arg1,
                in("edx") arg2,
                in("ecx") arg3,
                lateout("eax") _,
                lateout("ecx") _,
                lateout("edx") _,
                out("st(0)") _,
                out("st(1)") _,
                out("st(2)") _,
                out("st(3)") _,
                out("st(4)") _,
                out("st(5)") _,
                out("st(6)") _,
                out("st(7)") _,
                out("xmm0") ret_val,
                out("xmm1") _,
                out("xmm2") _,
                out("xmm3") _,
                out("xmm4") _,
                out("xmm5") _,
                out("xmm6") _,
                out("xmm7") _
            );
        }
        ret_val
    }
}

impl RegisterCall<i32> for (i32, i32, i32, i32) {
    pub fn register_call(fn_ptr: usize, (arg1, arg2, arg3, arg4): Self) -> i32 {
        let ret_val: i32;
        unsafe {
            core::arch::asm!(
                "finit; push edi; call ebx",
                in("edi") arg4,
                in("ebx") fn_ptr,
                in("eax") arg1,
                in("edx") arg2,
                in("ecx") arg3,
                lateout("eax") ret_val,
                lateout("ecx") _,
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
        ret_val
    }
}

impl RegisterCall<f64> for (i32, i32, i32, i32) {
    pub fn register_call(fn_ptr: usize, (arg1, arg2, arg3, arg4): Self) -> f64 {
        let ret_val: f64;
        unsafe {
            core::arch::asm!(
                "finit; push edi; call ebx",
                in("edi") arg4,
                in("ebx") fn_ptr,
                in("eax") arg1,
                in("edx") arg2,
                in("ecx") arg3,
                lateout("eax") _,
                lateout("ecx") _,
                lateout("edx") _,
                out("st(0)") _,
                out("st(1)") _,
                out("st(2)") _,
                out("st(3)") _,
                out("st(4)") _,
                out("st(5)") _,
                out("st(6)") _,
                out("st(7)") _,
                out("xmm0") ret_val,
                out("xmm1") _,
                out("xmm2") _,
                out("xmm3") _,
                out("xmm4") _,
                out("xmm5") _,
                out("xmm6") _,
                out("xmm7") _
            );
        }
        ret_val
    }
}

#[macro_export]
macro_rules! foreign_fn {
    ($ADDR:expr, $FN_PTR_NAME:ident, $FN_SIGNATURE:ty) => {
        lazy_static! {
            static ref $FN_PTR_NAME: $FN_SIGNATURE =
                unsafe { std::mem::transmute::<usize, $FN_SIGNATURE>($ADDR as usize) };
        }
    };
}

#[macro_export]
macro_rules! hijack {
    (@output_type) => { i32 };
    (@output_type $RET_TY:ty) => { $RET_TY };
    (@call RegisterCall0,    $($REST:tt)*) => { <()                   as RegisterCall<i32>>::register_call($($REST)*) };
    (@call RegisterCall1,    $($REST:tt)*) => { < i32                 as RegisterCall<i32>>::register_call($($REST)*) };
    (@call RegisterCall2,    $($REST:tt)*) => { <(i32, i32)           as RegisterCall<i32>>::register_call($($REST)*) };
    (@call RegisterCall3,    $($REST:tt)*) => { <(i32, i32, i32)      as RegisterCall<i32>>::register_call($($REST)*) };
    (@call RegisterCall4,    $($REST:tt)*) => { <(i32, i32, i32, i32) as RegisterCall<i32>>::register_call($($REST)*) };
    (@call RegisterCall0F64, $($REST:tt)*) => { <()                   as RegisterCall<f64>>::register_call($($REST)*) };
    (@call RegisterCall1F64, $($REST:tt)*) => { < i32                 as RegisterCall<f64>>::register_call($($REST)*) };
    (@call RegisterCall2F64, $($REST:tt)*) => { <(i32, i32)           as RegisterCall<f64>>::register_call($($REST)*) };
    (@call RegisterCall3F64, $($REST:tt)*) => { <(i32, i32, i32)      as RegisterCall<f64>>::register_call($($REST)*) };
    (@call RegisterCall4F64, $($REST:tt)*) => { <(i32, i32, i32, i32) as RegisterCall<f64>>::register_call($($REST)*) };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        $TRAMPOLINE_TYPE: ident,
        RegisterCallEnum::$FN_CALLER: ident,
        ($($ARG:ident:$ARG_TY:ty),*) -> $RET_TY:ty {$($BLOCK:tt)*}
    ) => {
        struct $HOOK_STRUCT_NAME(RegisterCallEnum);

        impl HookBase for $HOOK_STRUCT_NAME {
            //fn get_execution_context(&self) -> RegisterCall {
            //    let Self(register_call) = *self;
            //    register_call
            //}
        }

        impl $TRAMPOLINE_TYPE for $HOOK_STRUCT_NAME {
            unsafe extern "C" fn real_func($($ARG:$ARG_TY),*) -> $RET_TY {$($BLOCK)*}
        }

        impl $HOOK_STRUCT_NAME {
            fn new(fn_addr: usize) -> Self {
                Self(
                    RegisterCallEnum::$FN_CALLER(
                        RegisterExecutionContext::<($($ARG_TY),*), $RET_TY> {
                            parameters: ($($ARG),*),
                            result: <$RET_TY>::default(),
                            fn_addr: fn_addr,
                            detour: unsafe {
                                RawDetour::new(fn_addr as *const (), Self::trampoline as *const ()).unwrap()
                            },
                        }
                    )
                )
            }

            #[allow(dead_code)]
            fn call_detour(&self, $($ARG:$ARG_TY),*) -> $RET_TY {
                hijack!(@call $FN_CALLER, self.fn_ptr as usize, ($($ARG),*))
            }

            #[allow(dead_code)]
            fn call_trampoline(&self, $($ARG:$ARG_TY),*) -> $RET_TY {
                hijack!(@call $FN_CALLER, self.detour.trampoline() as *const _ as usize, ($($ARG),*))
            }
        }

        lazy_static!(
            static ref $HOOK_STATIC_INSTANCE_NAME: $HOOK_STRUCT_NAME = $HOOK_STRUCT_NAME::new($ADDR);
        );
    //unsafe {
    //$HOOK_STRUCT_NAME::new(std::mem::transmute::<usize, fn($($ARG_TY),*) -> $RET_TY>($ADDR))
    // TODO I don't even need to use this transmute because i'm using RawDetour
    //};
        #[allow(unused_unsafe)]
        unsafe { $HOOK_STATIC_INSTANCE_NAME.detour.enable() }.unwrap();
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        () -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline0F64, RegisterCallEnum::RegisterCall0F64,
                () -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        () -> $RET_TY:ty {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline0, RegisterCallEnum::RegisterCall0,
                () -> $RET_TY {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline1F64, RegisterCallEnum::RegisterCall1F64,
                ($ARG1:$ARG1_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty) -> $RET_TY:ty {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline1, RegisterCallEnum::RegisterCall1,
                ($ARG1:$ARG1_TY) -> $RET_TY {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline2F64, RegisterCallEnum::RegisterCall2F64,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty, $ARG2:ident:$ARG2_TY:ty) -> $RET_TY:ty {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline2, RegisterCallEnum::RegisterCall2,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY) -> $RET_TY {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline3F64, RegisterCallEnum::RegisterCall3F64,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty) -> $RET_TY:ty {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline3, RegisterCallEnum::RegisterCall3,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY) -> $RET_TY {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty,$ARG4:ident:$ARG4_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline4F64, RegisterCallEnum::RegisterCall4F64,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY,$ARG4:$ARG4_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty,$ARG4:ident:$ARG4_TY:ty) -> $RET_TY:ty {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline4, RegisterCallEnum::RegisterCall4,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY,$ARG4:$ARG4_TY) -> $RET_TY {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($($ARG:ident:$ARG_TY:ty),*) -> f64
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME,
                ($($ARG:$ARG_TY),*) -> f64 {
                    println!(" >> Detour for {}: {:?}", stringify!($HOOK_STRUCT_NAME), ($($ARG),*));
                    let result = $HOOK_STATIC_INSTANCE_NAME.call_trampoline($($ARG),*);
                    println!("<<  Reached end of detour for {}. Result = {}", stringify!($HOOK_STRUCT_NAME), result);
                    result
                })
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($($ARG:ident:$ARG_TY:ty),*) -> $RET_TY:ty
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME,
                ($($ARG:$ARG_TY),*) -> $RET_TY {
                    println!(" >> Detour for {}: {:?}", stringify!($HOOK_STRUCT_NAME), ($($ARG),*));
                    let result = $HOOK_STATIC_INSTANCE_NAME.call_trampoline($($ARG),*);
                    println!("<<  Reached end of detour for {}.",  stringify!($HOOK_STRUCT_NAME));
                    result
        })
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($($ARG:ident:$ARG_TY:ty),*)
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME,
                ($($ARG:$ARG_TY),*) -> i32 {
                    println!(" >> Detour for {}: {:?}", stringify!($HOOK_STRUCT_NAME), ($($ARG),*));
                    $HOOK_STATIC_INSTANCE_NAME.call_trampoline($($ARG),*);
                    println!("<<  Reached end of detour for {}.", stringify!($HOOK_STRUCT_NAME));
                    0
        })
    };
}
