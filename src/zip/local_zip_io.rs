// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 rhythmcache
// https://github.com/rhythmcache/payload-dumper-rust

use crate::error::Result;
use crate::zip::zip_io::ZipIO;
use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;

pub struct LocalZipIO {
    file: Arc<std::fs::File>,
    size: u64,
}

impl LocalZipIO {
    pub async fn new(path: PathBuf) -> Result<Self> {
        let file = std::fs::File::open(&path)?;
        let size = file.metadata()?.len();

        Ok(Self {
            file: Arc::new(file),
            size,
        })
    }
}

/// positional read that fills the whole buffer
fn read_exact_at(file: &std::fs::File, buf: &mut [u8], offset: u64) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::FileExt;
        file.read_exact_at(buf, offset)
    }
    #[cfg(windows)]
    {
        // seek_read may return short reads, so loop until the buffer is full
        use std::os::windows::fs::FileExt;
        let mut filled = 0;
        while filled < buf.len() {
            let n = file.seek_read(&mut buf[filled..], offset + filled as u64)?;
            if n == 0 {
                return Err(std::io::ErrorKind::UnexpectedEof.into());
            }
            filled += n;
        }
        Ok(())
    }
}

#[async_trait]
impl ZipIO for LocalZipIO {
    async fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<()> {
        // spawn_blocking works on both current-thread and multi-thread runtimes,
        // unlike block_in_place which panics on a current-thread runtime
        let file = Arc::clone(&self.file);
        let len = buf.len();
        let data = tokio::task::spawn_blocking(move || {
            let mut data = vec![0u8; len];
            read_exact_at(&file, &mut data, offset).map(|_| data)
        })
        .await
        .map_err(std::io::Error::other)??;

        buf.copy_from_slice(&data);
        Ok(())
    }

    async fn size(&self) -> Result<u64> {
        Ok(self.size)
    }
}
