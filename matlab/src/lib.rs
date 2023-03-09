#![cfg(target_os = "windows")]
#![allow(dead_code)]

use std::ffi::{c_char, c_int, CStr, CString};

use config::ConfigError;

#[no_mangle]
pub extern "C" fn mexFunction(
    nlhs: c_int,
    _plhs: *mut *mut mxArray,
    nrhs: c_int,
    prhs: *const *const mxArray,
) {
    // Check that the number of inputs and outputs are correct
    if nrhs != 1 {
        matlab_error(
            "The sett function must be called with one input: sett('path_to_config.toml')",
        );
    }
    if nlhs > 1 {
        matlab_error("A single struct is returned");
    }

    // Read the config file
    let config = match config_from_rhs_arg(prhs) {
        Ok(config) => config,
        Err(message) => {
            matlab_error(message);
            return;
        }
    };

    // Run the engine
    sett::run_from_config(config);

    // TODO:
    // 1) Check that prhs arg is char array and not string
    // 2) Modify `run_from_config` to return `RunResults`
    // 3) Create and fill MATLAB struct for `RunResults`

    matlab_error("things worked!");
}

/// Attempt to read the config file provided as MATLAB argument
///
/// If anything goes wrong, this function returns a `&'static str` with an
/// error message intended to be sent to MATLAB.
fn config_from_rhs_arg(prhs: *const *const mxArray) -> Result<sett::Config, &'static str> {
    let file_path = unsafe {
        let str_ptr = mxArrayToString(*prhs);
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(str) => str,
            Err(_) => {
                return Err("unknown error"); // unlikely to ever happen
            }
        }
    };

    let config = match config::Config::builder()
        .add_source(config::File::with_name(file_path))
        .build()
    {
        Ok(config) => config,
        Err(err) => {
            let error = match err {
                ConfigError::NotFound(_) => "file not found",
                _ => "unknown config error",
            };
            return Err(error);
        }
    };

    config
        .try_deserialize::<sett::Config>()
        .map_err(|_| "invalid config file")
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
    fn mxArrayToString(pa: *const mxArray) -> *const c_char;
}

fn matlab_error(msg: &str) {
    let msg = CString::new(msg).expect("CString::new should never fail");
    unsafe {
        mexErrMsgTxt(msg.as_ptr());
    }
}
