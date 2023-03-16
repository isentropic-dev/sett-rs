#![cfg(target_os = "windows")]
#![allow(dead_code)]

use std::ffi::{c_char, c_int, CStr, CString};

use config::ConfigError;

use sett::api::RunResults;

#[no_mangle]
pub extern "C" fn mexFunction(
    nlhs: c_int,
    plhs: *mut *mut mxArray,
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

    let results = RunResults::default();
    let matrix = create_struct_matrix(results);

    // TODO:
    // 1) Check that prhs arg is char array and not string
    // 2) Modify `run_from_config` to return `RunResults`
    // 3) Create and fill MATLAB struct for `RunResults`

    unsafe {
        *plhs = matrix;
    }
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

struct FieldNames {
    names: Vec<CString>,
    pointers: Vec<*const i8>,
    count: i32,
}

impl FieldNames {
    /// Create a new `FieldNames`
    fn new<const N: usize>(names: [&str; N]) -> Self {
        let names: Vec<_> = names
            .into_iter()
            .map(|name| CString::new(name).unwrap())
            .collect();
        let pointers = names.iter().map(|x| x.as_ptr()).collect();
        Self {
            names,
            pointers,
            count: N as i32,
        }
    }

    /// Create a MATLAB struct matrix using names from `self`
    fn create_matrix(&self) -> *mut mxArray {
        unsafe { mxCreateStructMatrix(1, 1, self.count, self.pointers.as_ptr()) }
    }
}

/// Create a scalar matrix
unsafe fn scalar(value: f64) -> *const mxArray {
    let matrix = mxCreateDoubleMatrix_730(1, 1, mxComplexity::Real);
    *mxGetPr(matrix) = value;
    matrix
}

/// Create a vector matrix
unsafe fn vector(values: Vec<f64>) -> *const mxArray {
    let matrix = mxCreateDoubleMatrix_730(values.len(), 1, mxComplexity::Real);
    let ptr = mxGetPr(matrix);
    for (i, value) in values.into_iter().enumerate() {
        *ptr.offset(i as isize) = value;
    }
    matrix
}

/// Convert `RunResults` into a MATLAB struct matrix
fn create_struct_matrix(results: RunResults) -> *mut mxArray {
    let efficiency = unsafe {
        let names = FieldNames::new(["mechanical", "overall"]);
        let matrix = names.create_matrix();
        mxSetFieldByNumber(matrix, 0, 0, scalar(results.efficiency.mechanical));
        mxSetFieldByNumber(matrix, 0, 1, scalar(results.efficiency.overall));
        matrix
    };

    let heat_flow = unsafe {
        let names = FieldNames::new(["input", "rejection", "chx", "regen", "hhx"]);
        let matrix = names.create_matrix();
        mxSetFieldByNumber(matrix, 0, 0, scalar(results.heat_flow.input));
        mxSetFieldByNumber(matrix, 0, 1, scalar(results.heat_flow.rejection));
        mxSetFieldByNumber(matrix, 0, 2, scalar(results.heat_flow.chx));
        mxSetFieldByNumber(matrix, 0, 3, scalar(results.heat_flow.regen));
        mxSetFieldByNumber(matrix, 0, 4, scalar(results.heat_flow.hhx));
        matrix
    };

    let mass_flow = unsafe {
        let names = FieldNames::new(["chx", "regen", "hhx"]);
        let matrix = names.create_matrix();
        mxSetFieldByNumber(matrix, 0, 0, scalar(results.mass_flow.chx));
        mxSetFieldByNumber(matrix, 0, 1, scalar(results.mass_flow.regen));
        mxSetFieldByNumber(matrix, 0, 2, scalar(results.mass_flow.hhx));
        matrix
    };

    let power = unsafe {
        let names = FieldNames::new(["ideal_indicated", "indicated", "shaft", "net"]);
        let matrix = names.create_matrix();
        mxSetFieldByNumber(matrix, 0, 0, scalar(results.power.ideal_indicated));
        mxSetFieldByNumber(matrix, 0, 1, scalar(results.power.indicated));
        mxSetFieldByNumber(matrix, 0, 2, scalar(results.power.shaft));
        mxSetFieldByNumber(matrix, 0, 3, scalar(results.power.net));
        matrix
    };

    let pressure = unsafe {
        let names = FieldNames::new(["avg", "max", "min", "t_zero"]);
        let matrix = names.create_matrix();
        mxSetFieldByNumber(matrix, 0, 0, scalar(results.pressure.avg));
        mxSetFieldByNumber(matrix, 0, 1, scalar(results.pressure.max));
        mxSetFieldByNumber(matrix, 0, 2, scalar(results.pressure.min));
        mxSetFieldByNumber(matrix, 0, 3, scalar(results.pressure.t_zero));
        matrix
    };

    let temperature = unsafe {
        let names = FieldNames::new([
            "sink",
            "chx",
            "regen_cold",
            "regen_avg",
            "regen_hot",
            "hhx",
            "source",
        ]);
        let matrix = names.create_matrix();
        mxSetFieldByNumber(matrix, 0, 0, scalar(results.temperature.sink));
        mxSetFieldByNumber(matrix, 0, 1, scalar(results.temperature.chx));
        mxSetFieldByNumber(matrix, 0, 2, scalar(results.temperature.regen_cold));
        mxSetFieldByNumber(matrix, 0, 3, scalar(results.temperature.regen_avg));
        mxSetFieldByNumber(matrix, 0, 4, scalar(results.temperature.regen_hot));
        mxSetFieldByNumber(matrix, 0, 5, scalar(results.temperature.hhx));
        mxSetFieldByNumber(matrix, 0, 6, scalar(results.temperature.source));
        matrix
    };

    let values = unsafe {
        let names = FieldNames::new([
            "time", "P", "P_c", "P_e", "T_c", "T_e", "m_dot_ck", "m_dot_kr", "m_dot_rl",
            "m_dot_le", "Q_dot_k", "Q_dot_r", "Q_dot_l",
        ]);
        let matrix = names.create_matrix();
        mxSetFieldByNumber(matrix, 0, 0, vector(results.values.time));
        mxSetFieldByNumber(matrix, 0, 1, vector(results.values.P));
        mxSetFieldByNumber(matrix, 0, 2, vector(results.values.P_c));
        mxSetFieldByNumber(matrix, 0, 3, vector(results.values.P_e));
        mxSetFieldByNumber(matrix, 0, 4, vector(results.values.T_c));
        mxSetFieldByNumber(matrix, 0, 5, vector(results.values.T_e));
        mxSetFieldByNumber(matrix, 0, 6, vector(results.values.m_dot_ck));
        mxSetFieldByNumber(matrix, 0, 7, vector(results.values.m_dot_kr));
        mxSetFieldByNumber(matrix, 0, 8, vector(results.values.m_dot_rl));
        mxSetFieldByNumber(matrix, 0, 9, vector(results.values.m_dot_le));
        mxSetFieldByNumber(matrix, 0, 10, vector(results.values.Q_dot_k));
        mxSetFieldByNumber(matrix, 0, 11, vector(results.values.Q_dot_r));
        mxSetFieldByNumber(matrix, 0, 12, vector(results.values.Q_dot_l));
        matrix
    };

    let matrix = unsafe {
        let names = FieldNames::new([
            "efficiency",
            "heat_flow",
            "mass_flow",
            "power",
            "pressure",
            "temperature",
            "values",
        ]);
        let matrix = names.create_matrix();
        mxSetFieldByNumber(matrix, 0, 0, efficiency);
        mxSetFieldByNumber(matrix, 0, 1, heat_flow);
        mxSetFieldByNumber(matrix, 0, 2, mass_flow);
        mxSetFieldByNumber(matrix, 0, 3, power);
        mxSetFieldByNumber(matrix, 0, 4, pressure);
        mxSetFieldByNumber(matrix, 0, 5, temperature);
        mxSetFieldByNumber(matrix, 0, 6, values);
        matrix
    };

    return matrix;
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
    fn mxSetFieldByNumber(pm: *mut mxArray, index: usize, fieldnumber: i32, pvalue: *const mxArray);
    fn mxArrayToString(pa: *const mxArray) -> *const c_char;
}

fn matlab_error(msg: &str) {
    let msg = CString::new(msg).expect("CString::new should never fail");
    unsafe {
        mexErrMsgTxt(msg.as_ptr());
    }
}
