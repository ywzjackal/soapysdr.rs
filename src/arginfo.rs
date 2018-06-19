use std;
use super::sys;

pub enum ArgInfoType {
  Bool = sys::SoapySDRArgInfoType_SOAPY_SDR_ARG_INFO_BOOL as isize,
  Int = sys::SoapySDRArgInfoType_SOAPY_SDR_ARG_INFO_INT as isize,
  Float = sys::SoapySDRArgInfoType_SOAPY_SDR_ARG_INFO_FLOAT as isize,
  String = sys::SoapySDRArgInfoType_SOAPY_SDR_ARG_INFO_STRING as isize,
}

pub struct ArgInfo {
  pub(crate) inner: sys::SoapySDRArgInfo,
}

pub struct ArgInfoList<'a> {
  pub(crate) inner: &'a[sys::SoapySDRArgInfo],
}

impl ArgInfo {
  pub fn empty() -> Self {
    ArgInfo {
      inner: sys::SoapySDRArgInfo {
        key: std::ptr::null_mut(),
        value : std::ptr::null_mut(),
        name: std::ptr::null_mut(),
        description: std::ptr::null_mut(),
        units: std::ptr::null_mut(),
        type_: 0,
        range: sys::SoapySDRRange {
          minimum: 0f64,
          maximum: 0f64,
          step: 0f64,
        },
        numOptions: 0,
        options: std::ptr::null_mut(),
        optionNames: std::ptr::null_mut(),
        __bindgen_padding_0: 0,
      }
    }
  }
}

impl Drop for ArgInfo {
  fn drop(&mut self) {
    unsafe {
      sys::SoapySDRArgInfo_clear(&mut self.inner);
    }
  }
}

impl <'a>Drop for ArgInfoList<'a> {
  fn drop(&mut self) {
    unsafe {
      sys::SoapySDRArgInfoList_clear(
        self.inner.as_ptr() as *mut sys::SoapySDRArgInfo, 
        self.inner.len(),
      );
    }
  }
}

impl From<sys::SoapySDRArgInfo> for ArgInfo {
  fn from(inner: sys::SoapySDRArgInfo) -> ArgInfo {
    ArgInfo{inner}
  }
}

impl <'a>From<(usize, *mut sys::SoapySDRArgInfo)> for ArgInfoList<'a> {
  fn from((length, inner): (usize, *mut sys::SoapySDRArgInfo)) -> ArgInfoList<'a> {
    ArgInfoList {
      inner: unsafe{ std::slice::from_raw_parts(inner, length) }
    }
  }
}