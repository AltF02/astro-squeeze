#![deny(clippy::all)]
mod compressor;

#[macro_use]
extern crate napi_derive;

use crate::compressor::Compressor;
use log::info;
use napi::Error;

#[napi(object)]
#[derive(Clone)]
pub struct Options {
  pub gzip: bool,
  pub brotli: bool,
  pub file_extensions: Vec<String>,
  pub batch_size: i32,
}

#[napi]
pub async fn run_squeezer(options: Options, dir: String) -> Result<(), Error> {
  tracing_subscriber::fmt::init();

  let compressor = Compressor::new()
    .with_dir(dir)
    .with_extensions(options.file_extensions)
    .with_batch_size(std::thread::available_parallelism().map_or(10, |v| v.get() * 2));

  if options.brotli {
    info!("starting brotli compression");
    compressor.brotli().await.expect("compressor brotli");
  }

  if options.gzip {
    info!("starting gzip compression");
    compressor.gzip().await.expect("compressor gzip");
  }

  Ok(())
}
