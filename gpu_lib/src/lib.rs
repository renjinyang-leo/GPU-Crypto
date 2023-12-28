#![cfg_attr(
    target_os = "cuda",
    no_std,
    feature(register_attr),
    register_attr(nvvm_internal)
)]
#![allow(improper_ctypes_definitions)]

use cuda_std::*;
pub type Vec16<T = f32> = vek::Vec16<T>;

#[kernel]
pub unsafe fn encrypt() {
    
}