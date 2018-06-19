use std;
use super::sys;
use super::*;

pub struct Device {
  pub(crate)inner: *mut sys::SoapySDRDevice,
}

impl Device {
  pub fn last_status() -> i32 {
    unsafe{
      sys::SoapySDRDevice_lastStatus()
    }
  }

  pub fn last_error() -> Result<&'static str> {
    let cstr = unsafe {
      std::ffi::CStr::from_ptr(sys::SoapySDRDevice_lastError())
    };
    cstr.to_str().map_err(|e| e.into())
  }

  pub fn enumerate(args: &Kwargs) -> Result<KwargsList> {
    let mut length = 0;
    let list = unsafe {
      chkp(sys::SoapySDRDevice_enumerate(&args.inner, &mut length))
    }?;
    let slice = unsafe { std::slice::from_raw_parts(list, length) };
    Ok(KwargsList{inner: slice})
  }

  pub fn enumerate_str_args(args: &str) -> Result<KwargsList> {
    let mut length = 0;
    let list = unsafe {
      chkp(sys::SoapySDRDevice_enumerateStrArgs(args.as_ptr() as *const i8, &mut length))
    }?;
    let slice = unsafe { std::slice::from_raw_parts(list, length) };
    Ok(KwargsList{inner: slice})
  }

  pub fn make(args: &Kwargs) -> Result<Self> {
    let dev = unsafe {
      chkp(sys::SoapySDRDevice_make(&args.inner))
    }?;
    Ok(dev.into())
  }

  pub fn make_str_arg(args: &str) -> Result<Self> {
    let dev = unsafe {
      chkp(sys::SoapySDRDevice_makeStrArgs(
        args.as_ptr() as *const i8
      ))
    }?;
    Ok(dev.into())
  }

  pub fn driver_key(&self) -> Result<&str> {
    unsafe_into_str!(sys::SoapySDRDevice_getDriverKey(self.inner))
  }

  pub fn hardware_key(&self) -> Result<&str> {
    unsafe_into_str!(sys::SoapySDRDevice_getHardwareKey(self.inner))
  }

  pub fn hardware_info(&self) -> Result<Kwargs> {
    unsafe_into!(sys::SoapySDRDevice_getHardwareInfo(self.inner))
  }

  pub fn set_frontend_mapping(&mut self, direction: i32, mapping: &str) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setFrontendMapping(
      self.inner as *mut sys::SoapySDRDevice,
      direction,
      mapping.as_ptr() as *const i8,
    ))
  }

  pub fn frontend_mapping(&self, direction: i32) -> Result<&str> {
    unsafe_into_str!(sys::SoapySDRDevice_getFrontendMapping(
      self.inner,
      direction,
    ))
  }

  pub fn num_channels(&self, direction: i32) -> Result<usize> {
    unsafe_into!(sys::SoapySDRDevice_getNumChannels(
      self.inner,
      direction,
    ))
  }

  pub fn channel_info(&self, direction: i32, channel: usize) -> Result<Kwargs> {
    unsafe_into!(sys::SoapySDRDevice_getChannelInfo(
      self.inner,
      direction,
      channel,
    ))
  }

  pub fn full_duplex(&self, direction: i32, channel: usize) -> Result<bool> {
    unsafe_into!(sys::SoapySDRDevice_getFullDuplex(
      self.inner,
      direction,
      channel,
    ))
  }

  pub fn stream_formats(&self, direction: i32, channel: usize) -> CStrList {
    let mut length = 0;
    let s = unsafe { sys::SoapySDRDevice_getStreamFormats(
      self.inner,
      direction,
      channel,
      &mut length,
    ) };
    (length, s).into()
  }

  pub fn native_stream_format(&self, direction: i32, channel: usize) -> Result<(f64, &str)> {
    let mut full_scale = 0f64;
    let rt: Result<&str> = unsafe_into_str!(sys::SoapySDRDevice_getNativeStreamFormat(
      self.inner,
      direction,
      channel,
      &mut full_scale,
    ));
    Ok((full_scale, rt?))
  }

  pub fn stream_args_info(&self, direction: i32, channel: usize) -> ArgInfoList {
    let mut length = 0;
    let s = unsafe { sys::SoapySDRDevice_getStreamArgsInfo(
      self.inner,
      direction,
      channel,
      &mut length,
    ) };
    (length, s).into()
  }

  pub fn setup_stream(&self, direction: i32, format: &str, channels: &[usize], args: &Kwargs) -> Result<Stream> {
    let mut stream: *mut sys::SoapySDRStream = std::ptr::null_mut();
    unsafe_check_return!(sys::SoapySDRDevice_setupStream(
      self.inner as *mut sys::SoapySDRDevice,
      &mut stream,
      direction,
      format.as_ptr() as *const i8,
      channels.as_ptr(),
      channels.len(),
      &args.inner,
    ))?;
    Ok((self.inner as *mut sys::SoapySDRDevice, stream).into())
  }

  pub fn list_antennas(&self, direction: i32, channel: usize) -> CStrList {
    let mut length = 0;
    let s = unsafe { sys::SoapySDRDevice_listAntennas(
      self.inner,
      direction,
      channel,
      &mut length,
    ) };
    (length, s).into()
  }

  pub fn set_antenna(&self, direction: i32, channel: usize, name: &str) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setAntenna(
      self.inner,
      direction,
      channel,
      name.as_ptr() as *const i8,
    ))
  }

  pub fn antenna(&self, direction: i32, channel: usize) -> Result<&str> {
    unsafe {
      let cs = sys::SoapySDRDevice_getAntenna(self.inner, direction, channel);
      std::ffi::CStr::from_ptr(cs).to_str().map_err(|e| e.into())
    }
  }
  pub fn has_DC_offset_mode(&self, direction: i32, channel: usize) -> bool {
    unsafe {
      sys::SoapySDRDevice_hasDCOffsetMode(self.inner, direction, channel)
    }
  }

  pub fn set_DC_offset_mode(&self, direction: i32, channel: usize, automatic: bool) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setDCOffsetMode(
      self.inner, direction, channel, automatic,
    ))
  }

  pub fn dc_offset_mode(&self, direction: i32, channel: usize) -> bool {
    unsafe {
      sys::SoapySDRDevice_getDCOffsetMode(self.inner, direction, channel)
    }
  }

  pub fn has_dc_offset(&self, direction: i32, channel: usize) -> bool {
    unsafe {
      sys::SoapySDRDevice_hasDCOffset(self.inner, direction, channel)
    }
  }

  pub fn set_DC_offset(&self, direction: i32, channel: usize, offsetI: f64, offsetQ: f64) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setDCOffset(
      self.inner, direction, channel, offsetI, offsetQ
    ))
  }

  pub fn dc_offset(&self, direction: i32, channel: usize) -> Result<(f64, f64)> {
    let mut i = 0f64;
    let mut q = 0f64;
    unsafe_check_return!(sys::SoapySDRDevice_getDCOffset(
      self.inner, direction, channel, &mut i, &mut q,
    ))?;
    Ok((i, q))
  }

  pub fn has_iq_balance(&self, direction: i32, channel: usize) -> bool {
    unsafe {
      sys::SoapySDRDevice_hasIQBalance(self.inner, direction, channel)
    }
  }

  pub fn set_iq_balance(&self, direction: i32, channel: usize, balanceI: f64, balanceQ: f64) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setIQBalance(
      self.inner, direction, channel, balanceI, balanceQ
    ))
  }

  pub fn iq_balance(&self, direction: i32, channel: usize) -> Result<(f64, f64)> {
    let mut i = 0f64;
    let mut q = 0f64;
    unsafe_check_return!(sys::SoapySDRDevice_getIQBalance(
      self.inner, direction, channel, &mut i, &mut q,
    ))?;
    Ok((i, q))
  }

  pub fn has_freq_correction(&self, direction: i32, channel: usize) -> bool {
    unsafe {
      sys::SoapySDRDevice_hasFrequencyCorrection(
        self.inner, direction, channel,
      )
    }
  }

  pub fn set_freq_correction(&self, direction: i32, channel: usize, value: f64) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setFrequencyCorrection(
      self.inner, direction, channel, value,
    ))
  }

  pub fn freq_correction(&self, direction: i32, channel: usize) -> f64 {
    unsafe {sys::SoapySDRDevice_getFrequencyCorrection(
      self.inner, direction, channel
    )}
  }

  pub fn list_gains(&self, direction: i32, channel: usize) -> CStrList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_listGains(
      self.inner, direction, channel, &mut len
    ) };
    (len, s).into()
  }

  pub fn has_gain_mode(&self, direction: i32, channel: usize) -> bool {
    unsafe {
      sys::SoapySDRDevice_hasGainMode(self.inner, direction, channel)
    }
  }

  pub fn set_gain_mode(&self, direction: i32, channel: usize, automatic: bool) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setGainMode(
      self.inner, direction, channel, automatic
    ))
  }

  pub fn gain_mode(&self, direction: i32, channel: usize) -> bool {
    unsafe {
      sys::SoapySDRDevice_getGainMode(self.inner, direction, channel)
    }
  }

  pub fn set_gain(&self, direction: i32, channel: usize, value: f64) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setGain(
      self.inner, direction, channel, value
    ))
  }

  pub fn set_gain_element(&self, direction: i32, channel: usize, name: &str, value: f64) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setGainElement(
      self.inner, direction, channel, name.as_ptr() as *const i8, value,
    ))
  }

  pub fn gain(&self, direction: i32, channel: usize) -> f64 {
    unsafe {
      sys::SoapySDRDevice_getGain(self.inner, direction, channel)
    }
  }

  pub fn gain_element(&self, direction: i32, channel: usize, name: &str) -> f64 {
    unsafe {
      sys::SoapySDRDevice_getGainElement(self.inner, direction, channel, name.as_ptr() as *const i8)
    }
  }

  pub fn gain_range(&self, direction: i32, channel: usize) -> Range {
    unsafe {
      sys::SoapySDRDevice_getGainRange(self.inner, direction, channel).into()
    }
  }

  pub fn gain_element_range(&self, direction: i32, channel: usize, name: &str) -> Range {
    unsafe {
      sys::SoapySDRDevice_getGainElementRange(
        self.inner, direction, channel, name.as_ptr() as *const i8
      ).into()
    }
  }

  pub fn set_freq(&self, direction: i32, channel: usize, freq: f64, args: Option<&Kwargs>) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setFrequency(
      self.inner, direction, channel, freq, args.map(|a| &a.inner as *const sys::SoapySDRKwargs).unwrap_or(std::ptr::null())
    ))
  }

  pub fn set_freq_component(&self, direction: i32, channel: usize, name: &str, freq: f64, args: &Kwargs) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setFrequencyComponent(
      self.inner, direction, channel, name.as_ptr() as *const i8, freq, &args.inner
    ))
  }

  pub fn freq(&self, direction: i32, channel: usize) -> f64 {
    unsafe {
      sys::SoapySDRDevice_getFrequency(self.inner, direction, channel)
    }
  }

  pub fn freq_component(&self, direction: i32, channel: usize, name: &str) -> f64 {
    unsafe {
      sys::SoapySDRDevice_getFrequencyComponent(
        self.inner, direction, channel, name.as_ptr() as *const i8
      )
    }
  }

  pub fn list_freqs(&self, direction: i32, channel: usize) -> CStrList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_listFrequencies(
      self.inner, direction, channel, &mut len
    ) };
    (len, s).into()
  }

  pub fn freq_range(&self, direction: i32, channel: usize) -> RangeList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_getFrequencyRange(
      self.inner, direction, channel, &mut len
    ) };
    (len, s).into()
  }

  pub fn freq_range_component(&self, direction: i32, channel: usize, name: &str) -> RangeList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_getFrequencyRangeComponent(
      self.inner, direction, channel, name.as_ptr() as *const i8, &mut len
    ) };
    (len, s).into()
  }

  pub fn freq_args_info(&self, direction: i32, channel: usize) -> ArgInfoList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_getFrequencyArgsInfo(
      self.inner, direction, channel, &mut len
    ) };
    (len, s).into()
  }

  pub fn set_sample_rate(&self, direction: i32, channel: usize, rate: f64) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setSampleRate(
      self.inner, direction, channel, rate
    ))
  }

  pub fn sample_rate(&self, direction: i32, channel: usize) -> f64 {
    unsafe{
      sys::SoapySDRDevice_getSampleRate(self.inner, direction, channel)
    }
  }

  pub fn list_sample_rates(&self, direction: i32, channel: usize) -> &[f64] {
    unsafe {
      use libc;
      let mut len = 0;
      let rs = sys::SoapySDRDevice_listSampleRates(self.inner, direction, channel, &mut len);
      let rt = std::slice::from_raw_parts(rs, len);
      libc::free(rs as *mut libc::c_void);
      rt
    }
  }

  pub fn sample_rate_range(&self, direction: i32, channel: usize) -> RangeList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_getSampleRateRange(
      self.inner, direction, channel, &mut len
    )};
    (len, s).into()
  }

  pub fn set_bandwidth(&self, direction: i32, channel: usize, bw: f64) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setBandwidth(
      self.inner, direction, channel, bw,
    ))
  }

  pub fn bandwidth(&self, direction: i32, channel: usize) -> f64 {
    unsafe {
      sys::SoapySDRDevice_getBandwidth(self.inner, direction, channel)
    }
  }

  pub fn list_bandwidths(&self, direction: i32, channel: usize) -> &[f64] {
    unsafe {
      use libc;
      let mut len = 0;
      let rs = sys::SoapySDRDevice_listBandwidths(self.inner, direction, channel, &mut len);
      let rt = std::slice::from_raw_parts(rs, len);
      libc::free(rs as *mut libc::c_void);
      rt
    }
  }

  pub fn bandwidth_range(&self, direction: i32, channel: usize) -> RangeList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_getBandwidthRange(
      self.inner, direction, channel, &mut len
    )};
    (len, s).into()
  }

  pub fn set_master_clock_rate(&self, rate: f64) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setMasterClockRate(
      self.inner, rate
    ))
  }

  pub fn master_clock_rate(&self) -> f64 {
    unsafe {
      sys::SoapySDRDevice_getMasterClockRate(self.inner)
    }
  }

  pub fn masterClockRates(&self) -> RangeList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_getMasterClockRates(
      self.inner, &mut len
    )};
    (len, s).into()
  }

  pub fn list_clock_sources(&self) -> CStrList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_listClockSources(
      self.inner, &mut len
    )};
    (len, s).into()
  }

  pub fn set_clock_source(&self, source: &str) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setClockSource(
      self.inner, source.as_ptr() as *const i8
    ))
  }

  pub fn clock_source(&self) -> Result<&str> {
    unsafe {
      let cs = sys::SoapySDRDevice_getClockSource(self.inner);
      std::ffi::CStr::from_ptr(cs).to_str().map_err(|e| e.into())
    }
  }

  pub fn list_time_sources(&self) -> CStrList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_listTimeSources(
      self.inner, 
      &mut len,
    )};
    (len, s).into()
  }

  pub fn set_time_source(&self, source: &str) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setTimeSource(
      self.inner, source.as_ptr() as *const i8
    ))
  }

  pub fn time_source(&self) -> Result<&str> {
    unsafe_into_str!(sys::SoapySDRDevice_getTimeSource(
      self.inner
    ))
  }

  pub fn has_hardware_time(&self, what: &str) -> bool {
    unsafe {
      sys::SoapySDRDevice_hasHardwareTime(self.inner, what.as_ptr() as *const i8)
    }
  }

  pub fn hardware_time(&self, what: &str) -> i64 {
    unsafe {
      sys::SoapySDRDevice_getHardwareTime(self.inner, what.as_ptr() as *const i8)
    }
  }

  pub fn set_hardware_time(&self, time_ns: i64, what: &str) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setHardwareTime(
      self.inner, time_ns, what.as_ptr() as *const i8
    ))
  }

  pub fn set_command_time(&self, time_ns: i64, what: &str) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_setCommandTime(
      self.inner, time_ns, what.as_ptr() as *const i8,
    ))
  }

  pub fn list_sensors(&self) -> CStrList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_listSensors(
      self.inner, &mut len,
    )};
    (len, s).into()
  }

  pub fn sensor_info(&self, key: &str) -> Result<ArgInfo> {
    unsafe_into!(sys::SoapySDRDevice_getSensorInfo(
      self.inner, key.as_ptr() as *const i8
    ))
  }

  pub fn read_sensor(&self, key: &str) -> Result<&str> {
    unsafe_into_str!(sys::SoapySDRDevice_readSensor(
      self.inner, key.as_ptr() as *const i8
    ))
  }

  pub fn list_channel_sensors(&self, direction: i32, channel: usize) -> CStrList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_listChannelSensors(
      self.inner, direction, channel, &mut len
    )};
    (len, s).into()
  }

  pub fn channel_sensors_info(&self, direction: i32, channel: usize, key: &str) -> Result<ArgInfo> {
    unsafe_into!(sys::SoapySDRDevice_getChannelSensorInfo(
      self.inner, direction, channel, key.as_ptr() as *const i8
    ))
  }

  pub fn read_channel_sensor(&self, direction: i32, channel: usize, key: &str) -> Result<&str> {
    unsafe_into_str!(sys::SoapySDRDevice_readChannelSensor(
      self.inner, direction, channel, key.as_ptr() as *const i8
    ))
  }

  pub fn list_register_interfaces(&self) -> CStrList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_listRegisterInterfaces(
      self.inner, &mut len,
    )};
    (len, s).into()
  }

  pub fn write_register(&self, name: &str, addr: u32, value: u32) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_writeRegister(
      self.inner, name.as_ptr() as *const i8, addr, value
    ))
  }

  pub fn read_register(&self, name: &str, addr: u32) -> u32 {
    unsafe {
      sys::SoapySDRDevice_readRegister(self.inner, name.as_ptr() as *const i8, addr)
    }
  }

  pub fn write_registers(&self, name: &str, addr: u32, value: &[u32]) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_writeRegisters(
      self.inner, name.as_ptr() as *const i8, addr, value.as_ptr(), value.len()
    ))
  }

  pub fn read_registers(&self, name: &str, addr: u32) -> &[u32] {
    unsafe {
      use libc;
      let mut len = 0;
      let c = sys::SoapySDRDevice_readRegisters(self.inner, name.as_ptr() as *const i8, addr, &mut len);
      let rt = std::slice::from_raw_parts(c, len);
      libc::free(c as *mut libc::c_void);
      rt
    }
  }

  pub fn setting_info(&self) -> ArgInfoList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_getSettingInfo(
      self.inner, &mut len
    )};
    (len, s).into()
  }

  pub fn write_setting(&self, key: &str, value: &str) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_writeSetting(
      self.inner, key.as_ptr() as *const i8, value.as_ptr() as *const i8,
    ))
  }

  pub fn read_setting(&self, key: &str) -> Result<&str> {
    unsafe_into_str!(sys::SoapySDRDevice_readSetting(
      self.inner, key.as_ptr() as *const i8
    ))
  }

  pub fn channel_setting_info(&self, direction: i32, channel: usize) -> ArgInfoList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_getChannelSettingInfo(
      self.inner, direction, channel, &mut len
    )};
    (len, s).into()
  }

  pub fn write_channel_setting(&self, direction: i32, channel: usize, key: &str, value: &str) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_writeChannelSetting(
      self.inner, direction, channel, key.as_ptr() as *const i8, value.as_ptr() as *const i8
    ))
  }

  pub fn read_channel_setting(&self, direction: i32, channel: usize, key: &str) -> Result<&str> {
    unsafe_into_str!(sys::SoapySDRDevice_readChannelSetting(
      self.inner, direction, channel, key.as_ptr() as *const i8
    ))
  }

  pub fn list_gpio_banks(&self) -> CStrList {
    let mut len = 0;
    let s = unsafe { sys::SoapySDRDevice_listGPIOBanks(
      self.inner, &mut len
    )};
    (len, s).into()
  }

  pub fn write_gpio(&self, bank: &str, value: u32) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_writeGPIO(
      self.inner, bank.as_ptr() as *const i8, value
    ))
  }

  pub fn write_gpio_masked(self, bank: &str, value: u32, mask: u32) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_writeGPIOMasked(
      self.inner, bank.as_ptr() as *const i8, value, mask
    ))
  }

  pub fn read_gpio(&self, bank: &str) -> u32 {
    unsafe {
      sys::SoapySDRDevice_readGPIO(self.inner, bank.as_ptr() as *const i8)
    }
  }

  pub fn write_gpio_dir(&self, bank: &str, dir: u32) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_writeGPIODir(
      self.inner, bank.as_ptr() as *const i8, dir
    ))
  }

  pub fn write_gpio_dir_masked(&self, bank: &str, dir: u32, mask: u32) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_writeGPIODirMasked(
      self.inner, bank.as_ptr() as *const i8, dir, mask
    ))
  }

  pub fn read_gpio_dir(&self, bank: &str) -> u32 {
    unsafe {
      sys::SoapySDRDevice_readGPIODir(self.inner, bank.as_ptr() as *const i8)
    }
  }

  pub fn write_i2c(&self, addr: i32, data: &[u8]) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_writeI2C(
      self.inner, addr, data.as_ptr() as *const i8, data.len()
    ))
  }

  pub fn read_i2c(&self, addr: i32, mut len: usize) -> &[u8] {
    unsafe {
      use libc;
      let c = sys::SoapySDRDevice_readI2C(self.inner, addr, &mut len);
      let rt = std::slice::from_raw_parts(c as *const u8, len);
      libc::free(c as *mut libc::c_void);
      rt
    }
  }

  pub fn transact_spi(&self, addr: i32, data: u32, num_bits: usize) -> u32 {
    unsafe {
      sys::SoapySDRDevice_transactSPI(self.inner, addr, data, num_bits)
    }
  }

  pub fn list_uarts(&self) -> CStrList {
    let mut len = 0;
    let s = unsafe{ sys::SoapySDRDevice_listUARTs(
      self.inner, &mut len
    )};
    (len, s).into()
  }

  pub fn write_uart(&self, which: &str, data: &str) -> Result<i32> {
    unsafe_check_return!(sys::SoapySDRDevice_writeUART(
      self.inner, which.as_ptr() as *const i8, data.as_ptr() as *const i8
    ))
  }

  pub fn read_uart(&self, which: &str, timeout_us: i32) -> Result<&str> {
    unsafe_into_str!(sys::SoapySDRDevice_readUART(
      self.inner, which.as_ptr() as *const i8, timeout_us
    ))
  }
}

impl From<*mut sys::SoapySDRDevice> for Device {
  fn from(inner: *mut sys::SoapySDRDevice) -> Self {
    Device{inner}
  }
}

impl From<*const sys::SoapySDRDevice> for Device {
  fn from(inner: *const sys::SoapySDRDevice) -> Self {
    Device{inner: inner as *mut sys::SoapySDRDevice}
  }
}

impl Drop for Device {
  fn drop(&mut self) {
    if let Ok(p) = chkp(self.inner) {
      unsafe {
        sys::SoapySDRDevice_unmake(p as *mut sys::SoapySDRDevice);
      }
    }
  }
}