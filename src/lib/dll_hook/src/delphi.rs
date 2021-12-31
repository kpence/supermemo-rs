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
                call {};
                add esp, 4",
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
                call {};
                add esp, 4",
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
                call {};
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

pub trait Trampoline2F64 {
    unsafe extern "C" fn real_func(arg1: i32, arg2: i32) -> f64;

    #[naked]
    unsafe extern "C" fn trampoline() -> f64 {
        core::arch::asm!(
            "push ebp;
                mov ebp, esp",
            "push edx;
                push eax;
                call {};
                add esp, 8",
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
    println!("register_call2");
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
    println!("register_call2");
    ret_val
}

#[macro_export]
macro_rules! foreign_fn {
    ($ADDR:expr, $FN_PTR_NAME:ident, $FN_SIGNATURE:ty) => {
        lazy_static! {
            static ref $FN_PTR_NAME: $FN_SIGNATURE =
                unsafe { std::mem::transmute::<u32, $FN_SIGNATURE>($ADDR) };
        }
    };
}

#[macro_export]
macro_rules! hijack {
    (
        $ADDR:expr,
        $FOREIGN_FN_PTR_NAME:ident,
        $DETOUR_NAME: ident,
        $TRAMPOLINE_NAME: ident,
        $TRAMPOLINE_TYPE: ident,
        ($($ARG:ident:$ARG_TY:ty,)*) $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        struct $TRAMPOLINE_NAME { }

        impl $TRAMPOLINE_TYPE for $TRAMPOLINE_NAME {
            unsafe extern "C" fn real_func($($ARG:$ARG_TY,)*) $(-> $RET_TY)? {$($BLOCK)*}
        }
        lazy_static!(
            static ref $FOREIGN_FN_PTR_NAME: fn($($ARG_TY,)*) $(-> $RET_TY)? =
                unsafe { std::mem::transmute::<usize, fn($($ARG_TY,)*) $(-> $RET_TY)?>($ADDR) };
            static ref $DETOUR_NAME: RawDetour = unsafe {
                RawDetour::new(*$FOREIGN_FN_PTR_NAME as *const (), $TRAMPOLINE_NAME::trampoline as *const ()).unwrap()
            };
        );
        unsafe { $DETOUR_NAME.enable().unwrap() };
    };
    (
        $ADDR:expr,
        $FOREIGN_FN_PTR_NAME:ident,
        $DETOUR_NAME: ident,
        $TRAMPOLINE_NAME: ident,
        () $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $FOREIGN_FN_PTR_NAME, $DETOUR_NAME, $TRAMPOLINE_NAME, Trampoline0, () $(-> $RET_TY)? {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $FOREIGN_FN_PTR_NAME:ident,
        $DETOUR_NAME: ident,
        $TRAMPOLINE_NAME: ident,
        ($ARG1:ident:$ARG1_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $FOREIGN_FN_PTR_NAME, $DETOUR_NAME, $TRAMPOLINE_NAME, Trampoline1F64, ($ARG1:$ARG1_TY,) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $FOREIGN_FN_PTR_NAME:ident,
        $DETOUR_NAME: ident,
        $TRAMPOLINE_NAME: ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $FOREIGN_FN_PTR_NAME, $DETOUR_NAME, $TRAMPOLINE_NAME, Trampoline2F64,
                ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $FOREIGN_FN_PTR_NAME:ident,
        $DETOUR_NAME: ident,
        $TRAMPOLINE_NAME: ident,
        ($ARG1:ident:$ARG1_TY:ty) $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $FOREIGN_FN_PTR_NAME, $DETOUR_NAME, $TRAMPOLINE_NAME, Trampoline1, ($ARG1:$ARG1_TY,) $(-> $RET_TY)? {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $FOREIGN_FN_PTR_NAME:ident,
        $DETOUR_NAME: ident,
        $TRAMPOLINE_NAME: ident,
        ($ARG1:ident:$ARG1_TY:ty, $ARG2:ident:$ARG2_TY:ty) $(-> $RET_TY:ty)? {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $FOREIGN_FN_PTR_NAME, $DETOUR_NAME, $TRAMPOLINE_NAME, Trampoline2,
                ($ARG1:$ARG1_TY, $ARG2:$ARG2_TY,) $(-> $RET_TY)? {$($BLOCK)*})
    };
}
