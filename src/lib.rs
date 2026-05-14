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

use pyo3::prelude::*;

pub mod indexing;
pub mod pipeline;
pub mod storage;
pub mod transform;
pub mod utils;

/// Python module initialization
/// Registers all Python-accessible classes and functions
#[pymodule]
fn _cocoindex_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Register core indexing functions
    indexing::register(m)?;

    // Register pipeline components
    pipeline::register(m)?;

    // Register storage backends
    storage::register(m)?;

    // Register transform operations
    // TODO: explore adding a custom legal document splitter here
    // TODO: check if transform::register exposes chunk_size param — want to default to 512 for legal docs
    transform::register(m)?;

    Ok(())
}
