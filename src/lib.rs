//! Xenoverse 2 Save Converter Library
//! Provides functionality to convert between PS4 and PC save formats

mod constants;
mod utils;
mod io;
mod marker;
mod conversion;

pub use constants::*;
pub use utils::sha1_hex;
pub use io::write_output_file;
pub use marker::{has_dual_magic, has_any_marker_at_08, has_magic_at, make_marker, try_read_marker, looks_like_v2};

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Converts a PS4 save file to PC-ready format
pub fn ps4_to_pcready(data: &[u8], input_path: &str, dir: &str) -> Result<Vec<u8>, anyhow::Error> {
    conversion::ps4_to_pcready(data, input_path, dir)
}

/// Converts a PC-ready save file to PS4 format
pub fn pcready_to_ps4(data: &[u8], input_path: &str, dir: &str, has_leftovers_flag: bool) -> Result<Vec<u8>, anyhow::Error> {
    conversion::pcready_to_ps4(data, input_path, dir, has_leftovers_flag)
}

/// Automatically detects the format and converts accordingly
pub fn convert_auto(data: &[u8], input_path: &str, dir: &str) -> Result<Vec<u8>, anyhow::Error> {
    conversion::convert_auto(data, input_path, dir)
}

#[cfg(feature = "python")]
/// Python bindings for the Xenoverse 2 save converter
#[pymethods]
impl PyXenoverse2Converter {
    #[new]
    fn new() -> Self {
        PyXenoverse2Converter {}
    }

    fn ps4_to_pcready(&self, data: &[u8], input_path: &str, dir: &str) -> PyResult<Vec<u8>> {
        ps4_to_pcready(data, input_path, dir)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))
    }

    fn pcready_to_ps4(&self, data: &[u8], input_path: &str, dir: &str, has_leftovers_flag: bool) -> PyResult<Vec<u8>> {
        pcready_to_ps4(data, input_path, dir, has_leftovers_flag)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))
    }

    fn convert_auto(&self, data: &[u8], input_path: &str, dir: &str) -> PyResult<Vec<u8>> {
        convert_auto(data, input_path, dir)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))
    }
}

#[cfg(feature = "python")]
#[pyclass]
pub struct PyXenoverse2Converter {}

#[cfg(feature = "python")]
#[pymodule]
fn xv2_converter_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyXenoverse2Converter>()?;
    Ok(())
}

#[cfg(feature = "c")]
mod c_api {
    use std::os::raw::c_char;
    use std::ffi::CStr;
    use std::ptr;
    use libc::{malloc, free, c_void};
    
    /// C-compatible function to convert PS4 to PC-ready format
    /// Returns a pointer to the converted data (caller must free with free_buffer)
    #[no_mangle]
    pub extern "C" fn ps4_to_pcready_c(
        data: *const u8,
        data_len: usize,
        input_path: *const c_char,
        dir: *const c_char,
        out_len: *mut usize,
    ) -> *mut u8 {
        if data.is_null() || input_path.is_null() || dir.is_null() || out_len.is_null() {
            return ptr::null_mut();
        }

        let input_data = unsafe { std::slice::from_raw_parts(data, data_len) };
        
        let input_path_str = unsafe {
            match CStr::from_ptr(input_path).to_str() {
                Ok(s) => s,
                Err(_) => return ptr::null_mut(),
            }
        };
        
        let dir_str = unsafe {
            match CStr::from_ptr(dir).to_str() {
                Ok(s) => s,
                Err(_) => return ptr::null_mut(),
            }
        };

        match super::ps4_to_pcready(input_data, input_path_str, dir_str) {
            Ok(result) => {
                unsafe { *out_len = result.len(); }
                
                // Allocate memory and copy the result
                let output_ptr = unsafe { 
                    malloc(result.len()) as *mut u8
                };
                
                if output_ptr.is_null() {
                    return ptr::null_mut();
                }
                
                unsafe {
                    std::ptr::copy_nonoverlapping(result.as_ptr(), output_ptr, result.len());
                }
                
                output_ptr
            }
            Err(_) => ptr::null_mut(),
        }
    }

    /// C-compatible function to convert PC-ready to PS4 format
    #[no_mangle]
    pub extern "C" fn pcready_to_ps4_c(
        data: *const u8,
        data_len: usize,
        input_path: *const c_char,
        dir: *const c_char,
        has_leftovers_flag: bool,
        out_len: *mut usize,
    ) -> *mut u8 {
        if data.is_null() || input_path.is_null() || dir.is_null() || out_len.is_null() {
            return ptr::null_mut();
        }

        let input_data = unsafe { std::slice::from_raw_parts(data, data_len) };
        
        let input_path_str = unsafe {
            match CStr::from_ptr(input_path).to_str() {
                Ok(s) => s,
                Err(_) => return ptr::null_mut(),
            }
        };
        
        let dir_str = unsafe {
            match CStr::from_ptr(dir).to_str() {
                Ok(s) => s,
                Err(_) => return ptr::null_mut(),
            }
        };

        match super::pcready_to_ps4(input_data, input_path_str, dir_str, has_leftovers_flag) {
            Ok(result) => {
                unsafe { *out_len = result.len(); }
                
                // Allocate memory and copy the result
                let output_ptr = unsafe { 
                    malloc(result.len()) as *mut u8
                };
                
                if output_ptr.is_null() {
                    return ptr::null_mut();
                }
                
                unsafe {
                    std::ptr::copy_nonoverlapping(result.as_ptr(), output_ptr, result.len());
                }
                
                output_ptr
            }
            Err(_) => ptr::null_mut(),
        }
    }

    /// C-compatible function to automatically detect and convert save format
    #[no_mangle]
    pub extern "C" fn convert_auto_c(
        data: *const u8,
        data_len: usize,
        input_path: *const c_char,
        dir: *const c_char,
        out_len: *mut usize,
    ) -> *mut u8 {
        if data.is_null() || input_path.is_null() || dir.is_null() || out_len.is_null() {
            return ptr::null_mut();
        }

        let input_data = unsafe { std::slice::from_raw_parts(data, data_len) };
        
        let input_path_str = unsafe {
            match CStr::from_ptr(input_path).to_str() {
                Ok(s) => s,
                Err(_) => return ptr::null_mut(),
            }
        };
        
        let dir_str = unsafe {
            match CStr::from_ptr(dir).to_str() {
                Ok(s) => s,
                Err(_) => return ptr::null_mut(),
            }
        };

        match super::convert_auto(input_data, input_path_str, dir_str) {
            Ok(result) => {
                unsafe { *out_len = result.len(); }
                
                // Allocate memory and copy the result
                let output_ptr = unsafe { 
                    malloc(result.len()) as *mut u8
                };
                
                if output_ptr.is_null() {
                    return ptr::null_mut();
                }
                
                unsafe {
                    std::ptr::copy_nonoverlapping(result.as_ptr(), output_ptr, result.len());
                }
                
                output_ptr
            }
            Err(_) => ptr::null_mut(),
        }
    }

    /// Free memory allocated by the conversion functions
    #[no_mangle]
    pub extern "C" fn free_buffer(ptr: *mut u8) {
        if !ptr.is_null() {
            unsafe {
                free(ptr as *mut c_void);
            }
        }
    }
}