use std;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
  /// Io error
  Io(std::io::Error),
  /// Utf8Error
  Utf8Error(std::str::Utf8Error),
  /// Pointer is null.
  NullPointer,
  /// Returnd
  ReturnCode(i32),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Io(ref io) => io.fmt(f),
      Error::Utf8Error(ref e) => e.fmt(f),
      Error::NullPointer => write!(f, "Null Pointer"),
      Error::ReturnCode(code) => {     
        use super::sys;   
        match code {
          sys::SOAPY_SDR_TIMEOUT => write!(f, "Function Return 'TIMEOUT' Code"),
          sys::SOAPY_SDR_STREAM_ERROR => write!(f, "Function Return 'STREAM_ERROR' Code"),
          sys::SOAPY_SDR_CORRUPTION => write!(f, "Function Return 'CORRUPTION' Code"),
          sys::SOAPY_SDR_OVERFLOW => write!(f, "Function Return 'OVERFLOW' Code"),
          sys::SOAPY_SDR_NOT_SUPPORTED => write!(f, "Function Return 'NOT_SUPPORTED' Code"),
          sys::SOAPY_SDR_TIME_ERROR => write!(f, "Function Return 'TIME_ERROR' Code"),
          sys::SOAPY_SDR_UNDERFLOW => write!(f, "Function Return 'UNDERFLOW' Code"),
          _ => write!(f, "Function Return '{}' Code", code),
        }
      }
    }
  }
}

impl std::fmt::Debug for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Error::Io(ref io) => io.fmt(f),
      Error::Utf8Error(ref e) => e.fmt(f),
      Error::NullPointer => write!(f, "Null Pointer"),
      Error::ReturnCode(code) => {     
        use super::sys;   
        match code {
          sys::SOAPY_SDR_TIMEOUT => write!(f, "Function Return 'TIMEOUT' Code"),
          sys::SOAPY_SDR_STREAM_ERROR => write!(f, "Function Return 'STREAM_ERROR' Code"),
          sys::SOAPY_SDR_CORRUPTION => write!(f, "Function Return 'CORRUPTION' Code"),
          sys::SOAPY_SDR_OVERFLOW => write!(f, "Function Return 'OVERFLOW' Code"),
          sys::SOAPY_SDR_NOT_SUPPORTED => write!(f, "Function Return 'NOT_SUPPORTED' Code"),
          sys::SOAPY_SDR_TIME_ERROR => write!(f, "Function Return 'TIME_ERROR' Code"),
          sys::SOAPY_SDR_UNDERFLOW => write!(f, "Function Return 'UNDERFLOW' Code"),
          _ => write!(f, "Function Return '{}' Code", code),
        }
      }
    }
  }
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    match *self {
      Error::Io(ref io) => io.description(),
      Error::Utf8Error(ref e) => e.description(),
      Error::NullPointer => "invalid pointer(null pointer)",
      Error::ReturnCode(_) => "function return none zero code",
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Error {
    Error::Io(err)
  }
}

impl From<std::str::Utf8Error> for Error {
  fn from(err: std::str::Utf8Error) -> Error {
    Error::Utf8Error(err)
  }
}

impl From<i32> for Error {
  fn from(err: i32) -> Error {
    Error::ReturnCode(err)
  }
}

/// 检查ffi返回的指针是否有效
#[inline]
pub fn chkp<T>(p: *const T) -> Result<*const T> {;
  if p.is_null() {
    return Err(Error::NullPointer)
  }
  Ok(p)
}
#[inline]
pub fn chkp_mut<T>(p: *mut T) -> Result<*mut T> {
  if p.is_null() {
    return Err(Error::NullPointer)
  }
  Ok(p)
}