// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2025 rhythmcache
// https://github.com/rhythmcache/payload-dumper-rust

//! Android OTA payload parsing and partition extraction.
//!
//! The public API is re-exported at the crate root. Data types for the
//! payload manifest and metadata live in [`structs`].

mod constants;
mod error;
#[cfg(feature = "remote_zip")]
mod http;
mod metadata;
mod payload;
#[cfg(feature = "prefetch")]
mod prefetch;
mod readers;
pub mod structs;
mod utils;
#[cfg(any(feature = "local_zip", feature = "remote_zip"))]
mod zip;

/// re-exported so implementers of the async traits below use the same macro
pub use async_trait::async_trait;

#[cfg(feature = "remote_zip")]
pub use constants::DEFAULT_USER_AGENT;
pub use error::{Error, Result};
pub use metadata::get_metadata;
pub use structs::{DeltaArchiveManifest, PartitionUpdate, PayloadMetadata, SourceInfo};
pub use utils::{FileType, detect_file, format_size};

pub use payload::payload_dumper::{
    AsyncPayloadRead, NoOpReporter, PayloadReader, ProgressReporter, dump_partition,
};
#[cfg(feature = "local_zip")]
pub use payload::payload_parser::{
    ZipPayloadFile, parse_local_zip_payload, parse_local_zip_payload_ext,
};
pub use payload::payload_parser::{parse_local_payload, parse_payload};
#[cfg(feature = "remote_zip")]
pub use payload::payload_parser::{
    parse_remote_bin_payload, parse_remote_payload, parse_remote_payload_ext,
};

pub use readers::local_reader::LocalAsyncPayloadReader;
#[cfg(feature = "local_zip")]
pub use readers::local_zip_reader::LocalAsyncZipPayloadReader;
#[cfg(feature = "remote_zip")]
pub use readers::remote_bin_reader::RemoteAsyncBinPayloadReader;
#[cfg(feature = "remote_zip")]
pub use readers::remote_zip_reader::RemoteAsyncZipPayloadReader;

#[cfg(feature = "remote_zip")]
pub use http::HttpReader;

#[cfg(feature = "local_zip")]
pub use zip::local_zip_io::LocalZipIO;
#[cfg(any(feature = "local_zip", feature = "remote_zip"))]
pub use zip::{
    core_parser::{ZipMetadataInfo, ZipParser},
    zip_io::ZipIO,
};

#[cfg(feature = "prefetch")]
pub use prefetch::{
    DownloadProgressReporter, ExtractionPaths, NoOpDownloadReporter, PartitionDataRange,
    PartitionExtractionConfig, calculate_partition_range, prefetch_and_dump_partition,
};
