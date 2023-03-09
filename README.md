# sett-rs

## Building a MATLAB mex file

In order to compile a mex file for use in MATLAB, cargo must know where the
MATLAB-provided "libmex.lib" and "libmx.lib" files are located.  The location
of these files depends on the platform and installed MATLAB version, so we
provide it to cargo as an environment variable.

1. Build the MATLAB package with `MATLAB_LIB_DIR=path_to_matlab_lib_dir cargo build --release -p matlab`
2. Rename the compiled library with `mv target/release/matlab.[dll/dylib/so] sett.[mexw64/mexmaci64/mexa64]`

Windows example using PowerShell:
```
$env:MATLAB_LIB_DIR="C:\Program Files\MATLAB\R2021b\extern\lib\win64\microsoft";
cargo build --release -p matlab;
mv target/release/matlab.dll sett.mexw64;
$env:MATLAB_LIB_DIR=$null
```
