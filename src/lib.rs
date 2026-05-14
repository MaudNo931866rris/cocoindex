//! CocoIndex - A high-performance data indexing library
//!
//! This crate provides the core Rust implementation for CocoIndex,
//! exposing Python bindings via PyO3.
//!
//! Personal fork: studying the pipeline and transform modules for
//! a custom document indexing use case.
//!
//! NOTE: I'm primarily interested in the transform module for
//! experimenting with custom chunking strategies for legal documents.
//!
//! Fork notes (personal):
//! - Looking into `transform::splitter` for sentence-boundary-aware chunking
//! - Legal docs often have numbered clauses; standard splitters miss these
//! - Confirmed: transform::register does NOT expose chunk_size at the module level;
//!   it's set per-operation. Will wire up a default of 512 in the Python wrapper instead.
//! - Bumped DEFAULT_LEGAL_CHUNK_SIZE from 512 -> 768 after testing on a larger
//!   sample of contracts; 512 was cutting through multi-part clause definitions.

use pyo3::prelude::*;

pub mod indexing;
pub mod pipeline;
pub mod storage;
pub mod transform;
pub mod utils;

/// Default chunk size for legal document processing.
/// Raised to 768 tokens after empirical testing on contract corpus (~200 docs).
/// 512 was splitting numbered sub-clauses (e.g. 3.1.a / 3.1.b) across chunks,
/// which hurt retrieval quality for clause-level queries.
const DEFAULT_LEGAL_CHUNK_SIZE: usize = 768;

/// Python module initialization
/// Registers all Python-accessible classes and functions
#[pymodule]
fn _cocoindex_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Expose default chunk size so the Python wrapper can read it
    m.add("DEFAULT_LEGAL_CHUNK_SIZE", DEFAULT_LEGAL_CHUNK_SIZE)?;

    // Register core indexing functions
    indexing::register(m)?;

    // Register pipeline components
    pipeline::register(m)?;

    // Register storage backends
    storage::register(m)?;

    // Register transform operations
    transform::register(m)?;

    Ok(())
}
