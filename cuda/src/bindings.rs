#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(missing_debug_implementations)]
#![allow(improper_ctypes)]

use std::ffi::{c_void, c_float};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const CU_LAUNCH_PARAM_END: *mut c_void = 0 as *mut c_void;
pub const CU_LAUNCH_PARAM_BUFFER_POINTER: *mut c_void = 1 as *mut c_void;
pub const CU_LAUNCH_PARAM_BUFFER_SIZE: *mut c_void = 2 as *mut c_void;

#[link(name = "kernels", kind = "static")]
extern "C" {
    pub fn calcGradient(
        batch_size: usize,
        hidden_size: usize,
        input_size: usize,
        featureWeights: *const c_float,
        featureBiases: *const c_float,
        outputWeights: *const c_float,
        outputBiases: *const c_float,
        ourInputs: *const u16,
        oppInputs: *const u16,
        results: *const c_float,
        featureWeightsGradient: *mut c_float,
        featureBiasesGradient: *mut c_float,
        outputWeightsGradient: *mut c_float,
        outputBiasesGradient: *mut c_float,
        error: *mut c_float
    ) -> cudaError_t;
}