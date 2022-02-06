pub trait RegisterCall<Output> {
    fn register_call(fn_ptr: usize, args: Self) -> Output;
}

impl RegisterCall<i32> for () {
    fn register_call(fn_ptr: usize, _: Self) -> i32 {
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
    fn register_call(fn_ptr: usize, _: Self) -> f64 {
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
    fn register_call(fn_ptr: usize, arg1: Self) -> i32 {
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
    fn register_call(fn_ptr: usize, arg1: Self) -> f64 {
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
    fn register_call(fn_ptr: usize, (arg1, arg2): Self) -> i32 {
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
    fn register_call(fn_ptr: usize, (arg1, arg2): Self) -> f64 {
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
    fn register_call(fn_ptr: usize, (arg1, arg2, arg3): Self) -> i32 {
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
    fn register_call(fn_ptr: usize, (arg1, arg2, arg3): Self) -> f64 {
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
    fn register_call(fn_ptr: usize, (arg1, arg2, arg3, arg4): Self) -> i32 {
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
    fn register_call(fn_ptr: usize, (arg1, arg2, arg3, arg4): Self) -> f64 {
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
