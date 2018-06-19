use std;

use super::*;

#[derive(Debug)]
pub struct Range {
  pub minimum: f64,
  pub maximum: f64,
  pub step: f64,
}

#[derive(Debug)]
pub struct CStrList {
  pub(crate) length: usize,
  pub(crate) inner: *mut *mut std::os::raw::c_char,
}

#[derive(Debug)]
pub struct StreamBuffer {
  pub(crate) inner: *mut std::os::raw::c_void,
}

#[derive(Debug)]
pub struct RangeList {
  pub(crate) length: usize,
  pub(crate) inner: *mut sys::SoapySDRRange,
}

impl CStrList {
  pub fn new(length: usize, inner: *mut *mut std::os::raw::c_char) -> Self {
    Self {length, inner}
  }

  pub fn to_slice_cstr(&self) -> &[*mut std::os::raw::c_char] {
    unsafe { std::slice::from_raw_parts(self.inner, self.length) }
  }

  pub fn collect_string(&self) -> Vec<String> {
    let mut vec = Vec::with_capacity(self.length);
    let slice = self.to_slice_cstr();
    for i in 0 .. slice.len() {
      match unsafe{ std::ffi::CStr::from_ptr(slice[i])}.to_str() {
        Ok(s) => vec.push(s.to_owned()),
        Err(e) => vec.push(format!("{}", e)),
      }
    }
    vec
  }
}

impl From<sys::SoapySDRRange> for Range {
  fn from(s: sys::SoapySDRRange) -> Self {
    Self {
      minimum: s.minimum,
      maximum: s.maximum,
      step: s.step,
    }
  }
}

impl From<(usize, *mut *mut std::os::raw::c_char)> for CStrList {
  fn from((length, inner):(usize, *mut *mut std::os::raw::c_char)) -> CStrList {
    CStrList::new(length, inner)
  }
}

impl From<*mut std::os::raw::c_void> for StreamBuffer {
  fn from(inner: *mut std::os::raw::c_void) -> Self {
    Self {inner}
  }
}

impl From<(usize, *mut sys::SoapySDRRange)> for RangeList {
  fn from((length, inner): (usize, *mut sys::SoapySDRRange)) -> Self {
    Self{length, inner}
  }
}

impl std::fmt::Display for Range {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "minimum: {}, maximum: {}, step: {}", self.minimum, self.maximum, self.step)
  }
}

impl std::fmt::Display for CStrList {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let slice_of_cstr: &[*mut std::os::raw::c_char] = unsafe { std::slice::from_raw_parts(self.inner, self.length) };
    for i in 0 .. slice_of_cstr.len() {
      write!(f, "{}", unsafe{ std::ffi::CStr::from_ptr(slice_of_cstr[i]) }.to_str().unwrap_or("to_str fail"))?;
      if i < slice_of_cstr.len() - 1 {
        write!(f, ",")?;
      }
    }
    Ok(())
  }
}

impl std::fmt::Display for StreamBuffer {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "StreamBuffer: {:?}", self.inner)
  }
}

impl std::fmt::Display for RangeList {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    unsafe {
      let slice = std::slice::from_raw_parts(self.inner, self.length);
      write!(f, "[")?;
      for i in slice {
        write!(f, "{{minimum: {}, maximum: {}, step: {}}},", i.minimum, i.maximum, i.step)?;
      }
      write!(f, "]")?;
    }
    Ok(())
  }
}

impl Drop for CStrList {
  fn drop(&mut self) {
    unsafe {
      sys::SoapySDRStrings_clear(&mut self.inner as *mut *mut *mut std::os::raw::c_char, self.length);
    }
  }
}

impl Drop for RangeList {
  fn drop(&mut self) {
    unsafe {
      use libc;
      libc::free(self.inner as *mut libc::c_void);
    }
  }
}