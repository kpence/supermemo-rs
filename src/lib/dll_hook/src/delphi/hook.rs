use detour::*;
use super::register_call::RegisterCall;

#[allow(dead_code)]
pub enum HookParameters {
    Args0,
    Args1(i32),
    Args2(i32, i32),
    Args3(i32, i32, i32),
    Args4(i32, i32, i32, i32),
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum HookResult {
    i32(i32),
    f64(f64),
}

#[allow(dead_code)]
impl HookResult {
    pub fn unwrap_i32(&self) -> i32 {
        match *self {
            HookResult::i32(result) => { result },
            _ => {
                panic!("Attempted to unwrap HookResult. Expected i32, got f64.");
            },
        }
    }

    pub fn unwrap_f64(&self) -> f64 {
        match *self {
            HookResult::f64(result) => { result },
            _ => {
                panic!("Attempted to unwrap HookResult. Expected f64, got i32.");
            },
        }
    }
}

#[allow(dead_code)]
pub enum FnSignature {
    Sig0,
    Sig1,
    Sig2,
    Sig3,
    Sig4,
    Sig0F64,
    Sig1F64,
    Sig2F64,
    Sig3F64,
    Sig4F64,
}

pub struct Hook {
    pub fn_addr: usize,
    pub detour: RawDetour,
    pub fn_signature: FnSignature,
}

impl Hook {
    pub fn call_detour(&self, parameters: &HookParameters) -> HookResult {
        match *parameters {
            HookParameters::Args0 => {
                match self.fn_signature {
                    FnSignature::Sig0 => {
                        HookResult::i32(<() as RegisterCall<i32>>::register_call(self.fn_addr, ()))
                    },
                    FnSignature::Sig0F64 => {
                        HookResult::f64(<() as RegisterCall<f64>>::register_call(self.fn_addr, ()))
                    },
                    _ => {
                        panic!("Not matched");
                    },
                }

            },
            HookParameters::Args1(arg1) => {
                match self.fn_signature {
                    FnSignature::Sig1 => {
                        HookResult::i32(<i32 as RegisterCall<i32>>::register_call(self.fn_addr, arg1))
                    },
                    FnSignature::Sig1F64 => {
                        HookResult::f64(<i32 as RegisterCall<f64>>::register_call(self.fn_addr, arg1))
                    },
                    _ => {
                        panic!("Not matched");
                    },
                }
            },
            HookParameters::Args2(arg1, arg2) => {
                match self.fn_signature {
                    FnSignature::Sig2 => {
                        HookResult::i32(<(i32, i32) as RegisterCall<i32>>::register_call(self.fn_addr, (arg1, arg2)))
                    },
                    FnSignature::Sig2F64 => {
                        HookResult::f64(<(i32, i32) as RegisterCall<f64>>::register_call(self.fn_addr, (arg1, arg2)))
                    },
                    _ => {
                        panic!("Not matched");
                    },
                }
            },
            HookParameters::Args3(arg1, arg2, arg3) => {
                match self.fn_signature {
                    FnSignature::Sig3 => {
                        HookResult::i32(<(i32, i32, i32) as RegisterCall<i32>>::register_call(self.fn_addr, (arg1, arg2, arg3)))
                    },
                    FnSignature::Sig3F64 => {
                        HookResult::f64(<(i32, i32, i32) as RegisterCall<f64>>::register_call(self.fn_addr, (arg1, arg2, arg3)))
                    },
                    _ => {
                        panic!("Not matched");
                    },
                }
            },
            HookParameters::Args4(arg1, arg2, arg3, arg4) => {
                match self.fn_signature {
                    FnSignature::Sig4 => {
                        HookResult::i32(<(i32, i32, i32, i32) as RegisterCall<i32>>::register_call(self.fn_addr, (arg1, arg2, arg3, arg4)))
                    },
                    FnSignature::Sig4F64 => {
                        HookResult::f64(<(i32, i32, i32, i32) as RegisterCall<f64>>::register_call(self.fn_addr, (arg1, arg2, arg3, arg4)))
                    },
                    _ => {
                        panic!("Not matched");
                    },
                }
            },
        }
    }
}
