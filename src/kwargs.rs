use std;
use super::error::*;
use super::sys;

pub struct Kwargs {
  pub(crate) inner: sys::SoapySDRKwargs,
}

pub struct KwargsList<'a> {
  pub(crate) inner: &'a[sys::SoapySDRKwargs],
}

impl Kwargs {
  pub fn empty() -> Self {
    Kwargs {
      inner: sys::SoapySDRKwargs {
        size: 0,
        keys: std::ptr::null_mut(),
        vals: std::ptr::null_mut(),
      }
    }
  }

  pub fn from_str(s: &str) -> Self {
    let inner = unsafe {sys::SoapySDRKwargs_fromString(s.as_ptr() as *const i8)};
    Kwargs{inner}
  }

  pub fn to_str(&self) -> Result<&str> {
    let ptr = chkp(unsafe{sys::SoapySDRKwargs_toString(&self.inner)})?;
    let cstr = unsafe{std::ffi::CStr::from_ptr(ptr)};
    cstr.to_str().map_err(|e| e.into())
  }

  pub fn set(&mut self, key: &str, val: &str) {
    unsafe {
      sys::SoapySDRKwargs_set(
        &mut self.inner, 
        key.as_ptr() as *const i8, 
        val.as_ptr() as *const i8,
      )
    }
  }

  pub fn get<'a>(&'a mut self, key: &str) -> Result<&str> {
    let cstr = 
      unsafe {
        sys::SoapySDRKwargs_get(
          &mut self.inner,
          key.as_ptr() as *const i8,
        )
      };
    unsafe {std::ffi::CStr::from_ptr(cstr).to_str().map_err(|e| e.into())}
  }

}

impl From<sys::SoapySDRKwargs> for Kwargs {
  fn from(inner: sys::SoapySDRKwargs) -> Kwargs {
    Kwargs{inner}
  }
}

impl <'a>From<(usize, *mut sys::SoapySDRKwargs)> for KwargsList<'a> {
  fn from((length, inner): (usize, *mut sys::SoapySDRKwargs)) -> KwargsList<'a> {
    use libc;
    let rt = KwargsList {
      inner: unsafe { std::slice::from_raw_parts(inner, length) }
    };
    unsafe { libc::free(inner as *mut libc::c_void) };
    rt
  }
}

impl Drop for Kwargs {
  fn drop(&mut self) {
    unsafe {
      sys::SoapySDRKwargs_clear(&mut self.inner);
    }
  }
}

impl <'a>Drop for KwargsList<'a> {
  fn drop(&mut self) {
    unsafe {
      sys::SoapySDRKwargsList_clear(
        self.inner.as_ptr() as *mut sys::SoapySDRKwargs, 
        self.inner.len(),
      );
    }
  }
}

impl std::fmt::Display for Kwargs {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use std::error::Error;
    match self.to_str() {
      Ok(s) => write!(f, "{}", s),
      Err(e) => write!(f, "{}", e.description())
    }
  } 
}

impl <'a>std::fmt::Display for KwargsList<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for i in self.inner {
      let ptr = unsafe{sys::SoapySDRKwargs_toString(i)};
      let cstr = unsafe{std::ffi::CStr::from_ptr(ptr)};
      write!(f, "{}, ", cstr.to_str().unwrap_or("to_str fail"))?;
    }
    Ok(())
  }
}