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
        $SIGNATURE_TYPE: ident,
        ($($ARG:ident:$ARG_TY:ty),*) -> $RET_TY:ty {$($BLOCK:tt)*}
    ) => {
        struct $HOOK_STRUCT_NAME(Hook);

        impl $TRAMPOLINE_TYPE for $HOOK_STRUCT_NAME {
            unsafe extern "C" fn real_func($($ARG:$ARG_TY),*) -> $RET_TY {$($BLOCK)*}
        }

        #[allow(unused_parens)]
        #[allow(dead_code)]
        impl $HOOK_STRUCT_NAME {
            fn new(fn_addr: usize) -> Self {
                Self(
                    Hook {
                        fn_addr: fn_addr,
                        detour: unsafe {
                            RawDetour::new(fn_addr as *const (), Self::trampoline as *const ()).unwrap()
                        },
                        fn_signature: FnSignature::$SIGNATURE_TYPE,
                    }
                )
            }

            fn call_detour(&self, $($ARG:$ARG_TY),*) -> $RET_TY {
                <($($ARG_TY),*) as RegisterCall<$RET_TY>>::register_call(
                    self.0.fn_addr,
                    ($($ARG),*)
                )
            }

            fn call_trampoline(&self, $($ARG:$ARG_TY),*) -> $RET_TY {
                <($($ARG_TY),*) as RegisterCall<$RET_TY>>::register_call(
                    self.0.detour.trampoline() as *const _ as usize,
                    ($($ARG),*)
                )
            }
        }

        static $HOOK_STATIC_INSTANCE_NAME: Lazy<$HOOK_STRUCT_NAME> = Lazy::new(|| $HOOK_STRUCT_NAME::new($ADDR));

        #[allow(unused_unsafe)]
        unsafe { $HOOK_STATIC_INSTANCE_NAME.0.detour.enable() }.unwrap();
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        () -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline0F64, Sig0F64,
                () -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        () -> $RET_TY:ty {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline0, Sig0,
                () -> $RET_TY {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline1F64, Sig1F64,
                ($ARG1:$ARG1_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty) -> $RET_TY:ty {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline1, Sig1,
                ($ARG1:$ARG1_TY) -> $RET_TY {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline2F64, Sig2F64,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty, $ARG2:ident:$ARG2_TY:ty) -> $RET_TY:ty {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline2, Sig2,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY) -> $RET_TY {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline3F64, Sig3F64,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty) -> $RET_TY:ty {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline3, Sig3,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY) -> $RET_TY {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty,$ARG4:ident:$ARG4_TY:ty) -> f64 {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline4F64, Sig4F64,
                ($ARG1:$ARG1_TY,$ARG2:$ARG2_TY,$ARG3:$ARG3_TY,$ARG4:$ARG4_TY) -> f64 {$($BLOCK)*})
    };
    (
        $ADDR:expr,
        $HOOK_STATIC_INSTANCE_NAME:ident,
        $HOOK_STRUCT_NAME:ident,
        ($ARG1:ident:$ARG1_TY:ty,$ARG2:ident:$ARG2_TY:ty,$ARG3:ident:$ARG3_TY:ty,$ARG4:ident:$ARG4_TY:ty) -> $RET_TY:ty {$($BLOCK:tt)*}
    ) => {
        hijack!($ADDR, $HOOK_STATIC_INSTANCE_NAME, $HOOK_STRUCT_NAME, Trampoline4, Sig4,
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
