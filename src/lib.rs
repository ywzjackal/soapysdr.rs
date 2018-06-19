#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;
extern crate num;

macro_rules! unsafe_into_str {
    ($exp: expr) => ({        
        let cstr = unsafe {
            let p = chkp($exp)?;
            std::ffi::CStr::from_ptr(p)
        };
        cstr.to_str().map_err(|e| e.into())
    })
}

macro_rules! unsafe_into {
    ($exp: expr) => ({
        let p = unsafe { $exp };
        Ok(p.into())
    })
}

macro_rules! unsafe_check_return {
    ($exp: expr) => ({
        let p = unsafe { $exp };
        if p >= 0 {
            Ok(p)
        } else {
            Err(Error::ReturnCode(p))
        }
    })
}

mod types;
mod wrapper;
mod error;
mod kwargs;
mod arginfo;
mod device;
mod stream;

pub mod sys {
    pub use wrapper::*;
}
pub use types::*;
pub use kwargs::*;
pub use arginfo::*;
pub use error::*;
pub use device::*;
pub use stream::*;

pub fn root_path<'a>() -> Result<&'a str> {
    unsafe_into_str! (
        sys::SoapySDR_getRootPath()
    )
}

pub fn search_paths() -> Result<CStrList> {
    let mut len = 0;
    unsafe_into!((len, sys::SoapySDR_listSearchPaths(&mut len)))
}

pub fn modules() -> Result<CStrList> {
    let mut len = 0;
    unsafe_into!((len, sys::SoapySDR_listModules(&mut len)))
}

pub fn modules_path(path: &str) -> Result<CStrList> {
    let mut len = 0;
    unsafe_into!((len, sys::SoapySDR_listModulesPath(path.as_ptr() as *const i8, &mut len)))
}

pub fn load_module(path: &str) -> Result<&str> {
    unsafe_into_str!(sys::SoapySDR_loadModule(
        path.as_ptr() as *const i8
    ))
}

pub fn loader_result(path: &str) -> Result<Kwargs> {
    unsafe_into!(sys::SoapySDR_getLoaderResult(path.as_ptr() as *const i8))
}

pub fn module_version(path: &str) -> Result<&str> {
    unsafe_into_str!(sys::SoapySDR_getModuleVersion(path.as_ptr() as *const i8))
}

pub fn unload_module(path: &str) -> Result<&str> {
    unsafe_into_str!(sys::SoapySDR_unloadModule(path.as_ptr() as *const i8))
}

pub fn load_modules() {
    unsafe {
        sys::SoapySDR_loadModules();
    }
}

pub fn ticks_to_time_ns(ticks: i64, rate: f64) -> i64 {
    unsafe {
        sys::SoapySDR_ticksToTimeNs(
            ticks, rate
        )
    }
}

pub fn time_ns_to_ticks(time_ns: i64, rate: f64) -> i64 {
    unsafe {
        sys::SoapySDR_timeNsToTicks(
            time_ns, rate
        )
    }
}

pub fn api_version<'a>() -> Result<&'a str> {
    unsafe_into_str!(sys::SoapySDR_getAPIVersion())
}

pub fn abi_version<'a>() -> Result<&'a str> {
    unsafe_into_str!(sys::SoapySDR_getABIVersion())
}

pub fn lib_version<'a>() -> Result<&'a str> {
    unsafe_into_str!(sys::SoapySDR_getLibVersion())
}

pub const TX: i32 = sys::SOAPY_SDR_TX as i32;
pub const RX: i32 = sys::SOAPY_SDR_RX as i32;