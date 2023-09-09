#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "klu")]
use crate::{
    klu_common as klu_common_, klu_l_common as klu_l_common_, klu_l_numeric as klu_l_numeric_,
    klu_l_symbolic as klu_l_symbolic_, klu_numeric as klu_numeric_, klu_symbolic as klu_symbolic_,
};

#[cfg(feature = "klu")]
extern "C" {
    pub fn klu_analyze(
        n: i32,
        Ap: *const i32,
        Ai: *const i32,
        Common: *mut klu_common_,
    ) -> *mut klu_symbolic_;

    pub fn klu_factor(
        Ap: *const i32,
        Ai: *const i32,
        Ax: *const f64,
        Symbolic: *mut klu_symbolic_,
        Common: *mut klu_common_,
    ) -> *mut klu_numeric_;

    pub fn klu_l_analyze(
        n: i64,
        Ap: *const i64,
        Ai: *const i64,
        Common: *mut klu_l_common_,
    ) -> *mut klu_l_symbolic_;

    pub fn klu_l_factor(
        Ap: *const i64,
        Ai: *const i64,
        Ax: *const f64,
        Symbolic: *mut klu_l_symbolic_,
        Common: *mut klu_l_common_,
    ) -> *mut klu_l_numeric_;
}
