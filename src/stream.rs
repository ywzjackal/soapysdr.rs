use std;
use super::sys;
use super::error::*;
use super::types::StreamBuffer;

pub struct Stream {
  pub(crate) device: *mut sys::SoapySDRDevice,
  pub(crate) inner: *mut sys::SoapySDRStream,
}

impl Stream {
  pub fn mtu(&self) -> usize {
    unsafe{
      sys::SoapySDRDevice_getStreamMTU(self.device, self.inner)
    }
  }

  pub fn activate(&self, flag: i32, time_ns: i64, num_elems: usize) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_activateStream(
      self.device,
      self.inner,
      flag,
      time_ns,
      num_elems,
    ))
  }

  pub fn deactivate(&self, flag: i32, time_ns: i64) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_deactivateStream(
      self.device,
      self.inner,
      flag,
      time_ns,
    ))
  }

  pub fn read<T>(&self, buffs: &[&mut [T]], timeout_us: i32) -> Result<(usize, i32, i64)> {
    let mut flags = 0;
    let mut time_ns = 0;
    let elms = buffs[0].len();

    let size = unsafe_check_return!(sys::SoapySDRDevice_readStream(
      self.device,
      self.inner,
      buffs.as_ptr() as *const *mut std::os::raw::c_void,
      elms,
      &mut flags,
      &mut time_ns,
      timeout_us,
    ))?;

    Ok((size as usize, flags, time_ns))
  }

  pub fn write<T>(&self, buffs: &[&[T]], timeout_us: i32) -> Result<(usize, i32, i64)> {
    let mut flags = 0;
    let mut time_ns = 0;
    let elms = buffs[0].len();

    let size = unsafe_check_return!(sys::SoapySDRDevice_readStream(
      self.device,
      self.inner,
      buffs.as_ptr() as *const *mut std::os::raw::c_void,
      elms,
      &mut flags,
      &mut time_ns,
      timeout_us,
    ))?;

    Ok((size as usize, flags, time_ns))
  }

  pub fn status(&self, timeout_us: i32) -> Result<(usize, i32, i64)> {
    let mut chan_mask = 0;
    let mut flags = 0;
    let mut time_ns = 0;
    unsafe_check_return!(sys::SoapySDRDevice_readStreamStatus(
      self.device,
      self.inner,
      &mut chan_mask,
      &mut flags,
      &mut time_ns,
      timeout_us,
    ))?;
    Ok((chan_mask, flags, time_ns))
  }

  pub fn num_direct_access_buffers(&self) -> usize {
    unsafe { sys::SoapySDRDevice_getNumDirectAccessBuffers(
      self.device,
      self.inner,
    ) }
  }

  pub fn direct_access_buffer_addrs(&self, handle: usize) -> Result<StreamBuffer> {
    let mut buff: *mut std::os::raw::c_void = std::ptr::null_mut();
    unsafe_check_return!(sys::SoapySDRDevice_getDirectAccessBufferAddrs(
      self.device,
      self.inner,
      handle,
      &mut buff,
    ))?;
    Ok(buff.into())
  }
}

impl From<(*mut sys::SoapySDRDevice, *mut sys::SoapySDRStream)> for Stream {
  fn from((device, inner):(*mut sys::SoapySDRDevice, *mut sys::SoapySDRStream)) -> Stream {
    Stream{device, inner}
  }
}

impl Drop for Stream {
  fn drop(&mut self) {
    unsafe {
      sys::SoapySDRDevice_closeStream(self.device, self.inner);
    }
  }
}