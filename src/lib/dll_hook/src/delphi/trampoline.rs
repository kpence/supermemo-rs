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
            options(noreturn)
        );
    }
}
