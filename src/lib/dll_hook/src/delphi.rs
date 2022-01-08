#![feature(libc, asm, asm_sym, naked_functions)]
pub use detour::*;
//use crate::structs::{UnicodeString, UnicodeStringData};
use std::ptr;
use std::slice;
use std::mem;
use std::alloc::{alloc, dealloc, Layout, LayoutError};

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
            clobber_abi("C"),
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
            clobber_abi("C"),
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
            clobber_abi("C"),
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
            clobber_abi("C"),
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
            clobber_abi("C"),
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
            clobber_abi("C"),
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
            clobber_abi("C"),
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
            clobber_abi("C"),
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
            clobber_abi("C"),
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
            clobber_abi("C"),
            options(noreturn)
        );
    }
}

pub fn register_call0(ptr: usize) -> i32 {
    let ret_val: i32;
    unsafe {
        core::arch::asm!(
            "finit; call {f}",
            f = in(reg) ptr,
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

pub fn register_call0_f64(ptr: usize) -> f64 {
    let ret_val: f64;
    unsafe {
        core::arch::asm!(
            "finit; call {f}",
            f = in(reg) ptr,
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

pub fn register_call1(ptr: usize, arg1: i32) -> i32 {
    let ret_val: i32;
    unsafe {
        core::arch::asm!(
            "finit; call {f}",
            f = in(reg) ptr,
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

pub fn register_call1_f64(ptr: usize, arg1: i32) -> f64 {
    let ret_val: f64;
    unsafe {
        core::arch::asm!(
            "finit; call {f}",
            f = in(reg) ptr,
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

pub fn register_call2(ptr: usize, arg1: i32, arg2: i32) -> i32 {
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
    ret_val
}

pub fn register_call2_f64(ptr: usize, arg1: i32, arg2: i32) -> f64 {
    let ret_val: f64;
    unsafe {
        core::arch::asm!(
            "finit; call {f}",
            f = in(reg) ptr,
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

pub fn register_call3(ptr: usize, arg1: i32, arg2: i32, arg3: i32) -> i32 {
    let ret_val: i32;
    unsafe {
        core::arch::asm!(
            "finit; call {f}",
            f = in(reg) ptr,
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

pub fn register_call3_f64(ptr: usize, arg1: i32, arg2: i32, arg3: i32) -> f64 {
    let ret_val: f64;
    unsafe {
        core::arch::asm!(
            "finit; call {f}",
            f = in(reg) ptr,
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

pub fn register_call4(ptr: usize, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32 {
    let ret_val: i32;
    unsafe {
        core::arch::asm!(
            "finit; push edi; call ebx",
            in("edi") arg4,
            in("ebx") ptr,
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

// TODO We should use the ptr to the function instead of a usize in order to prevent using the wrong register call function (e.g. f64 instead of i32 output
pub fn register_call4_f64(ptr: usize, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> f64 {
    let ret_val: f64;
    unsafe {
        core::arch::asm!(
            "finit; push edi; call ebx",
            in("edi") arg4,
            in("ebx") ptr,
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
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        $TRAMPOLINE_TYPE: ident,
        $ORIGINAL_FN_CALLER: ident,
        ($($ARG:ident:$ARG_TY:ty),*) $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        struct $HOOK_STRUCT_NAME {
            fn_ptr: fn($($ARG_TY),*) $(-> $RET_TY)?,
            detour: RawDetour,
        }

        impl $TRAMPOLINE_TYPE for $HOOK_STRUCT_NAME {
            unsafe extern "C" fn real_func($($ARG:$ARG_TY),*) $(-> $RET_TY)? {$($BLOCK)*}
        }

        impl $HOOK_STRUCT_NAME {
            fn new(fn_ptr: fn($($ARG_TY),*) $(-> $RET_TY)?) -> Self {
                Self {
                    fn_ptr: fn_ptr,
                    detour: unsafe {
                        RawDetour::new(fn_ptr as *const (), Self::trampoline as *const ()).unwrap()
                    },
                }
            }

            fn call_trampoline(&self, $($ARG:$ARG_TY),*) $(-> $RET_TY)? {
                $ORIGINAL_FN_CALLER(self.detour.trampoline() as *const _ as usize, $($ARG),*)
            }
        }

        lazy_static!(
            static ref $HOOK_STATIC_INSTANCE_NAME: $HOOK_STRUCT_NAME = unsafe {
                $HOOK_STRUCT_NAME::new(std::mem::transmute::<usize, fn($($ARG_TY),*) $(-> $RET_TY)?>($ADDR))
            };
        );
        unsafe { $HOOK_STATIC_INSTANCE_NAME.detour.enable().unwrap() };
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        () -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline0F64, register_call0_f64, () -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        () $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline0, register_call0, () $(-> $RET_TY)? {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline1F64, register_call1_f64, ($ARG1:$ARG1_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty) $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline1, register_call1, ($ARG1:$ARG1_TY) $(-> $RET_TY)? {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline2F64, register_call2_f64,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty, $ARG2:ident:$ARG2_TY:ty) $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline2, register_call2,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY) $(-> $RET_TY)? {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline3F64, register_call3_f64,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty) $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline3, register_call3,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY) $(-> $RET_TY)? {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty,$ARG4:ident:$ARG4_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline4F64, register_call4_f64,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY,$ARG4:$ARG4_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty,$ARG4:ident:$ARG4_TY:ty) $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline4, register_call4,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY,$ARG4:$ARG4_TY) $(-> $RET_TY)? {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($($ARG:ident:$ARG_TY:ty),*) -> $RET_TY:ty
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME,
                ($($ARG:ident:$ARG_TY:ty),*) -> $RET_TY:ty {
                    println!("Detour for {}: {}", stringify!($HOOK_STRUCT_NAME), arg1);
                    let result = $HOOK_STATIC_INSTANCE_NAME.call_trampoline($($ARG),*);
                    println!("Reached end of detour for {}. Result = {}", stringify!($HOOK_STRUCT_NAME), result);
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
                ($($ARG:ident:$ARG_TY:ty),*) {
                    println!("Detour for {}: {}", stringify!($HOOK_STRUCT_NAME), $($ARG),*);
                    $HOOK_STATIC_INSTANCE_NAME.call_trampoline($($ARG),*);
                    println!("Reached end of detour for {}.", stringify!($HOOK_STRUCT_NAME));
                    result
                })
    };
}
