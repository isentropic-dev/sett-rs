#![cfg(target_os = "windows")]
#![allow(dead_code)]

use std::ffi::CString;
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub extern "C" fn mexFunction(
    nlhs: c_int,
    _plhs: *mut *mut mxArray,
    nrhs: c_int,
    _prhs: *const *const mxArray,
) {
    // Check that the number of inputs and outputs are correct
    if nrhs != 1 {
        matlab_error(
            "The sett function must be called with one input: sett(\"path_to_config.toml\")",
        );
    }
    if nlhs > 1 {
        matlab_error("A single struct is returned");
    }
}

#[repr(C)]
pub struct mxArray {
    _private: [u8; 0],
}

#[repr(i32)]
#[allow(non_camel_case_types)]
enum mxComplexity {
    Real = 0,
    Complex = 1,
}

#[link(name = "libmex")]
extern "C" {
    fn mexErrMsgTxt(fmt: *const c_char);
}

#[link(name = "libmx")]
extern "C" {
    fn mxIsDouble(ps: *const mxArray) -> bool;
    fn mxIsComplex(ps: *const mxArray) -> bool;
    fn mxGetM(ps: *const mxArray) -> usize;
    fn mxGetN(ps: *const mxArray) -> usize;
    fn mxGetPr(pa: *const mxArray) -> *mut f64;
    fn mxCreateDoubleMatrix_730(
        mrows: usize,
        ncols: usize,
        complex_flag: mxComplexity,
    ) -> *mut mxArray;
    fn mxCreateStructMatrix(
        m: usize,
        n: usize,
        nfields: i32,
        fieldnames: *const *const c_char,
    ) -> *mut mxArray;
    fn mxSetFieldByNumber(pm: *mut mxArray, index: usize, fieldnumber: i32, pvalue: *mut mxArray);
}

fn matlab_error(msg: &str) {
    let msg = CString::new(msg).expect("CString::new failed");
    unsafe {
        mexErrMsgTxt(msg.as_ptr());
    }
}
