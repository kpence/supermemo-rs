#![feature(libc, asm, asm_sym, naked_functions)]
pub use detour::*;

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

//"finit; push {arg4}; call {f}; add esp, 4",
//arg4 = in(reg) arg4,
pub fn register_call4(ptr: usize, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> i32 {
    let ret_val: i32;
    unsafe {
        core::arch::asm!(
            "finit; push edi; call ebx; add esp, 4",
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
            "finit; push edi; call ebx; add esp, 4",
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
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline0F64, () -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        () $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline0, () $(-> $RET_TY)? {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline1F64, ($ARG1:$ARG1_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty) $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline1, ($ARG1:$ARG1_TY) $(-> $RET_TY)? {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline2F64,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty, $ARG2:ident:$ARG2_TY:ty) $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline2,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY) $(-> $RET_TY)? {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline3F64,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty) $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline3,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY) $(-> $RET_TY)? {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty,$ARG4:ident:$ARG4_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline4F64,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY,$ARG4:$ARG4_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty,$ARG4:ident:$ARG4_TY:ty) $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline4,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY,$ARG4:$ARG4_TY) $(-> $RET_TY)? {$($BLOCK)*})
    };
}
