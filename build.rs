use std::env;

fn main() {
  println!("cargo:rustc-link-search={}", env::var("SOAPY_SDR_LIB_PATH").unwrap_or("C:\\Program Files (x86)\\SoapySDR\\lib".to_string()));
  println!("cargo:rustc-link-lib=SoapySDR");
}