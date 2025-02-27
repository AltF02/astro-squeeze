use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{io, sync::Arc};

use flate2::write::GzEncoder;
use flate2::Compression;
use futures::stream;
use futures::stream::StreamExt;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::BufReader;
use tokio::{fs, io::AsyncReadExt as TokioAsyncReadExt, task};
use tracing::{info, instrument};
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum CompressionError {
  #[error("IO error: {0}")]
  Io(#[from] io::Error),

  #[error("No files found to compress")]
  NoFiles,
}

#[derive(Debug, Clone)]
pub enum CompressionType {
  Gzip,
  Brotli,
}

impl CompressionType {
  fn extension(&self) -> &'static str {
    match self {
      Self::Gzip => "gz",
      Self::Brotli => "br",
    }
  }

  fn name(&self) -> &'static str {
    match self {
      Self::Gzip => "gzip",
      Self::Brotli => "brotli",
    }
  }

  fn compress_data(&self, data: Vec<u8>) -> io::Result<Vec<u8>> {
    match self {
      Self::Gzip => {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        io::copy(&mut data.as_slice(), &mut encoder)?;
        encoder.finish()
      }
      Self::Brotli => {
        let mut output = Vec::new();
        let params = brotli::enc::BrotliEncoderParams::default();
        brotli::BrotliCompress(&mut data.as_slice(), &mut output, &params)?;
        Ok(output)
      }
    }
  }
}

#[derive(Clone)]
pub struct CompressorConfig {
  pub dir: PathBuf,
  pub extensions: Vec<String>,
  pub batch_size: usize,
}

impl Default for CompressorConfig {
  fn default() -> Self {
    Self {
      dir: PathBuf::from("dist"),
      extensions: vec![".html".into(), ".css".into(), ".js".into()],
      batch_size: 10,
    }
  }
}

#[derive(Clone, Default)]
pub struct Compressor {
  config: CompressorConfig,
}

impl Compressor {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_dir(mut self, dir: impl AsRef<Path>) -> Self {
    self.config.dir = dir.as_ref().to_path_buf();
    self
  }

  pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
    self.config.extensions = extensions;
    self
  }

  pub fn with_batch_size(mut self, batch_size: usize) -> Self {
    self.config.batch_size = batch_size;
    self
  }

  async fn find_files(&self) -> Vec<PathBuf> {
    let walker = WalkDir::new(&self.config.dir).into_iter();
    stream::iter(walker)
      .filter_map(|entry| async move {
        entry.ok().and_then(|e| {
          let ext = e.path().extension()?.to_str()?;
          (e.file_type().is_file()
            && self
              .config
              .extensions
              .iter()
              .any(|cfg_ext| cfg_ext == &format!(".{}", ext)))
          .then_some(e.path().to_path_buf())
        })
      })
      .collect()
      .await
  }

  async fn compress_file(
    path: PathBuf,
    compression_type: Arc<CompressionType>,
  ) -> Result<(), CompressionError> {
    let file = File::open(&path).await?;
    let mut reader = BufReader::new(file);
    let mut content = Vec::new();
    reader.read_to_end(&mut content).await?;

    let compression_type_clone = Arc::clone(&compression_type);

    let compressed = task::spawn_blocking(move || compression_type_clone.compress_data(content))
      .await
      .unwrap()?;

    let output_path = format!("{}.{}", path.display(), compression_type.extension());
    fs::write(&output_path, compressed).await?;

    Ok(())
  }

  #[instrument(skip(self), err)]
  pub async fn compress(&self, compression_type: CompressionType) -> Result<(), CompressionError> {
    let files = self.find_files().await;
    if files.is_empty() {
      return Err(CompressionError::NoFiles);
    }

    let start = Instant::now();
    let file_count = files.len();
    let compression_type = Arc::new(compression_type);

    stream::iter(files)
      .map(|path| {
        let compression_type = Arc::clone(&compression_type);
        async move { Compressor::compress_file(path, compression_type).await }
      })
      .buffer_unordered(self.config.batch_size)
      .for_each(|result| async {
        if let Err(e) = result {
          tracing::error!("Error compressing file: {:?}", e);
        }
      })
      .await;

    let duration = start.elapsed();
    info!(
      "{:<8} compressed {} files in {}ms",
      compression_type.name(),
      file_count,
      duration.as_millis()
    );

    Ok(())
  }

  pub async fn gzip(&self) -> Result<(), CompressionError> {
    self.compress(CompressionType::Gzip).await
  }

  pub async fn brotli(&self) -> Result<(), CompressionError> {
    self.compress(CompressionType::Brotli).await
  }
}
