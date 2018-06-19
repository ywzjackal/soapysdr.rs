extern crate soapysdr_rs;
extern crate num;

use num::Complex;
use soapysdr_rs::*;

pub fn main() {
  println!("== soapysdr_rs test ==");
  let args = Kwargs::empty();

  let arglist = Device::enumerate(&args).expect("fail to enumerate");
  println!("{}", arglist);

  let mut args = Kwargs::empty();
  args.set("driver", "lime");
  let dev = Device::make(&args).expect("fail to make device");

  let list = dev.list_antennas(RX, 0);
  println!("antennas: {}", list);

  let list = dev.list_gains(RX, 0);
  println!("gains: {}", list);

  let range = dev.freq_range(RX, 0);
  println!("freq range: {}", range);


  dev.set_sample_rate(RX, 0, 1e6).expect("fail to set sample rate");
  
  dev.set_freq(RX, 0, 103.9e6, None).expect("fail to set freq");

  let stream = dev.setup_stream(RX, "CF32", &[0], &args)
    .expect("fail to setup stream");
  
  stream.activate(0, 0, 0).expect("fail to activate stream");

  for _ in 0..10 {
    let buffs = [&mut [Complex::new(0f32, 0f32); 1024][..]];
    match stream.read(&buffs[..], 10000) {
      Ok((elms, flags, time_ns)) => {
        println!("elms:{}, flags:{}, time_ns:{}", elms, flags, time_ns);
      }
      Err(e) => {
        println!("{:?},{:?}", e, Device::last_error());
      }
    }
  }
}