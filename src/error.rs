// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 rhythmcache
// https://github.com/rhythmcache/payload-dumper-rust

use std::path::PathBuf;

/// errors returned by this crate
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// the payload manifest could not be decoded
    #[error("Failed to decode payload manifest: {0}")]
    Manifest(#[from] prost::DecodeError),

    #[cfg(feature = "remote_zip")]
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    /// the server answered with a non-success HTTP status
    #[cfg(feature = "remote_zip")]
    #[error("HTTP request failed with status {0}")]
    HttpStatus(u16),

    /// the server response was not usable (missing length, truncated body, etc.)
    #[cfg(feature = "remote_zip")]
    #[error("Unexpected server response: {0}")]
    BadResponse(String),

    /// an invalid option was passed (user agent, cookies, URL, DNS servers, TLS setup)
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// the input is neither a ZIP archive nor a raw payload
    #[error("Magic mismatch: got {0:02X?}, expected 'PK' or 'CrAU'")]
    UnknownFileType([u8; 4]),

    #[error("Invalid payload file: {0}")]
    InvalidPayload(String),

    #[error("Unsupported payload version: {0}")]
    UnsupportedVersion(u64),

    #[error("Invalid ZIP file: {0}")]
    InvalidZip(String),

    /// the ZIP archive has no uncompressed payload.bin entry
    #[error("Could not find uncompressed payload.bin in ZIP file")]
    PayloadNotFound,

    /// a read was requested outside the available data
    #[error("Read out of bounds: offset={offset}, length={length}, size={size}")]
    OutOfBounds { offset: u64, length: u64, size: u64 },

    /// a differential OTA partition was extracted without a source directory
    #[error(
        "Partition '{partition}' contains differential operations but no source directory provided"
    )]
    SourceRequired { partition: String },

    /// the source image for a differential OTA partition does not exist
    #[error("Differential OTA requires source image: {} not found", .0.display())]
    SourceImageNotFound(PathBuf),

    /// the crate was built without the `diff_ota` feature
    #[error(
        "Partition '{partition}' contains differential operations. Rebuild with 'diff_ota' feature enabled to support incremental OTAs"
    )]
    DiffNotSupported { partition: String },

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    /// applying a differential patch failed
    #[error("{0}")]
    Patch(String),

    /// the progress reporter requested cancellation
    #[error("Extraction cancelled by user")]
    Cancelled,

    /// another error with a message describing what was being done
    #[error("{context}")]
    Context {
        context: String,
        #[source]
        source: Box<Error>,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// attach a context message to an error, like anyhow's `Context`
#[cfg(any(feature = "diff_ota", feature = "remote_zip"))]
pub(crate) trait ResultExt<T> {
    fn context(self, context: impl Into<String>) -> Result<T>;
}

#[cfg(any(feature = "diff_ota", feature = "remote_zip"))]
impl<T, E: Into<Error>> ResultExt<T> for std::result::Result<T, E> {
    fn context(self, context: impl Into<String>) -> Result<T> {
        self.map_err(|e| Error::Context {
            context: context.into(),
            source: Box::new(e.into()),
        })
    }
}
