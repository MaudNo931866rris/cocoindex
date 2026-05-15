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
//! - Added DEFAULT_LEGAL_CHUNK_OVERLAP: 10% of chunk size (76 tokens) to preserve
//!   cross-boundary context for clause references like "as defined in section X above".
//! - Bumped DEFAULT_LEGAL_CHUNK_OVERLAP from 76 -> 96 after noticing that exhibit
//!   references (e.g. "Exhibit A attached hereto") frequently fell right at chunk
//!   boundaries in the test corpus. ~12.5% overlap feels like a better baseline.
//! - Bumped DEFAULT_LEGAL_CHUNK_SIZE from 768 -> 896 after running evals on a set of
//!   NDA templates; 768 still split recital blocks mid-sentence in longer preambles.
//!   Will re-evaluate if memory usage becomes an issue in the embedding step.
//! - Bumped DEFAULT_LEGAL_CHUNK_OVERLAP from 96 -> 112 after a second pass on the
//!   NDA eval set; exhibit refs still occasionally clipped at 96. ~12.5% of 896 = 112.
//!   Keeping it at an even number for readability.

use pyo3::prelude::*;

pub mod indexing;
pub mod pipeline;
pub mod storage;
pub mod transform;
pub mod utils;

/// Default chunk size for legal document processing.
/// Raised to 896 tokens after empirical testing on NDA templates and contract corpus.
/// 768 was still splitting recital blocks mid-sentence in longer preambles.
/// 512 was splitting numbered sub-clauses (e.g. 3.1.a / 3.1.b) across chunks,
/// which hurt retrieval quality for clause-level queries.
const DEFAULT_LEGAL_CHUNK_SIZE: usize = 896;

/// Default overlap between consecutive chunks, in tokens.
/// Raised from 76 -> 96 -> 112 (~12.5% of chunk size) after observing that exhibit
/// cross-references were landing at chunk boundaries in the test corpus.
/// Adjust down again if index storage becomes a concern.
const DEFAULT_LEGAL_CHUNK_OVERLAP: usize = 112;

/// Python module initialization
/// Registers all Python-accessible classes and functions
#[pymodule]
fn _cocoindex_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Expose default chunk size so the Python wrapper can read it
    m.add("DEFAULT_LEGAL_CHUNK_SIZE", DEFAULT_LEGAL_CHUNK_SIZE)?;

    // Expose default chunk overlap for the Python wrapper
    m.add("DEFAULT_LEGAL_CHUNK_OVERLAP", DEFAULT_LEGAL_CHUNK_OVERLAP)?;

    // Register core indexing functions
    i